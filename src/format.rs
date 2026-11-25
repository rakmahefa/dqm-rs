//! Petits utilitaires de formatage et libellés de jeu.

pub const RANKS: [&str; 9] = ["", "F", "E", "D", "C", "B", "A", "S", "SS"];
pub const SIZES: [&str; 4] = ["Small", "Medium", "Large", "Extra Large"];

pub fn rank_label(rank: u32) -> &'static str {
    RANKS.get(rank as usize).copied().unwrap_or("?")
}

pub fn size_label(size: u32) -> &'static str {
    SIZES.get(size as usize).copied().unwrap_or("?")
}

/// Pourcentage lisible depuis une fraction num/den.
pub fn percent(num: Option<u32>, den: Option<u32>) -> String {
    match (num, den) {
        (Some(n), Some(d)) if d > 0 => {
            let v = n as f64 / d as f64 * 100.0;
            if v >= 10.0 {
                format!("{v:.1}%")
            } else {
                format!("{v:.2}%")
            }
        }
        _ => "-".into(),
    }
}

/// Nom d'œuf à partir des parts pondérées (générateur offline).
pub fn roll_egg_name(parts: &[crate::data::NamePart], seed: u64) -> String {
    let mut rng = Rng { state: seed };
    let mut out = String::new();
    for part in parts {
        let total: u32 = part
            .options
            .iter()
            .map(|o| if o.is_empty { 0 } else { o.weight })
            .sum();
        if total == 0 {
            continue;
        }
        let mut pick = (rng.next() % total as u64) as u32;
        for opt in &part.options {
            if opt.is_empty {
                continue;
            }
            if pick < opt.weight {
                out.push_str(&opt.text);
                break;
            }
            pick -= opt.weight;
        }
    }
    out
}

/// Petit générateur congruentiel (aucune dépendance externe).
pub struct Rng {
    pub state: u64,
}

impl Rng {
    pub fn next(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
}

pub fn time_seed() -> u64 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x9E3779B97F4A7C15);
    nanos | 1
}

/// Recherche insensible à la casse sur un préfixe/sous-chaîne.
pub fn matches(haystack: &str, needle: &str) -> bool {
    let h = haystack.to_lowercase();
    let n = needle.to_lowercase();
    n.is_empty() || h.contains(&n)
}
