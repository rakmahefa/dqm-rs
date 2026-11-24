# dqm-rs — DQM2 SP Companion

A **100% offline** companion app for **Dragon Quest Monsters 2 SP**, built with
[Rust](https://rust-lang.org) + [Dioxus](https://dioxuslabs.com).

All game data (monsters, synthesis recipes, skills, items, traits, abilities, egg
pools and sprites) is **embedded in the binary** — no network access, no tracking,
works entirely on-device, on Linux desktop and Android.

## Features

- **Monster encyclopedia** — all 903 monsters with sprites, family, rank, size,
  max stats, growth patterns, EXP curve, native skill + slot progression, traits,
  drops, locations and hatching info.
- **Synthesis** — full interactive recipe tree (explicit recipes, including
  4-parent grand recipes) and an **optimal planner** that computes the shortest
  breeding route from obtainable (wild / egg) monsters.
- **Egg database** — the 5 egg types (White, Silver, Gold, Rainbow, Exciting)
  with exact hatch probabilities and family-group sources, plus an offline
  procedural egg-name generator.
- **Skills** — 425 skill sets with SP slot tables, evolution and fusion chains.
- **Items** — 468 items with prices, bonuses and descriptions.
- **Traits & Abilities** — 534 traits and 306 abilities with targeting, values
  and tolerances.
- Powerful search + filters (family / rank / size / source / sort) everywhere.

> All data is embedded in the binary at build time: the app itself works fully
> offline with zero network access.

## Data source

Scraped (rate-limited, parallel, resumable) from the fan wiki
[soshage.com/dqm2sp](https://soshage.com/dqm2sp/) — see **Data pipeline**.

> Unofficial fan-made tool. Game data and images © Square Enix. Not affiliated
> with or endorsed by Square Enix or soshage.com.

## Build

Requirements: [Rust](https://rustup.rs), Python 3 (for the data pipeline) and, for the
desktop target, `libwebkit2gtk-4.1-dev` + `libgtk-3-dev` (Debian/Ubuntu:
`sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev`).

```sh
# 1. generate the game data (~4 min, one time — see "Data pipeline" below)
python3 scripts/fetch_all.py
python3 scripts/fetch_sprites.py
python3 scripts/build_data.py

# 2. restore the bundle icon (stored as base64 in-repo)
python3 -c "import base64; open('assets/icon.png','wb').write(base64.b64decode(open('assets/icon.png.b64').read().strip()))"

# 3. build & run (desktop)
cargo run --release

# or with the Dioxus CLI (hot reload, bundling)
cargo install dioxus-cli --version 0.7.10 --locked
dx serve
```

### Android

```sh
rustup target add aarch64-linux-android
# Android SDK + NDK + JDK required (ANDROID_NDK_HOME must be set)
dx build --platform android --release --no-default-features --features mobile --target aarch64-linux-android
```

### Features

| Feature  | Purpose                              |
|----------|--------------------------------------|
| `desktop`| Desktop webview build (default)      |
| `mobile` | Android/iOS webview build            |
| `web`    | Browser/WASM build                   |

## CI & artifacts

GitHub Actions (`.github/workflows/ci.yml`) builds on every push/PR:

- **Linux x86_64**: standalone binary `tar.gz`, `.deb` package and `AppImage`
- **Android**: release `arm64-v8a` APK (CI-signed with a generated keystore —
  replace with your own keystore secrets for production releases)

## Data pipeline

The game data is **not committed** to the repository (keeping copyrighted game
content out of git history). It is generated from the fan wiki
[soshage.com/dqm2sp](https://soshage.com/dqm2sp/) through its SvelteKit data
endpoints, then normalized into compact JSON:

```sh
python3 scripts/fetch_all.py     # scrape all pages → cache/ (parallel, resumable)
python3 scripts/fetch_sprites.py # download webp sprites → cache/sprites/
python3 scripts/build_data.py    # normalize → data/*.json + assets/sprites.json
```

The resulting `data/*.json` and `assets/sprites.json` are embedded into the
binary at compile time (`include_str!`) — **the built app never accesses the
network**. The CI runs the same pipeline automatically (cached between runs
with `actions/cache`).

## License

MIT for the application code. Game data and images belong to their respective
owners (Square Enix); this repository distributes extracted game-data tables for
personal, non-commercial fan use.
