#!/usr/bin/env python3
"""Module partagé : client SvelteKit/devalue pour soshage.com/dqm2sp/."""
import gzip
import json
import time
import urllib.request
import urllib.parse
import zlib

BASE = "https://soshage.com/dqm2sp"
UA = "Mozilla/5.0 (X11; Linux x86_64) dqm-rs-scraper/0.1 (github.com/rakmahefa/dqm-rs)"


def unflatten(flat):
    """Port Python de devalue.unflatten (v5)."""
    values = flat
    hydrated = [False] * len(values)

    def hydrate(index):
        if index == -1:
            return None
        if index in (-2, -3, -4, -5):
            return float("nan")
        if index == -6:
            return None
        if hydrated[index] is not False:
            return hydrated[index]
        value = values[index]
        if not isinstance(value, (dict, list)):
            hydrated[index] = value
            return value
        if isinstance(value, list):
            if len(value) > 0 and isinstance(value[0], str):
                t = value[0]
                if t == "Set":
                    out = []
                    hydrated[index] = out
                    for n in value[1:]:
                        out.append(hydrate(n))
                    return out
                if t == "Map":
                    out = {}
                    hydrated[index] = out
                    for i in range(1, len(value) - 1, 2):
                        out[hydrate(value[i])] = hydrate(value[i + 1])
                    return out
                if t in ("Date", "Object", "BigInt"):
                    hydrated[index] = value[1]
                    return value[1]
                raise ValueError(f"Type devalue inconnu: {t}")
            out = []
            hydrated[index] = out
            for n in value:
                out.append(hydrate(n))
            return out
        out = {}
        hydrated[index] = out
        for k, n in value.items():
            out[k] = hydrate(n)
        return out

    return hydrate(0) if values else None


class Client:
    def __init__(self, rate=0.12):
        self.rate = rate
        self._last = 0.0
        self.n_requests = 0
        self.bytes_wire = 0

    def _throttle(self):
        wait = self.rate - (time.time() - self._last)
        if wait > 0:
            time.sleep(wait)
        self._last = time.time()

    def raw(self, path, accept_gzip=True):
        self._throttle()
        url = path if path.startswith("http") else f"{BASE}{path}"
        headers = {"User-Agent": UA}
        if accept_gzip:
            headers["Accept-Encoding"] = "gzip"
        req = urllib.request.Request(url, headers=headers)
        with urllib.request.urlopen(req, timeout=60) as r:
            data = r.read()
            if r.headers.get("Content-Encoding") == "gzip" or data[:2] == b"\x1f\x8b":
                data = gzip.decompress(data)
            self.n_requests += 1
            self.bytes_wire += len(data)
            return data

    def payload(self, path):
        """GET {path}/__data.json → liste des payloads dévalués."""
        sep = "" if path.endswith("/") else "/"
        if "?" in path:
            base, qs = path.split("?", 1)
            data = self.raw(f"{urllib.parse.quote(base)}{sep}__data.json?{qs}")
        else:
            data = self.raw(f"{urllib.parse.quote(path)}{sep}__data.json")
        doc = json.loads(data.decode("utf-8"))
        out = []
        for node in doc.get("nodes", []):
            if node and isinstance(node, dict) and "data" in node:
                out.append(unflatten(node["data"]))
        return out


if __name__ == "__main__":
    import sys

    c = Client()
    # Petit test
    pls = c.payload("/monster/151")
    for p in pls:
        if isinstance(p, dict) and "monster" in p:
            m = p["monster"]
            print(f"{m['name']} (id {m['id']}) — breeding={len(p.get('breeding') or m.get('breeding') or [])}")
            print("  clés payload:", list(p.keys()))
            br = p.get("usedAsParent") or []
            print(f"  usedAsParent: {len(br)} recettes")
            if br:
                print("   ex:", json.dumps(br[0], ensure_ascii=False)[:200])
            # structure breeding (résultat)
            for key in ("breeding", "direct_breeding", "generic_breeding"):
                v = p.get(key)
                if v:
                    print(f"  {key}: {len(v)} — ex: {json.dumps(v[0], ensure_ascii=False)[:260]}")
    print(f"requêtes={c.n_requests} octets={c.bytes_wire}")
