#!/usr/bin/env python3
"""Scrape parallèle de soshage.com/dqm2sp/ vers un cache local (avec reprise).

Usage : python3 scripts/fetch_all.py
Produit : cache/soshage/*.json
"""
import json
import os
import sys
import time
from concurrent.futures import ThreadPoolExecutor, as_completed

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from soshage import Client

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CACHE = os.path.join(ROOT, "cache", "soshage")
os.makedirs(CACHE, exist_ok=True)
N_THREADS = 6


def cache_path(path):
    safe = path.strip("/").replace("/", "_").replace("?", "@").replace("&", "-")
    return os.path.join(CACHE, safe + ".json")


def cached(path):
    """Récupère un payload (cache disque prioritaire). Thread-safe (1 fichier = 1 chemin)."""
    fn = cache_path(path)
    if os.path.exists(fn):
        return True
    c = Client(rate=0.4)  # par thread ; 6 threads ≈ 15 req/s max
    for attempt in range(5):
        try:
            payloads = c.payload(path)
            tmp = fn + f".tmp{os.getpid()}_{threading.get_ident()}"
            with open(tmp, "w") as f:
                json.dump(payloads, f, ensure_ascii=False)
            os.replace(tmp, fn)
            return True
        except Exception as e:
            if attempt == 4:
                print(f"ÉCHEC {path}: {e}", flush=True)
                return False
            time.sleep(1.5 * (attempt + 1))


import threading


def find_key(payloads, key):
    for p in payloads:
        if isinstance(p, dict) and key in p:
            return p[key]
    return None


def main():
    t0 = time.time()

    # 1. Listes globales (séquentiel)
    c = Client(rate=0.1)
    lists = {}
    for ep in ["monster", "skill", "item", "trait", "ability", "egg"]:
        fn = cache_path(f"/{ep}")
        if not os.path.exists(fn):
            payloads = c.payload(f"/{ep}")
            with open(fn, "w") as f:
                json.dump(payloads, f, ensure_ascii=False)
        lists[ep] = json.load(open(fn))
        print(f"[list] {ep} OK", flush=True)

    monsters = find_key(lists["monster"], "monsters")
    skills = find_key(lists["skill"], "skills")
    assert monsters and skills
    ids = [m["id"] for m in monsters]
    print(f"{len(ids)} monstres, {len(skills)} skills", flush=True)

    # 2. Fiches monstres en parallèle
    todo = [f"/monster/{i}" for i in ids if not os.path.exists(cache_path(f"/monster/{i}"))]
    print(f"[monster] {len(ids) - len(todo)}/{len(ids)} déjà en cache, {len(todo)} à faire", flush=True)
    done = 0
    with ThreadPoolExecutor(max_workers=N_THREADS) as ex:
        futs = {ex.submit(cached, p): p for p in todo}
        for fut in as_completed(futs):
            done += 1
            if done % 100 == 0:
                print(f"[monster] {done}/{len(todo)} ({time.time()-t0:.0f}s)", flush=True)

    # 3. Fiches skills en parallèle
    todo = [f"/skill/{s['id']}" for s in skills if not os.path.exists(cache_path(f"/skill/{s['id']}"))]
    print(f"[skill] {len(todo)} à faire", flush=True)
    done = 0
    with ThreadPoolExecutor(max_workers=N_THREADS) as ex:
        futs = {ex.submit(cached, p): p for p in todo}
        for fut in as_completed(futs):
            done += 1
            if done % 100 == 0:
                print(f"[skill] {done}/{len(todo)} ({time.time()-t0:.0f}s)", flush=True)

    n_mon = len([f for f in os.listdir(CACHE) if f.startswith("monster_")])
    n_sk = len([f for f in os.listdir(CACHE) if f.startswith("skill_")])
    print(f"TERMINÉ en {time.time()-t0:.0f}s — {n_mon} monstres, {n_sk} skills en cache", flush=True)


if __name__ == "__main__":
    main()
