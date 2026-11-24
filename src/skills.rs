//! Skills : liste + détail (évolutions, slots, monstres natifs).

use crate::data::db;
use crate::format::matches;
use crate::ui::*;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Skills() -> Element {
    let d = db();
    let mut search = use_signal(String::new);

    let filtered = use_memo(move || {
        db().skills
            .iter()
            .filter(|s| matches(&s.name, &search()))
            .map(|s| (s.id, s.name.clone(), s.monsters.len()))
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "page",
            h1 { "Skills" }
            p { class: "muted", "{filtered().len()} / {d.skills.len()} skills" }
            div { class: "controls card",
                SearchBox {
                    value: search(),
                    placeholder: "Search a skill…".to_string(),
                    on_input: move |v| search.set(v),
                }
            }
            div { class: "card tablecard",
                table { class: "tbl",
                    thead { tr { th { "Skill" }, th { "Evolution" }, th { "Native monsters" } } }
                    tbody {
                        {filtered().iter().map(|(id, name, n)| {
                            let evo = db()
                                .skill(*id)
                                .and_then(|s| s.evo_to_name.clone())
                                .unwrap_or_else(|| "—".into());
                            rsx! {
                                tr { key: "{id}",
                                    td { Link { class: "ilink", to: Route::SkillDetail { id: *id }, {name.clone()} } }
                                    td { class: "muted", {evo} }
                                    td { {n.to_string()} }
                                }
                            }
                        })}
                    }
                }
            }
        }
    }
}

#[component]
pub fn SkillDetail(id: u32) -> Element {
    let d = db();
    let s = match d.skill(id) {
        Some(s) => s,
        None => {
            return rsx! {
                div { class: "page",
                    h1 { "Unknown skill #{id}" }
                    Link { class: "btn", to: Route::Skills {}, "Back to skills" }
                }
            }
        }
    };

    let evo_to_id = s.evo_to;
    let evo_to_name = s.evo_to_name.clone().unwrap_or_default();
    let evo_cost = s.evo_cost.map(|c| format!(" — {c} SP"));
    let special_reqs: Vec<(u32, String, String)> = s
        .special
        .iter()
        .map(|sp| {
            let req = sp
                .req
                .iter()
                .map(|r| format!("{} ({} SP)", r.name, r.sp.unwrap_or(0)))
                .collect::<Vec<_>>()
                .join(" + ");
            (sp.to, sp.to_name.clone(), req)
        })
        .collect();

    let native_count = s.monsters.len();
    let native_title = if native_count == 1 {
        "Native monster".to_string()
    } else {
        format!("Native to {native_count} monsters")
    };

    rsx! {
        div { class: "page detail",
            div { class: "crumbs",
                Link { class: "crumb", to: Route::Skills {}, "Skills" }
                span { " / {s.name}" }
            }

            section { class: "card mhead",
                div { class: "mhead-main",
                    h1 { {s.name.clone()} }
                }
            }

            if !s.evo_from.is_empty() {
                section { class: "card",
                    h2 { class: "section-title", "Evolved from" }
                    ul { class: "traits",
                        {s.evo_from.iter().map(|r| {
                            let cost = r.sp.map(|sp| format!(" — {sp} SP")).unwrap_or_default();
                            rsx! {
                                li { key: "{r.skill}", class: "trait",
                                    Link { class: "ilink", to: Route::SkillDetail { id: r.skill }, {r.name.clone()} }
                                    span { class: "muted", {cost} }
                                }
                            }
                        })}
                    }
                }
            }

            if s.evo_to.is_some() || !s.special.is_empty() {
                section { class: "card",
                    h2 { class: "section-title", "Evolves into" }
                    if let Some(to) = evo_to_id {
                        p {
                            Link { class: "chip big", to: Route::SkillDetail { id: to },
                                {evo_to_name}
                            }
                            if let Some(cost) = evo_cost {
                                span { class: "muted", {cost} }
                            }
                        }
                    }
                    if !special_reqs.is_empty() {
                        h3 { class: "sub", "Special (fusion) evolutions" }
                        ul { class: "traits",
                            {special_reqs.iter().map(|(to, name, req)| {
                                rsx! {
                                    li { key: "{to}", class: "trait",
                                        Link { class: "ilink", to: Route::SkillDetail { id: *to }, {name.clone()} }
                                        span { class: "muted", " — requires: {req}" }
                                    }
                                }
                            })}
                        }
                    }
                }
            }

            section { class: "card",
                h2 { class: "section-title", "Skill slots" }
                if s.slots.is_empty() {
                    p { class: "muted", "No slot data." }
                } else {
                    crate::monsters::SlotsTable { slots: s.slots.clone() }
                }
            }

            if !s.monsters.is_empty() {
                section { class: "card",
                    h2 { class: "section-title", {native_title} }
                    div { class: "chips",
                        {s.monsters.iter().map(|&mid| {
                            rsx! { span { key: "{mid}", MonsterChip { id: mid } } }
                        })}
                    }
                }
            }
        }
    }
}
