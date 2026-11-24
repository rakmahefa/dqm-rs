//! Items : liste filtrable + détail.

use crate::data::db;
use crate::format::matches;
use crate::ui::*;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Items() -> Element {
    let d = db();
    let mut search = use_signal(String::new);
    let mut type_filter = use_signal(|| "all".to_string());

    let types: Vec<String> = {
        let mut t: Vec<String> = d
            .items
            .iter()
            .filter_map(|i| i.item_type.clone())
            .collect();
        t.sort();
        t.dedup();
        t
    };

    let filtered = use_memo(move || {
        db().items
            .iter()
            .filter(|i| {
                let t = type_filter();
                (t == "all" || i.item_type.as_deref() == Some(t.as_str()))
                    && matches(&i.name, &search())
            })
            .map(|i| (i.id, i.name.clone(), i.item_type.clone(), i.buy))
            .collect::<Vec<_>>()
    });

    let type_opts: Vec<(String, String)> = std::iter::once(("all".into(), "All types".into()))
        .chain(types.iter().map(|t| (t.clone(), prettify_type(t))))
        .collect();

    rsx! {
        div { class: "page",
            h1 { "Items" }
            p { class: "muted", "{filtered().len()} / {d.items.len()} items" }
            div { class: "controls card",
                SearchBox {
                    value: search(),
                    placeholder: "Search an item…".to_string(),
                    on_input: move |v| search.set(v),
                }
                SelectFilter { value: type_filter(), options: type_opts,
                    on_change: move |v| type_filter.set(v) }
            }
            div { class: "card tablecard",
                table { class: "tbl",
                    thead { tr { th { "Item" }, th { "Type" }, th { "Buy" } } }
                    tbody {
                        {filtered().iter().map(|(id, name, t, buy)| {
                            let type_label = t.clone().map(|t| prettify_type(&t)).unwrap_or_else(|| "—".into());
                            let buy_label = buy
                                .map(|b| if b > 0 { b.to_string() } else { "—".into() })
                                .unwrap_or_else(|| "—".into());
                            rsx! {
                                tr { key: "{id}",
                                    td { Link { class: "ilink", to: Route::ItemDetail { id: *id }, {name.clone()} } }
                                    td { class: "muted", {type_label} }
                                    td { {buy_label} }
                                }
                            }
                        })}
                    }
                }
            }
        }
    }
}

fn prettify_type(t: &str) -> String {
    t.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[component]
pub fn ItemDetail(id: u32) -> Element {
    let d = db();
    let it = match d.item(id) {
        Some(i) => i,
        None => {
            return rsx! {
                div { class: "page",
                    h1 { "Unknown item #{id}" }
                    Link { class: "btn", to: Route::Items {}, "Back to items" }
                }
            }
        }
    };

    let type_tag = it.item_type.clone().map(|t| prettify_type(&t));
    let buy_label = it.buy.filter(|b| *b > 0).map(|b| format!("Buy: {b} G"));
    let break_label = it.break_p.filter(|b| *b < 100).map(|b| format!("Break: {b}%"));

    let stat_rows: Vec<(&'static str, u32)> = vec![
        ("Attack", it.atk.unwrap_or(0)),
        ("Defense", it.def.unwrap_or(0)),
        ("Agility", it.agi.unwrap_or(0)),
        ("Wisdom", it.wis.unwrap_or(0)),
        ("Max HP", it.hp.unwrap_or(0)),
        ("Max MP", it.mp.unwrap_or(0)),
    ];
    let has_stats = stat_rows.iter().any(|(_, v)| *v > 0);

    rsx! {
        div { class: "page detail",
            div { class: "crumbs",
                Link { class: "crumb", to: Route::Items {}, "Items" }
                span { " / {it.name}" }
            }

            section { class: "card mhead",
                div { class: "mhead-main",
                    h1 { {it.name.clone()} }
                    div { class: "mhead-meta",
                        if let Some(t) = type_tag {
                            span { class: "tag tag-egg", {t} }
                        }
                        if let Some(t) = buy_label {
                            span { class: "muted", {t} }
                        }
                        if let Some(t) = break_label {
                            span { class: "muted", {t} }
                        }
                    }
                    p { {it.desc.clone()} }
                }
            }

            if has_stats {
                section { class: "card",
                    h2 { class: "section-title", "Equipment bonuses" }
                    table { class: "tbl",
                        tbody {
                            {stat_rows.iter().map(|(l, v)| {
                                let label = *l;
                                rsx! {
                                    tr { key: "{label}",
                                        td { class: "stat-l", {label} }
                                        td { class: "stat-v", {v.to_string()} }
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
