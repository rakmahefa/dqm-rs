#!/usr/bin/env python3
"""Valide les données de jeu embarquées (commitées) — utilisé par la CI et en local.

Échoue avec un message explicite si des fichiers manquent ou sont invalides.
Les données sont régénérées par scripts/fetch_all.py + fetch_sprites.py +
build_data.py (la source soshage.com/dqm2sp doit être en ligne) puis commitées.
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

REQUIRED = (
    "monsters", "recipes", "skills", "items", "traits",
    "abilities", "eggs", "families", "meta",
)

# Seuils de cohérence (valeurs attendues : ~903 monstres, ~425 skills,
# ~468 items, ~534 traits, ~306 abilities) — larges pour éviter les faux positifs.
MIN_COUNTS = {
    "monsters": 700,
    "recipes": 500,
    "skills": 300,
    "items": 300,
    "traits": 400,
    "abilities": 200,
    "eggs": 1,
    "families": 5,
}
MIN_SPRITES = 400


def main():
    problems = []
    counts = {}

    for name in REQUIRED:
        path = ROOT / "data" / f"{name}.json"
        if not path.is_file() or path.stat().st_size == 0:
            problems.append(f"missing or empty: {path}")
            continue
        try:
            with path.open(encoding="utf-8") as stream:
                data = json.load(stream)
        except ValueError as exc:
            problems.append(f"invalid JSON: {path} ({exc})")
            continue
        if not isinstance(data, list):
            problems.append(f"{path}: expected a JSON list")
            continue
        counts[name] = len(data)
        minimum = MIN_COUNTS.get(name)
        if minimum and len(data) < minimum:
            problems.append(f"{path}: {len(data)} entries < minimum {minimum}")

    sprites_path = ROOT / "assets" / "sprites.json"
    if not sprites_path.is_file() or sprites_path.stat().st_size == 0:
        problems.append(f"missing or empty: {sprites_path}")
    else:
        try:
            with sprites_path.open(encoding="utf-8") as stream:
                sprites = json.load(stream)
            if not isinstance(sprites, dict) or len(sprites) < MIN_SPRITES:
                problems.append(
                    f"{sprites_path}: {len(sprites) if isinstance(sprites, dict) else 0} sprites < minimum {MIN_SPRITES}"
                )
        except ValueError as exc:
            problems.append(f"invalid JSON: {sprites_path} ({exc})")

    if problems:
        print("Game data validation FAILED:")
        for p in problems:
            print(f"  - {p}")
        print()
        print(
            "The game data must be regenerated and committed. Requires the\n"
            "source wiki to be reachable, then:\n"
            "  python3 scripts/fetch_all.py\n"
            "  python3 scripts/fetch_sprites.py\n"
            "  python3 scripts/build_data.py\n"
            "  python3 scripts/verify_data.py\n"
            "  git add data assets/sprites.json && git commit -m \"data: refresh\""
        )
        sys.exit(1)

    print("Game data OK:", ", ".join(f"{n}={counts.get(n)}" for n in REQUIRED if n in counts))


if __name__ == "__main__":
    main()
