# data/ — generated game data

This directory holds the **committed** game data extracted from the fan wiki
[soshage.com/dqm2sp](https://soshage.com/dqm2sp/) (personal, non-commercial fan
use — game data © Square Enix, see the repository License section).

Committing the generated data makes builds **hermetic**: the CI and any fresh
clone can build the complete app without depending on the availability of the
wiki (which proved unreliable for CI runners — 502/timeout outages).

## Regeneration (only when the data needs an update — the wiki must be up)

```sh
python3 scripts/fetch_all.py      # scrape soshage.com/dqm2sp/ → cache/ (resumable)
python3 scripts/fetch_sprites.py  # download webp sprites → cache/sprites/
python3 scripts/build_data.py     # normalize → data/*.json + assets/sprites.json
python3 scripts/verify_data.py    # sanity-check the generated files
git add data assets/sprites.json && git commit -m "data: refresh from soshage.com"
```

The app embeds these files at compile time via `include_str!` — after the
build, no network access is ever needed. The CI only validates the committed
data (`scripts/verify_data.py`) and never scrapes.
