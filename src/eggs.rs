//! Base œufs : pools d'éclosion, probabilités, générateur de noms.

use crate::data::db;
use crate::format::{matches, percent, roll_egg_name, time_seed};
use crate::ui::*;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Eggs() -> Element {
    let d = db();
    let mut kind = use_signal(|| 0u32);
    let mut search = use_signal(String::new);
    let mut egg_name = use_signal(|| None::<(u32, String)>);

    let filtered = use_memo(move || {
        let k = kind();
        let e = db().eggs.iter().find(|e| e.kind == k);
        match e {
            Some(e) => e
                .monsters
                .iter()
                .filter(|em| {
                    search().is_empty()
                        || db()
                            .monster(em.id)
                            .map(|m| matches(&m.name, &search()))
                            .unwrap_or(false)
                })
                .cloned()
                .collect::<Vec<_>>(),
            None => Vec::new(),
        }
    });

    let current = d.eggs.iter().find(|e| e.kind == kind());
    let pool_count = filtered().len();
    let current_kind = kind();

    rsx! {
        div { class: "page",
            h1 { "Eggs" }
            p { class: "muted",
                "Hatch pools with exact probabilities — data from the in-game egg system."
            }

            div { class: "tabs card",
                {d.eggs.iter().map(|e| {
                    let k = e.kind;
                    let count = e.monsters.len();
                    rsx! {
                        button {
                            class: if kind() == k { "tab active" } else { "tab" },
                            key: "{k}",
                            onclick: move |_| { kind.set(k); search.set(String::new()); egg_name.set(None); },
                            "{e.name}"
                            span { class: "tab-n", "{count}" }
                        }
                    }
                })}
            }

            if let Some(e) = current {
                div { class: "controls card",
                    SearchBox {
                        value: search(),
                        placeholder: "Search a hatchable monster…".to_string(),
                        on_input: move |v| search.set(v),
                    }
                    if let Some(b) = e.buy.filter(|b| *b > 0) {
                        span { class: "muted", "Buy price: {b} G" }
                    }
                    button {
                        class: "btn",
                        onclick: move |_| {
                            let name = roll_egg_name(&e.name_parts, time_seed());
                            egg_name.set(Some((e.kind, name)));
                        },
                        "🎲 Generate egg name"
                    }
                }

                if let Some((k, name)) = egg_name() {
                    if k == current_kind {
                        div { class: "card egggen",
                            strong { {name} }
                            span { class: "muted small",
                                " — procedurally rolled name for this egg type"
                            }
                        }
                    }
                }

                p { class: "muted", "{pool_count} monsters in this pool" }

                div { class: "card tablecard",
                    table { class: "tbl",
                        thead { tr { th { "Monster" }, th { "Chance" }, th { "Groups" } } }
                        tbody {
                            {filtered().iter().map(|em| {
                                let m = db().monster(em.id);
                                let prob = percent(em.num, em.den);
                                let groups: Vec<String> = em
                                    .sources
                                    .iter()
                                    .map(|s| {
                                        if let Some(fams) = &s.families {
                                            if fams.len() >= 4 {
                                                format!("All families (w={})", s.weight.unwrap_or(0))
                                            } else {
                                                format!("{} (w={})", fams.join("+"), s.weight.unwrap_or(0))
                                            }
                                        } else if s.tier.is_some() {
                                            let r0 = s.range.first().copied().flatten().map(|v| v.to_string()).unwrap_or_else(|| "?".into());
                                            let r1 = s.range.get(1).copied().flatten().map(|v| v.to_string()).unwrap_or_else(|| "?".into());
                                            format!("Tier {} · lib {}-{} (w={})", s.tier.unwrap_or(0), r0, r1, s.weight.unwrap_or(0))
                                        } else {
                                            "-".to_string()
                                        }
                                    })
                                    .collect();
                                let groups_joined = groups.join(" · ");
                                rsx! {
                                    tr { key: "{em.id}",
                                        td {
                                            if let Some(m) = m {
                                                Link { class: "ilink", to: Route::MonsterDetail { id: m.id },
                                                    MonsterAvatar { id: m.id }
                                                    "{m.name}"
                                                }
                                            } else {
                                                "#{em.id}"
                                            }
                                        }
                                        td { class: "prob", {prob} }
                                        td { class: "muted small", {groups_joined} }
                                    }
                                }
                            })}
                        }
                    }
                }
            }
        }
    }
}
