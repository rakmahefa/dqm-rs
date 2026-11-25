//! Références : traits, abilities + page About.

use crate::data::db;
use crate::format::matches;
use crate::ui::SearchBox;
use dioxus::prelude::*;
use std::collections::HashSet;

#[component]
pub fn Traits() -> Element {
    let mut search = use_signal(String::new);
    let filtered = use_memo(move || {
        db().traits
            .iter()
            .filter(|t| matches(&t.name, &search()) || matches(&t.desc, &search()))
            .map(|t| (t.id, t.name.clone(), t.desc.clone()))
            .collect::<Vec<_>>()
    });

    let total = db().traits.len();
    let n = filtered().len();

    rsx! {
        div { class: "page",
            h1 { "Traits" }
            p { class: "muted", "{n} / {total} traits" }
            div { class: "controls card",
                SearchBox {
                    value: search(),
                    placeholder: "Search a trait…".to_string(),
                    on_input: move |v| search.set(v),
                }
            }
            div { class: "card listcard",
                ul { class: "reflist",
                    {filtered().iter().map(|(id, name, desc)| {
                        rsx! {
                            li { key: "{id}", class: "refrow",
                                strong { {name.clone()} }
                                span { class: "muted", {desc.clone()} }
                            }
                        }
                    })}
                }
            }
        }
    }
}

#[component]
pub fn Abilities() -> Element {
    let mut search = use_signal(String::new);
    let mut expanded: Signal<HashSet<u32>> = use_signal(HashSet::new);

    let filtered = use_memo(move || {
        db().abilities
            .iter()
            .filter(|a| matches(&a.name, &search()) || matches(&a.desc, &search()))
            .map(|a| a.id)
            .collect::<Vec<_>>()
    });

    let total = db().abilities.len();
    let n = filtered().len();

    rsx! {
        div { class: "page",
            h1 { "Abilities" }
            p { class: "muted", "{n} / {total} abilities" }
            div { class: "controls card",
                SearchBox {
                    value: search(),
                    placeholder: "Search an ability…".to_string(),
                    on_input: move |v| search.set(v),
                }
            }
            div { class: "card listcard",
                ul { class: "reflist",
                    {filtered().iter().map(|&aid| {
                        let a = db().ability(aid).unwrap();
                        let is_open = expanded().contains(&aid);
                        let cats = a.cats.join(", ");
                        let mp_label = match (a.mp, a.mpf) {
                            (Some(mp), false) => format!("MP: {mp}"),
                            (_, true) => "MP: formula".to_string(),
                            _ => String::new(),
                        };
                        rsx! {
                            li { key: "{aid}", class: "refrow ability",
                                div { class: "refrow-head",
                                    button {
                                        class: "tbtn",
                                        onclick: move |_| {
                                            let mut s = expanded.cloned();
                                            if s.contains(&aid) { s.remove(&aid); } else { s.insert(aid); }
                                            expanded.set(s);
                                        },
                                        if is_open { "▾" } else { "▸" }
                                    }
                                    strong { {a.name.clone()} }
                                    if !mp_label.is_empty() {
                                        span { class: "tag tag-mp", {mp_label} }
                                    }
                                    if !cats.is_empty() {
                                        span { class: "muted small", {cats} }
                                    }
                                }
                                if is_open {
                                    div { class: "refrow-body",
                                        if !a.desc.is_empty() {
                                            p { {a.desc.clone()} }
                                        }
                                        AbilityDetails { id: aid }
                                    }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}

#[component]
fn AbilityDetails(id: u32) -> Element {
    let a = match db().ability(id) {
        Some(a) => a,
        None => return rsx! {},
    };

    let mut details: Vec<String> = Vec::new();
    if let Some(v) = &a.vals {
        if let Some(x) = v.primary_min {
            let pmax = v.primary_max.unwrap_or(x);
            if pmax > x {
                details.push(format!(
                    "Damage: {x}–{pmax} (alt {}–{})",
                    v.alternate_min.unwrap_or(0),
                    v.alternate_max.unwrap_or(0)
                ));
            } else if x > 0 {
                details.push(format!("Value: {x}"));
            }
        }
        if let Some(w1) = v.wisdom_min {
            if w1 > 0 {
                details.push(format!("Wisdom scaling: {w1}–{}", v.wisdom_max.unwrap_or(0)));
            }
        }
        if let Some(cap) = v.cap {
            if cap > 0 && cap < 999 {
                details.push(format!("Cap: {cap}"));
            }
        }
    }
    let tags: Vec<String> = [
        a.res.as_ref().filter(|r| *r != "None").map(|r| format!("Resource: {r}")),
        a.tol1.as_ref().filter(|t| *t != "None").map(|t| format!("Tolerance: {t}")),
        a.tol2.as_ref().filter(|t| *t != "None").map(|t| format!("Tolerance 2: {t}")),
        a.status.as_ref().filter(|s| *s != "None").map(|s| format!("Status: {s}")),
    ]
    .into_iter()
    .flatten()
    .collect();

    rsx! {
        div { class: "ability-details",
            if !details.is_empty() {
                div { class: "kv",
                    {details.iter().enumerate().map(|(i, t)| {
                        rsx! { span { key: "{i}", class: "muted", {t.clone()} } }
                    })}
                }
            }
            if !tags.is_empty() {
                div { class: "kv",
                    {tags.iter().enumerate().map(|(i, t)| {
                        rsx! { span { key: "{i}", class: "tag tag-egg", {t.clone()} } }
                    })}
                }
            }
        }
    }
}

#[component]
pub fn About() -> Element {
    let d = db();
    let counts = [
        ("Monsters", d.monsters.len()),
        ("Synthesis recipes", d.recipes.len()),
        ("Skills", d.skills.len()),
        ("Items", d.items.len()),
        ("Traits", d.traits.len()),
        ("Abilities", d.abilities.len()),
        ("Egg types", d.eggs.len()),
    ];

    rsx! {
        div { class: "page",
            h1 { "About" }
            section { class: "card",
                h2 { class: "section-title", "DQM2 SP Companion" }
                p {
                    "A 100% offline companion app for Dragon Quest Monsters 2 SP. "
                    "All game data is embedded in the binary: no network access, no tracking, "
                    "works entirely on-device."
                }
                h3 { class: "sub", "Data" }
                ul {
                    li { "Source: {d.meta.source_name} ({d.meta.source})" }
                    li { "Scraped: {d.meta.scraped_at}" }
                    {counts.iter().map(|(label, n)| {
                        rsx! { li { key: "{label}", "{label}: {n}" } }
                    })}
                }
                h3 { class: "sub", "Credits & disclaimer" }
                p { class: "muted", {d.meta.note.clone()} }
                p { class: "muted", {d.meta.game.clone()} }
                p { class: "muted",
                    "Dragon Quest Monsters is a trademark of Square Enix. This is an unofficial, "
                    "fan-made tool and is not affiliated with or endorsed by Square Enix or the "
                    "soshage.com wiki."
                }
            }
            section { class: "card",
                h2 { class: "section-title", "Tech" }
                p {
                    "Built with Rust + Dioxus. Linux (tar.gz, .deb, AppImage) and Android (arm64-v8a) "
                    "binaries are produced by GitHub Actions CI."
                }
            }
        }
    }
}
