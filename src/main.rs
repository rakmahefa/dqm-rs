//! Point d'entrée : routeur + gabarit principal (barre latérale).

use dioxus::prelude::*;

mod data;
mod eggs;
mod format;
mod home;
mod items;
mod monsters;
mod reference;
mod skills;
mod sprites;
mod synthesis;
mod ui;

use eggs::Eggs;
use home::Home;
use items::{ItemDetail, Items};
use monsters::{MonsterDetail, Monsters};
use reference::{Abilities, About, Traits};
use skills::{SkillDetail, Skills};
use synthesis::Synthesis;

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        #[route("/monsters")]
        Monsters {},
        #[route("/monsters/:id")]
        MonsterDetail { id: u32 },
        #[route("/synthesis?:target")]
        Synthesis { target: Option<u32> },
        #[route("/skills")]
        Skills {},
        #[route("/skills/:id")]
        SkillDetail { id: u32 },
        #[route("/items")]
        Items {},
        #[route("/items/:id")]
        ItemDetail { id: u32 },
        #[route("/traits")]
        Traits {},
        #[route("/abilities")]
        Abilities {},
        #[route("/eggs")]
        Eggs {},
        #[route("/about")]
        About {},
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

const CSS: &str = include_str!("styles.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        style { {CSS} }
        Router::<Route> {}
    }
}

/// Gabarit : barre latérale + zone de contenu.
#[component]
fn AppLayout() -> Element {
    let mut drawer_open = use_signal(|| false);
    let meta = &data::db().meta;
    let scraped_date = meta.scraped_at.split('T').next().unwrap_or("").to_string();

    rsx! {
        div { class: "app",
            // Bandeau mobile
            header { class: "topbar",
                button { class: "btn burger", onclick: move |_| drawer_open.toggle(), "☰" }
                Link { class: "brand", to: Route::Home {}, "DQM2 SP" }
                span { class: "topbar-offline", "100% offline" }
            }
            if drawer_open() {
                div { class: "drawer-overlay", onclick: move |_| drawer_open.set(false) }
            }
            nav { class: if drawer_open() { "sidebar open" } else { "sidebar" },
                div { class: "brand-big",
                    Link { to: Route::Home {}, onclick: move |_| drawer_open.set(false),
                        "DQM2 SP"
                        span { class: "brand-sub", "Companion" }
                    }
                }
                SidebarLink { to: Route::Home {}, icon: "⌂", label: "Home",
                    on_click: move |_| drawer_open.set(false) }
                SidebarLink { to: Route::Monsters {}, icon: "★", label: "Monsters",
                    on_click: move |_| drawer_open.set(false) }
                SidebarLink { to: Route::Synthesis { target: None }, icon: "⑃", label: "Synthesis",
                    on_click: move |_| drawer_open.set(false) }
                SidebarLink { to: Route::Eggs {}, icon: "◍", label: "Eggs",
                    on_click: move |_| drawer_open.set(false) }
                SidebarLink { to: Route::Skills {}, icon: "✦", label: "Skills",
                    on_click: move |_| drawer_open.set(false) }
                SidebarLink { to: Route::Items {}, icon: "⚒", label: "Items",
                    on_click: move |_| drawer_open.set(false) }
                SidebarLink { to: Route::Traits {}, icon: "◈", label: "Traits",
                    on_click: move |_| drawer_open.set(false) }
                SidebarLink { to: Route::Abilities {}, icon: "⚔", label: "Abilities",
                    on_click: move |_| drawer_open.set(false) }
                SidebarLink { to: Route::About {}, icon: "ⓘ", label: "About",
                    on_click: move |_| drawer_open.set(false) }
                div { class: "sidebar-foot",
                    span { class: "muted", "Data: {meta.source_name}" }
                    span { class: "muted", {scraped_date} }
                }
            }
            main { class: "content",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
fn SidebarLink(to: Route, icon: &'static str, label: &'static str, on_click: EventHandler<()>) -> Element {
    rsx! {
        Link { class: "side-link", to: to, active_class: "active", onclick: move |_| on_click.call(()),
            span { class: "side-ico", {icon} }
            span { {label} }
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "page",
            h1 { "Page not found" }
            p { "No route for {route:?}" }
            Link { class: "btn", to: Route::Home {}, "Back to home" }
        }
    }
}
