#!/usr/bin/env python3
"""Télécharge les sprites webp de img.kusoge.xyz (icônes + monstres).

Usage : python3 scripts/fetch_sprites.py   (après fetch_all.py)
Produit : cache/sprites/
"""
import json
import os
import sys
import time
import urllib.error
import urllib.request

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
IMG = "https://img.kusoge.xyz/dqm2sp"
OUT = os.path.join(ROOT, "cache", "sprites")
CACHE = os.path.join(ROOT, "cache", "soshage")
UA = {"User-Agent": "Mozilla/5.0 (X11; Linux x86_64) dqm-rs-scraper/0.1"}


def fetch(url, dest, rate=0.02):
    if os.path.exists(dest) and os.path.getsize(dest) > 0:
        return True

    retryable = {408, 425, 429, 500, 502, 503, 504}
    max_attempts = 6
    req = urllib.request.Request(url, headers=UA)

    for attempt in range(1, max_attempts + 1):
        if rate:
            time.sleep(rate)
        try:
            with urllib.request.urlopen(req, timeout=30) as response:
                data = response.read()
            if not data:
                raise urllib.error.URLError(f"Réponse vide pour {url}")

            tmp = f"{dest}.tmp{os.getpid()}"
            try:
                with open(tmp, "wb") as stream:
                    stream.write(data)
                    stream.flush()
                    os.fsync(stream.fileno())
                os.replace(tmp, dest)
            finally:
                if os.path.exists(tmp):
                    os.remove(tmp)
            return True
        except urllib.error.HTTPError as exc:
            if exc.code == 404:
                return False
            if exc.code not in retryable or attempt == max_attempts:
                raise
            retry_after = exc.headers.get("Retry-After")
            delay = min(45.0, 2.0 ** attempt)
            try:
                if retry_after:
                    delay = max(delay, min(120.0, float(retry_after)))
            except (TypeError, ValueError):
                pass
            print(
                f"[http] {url}: HTTP {exc.code}, tentative {attempt}/{max_attempts}; "
                f"nouvel essai dans {delay:.0f}s",
                flush=True,
            )
        except (urllib.error.URLError, TimeoutError, ConnectionError) as exc:
            if attempt == max_attempts:
                raise
            delay = min(45.0, 2.0 ** attempt)
            print(
                f"[http] {url}: {exc}, tentative {attempt}/{max_attempts}; "
                f"nouvel essai dans {delay:.0f}s",
                flush=True,
            )
        time.sleep(delay)

    raise RuntimeError(f"Impossible de télécharger après {max_attempts} tentatives: {url}")


def main():
    os.makedirs(f"{OUT}/icon", exist_ok=True)
    os.makedirs(f"{OUT}/monster", exist_ok=True)

    # Icônes UI : familles (1..12), rangs (1..12), tailles (1..4), divers
    icon_names = ["icon_monster_frame"]
    for i in range(1, 13):
        icon_names.append(f"icon_family_{i}")
    for i in range(1, 13):
        icon_names.append(f"rank_{i}")
    for i in range(1, 5):
        icon_names.append(f"icon_size_{i}")
    ok = []
    for name in icon_names:
        if fetch(f"{IMG}/icon/{name}.webp", f"{OUT}/icon/{name}.webp"):
            ok.append(name)
    print(f"[icons] {len(ok)} ok", flush=True)

    # Sprites monstres : seulement ceux avec sprite_exists
    monsters = None
    p = os.path.join(CACHE, "monster.json")
    if os.path.exists(p):
        doc = json.load(open(p))
        for pl in doc:
            if isinstance(pl, dict) and "monsters" in pl:
                monsters = pl["monsters"]
    assert monsters, "lance d'abord fetch_all.py (liste monstres)"
    targets = [m for m in monsters if m.get("sprite_exists")]
    print(f"[sprites] {len(targets)} monstres avec sprite", flush=True)

    n_ok = 0
    for i, m in enumerate(targets):
        mid = m["id"]
        if fetch(f"{IMG}/monster/icon_monster_{mid:04d}.webp", f"{OUT}/monster/{mid:04d}.webp"):
            n_ok += 1
        if i % 100 == 0:
            print(f"[sprites] {i}/{len(targets)}", flush=True)

    print(f"[sprites] TERMINÉ: {n_ok}/{len(targets)} sprites", flush=True)


if __name__ == "__main__":
    main()
