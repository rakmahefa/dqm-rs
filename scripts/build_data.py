#!/usr/bin/env python3
"""Transforme le cache scrapé en JSON propres pour l'app Rust.

Usage : python3 scripts/build_data.py   (après fetch_all.py + fetch_sprites.py)
Produit : data/*.json + assets/sprites.json
"""
import base64
import datetime
import json
import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CACHE = os.path.join(ROOT, "cache", "soshage")
SPRITES = os.path.join(ROOT, "cache", "sprites")
OUT = os.path.join(ROOT, "data")
ASSETS = os.path.join(ROOT, "assets")
os.makedirs(OUT, exist_ok=True)
os.makedirs(ASSETS, exist_ok=True)


def load(name):
    return json.load(open(os.path.join(CACHE, name)))


def find_key(payloads, key):
    for p in payloads:
        if isinstance(p, dict) and key in p:
            return p[key]
    return None


def dump(name, obj):
    fn = os.path.join(OUT, name)
    with open(fn, "w") as f:
        json.dump(obj, f, ensure_ascii=False, separators=(",", ":"))
    print(f"  {name}: {os.path.getsize(fn)/1024:.0f} Ko", flush=True)


def main():
    t0 = datetime.datetime.now(datetime.timezone.utc)

    # ---------- Listes ----------
    monsters_list = find_key(load("monster.json"), "monsters")
    skills_list = find_key(load("skill.json"), "skills")
    items = find_key(load("item.json"), "items")
    traits = find_key(load("trait.json"), "traits")
    abilities = find_key(load("ability.json"), "abilities")
    eggs_raw = find_key(load("egg.json"), "eggs")
    item_by_id = {i["id"]: i for i in items}

    # ---------- Œufs ----------
    eggs = []
    egg_of_monster = {}  # id -> {kind: (num, den)}
    for e in eggs_raw:
        mons = []
        for m in e["monsters"]:
            sources = []
            for s in m.get("sources") or []:
                src = {"weight": s.get("family_group_roll_weight") or s.get("tier_roll_weight")}
                if s.get("families"):
                    src["families"] = [f["name"] for f in s["families"]]
                    src["group"] = s.get("group_id")
                else:
                    src["tier"] = s.get("tier_id")
                    src["range"] = [s.get("roll_start"), s.get("roll_end")]
                sources.append(src)
            mons.append({
                "id": m["id"],
                "num": m.get("initial_probability_num"),
                "den": m.get("initial_probability_den"),
                "sources": sources,
            })
            d = egg_of_monster.setdefault(m["id"], {})
            d[e["kind"]] = (m.get("initial_probability_num"), m.get("initial_probability_den"))
        eggs.append({
            "kind": e["kind"],
            "name": item_by_id[e["item_id"]]["name"],
            "item_id": e["item_id"],
            "buy": item_by_id[e["item_id"]].get("buy_price"),
            "name_parts": e.get("name_parts"),
            "monsters": mons,
        })
    print(f"Œufs: {[(e['name'], len(e['monsters'])) for e in eggs]}", flush=True)
    dump("eggs.json", eggs)

    # ---------- Monstres ----------
    monsters = []
    recipes = {}  # (result, tuple(parents)) -> type
    stats = {"no_detail": 0}
    for m in monsters_list:
        mid = m["id"]
        fn = os.path.join(CACHE, f"monster_{mid}.json")
        detail = None
        used_as_parent = []
        if os.path.exists(fn):
            doc = json.load(open(fn))
            detail = find_key(doc, "monster")
            used_as_parent = find_key(doc, "usedAsParent") or []
        if detail is None:
            stats["no_detail"] += 1
            detail = m  # fallback : données de liste

        # slots du skill natif
        slots = []
        for s in detail.get("skill_slots") or []:
            slot = {"sp": s.get("sp_threshold")}
            aa = s.get("ability_action")
            if aa:
                slot["ab"] = aa["id"]
                slot["ab_name"] = aa["name"]
            st = s.get("stat")
            if st:
                slot["auto"] = {"id": st["id"], "name": st["name"], "desc": st.get("desc", "")}
            slots.append(slot)

        drops = [{
            "item": d["item"]["id"],
            "item_name": d["item"]["name"],
            "den": d.get("denominator"),
            "code": d.get("rate_code"),
        } for d in detail.get("drops") or []]

        trs = [{
            "t": t["trait"]["id"],
            "name": t["trait"]["name"],
            "desc": t["trait"].get("desc", ""),
            "size": t.get("required_size_class"),
            "body": bool(t.get("is_body_size_trait")),
        } for t in detail.get("traits") or []]

        locs = [{
            "map": l["map"]["name"] if isinstance(l.get("map"), dict) else str(l.get("map")),
            "src": l.get("source"),
        } for l in detail.get("locations") or []]

        eg = egg_of_monster.get(mid, {})

        rec = {
            "id": mid,
            "name": m["name"],
            "family": m["family"]["name"],
            "family_id": m["family"]["id"],
            "rank": m["base_rank"],
            "size": m["size_class"],
            "lib": m.get("library_position"),
            "sprite": bool(m.get("sprite_exists")),
            "max": [m["max_stats"]["hp"], m["max_stats"]["mp"], m["max_stats"]["atk"],
                    m["max_stats"]["def"], m["max_stats"]["agi"], m["max_stats"]["wis"]],
            "growth": detail.get("growth") or {},
            "skill": (detail.get("skill") or {}).get("id") if detail.get("skill") else None,
            "skill_name": (detail.get("skill") or {}).get("name") if detail.get("skill") else None,
            "sp0": detail.get("starting_sp"),
            "exp50": detail.get("exp_to_level_50"),
            "exp100": detail.get("exp_to_level_100"),
            "holy": bool(detail.get("is_holy_beast")),
            "wild": bool(locs),
            "egg_kinds": sorted(eg.keys()),
            "slots": slots,
            "drops": drops,
            "traits": trs,
            "locations": locs,
            "generic_breeding": [
                {
                    "f1": g["parent1_family"]["name"],
                    "f2": g["parent2_family"]["name"],
                    "rank": g.get("required_rank"),
                    "lib": g.get("required_library_position"),
                    "label": g.get("source_label"),
                } for g in detail.get("generic_breeding") or []
            ],
        }
        monsters.append(rec)

        # recettes : ce monstre comme résultat
        for b in detail.get("breeding") or []:
            parents = tuple(sorted(p["id"] for p in b.get("parents") or []))
            if len(parents) >= 2:
                recipes[(mid, parents)] = b.get("type") or ""
        # recettes : ce monstre comme parent (croisement de vérification)
        for u in used_as_parent:
            parents = tuple(sorted(p["id"] for p in u.get("parents") or []))
            if len(parents) >= 2:
                key = (u["result"]["id"], parents)
                if key not in recipes:
                    recipes[key] = u.get("source") or ""

    monsters.sort(key=lambda x: (x.get("lib") or 9999, x["id"]))
    print(f"Monstres: {len(monsters)} (sans détail: {stats['no_detail']})", flush=True)
    dump("monsters.json", monsters)

    # ---------- Recettes ----------
    recipes_out = [{"r": r, "p": list(p), "t": t} for (r, p), t in sorted(recipes.items())]
    n4 = sum(1 for x in recipes_out if len(x["p"]) == 4)
    print(f"Recettes: {len(recipes_out)} (dont {n4} à 4 parents)", flush=True)
    dump("recipes.json", recipes_out)

    # ---------- Skills ----------
    skills = []
    for s in skills_list:
        sid = s["id"]
        fn = os.path.join(CACHE, f"skill_{sid}.json")
        d = None
        if os.path.exists(fn):
            d = find_key(json.load(open(fn)), "skill")
        if d is None:
            d = {"id": sid, "name": s["name"]}
        slots = []
        for sl in d.get("slots") or []:
            slot = {"sp": sl.get("sp_threshold")}
            aa = sl.get("ability_action")
            if aa:
                slot["ab"] = aa["id"]
                slot["ab_name"] = aa["name"]
            st = sl.get("stat")
            if st:
                slot["auto"] = {"id": st["id"], "name": st["name"], "desc": st.get("desc", "")}
            slots.append(slot)
        skills.append({
            "id": sid,
            "name": s["name"],
            "evo_from": [
                {"skill": r["skill"]["id"], "name": r["skill"]["name"], "sp": r["sp_cost"]}
                for grp in d.get("evolved_from") or [] for r in grp.get("requirements") or []
            ],
            "special": [
                {"to": x["evolves_to"]["id"], "to_name": x["evolves_to"]["name"],
                 "req": [{"skill": r["skill"]["id"], "name": r["skill"]["name"], "sp": r["sp_cost"]}
                         for r in x.get("requirements") or []]}
                for x in d.get("special_evolutions") or []
            ],
            "evo_to": (d.get("evolves_to") or {}).get("id") if d.get("evolves_to") else None,
            "evo_to_name": (d.get("evolves_to") or {}).get("name") if d.get("evolves_to") else None,
            "evo_cost": d.get("evo_sp_cost"),
            "slots": slots,
            "monsters": [m["id"] for m in d.get("monsters") or []],
        })
    print(f"Skills: {len(skills)}", flush=True)
    dump("skills.json", skills)

    # ---------- Items ----------
    items_out = [{
        "id": i["id"], "name": i["name"], "desc": i.get("desc", ""),
        "type": i.get("item_type"), "buy": i.get("buy_price"),
        "atk": i.get("attack"), "def": i.get("defense"), "agi": i.get("agility"),
        "wis": i.get("wisdom"), "hp": i.get("max_hp"), "mp": i.get("max_mp"),
        "break_p": i.get("break_probability"),
    } for i in items]
    dump("items.json", items_out)

    # ---------- Traits ----------
    dump("traits.json", [{"id": t["id"], "name": t["name"], "desc": t.get("desc", "")} for t in traits])

    # ---------- Abilities (allégées) ----------
    abilities_out = []
    for a in abilities:
        abilities_out.append({
            "id": a["id"], "name": a["name"], "desc": a.get("desc", ""),
            "mp": a.get("mp"), "mpf": a.get("mp_is_formula"),
            "cats": (a.get("targeting") or {}).get("categories") or [],
            "vals": a.get("values"),
            "res": ((a.get("resource") or {}).get("name")),
            "tol1": ((a.get("tolerance") or {}).get("primary") or {}).get("name"),
            "tol2": ((a.get("tolerance") or {}).get("secondary") or {}).get("name"),
            "status": (((a.get("status") or {}).get("status")) or {}).get("name"),
        })
    dump("abilities.json", abilities_out)

    # ---------- Familles (dérivées) ----------
    fams = {}
    for m in monsters:
        f = fams.setdefault(m["family_id"], {"id": m["family_id"], "name": m["family"], "count": 0})
        f["count"] += 1
    dump("families.json", sorted(fams.values(), key=lambda x: x["id"]))

    # ---------- Sprites → assets/sprites.json (base64) ----------
    sprites = {}
    icon_dir = os.path.join(SPRITES, "icon")
    if os.path.isdir(icon_dir):
        for f in os.listdir(icon_dir):
            name = f.removesuffix(".webp")
            if name.startswith("icon_family_"):
                key = f"fam{name.removeprefix('icon_family_')}"
            elif name.startswith("rank_"):
                key = f"rank{name.removeprefix('rank_')}"
            elif name.startswith("icon_size_"):
                key = f"size{name.removeprefix('icon_size_')}"
            elif name == "icon_monster_frame":
                key = "frame"
            else:
                key = name
            with open(os.path.join(icon_dir, f), "rb") as fh:
                sprites[key] = base64.b64encode(fh.read()).decode()
    mon_dir = os.path.join(SPRITES, "monster")
    if os.path.isdir(mon_dir):
        for f in os.listdir(mon_dir):
            raw = f.removesuffix(".webp").removeprefix("icon_monster_")
            mid = str(int(raw))  # supprime le padding (0001 → 1)
            with open(os.path.join(mon_dir, f), "rb") as fh:
                sprites[mid] = base64.b64encode(fh.read()).decode()
    fn = os.path.join(ASSETS, "sprites.json")
    with open(fn, "w") as f:
        json.dump(sprites, f, separators=(",", ":"))
    print(f"  sprites.json: {len(sprites)} sprites, {os.path.getsize(fn)/1024/1024:.1f} Mo", flush=True)

    # ---------- Meta ----------
    meta = {
        "source": "https://soshage.com/dqm2sp/",
        "source_name": "soshage.com DQM2 SP wiki (fan site)",
        "scraped_at": t0.strftime("%Y-%m-%dT%H:%M:%SZ"),
        "game": "Dragon Quest Monsters 2 SP (fan ROM hack of DQM2: Iru & Luca's Marvelous Mysterious Keys)",
        "note": "Unofficial fan-made companion app. Game data © Square Enix. Not affiliated.",
        "counts": {
            "monsters": len(monsters),
            "recipes": len(recipes_out),
            "skills": len(skills),
            "items": len(items_out),
            "traits": len(traits),
            "abilities": len(abilities_out),
            "eggs": len(eggs),
            "sprites": len(sprites),
        },
    }
    dump("meta.json", meta)

    obtainable = sum(1 for m in monsters if m["wild"] or m["egg_kinds"])
    print(f"\nObtenables sans synthèse: {obtainable}/{len(monsters)}", flush=True)


if __name__ == "__main__":
    main()
