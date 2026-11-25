//! Liste des monstres (recherche + filtres) et fiche détaillée.

use crate::data::{db, Monster};
use crate::format::{matches, rank_label, size_label};
use crate::ui::*;
use crate::Route;
use dioxus::prelude::*;

const PAGE_SIZE: usize = 96;

#[component]
pub fn Monsters() -> Element {
    let d = db();
    let mut search = use_signal(String::new);
    let mut family = use_signal(|| "all".to_string());
    let mut rank = use_signal(|| "all".to_string());
    let mut size = use_signal(|| "all".to_string());
    let mut source = use_signal(|| "all".to_string());
    let mut sort = use_signal(|| "lib".to_string());
    let mut page = use_signal(|| 1usize);

    let filtered = use_memo(move || {
        let fam: u32 = family().parse().unwrap_or(0);
        let rk: u32 = rank().parse().unwrap_or(0);
        let sz: u32 = size().parse().unwrap_or(0);
        let src = source();
        let mut list: Vec<&Monster> = db()
            .monsters
            .iter()
            .filter(|m| {
                (fam == 0 || m.family_id == fam)
                    && (rk == 0 || m.rank == rk)
                    && (sz == 99 || m.size == sz)
                    && matches(&m.name, &search())
                    && match src.as_str() {
                        "wild" => m.wild,
                        "egg" => !m.egg_kinds.is_empty(),
                        "synth" => !m.wild && m.egg_kinds.is_empty(),
                        _ => true,
                    }
            })
            .collect();
        match sort().as_str() {
            "name" => list.sort_by(|a, b| a.name.cmp(&b.name)),
            "rank" => list.sort_by(|a, b| b.rank.cmp(&a.rank).then(a.name.cmp(&b.name))),
            "hp" => list.sort_by(|a, b| b.max[0].cmp(&a.max[0])),
            "mp" => list.sort_by(|a, b| b.max[1].cmp(&a.max[1])),
            "atk" => list.sort_by(|a, b| b.max[2].cmp(&a.max[2])),
            "def" => list.sort_by(|a, b| b.max[3].cmp(&a.max[3])),
            "agi" => list.sort_by(|a, b| b.max[4].cmp(&a.max[4])),
            "wis" => list.sort_by(|a, b| b.max[5].cmp(&a.max[5])),
            _ => list.sort_by_key(|m| m.lib.unwrap_or(u32::MAX)),
        }
        list
    });

    let total = filtered().len();
    let total_pages = total.div_ceil(PAGE_SIZE);
    let cur_page = page().clamp(1, total_pages.max(1));
    let slice: Vec<&Monster> = filtered()
        .iter()
        .skip((cur_page - 1) * PAGE_SIZE)
        .take(PAGE_SIZE)
        .copied()
        .collect();

    let fam_opts: Vec<(String, String)> = std::iter::once(("all".into(), "All families".into()))
        .chain(
            d.families
                .iter()
                .map(|f| (f.id.to_string(), format!("{} ({})", f.name, f.count))),
        )
        .collect();
    let rank_opts: Vec<(String, String)> = std::iter::once(("all".into(), "All ranks".into()))
        .chain((1..=8u32).map(|r| (r.to_string(), format!("Rank {}", rank_label(r)))))
        .collect();
    let size_opts: Vec<(String, String)> = vec![
        ("all".into(), "All sizes".into()),
        ("0".into(), "Small".into()),
        ("1".into(), "Medium".into()),
        ("2".into(), "Large".into()),
        ("3".into(), "Extra Large".into()),
    ];
    let src_opts: Vec<(String, String)> = vec![
        ("all".into(), "Any source".into()),
        ("wild".into(), "Wild / field".into()),
        ("egg".into(), "Egg hatch".into()),
        ("synth".into(), "Synthesis only".into()),
    ];
    let sort_opts: Vec<(String, String)> = vec![
        ("lib".into(), "Library #".into()),
        ("name".into(), "Name A-Z".into()),
        ("rank".into(), "Rank (best first)".into()),
        ("hp".into(), "Max HP".into()),
        ("mp".into(), "Max MP".into()),
        ("atk".into(), "Max ATK".into()),
        ("def".into(), "Max DEF".into()),
        ("agi".into(), "Max AGI".into()),
        ("wis".into(), "Max WIS".into()),
    ];

    rsx! {
        div { class: "page",
            h1 { "Monsters" }
            p { class: "muted", "{total} / {d.monsters.len()} monsters" }

            div { class: "controls card",
                SearchBox {
                    value: search(),
                    placeholder: "Search a monster…".to_string(),
                    on_input: move |v| { search.set(v); page.set(1); },
                }
                SelectFilter { value: family(), options: fam_opts,
                    on_change: move |v| { family.set(v); page.set(1); } }
                SelectFilter { value: rank(), options: rank_opts,
                    on_change: move |v| { rank.set(v); page.set(1); } }
                SelectFilter { value: size(), options: size_opts,
                    on_change: move |v| { size.set(v); page.set(1); } }
                SelectFilter { value: source(), options: src_opts,
                    on_change: move |v| { source.set(v); page.set(1); } }
                SelectFilter { value: sort(), options: sort_opts,
                    on_change: move |v| { sort.set(v); } }
            }

            div { class: "mgrid",
                {slice.iter().map(|m| {
                    rsx! {
                        Link { class: "mcard", key: "{m.id}", to: Route::MonsterDetail { id: m.id },
                            div { class: "mcard-ava",
                                MonsterAvatar { id: m.id }
                            }
                            div { class: "mcard-name", {m.name.clone()} }
                            div { class: "mcard-meta",
                                RankBadge { rank: m.rank, label: true }
                                FamilyIcon { family_id: m.family_id }
                                span { class: "mcard-fam", {m.family.clone()} }
                                SizeIcon { size: m.size }
                            }
                            div { class: "mcard-badges",
                                if m.wild { span { class: "tag tag-wild", "Wild" } }
                                if !m.egg_kinds.is_empty() { span { class: "tag tag-egg", "Egg" } }
                                if !m.wild && m.egg_kinds.is_empty() { span { class: "tag tag-synth", "Synth only" } }
                                if m.holy { span { class: "tag tag-holy", "Holy beast" } }
                            }
                        }
                    }
                })}
            }

            Pager { page: cur_page, total_pages: total_pages,
                on_change: move |p| page.set(p) }
        }
    }
}

#[component]
pub fn MonsterDetail(id: u32) -> Element {
    let d = db();
    let m = match d.monster(id) {
        Some(m) => m,
        None => {
            return rsx! {
                div { class: "page",
                    h1 { "Unknown monster #{id}" }
                    Link { class: "btn", to: Route::Monsters {}, "Back to monsters" }
                }
            }
        }
    };

    // Recettes où ce monstre est le résultat / un parent
    let made_from: Vec<(usize, Vec<u32>)> = d
        .recipes_by_result
        .get(&id)
        .map(|v| v.iter().map(|&ri| (ri, d.recipes[ri].p.clone())).collect())
        .unwrap_or_default();
    let used_in: Vec<u32> = d
        .recipes_by_parent
        .get(&id)
        .map(|v| v.iter().map(|&ri| d.recipes[ri].r).collect())
        .unwrap_or_default();

    // Probabilités d'éclosion par type d'œuf
    let egg_rows: Vec<(String, String)> = d
        .eggs
        .iter()
        .filter_map(|e| {
            e.monsters
                .iter()
                .find(|em| em.id == id)
                .map(|em| (e.name.clone(), crate::format::percent(em.num, em.den)))
        })
        .collect();

    let depth = d.min_depth.get(&id).copied().flatten();
    let depth_label = depth.map(|dep| match dep {
        0 => "Directly obtainable (wild / egg)".to_string(),
        1 => "1 generation of synthesis".to_string(),
        n => format!("{n} generations of synthesis"),
    });
    let lib_label = m
        .lib
        .map(|l| format!("Library #{l}"))
        .unwrap_or_else(|| "Library —".to_string());
    let skill_name = m.skill_name.clone().unwrap_or_default();

    rsx! {
        div { class: "page detail",
            div { class: "crumbs",
                Link { class: "crumb", to: Route::Monsters {}, "Monsters" }
                span { " / {m.name}" }
            }

            // ---- En-tête
            section { class: "card mhead",
                div { class: "mhead-ava", MonsterAvatar { id: id, big: true } }
                div { class: "mhead-main",
                    h1 { {m.name.clone()} }
                    div { class: "mhead-meta",
                        RankBadge { rank: m.rank, label: true }
                        span { class: "mhead-fam",
                            FamilyIcon { family_id: m.family_id }
                            "{m.family}"
                        }
                        SizeIcon { size: m.size }
                        span { class: "muted", {lib_label} }
                        if m.holy { span { class: "tag tag-holy", "Holy beast" } }
                    }
                    div { class: "mhead-badges",
                        if m.wild { span { class: "tag tag-wild", "Found in the wild" } }
                        if !m.egg_kinds.is_empty() { span { class: "tag tag-egg", "Hatches from eggs" } }
                        if !m.wild && m.egg_kinds.is_empty() { span { class: "tag tag-synth", "Synthesis only" } }
                        if let Some(t) = depth_label {
                            span { class: "tag tag-depth", {t} }
                        }
                    }
                    div { class: "mhead-actions",
                        Link { class: "btn primary", to: Route::Synthesis { target: Some(id) },
                            "⑃ View synthesis tree"
                        }
                    }
                }
                div { class: "mhead-stats",
                    h3 { "Max stats (Lv 100)" }
                    table { class: "tbl",
                        tbody {
                            StatRow { label: "HP", value: m.max[0], growth: m.growth.hp.clone() }
                            StatRow { label: "MP", value: m.max[1], growth: m.growth.mp.clone() }
                            StatRow { label: "Attack", value: m.max[2], growth: m.growth.atk.clone() }
                            StatRow { label: "Defense", value: m.max[3], growth: m.growth.def.clone() }
                            StatRow { label: "Agility", value: m.max[4], growth: m.growth.agi.clone() }
                            StatRow { label: "Wisdom", value: m.max[5], growth: m.growth.wis.clone() }
                        }
                    }
                    if let Some(e50) = m.exp50 {
                        p { class: "muted small", "EXP to Lv50: {e50}" }
                    }
                    if let Some(e100) = m.exp100 {
                        p { class: "muted small", "EXP to Lv100: {e100}" }
                    }
                }
            }

            // ---- Skill natif
            section { class: "card",
                h2 { class: "section-title", "Native skill" }
                if let Some(sid) = m.skill {
                    div { class: "skill-head",
                        Link { class: "chip big", to: Route::SkillDetail { id: sid },
                            {skill_name}
                        }
                        if let Some(sp) = m.sp0 {
                            span { class: "muted", "Starting SP: {sp}" }
                        }
                    }
                    if !m.slots.is_empty() {
                        SlotsTable { slots: m.slots.clone() }
                    }
                } else {
                    p { class: "muted", "No native skill." }
                }
            }

            // ---- Traits
            if !m.traits.is_empty() {
                section { class: "card",
                    h2 { class: "section-title", "Traits" }
                    ul { class: "traits",
                        {m.traits.iter().map(|t| {
                            let size_note = t.size.map(|sz| format!("({}+)", size_label(sz)));
                            rsx! {
                                li { key: "{t.t}", class: "trait", title: t.desc.clone(),
                                    span { class: "trait-name", {t.name.clone()} }
                                    if t.body { span { class: "tag tag-holy", "Body" } }
                                    if let Some(note) = size_note {
                                        span { class: "muted small", {note} }
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            // ---- Synthèse
            section { class: "card",
                h2 { class: "section-title", "Synthesis" }
                if !made_from.is_empty() {
                    h3 { class: "sub", "Made from" }
                    div { class: "recipes",
                        {made_from.iter().map(|(ri, parents)| {
                            let grand = d.recipes[*ri].t == "4g";
                            rsx! {
                                div { class: "recipe", key: "{ri}",
                                    if grand { span { class: "tag tag-warn", "Grand (4 parents)" } }
                                    div { class: "recipe-line",
                                        {parents.iter().map(|&p| {
                                            rsx! { span { key: "{p}", MonsterChip { id: p } } }
                                        })}
                                    }
                                }
                            }
                        })}
                    }
                } else {
                    p { class: "muted", "No explicit recipe produces this monster." }
                }
                if !used_in.is_empty() {
                    h3 { class: "sub", "Used as parent in" }
                    div { class: "chips",
                        {used_in.iter().map(|&r| {
                            rsx! { span { key: "{r}", MonsterChip { id: r } } }
                        })}
                    }
                }
                if !m.generic_breeding.is_empty() {
                    h3 { class: "sub", "Generic family rules (can produce this monster)" }
                    ul { class: "generic",
                        {m.generic_breeding.iter().enumerate().map(|(i, g)| {
                            let rank_note = g.rank.map(|r| format!(" — rank ≥ {}", rank_label(r)));
                            let lib_note = g.lib.map(|l| format!(" — library #{l}"));
                            let label_note = format!(" ({})", g.label);
                            rsx! {
                                li { key: "{i}",
                                    "{g.f1} × {g.f2}"
                                    if let Some(note) = rank_note {
                                        span { class: "muted", {note} }
                                    }
                                    if let Some(note) = lib_note {
                                        span { class: "muted", {note} }
                                    }
                                    span { class: "muted small", {label_note} }
                                }
                            }
                        })}
                    }
                }
            }

            // ---- Œufs
            if !egg_rows.is_empty() {
                section { class: "card",
                    h2 { class: "section-title", "Egg hatching" }
                    ul { class: "egg-list",
                        {egg_rows.iter().map(|(name, prob)| {
                            rsx! {
                                li { key: "{name}",
                                    span { class: "egg-name", {name.clone()} }
                                    span { class: "egg-prob", {prob.clone()} }
                                }
                            }
                        })}
                    }
                    Link { class: "btn", to: Route::Eggs {}, "Egg database" }
                }
            }

            // ---- Drops
            if !m.drops.is_empty() {
                section { class: "card",
                    h2 { class: "section-title", "Drops" }
                    table { class: "tbl",
                        thead { tr { th { "Item" }, th { "Chance" } } }
                        tbody {
                            {m.drops.iter().map(|dp| {
                                let chance = dp.den.map(|dd| format!("1 / {dd}")).unwrap_or_else(|| "-".into());
                                rsx! {
                                    tr { key: "{dp.item}",
                                        td { Link { class: "ilink", to: Route::ItemDetail { id: dp.item }, {dp.item_name.clone()} } }
                                        td { {chance} }
                                    }
                                }
                            })}
                        }
                    }
                }
            }

            // ---- Localisations
            if !m.locations.is_empty() {
                section { class: "card",
                    h2 { class: "section-title", "Locations" }
                    ul { class: "locs",
                        {m.locations.iter().enumerate().map(|(i, l)| {
                            let src_note = l.src.as_ref().map(|s| format!(" ({s})"));
                            rsx! {
                                li { key: "{i}",
                                    "{l.map}"
                                    if let Some(note) = src_note {
                                        span { class: "muted small", {note} }
                                    }
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
fn StatRow(label: &'static str, value: u32, growth: Vec<u32>) -> Element {
    let g = if growth.is_empty() {
        String::new()
    } else {
        growth
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" · ")
    };
    rsx! {
        tr {
            td { class: "stat-l", {label} }
            td { class: "stat-v", {value.to_string()} }
            td { class: "stat-g muted", title: "Growth pattern", {g} }
        }
    }
}

/// Tableau des slots d'un skill (SP → capacité / auto-skill).
#[component]
pub fn SlotsTable(slots: Vec<crate::data::SkillSlot>) -> Element {
    rsx! {
        table { class: "tbl slots",
            thead { tr { th { "SP" }, th { "Unlocks" }, th { "Type" } } }
            tbody {
                {slots.iter().enumerate().map(|(i, s)| {
                    let sp = s.sp.map(|v| v.to_string()).unwrap_or_else(|| "—".into());
                    let what = if let Some(name) = &s.ab_name {
                        name.clone()
                    } else if let Some(auto) = &s.auto {
                        auto.name.clone()
                    } else {
                        "—".to_string()
                    };
                    let kind = if s.ab.is_some() {
                        "Ability"
                    } else if s.auto.is_some() {
                        "Auto-skill"
                    } else {
                        "—"
                    };
                    let title = s.auto.as_ref().map(|a| a.desc.clone()).unwrap_or_default();
                    rsx! {
                        tr { key: "{i}", title: "{title}",
                            td { {sp} }
                            td { {what} }
                            td { class: "muted", {kind} }
                        }
                    }
                })}
            }
        }
    }
}
