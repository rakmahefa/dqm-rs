# data/ — generated game data

This directory is populated by the data pipeline and is **not committed** to the
repository (no copyrighted game content in git history):

```sh
python3 scripts/fetch_all.py      # scrape soshage.com/dqm2sp/ → ../cache/ (resumable)
python3 scripts/fetch_sprites.py  # download webp sprites → ../cache/sprites/
python3 scripts/build_data.py     # normalize → data/*.json + assets/sprites.json
```

The CI runs the same pipeline automatically (results cached with
`actions/cache`, keyed on the scraper version) before building the app.

The app embeds these files at compile time via `include_str!` — after
generation, no network access is ever needed.
