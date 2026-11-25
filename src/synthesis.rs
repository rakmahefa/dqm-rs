//! Synthèse : arbre récursif exploratoire + planner (route optimale).

use crate::data::db;
use crate::format::{matches, rank_label};
use crate::ui::*;
use crate::Route;
use dioxus::prelude::*;
use std::collections::HashSet;

#[component]
pub fn Synthesis(target: Option<u32>) -> Element {
    let mut mode = use_signal(|| "tree".to_string());
    let mut search = use_signal(String::new);
    let mut target_id = use_signal(|| target);
    // nœuds repliés : clés "chemin" comme "0.1.0"
    let mut collapsed: Signal<HashSet<String>> = use_signal(HashSet::new);

    let results = use_memo(move || {
        let q = search();
        if q.len() < 2 {
            Vec::new()
        } else {
            db().monsters
                .iter()
                .filter(|m| matches(&m.name, &q))
                .take(12)
                .map(|m| (m.id, m.name.clone()))
                .collect::<Vec<_>>()
        }
    });

    let t = target_id();
    rsx! {
        div { class: "page",
            h1 { "Synthesis" }

            div { class: "controls card",
                SearchBox {
                    value: search(),
                    placeholder: "Search a target monster…".to_string(),
                    on_input: move |v| search.set(v),
                }
                div { class: "btn-group",
                    button {
                        class: if mode() == "tree" { "btn active" } else { "btn" },
                        onclick: move |_| mode.set("tree".into()),
                        "Tree view"
                    }
                    button {
                        class: if mode() == "plan" { "btn active" } else { "btn" },
                        onclick: move |_| mode.set("plan".into()),
                        "Optimal plan"
                    }
                }
            }

            if !results().is_empty() && search().len() >= 2 {
                div { class: "card suggest",
                    {results().iter().map(|(rid, name)| {
                        let rid = *rid;
                        let name = name.clone();
                        rsx! {
                            button {
                                class: "suggest-row", key: "{rid}",
                                onclick: move |_| {
                                    target_id.set(Some(rid));
                                    search.set(String::new());
                                    collapsed.set(HashSet::new());
                                },
                                MonsterAvatar { id: rid }
                                "{name}"
                                span { class: "muted small", " #{rid}" }
                            }
                        }
                    })}
                }
            }

            if let Some(tid) = t {
                if let Some(m) = db().monster(tid) {
                    if mode() == "tree" {
                        section { class: "card",
                            h2 { class: "section-title", "Tree for {m.name}" }
                            p { class: "muted",
                                "Click a node to expand / collapse. Green = obtainable in the wild or from eggs."
                            }
                            TreeNode { id: tid, path: "0".to_string(), depth: 0,
                                collapsed: collapsed }
                        }
                    } else {
                        section { class: "card",
                            h2 { class: "section-title", "Optimal plan for {m.name}" }
                            PlanView { id: tid }
                        }
                    }
                } else {
                    p { class: "warn", "Unknown monster #{tid}" }
                }
            } else {
                section { class: "card",
                    h2 { class: "section-title", "How it works" }
                    p {
                        "Search a monster above, then explore its full synthesis tree, "
                        "or get the shortest breeding route from obtainable monsters."
                    }
                    p { class: "muted",
                        "The optimal plan minimizes the number of synthesis generations. "
                        "Wild / egg monsters are used as starting points."
                    }
                }
            }
        }
    }
}

/// Nœud récursif de l'arbre de synthèse.
#[component]
fn TreeNode(id: u32, path: String, depth: u32, collapsed: Signal<HashSet<String>>) -> Element {
    let d = db();
    let m = match d.monster(id) {
        Some(m) => m,
        None => return rsx! { div { class: "tnode", "Unknown #{id}" } },
    };

    let is_collapsed = collapsed().contains(&path);
    let obtainable = m.wild || !m.egg_kinds.is_empty();
    let recipes: Vec<(usize, Vec<u32>)> = d
        .recipes_by_result
        .get(&id)
        .map(|v| v.iter().map(|&ri| (ri, d.recipes[ri].p.clone())).collect())
        .unwrap_or_default();

    let node_class = if obtainable { "tnode-head ok" } else { "tnode-head" };
    let can_toggle = !recipes.is_empty();
    let source_tag = if m.wild { "Wild" } else { "Egg" };
    let path_for_click = path.clone();

    rsx! {
        div { class: "tnode",
            div { class: node_class,
                if can_toggle {
                    button {
                        class: "tbtn",
                        onclick: move |_| {
                            let mut set = collapsed.cloned();
                            if set.contains(&path_for_click) { set.remove(&path_for_click); } else { set.insert(path_for_click.clone()); }
                            collapsed.set(set);
                        },
                        if is_collapsed { "▸" } else { "▾" }
                    }
                } else {
                    span { class: "tbtn leaf", "•" }
                }
                Link { class: "tnode-link", to: Route::MonsterDetail { id },
                    MonsterAvatar { id: id }
                    span { class: "tnode-name", {m.name.clone()} }
                    RankBadge { rank: m.rank, label: true }
                }
                if obtainable {
                    span { class: "tag tag-wild", {source_tag} }
                }
                if !can_toggle && !obtainable {
                    span { class: "tag tag-warn", "No recipe" }
                }
            }

            if !is_collapsed && can_toggle && depth < 12 {
                div { class: "tnode-kids",
                    {recipes.iter().enumerate().map(|(r_i, (ri, parents))| {
                        let grand = d.recipes[*ri].t == "4g";
                        let label = if grand {
                            "grand recipe".to_string()
                        } else {
                            format!("recipe {}", r_i + 1)
                        };
                        rsx! {
                            div { class: "tnode-recipe", key: "{ri}",
                                span { class: "muted small recipe-label", {label} }
                                div { class: "tnode-parents",
                                    {parents.iter().enumerate().map(|(pi, &p)| {
                                        let child_path = format!("{path}.{r_i}.{pi}");
                                        rsx! {
                                            TreeNode { id: p, key: "{child_path}",
                                                path: child_path, depth: depth + 1,
                                                collapsed: collapsed }
                                        }
                                    })}
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}

/// Vue du plan optimal : étapes triées par génération décroissante.
#[component]
fn PlanView(id: u32) -> Element {
    let d = db();
    let plan = d.plan_for(id);

    match plan {
        None => {
            let generic: Vec<String> = d
                .monster(id)
                .map(|m| {
                    m.generic_breeding
                        .iter()
                        .map(|g| {
                            let mut s = format!("{} × {}", g.f1, g.f2);
                            if let Some(r) = g.rank {
                                s.push_str(&format!(" — rank ≥ {}", rank_label(r)));
                            }
                            s
                        })
                        .collect()
                })
                .unwrap_or_default();
            rsx! {
                div { class: "plan",
                    p { class: "warn",
                        "This monster cannot be reached through explicit synthesis recipes "
                        "from obtainable monsters. It may come from generic family breeding, "
                        "eggs, or special events."
                    }
                    if !generic.is_empty() {
                        p { class: "muted", "Generic family rules exist for this monster:" }
                        ul {
                            {generic.iter().enumerate().map(|(i, g)| {
                                rsx! { li { key: "{i}", {g.clone()} } }
                            })}
                        }
                    }
                }
            }
        }
        Some(steps) => {
            let max_gen = steps.first().map(|(g, _, _)| *g).unwrap_or(0);
            let summary = if max_gen > 1 {
                format!("{max_gen} generations of synthesis — {} steps", steps.len())
            } else {
                format!("{max_gen} generation of synthesis — {} steps", steps.len())
            };
            let bases = base_monsters(&steps);
            rsx! {
                div { class: "plan",
                    p { class: "plan-summary", {summary} }
                    div { class: "plan-steps",
                        {steps.iter().map(|(gen, res, parents)| {
                            let gen_label = format!("G{gen}");
                            rsx! {
                                div { class: "plan-step", key: "{res}",
                                    span { class: "plan-gen", {gen_label} }
                                    div { class: "plan-body",
                                        div { class: "plan-parents",
                                            {parents.iter().map(|&p| {
                                                rsx! { span { key: "{p}", MonsterChip { id: p } } }
                                            })}
                                        }
                                        span { class: "plan-arrow", "↓" }
                                        div { class: "plan-result",
                                            MonsterChip { id: *res }
                                        }
                                    }
                                }
                            }
                        })}
                    }
                    p { class: "muted", "Starting monsters (obtainable without synthesis):" }
                    div { class: "chips",
                        {bases.iter().map(|&b| {
                            rsx! { span { key: "{b}", MonsterChip { id: b } } }
                        })}
                    }
                }
            }
        }
    }
}

fn base_monsters(steps: &[(u32, u32, Vec<u32>)]) -> Vec<u32> {
    let d = db();
    let mut set: HashSet<u32> = HashSet::new();
    for (_, _, parents) in steps {
        for &p in parents {
            if d.obtainable(p) {
                set.insert(p);
            }
        }
    }
    let mut v: Vec<u32> = set.into_iter().collect();
    v.sort_by_key(|&id| db().monster(id).and_then(|m| m.lib).unwrap_or(u32::MAX));
    v
}
