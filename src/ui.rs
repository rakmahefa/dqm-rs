//! Composants UI partagés (avatars, badges, recherche, pagination).

use crate::data::db;
use crate::format::{rank_label, size_label};
use crate::sprites;
use dioxus::prelude::*;

/// Avatar d'un monstre : sprite si dispo, sinon cadre + icône de famille.
#[component]
pub fn MonsterAvatar(id: u32, #[props(default)] big: bool) -> Element {
    let m = db().monster(id);
    let (name, family_id, has_sprite) = match m {
        Some(m) => (m.name.clone(), m.family_id, m.sprite),
        None => ("?".to_string(), 0, false),
    };
    let class = if big { "avatar big" } else { "avatar" };
    rsx! {
        span { class: class, title: "{name}",
            if has_sprite {
                img { class: "avatar-img", src: sprites::monster_sprite(id), alt: "" }
            } else {
                span { class: "avatar-fallback",
                    img { class: "avatar-fam", src: sprites::sprite(&format!("fam{family_id}")), alt: "" }
                }
            }
        }
    }
}

/// Puce monstre cliquable (sprite + nom).
#[component]
pub fn MonsterChip(id: u32) -> Element {
    let m = match db().monster(id) {
        Some(m) => m,
        None => return rsx! { span { class: "chip", "Unknown #{id}" } },
    };
    rsx! {
        Link { class: "chip", to: crate::Route::MonsterDetail { id },
            MonsterAvatar { id: id }
            span { "{m.name}" }
        }
    }
}

/// Badge de rang (icône + lettre).
#[component]
pub fn RankBadge(rank: u32, #[props(default)] label: bool) -> Element {
    let rl = rank_label(rank);
    rsx! {
        span { class: "rank rank-{rank}", title: "Rank {rl}",
            img { class: "rank-img", src: sprites::sprite(&format!("rank{rank}")), alt: "" }
            if label {
                span { class: "rank-letter", "{rl}" }
            }
        }
    }
}

/// Icône de famille.
#[component]
pub fn FamilyIcon(family_id: u32) -> Element {
    rsx! {
        img { class: "fam-ico", src: sprites::sprite(&format!("fam{family_id}")), alt: "" }
    }
}

/// Icône de taille.
#[component]
pub fn SizeIcon(size: u32) -> Element {
    rsx! {
        span { class: "size-badge", title: size_label(size),
            img { class: "size-img", src: sprites::sprite(&format!("size{}", size + 1)), alt: "" }
        }
    }
}

/// Champ de recherche.
#[component]
pub fn SearchBox(value: String, placeholder: String, on_input: EventHandler<String>) -> Element {
    rsx! {
        input {
            class: "search",
            r#type: "text",
            placeholder: "{placeholder}",
            value: "{value}",
            oninput: move |e| on_input.call(e.value()),
        }
    }
}

/// Pagination simple.
#[component]
pub fn Pager(page: usize, total_pages: usize, on_change: EventHandler<usize>) -> Element {
    if total_pages <= 1 {
        return rsx! {};
    }
    let mut pages: Vec<usize> = Vec::new();
    for p in 1..=total_pages {
        if p == 1 || p == total_pages || p.abs_diff(page) <= 1 {
            pages.push(p);
        }
    }
    rsx! {
        div { class: "pager",
            button {
                class: "btn page",
                disabled: page <= 1,
                onclick: move |_| on_change.call(page - 1),
                "‹ Prev"
            }
            {pages.iter().copied().map(|p| {
                let cur = p;
                rsx! {
                    button {
                        class: if cur == page { "btn page active" } else { "btn page" },
                        key: "{p}",
                        onclick: move |_| on_change.call(cur),
                        "{p}"
                    }
                }
            })}
            button {
                class: "btn page",
                disabled: page >= total_pages,
                onclick: move |_| on_change.call(page + 1),
                "Next ›"
            }
        }
    }
}

/// Sélecteur générique (menu dérouant).
#[component]
pub fn SelectFilter(value: String, options: Vec<(String, String)>, on_change: EventHandler<String>) -> Element {
    rsx! {
        select {
            class: "select",
            value: "{value}",
            onchange: move |e| on_change.call(e.value()),
            {options.iter().map(|(v, l)| {
                let v = v.clone();
                let l = l.clone();
                rsx! {
                    option { key: "{v}", value: "{v}", selected: v == value, "{l}" }
                }
            })}
        }
    }
}

/// En-tête de section réutilisable.
#[component]
pub fn SectionTitle(text: String) -> Element {
    rsx! { h2 { class: "section-title", "{text}" } }
}

/// Carte générique.
#[component]
pub fn Card(children: Element) -> Element {
    rsx! { div { class: "card", {children} } }
}
