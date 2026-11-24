//! Page d'accueil : vue d'ensemble + navigation rapide.

use crate::data::db;
use crate::format::time_seed;
use crate::ui::{FamilyIcon, MonsterAvatar, RankBadge, SizeIcon};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let d = db();
    let counts = [
        ("Monsters", d.monsters.len()),
        ("Synthesis recipes", d.recipes.len()),
        ("Skills", d.skills.len()),
        ("Items", d.items.len()),
        ("Traits", d.traits.len()),
        ("Abilities", d.abilities.len()),
    ];
    let obtainable = d.monsters.iter().filter(|m| m.wild || !m.egg_kinds.is_empty()).count();
    let eggs_total: usize = d.eggs.iter().map(|e| e.monsters.len()).sum();

    let mut random_id = use_signal(|| None::<u32>);

    rsx! {
        div { class: "page home",
            section { class: "hero card",
                h1 { "Dragon Quest Monsters 2 SP" }
                p { class: "hero-sub",
                    "Offline companion — encyclopedia, synthesis planner & egg database"
                }
                div { class: "hero-actions",
                    Link { class: "btn primary big", to: Route::Monsters {}, "Browse monsters" }
                    Link { class: "btn big", to: Route::Synthesis { target: None }, "Synthesis planner" }
                    button {
                        class: "btn big",
                        onclick: move |_| {
                            let d = db();
                            let n = d.monsters.len();
                            let idx = (crate::format::Rng { state: time_seed() }.next() % n as u64) as usize;
                            random_id.set(Some(d.monsters[idx].id));
                        },
                        "🎲 Random monster"
                    }
                }
                if let Some(id) = random_id() {
                    div { class: "random-box",
                        MonsterAvatar { id: id, big: true }
                        Link { class: "chip big", to: Route::MonsterDetail { id },
                            {db().monster(id).map(|m| m.name.clone()).unwrap_or_default()}
                        }
                    }
                }
            }

            section { class: "stats-grid",
                {counts.iter().map(|(label, n)| {
                    let label = *label;
                    let n = *n;
                    rsx! {
                        div { class: "card stat", key: "{label}",
                            div { class: "stat-n", "{n}" }
                            div { class: "stat-l", {label} }
                        }
                    }
                })}
                div { class: "card stat", key: "obtainable",
                    div { class: "stat-n", "{obtainable}" }
                    div { class: "stat-l", "Obtainable in the wild / eggs" }
                }
                div { class: "card stat", key: "eggs",
                    div { class: "stat-n", "{eggs_total}" }
                    div { class: "stat-l", "Egg hatch entries" }
                }
            }

            section { class: "card",
                h2 { class: "section-title", "Families" }
                div { class: "fam-grid",
                    {d.families.iter().map(|f| {
                        rsx! {
                            Link { class: "fam-card", key: "{f.id}",
                                to: Route::Monsters {},
                                FamilyIcon { family_id: f.id }
                                span { class: "fam-name", {f.name.clone()} }
                                span { class: "fam-count", "{f.count} monsters" }
                            }
                        }
                    })}
                }
            }

            section { class: "card",
                h2 { class: "section-title", "Quick links" }
                div { class: "quick-grid",
                    Link { class: "quick", to: Route::Eggs {},
                        span { class: "quick-ico", "◍" }
                        h3 { "Eggs" }
                        p { "Hatch pools & exact probabilities for the 5 egg types" }
                    }
                    Link { class: "quick", to: Route::Skills {},
                        span { class: "quick-ico", "✦" }
                        h3 { "Skills" }
                        p { "425 skill sets, evolutions and SP costs" }
                    }
                    Link { class: "quick", to: Route::Items {},
                        span { class: "quick-ico", "⚒" }
                        h3 { "Items" }
                        p { "468 items with prices and bonuses" }
                    }
                    Link { class: "quick", to: Route::Abilities {},
                        span { class: "quick-ico", "⚔" }
                        h3 { "Abilities" }
                        p { "306 abilities with targeting and damage values" }
                    }
                }
            }

            section { class: "card",
                h2 { class: "section-title", "Top monsters by rank" }
                div { class: "top-grid",
                    {[8u32, 7, 6].iter().map(|rank| {
                        let rank = *rank;
                        let mut best: Vec<_> = d.monsters.iter()
                            .filter(|m| m.rank == rank)
                            .take(6)
                            .collect();
                        best.sort_by_key(|m| m.max[0]);
                        best.reverse();
                        rsx! {
                            div { class: "top-col", key: "{rank}",
                                h3 { RankBadge { rank: rank, label: true } }
                                {best.iter().take(4).map(|m| {
                                    let name = m.name.clone();
                                    rsx! {
                                        Link { class: "chip", key: "{m.id}", to: Route::MonsterDetail { id: m.id },
                                            MonsterAvatar { id: m.id }
                                            span { {name} }
                                            SizeIcon { size: m.size }
                                        }
                                    }
                                })}
                            }
                        }
                    })}
                }
            }
        }
    }
}
