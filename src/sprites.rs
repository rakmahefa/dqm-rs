//! Sprites embarqués (base64 → data-URL). Aucun fichier externe : tout est
//! dans le binaire via `include_str!("../assets/sprites.json")`.

use std::collections::HashMap;
use std::sync::OnceLock;

static SPRITES: OnceLock<HashMap<String, String>> = OnceLock::new();

fn sprites() -> &'static HashMap<String, String> {
    SPRITES.get_or_init(|| {
        let raw = include_str!("../assets/sprites.json");
        serde_json::from_str::<HashMap<String, String>>(raw).expect("sprites.json invalide")
    })
}

/// Data-URL d'un sprite par sa clé (ex: "1", "fam1", "rank3", "size2", "frame").
pub fn sprite(key: &str) -> String {
    match sprites().get(key) {
        Some(b64) => format!("data:image/webp;base64,{b64}"),
        None => String::new(),
    }
}

/// Sprite d'un monstre (ou chaîne vide si absent).
pub fn monster_sprite(id: u32) -> String {
    sprite(&id.to_string())
}
