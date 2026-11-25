//! Couche de données : structures sérialisables + base embarquée (100% offline).
//!
//! Toutes les données sont compilées dans le binaire via `include_str!` et
//! analysées une seule fois au démarrage. Aucun accès réseau n'est effectué.

use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

// ---------------------------------------------------------------- structures

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct AutoSkill {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub desc: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct SkillSlot {
    #[serde(default)]
    pub sp: Option<u32>,
    #[serde(default)]
    pub ab: Option<u32>,
    #[serde(default)]
    pub ab_name: Option<String>,
    #[serde(default)]
    pub auto: Option<AutoSkill>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct MonsterTrait {
    pub t: u32,
    pub name: String,
    #[serde(default)]
    pub desc: String,
    #[serde(default)]
    pub size: Option<u32>,
    #[serde(default)]
    pub body: bool,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct MonsterDrop {
    pub item: u32,
    pub item_name: String,
    #[serde(default)]
    pub den: Option<u32>,
    #[serde(default)]
    pub code: Option<u32>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct MonsterLocation {
    #[serde(default)]
    pub map: String,
    #[serde(default)]
    pub src: Option<String>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct GenericBreeding {
    #[serde(default)]
    pub f1: String,
    #[serde(default)]
    pub f2: String,
    #[serde(default)]
    pub rank: Option<u32>,
    #[serde(default)]
    pub lib: Option<u32>,
    #[serde(default)]
    pub label: String,
}

#[derive(Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Growth {
    #[serde(default)]
    pub hp: Vec<u32>,
    #[serde(default)]
    pub mp: Vec<u32>,
    #[serde(default)]
    pub atk: Vec<u32>,
    #[serde(default)]
    pub def: Vec<u32>,
    #[serde(default)]
    pub agi: Vec<u32>,
    #[serde(default)]
    pub wis: Vec<u32>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Monster {
    pub id: u32,
    pub name: String,
    pub family: String,
    pub family_id: u32,
    pub rank: u32,
    pub size: u32,
    #[serde(default)]
    pub lib: Option<u32>,
    pub sprite: bool,
    pub max: [u32; 6],
    #[serde(default)]
    pub growth: Growth,
    #[serde(default)]
    pub skill: Option<u32>,
    #[serde(default)]
    pub skill_name: Option<String>,
    #[serde(default)]
    pub sp0: Option<u32>,
    #[serde(default)]
    pub exp50: Option<u32>,
    #[serde(default)]
    pub exp100: Option<u32>,
    #[serde(default)]
    pub holy: bool,
    #[serde(default)]
    pub wild: bool,
    #[serde(default)]
    pub egg_kinds: Vec<u32>,
    #[serde(default)]
    pub slots: Vec<SkillSlot>,
    #[serde(default)]
    pub drops: Vec<MonsterDrop>,
    #[serde(default)]
    pub traits: Vec<MonsterTrait>,
    #[serde(default)]
    pub locations: Vec<MonsterLocation>,
    #[serde(default)]
    pub generic_breeding: Vec<GenericBreeding>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Recipe {
    pub r: u32,
    pub p: Vec<u32>,
    #[serde(default)]
    pub t: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct SkillEvoReq {
    pub skill: u32,
    pub name: String,
    #[serde(default)]
    pub sp: Option<u32>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct SpecialEvo {
    pub to: u32,
    pub to_name: String,
    #[serde(default)]
    pub req: Vec<SkillEvoReq>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Skill {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub evo_from: Vec<SkillEvoReq>,
    #[serde(default)]
    pub special: Vec<SpecialEvo>,
    #[serde(default)]
    pub evo_to: Option<u32>,
    #[serde(default)]
    pub evo_to_name: Option<String>,
    #[serde(default)]
    pub evo_cost: Option<u32>,
    #[serde(default)]
    pub slots: Vec<SkillSlot>,
    #[serde(default)]
    pub monsters: Vec<u32>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Item {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub desc: String,
    #[serde(default)]
    pub item_type: Option<String>,
    #[serde(default)]
    pub buy: Option<u32>,
    #[serde(default)]
    pub atk: Option<u32>,
    #[serde(default)]
    pub def: Option<u32>,
    #[serde(default)]
    pub agi: Option<u32>,
    #[serde(default)]
    pub wis: Option<u32>,
    #[serde(default)]
    pub hp: Option<u32>,
    #[serde(default)]
    pub mp: Option<u32>,
    #[serde(default)]
    pub break_p: Option<u32>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct GameTrait {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub desc: String,
}

#[derive(Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AbilityValues {
    #[serde(default)]
    pub primary_min: Option<i64>,
    #[serde(default)]
    pub primary_max: Option<i64>,
    #[serde(default)]
    pub alternate_min: Option<i64>,
    #[serde(default)]
    pub alternate_max: Option<i64>,
    #[serde(default)]
    pub wisdom_min: Option<i64>,
    #[serde(default)]
    pub wisdom_max: Option<i64>,
    #[serde(default)]
    pub cap: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Ability {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub desc: String,
    #[serde(default)]
    pub mp: Option<u32>,
    #[serde(default)]
    pub mpf: bool,
    #[serde(default)]
    pub cats: Vec<String>,
    #[serde(default)]
    pub vals: Option<AbilityValues>,
    #[serde(default)]
    pub res: Option<String>,
    #[serde(default)]
    pub tol1: Option<String>,
    #[serde(default)]
    pub tol2: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct EggSource {
    #[serde(default)]
    pub weight: Option<u32>,
    #[serde(default)]
    pub families: Option<Vec<String>>,
    #[serde(default)]
    pub group: Option<i64>,
    #[serde(default)]
    pub tier: Option<i64>,
    #[serde(default)]
    pub range: Vec<Option<u32>>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct EggMonster {
    pub id: u32,
    #[serde(default)]
    pub num: Option<u32>,
    #[serde(default)]
    pub den: Option<u32>,
    #[serde(default)]
    pub sources: Vec<EggSource>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct NameOption {
    pub key: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub is_empty: bool,
    #[serde(default)]
    pub weight: u32,
    #[serde(default)]
    pub total_weight: u32,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct NamePart {
    #[serde(default)]
    pub byte_offset: u32,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub options: Vec<NameOption>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct EggKind {
    pub kind: u32,
    pub name: String,
    #[serde(default)]
    pub item_id: Option<u32>,
    #[serde(default)]
    pub buy: Option<u32>,
    #[serde(default)]
    pub name_parts: Vec<NamePart>,
    pub monsters: Vec<EggMonster>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Family {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub count: u32,
}

#[derive(Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Meta {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub source_name: String,
    #[serde(default)]
    pub scraped_at: String,
    #[serde(default)]
    pub game: String,
    #[serde(default)]
    pub note: String,
}

// ---------------------------------------------------------------- database

pub struct Db {
    pub monsters: Vec<Monster>,
    pub by_id: HashMap<u32, usize>,
    pub recipes: Vec<Recipe>,
    pub recipes_by_result: HashMap<u32, Vec<usize>>,
    pub recipes_by_parent: HashMap<u32, Vec<usize>>,
    pub skills: Vec<Skill>,
    pub skill_by_id: HashMap<u32, usize>,
    pub items: Vec<Item>,
    pub item_by_id: HashMap<u32, usize>,
    pub traits: Vec<GameTrait>,
    pub abilities: Vec<Ability>,
    pub ability_by_id: HashMap<u32, usize>,
    pub eggs: Vec<EggKind>,
    pub families: Vec<Family>,
    pub meta: Meta,
    /// Nombre minimal de générations de synthèse pour obtenir un monstre
    /// (None = non obtenable via synthèse explicite).
    pub min_depth: HashMap<u32, Option<u32>>,
}

static DB: OnceLock<Db> = OnceLock::new();

pub fn db() -> &'static Db {
    DB.get_or_init(Db::load)
}

impl Db {
    fn load() -> Db {
        let monsters: Vec<Monster> =
            serde_json::from_str(include_str!("../data/monsters.json")).expect("monsters.json");
        let recipes: Vec<Recipe> =
            serde_json::from_str(include_str!("../data/recipes.json")).expect("recipes.json");
        let skills: Vec<Skill> =
            serde_json::from_str(include_str!("../data/skills.json")).expect("skills.json");
        let items: Vec<Item> =
            serde_json::from_str(include_str!("../data/items.json")).expect("items.json");
        let traits: Vec<GameTrait> =
            serde_json::from_str(include_str!("../data/traits.json")).expect("traits.json");
        let abilities: Vec<Ability> =
            serde_json::from_str(include_str!("../data/abilities.json")).expect("abilities.json");
        let eggs: Vec<EggKind> =
            serde_json::from_str(include_str!("../data/eggs.json")).expect("eggs.json");
        let families: Vec<Family> =
            serde_json::from_str(include_str!("../data/families.json")).expect("families.json");
        let meta: Meta =
            serde_json::from_str(include_str!("../data/meta.json")).expect("meta.json");

        let by_id = monsters.iter().enumerate().map(|(i, m)| (m.id, i)).collect();

        let mut recipes_by_result: HashMap<u32, Vec<usize>> = HashMap::new();
        let mut recipes_by_parent: HashMap<u32, Vec<usize>> = HashMap::new();
        for (i, r) in recipes.iter().enumerate() {
            recipes_by_result.entry(r.r).or_default().push(i);
            recipes_by_parent.entry(r.r).or_default(); // assure la clé
            for &p in &r.p {
                recipes_by_parent.entry(p).or_default().push(i);
            }
        }

        let skill_by_id = skills.iter().enumerate().map(|(i, s)| (s.id, i)).collect();
        let item_by_id = items.iter().enumerate().map(|(i, s)| (s.id, i)).collect();
        let ability_by_id = abilities.iter().enumerate().map(|(i, s)| (s.id, i)).collect();

        let obtainable: HashSet<u32> = monsters
            .iter()
            .filter(|m| m.wild || !m.egg_kinds.is_empty())
            .map(|m| m.id)
            .collect();
        let min_depth = compute_min_depth(&recipes, &recipes_by_result, &obtainable);

        Db {
            monsters,
            by_id,
            recipes,
            recipes_by_result,
            recipes_by_parent,
            skills,
            skill_by_id,
            items,
            item_by_id,
            traits,
            abilities,
            ability_by_id,
            eggs,
            families,
            meta,
            min_depth,
        }
    }

    pub fn monster(&self, id: u32) -> Option<&Monster> {
        self.by_id.get(&id).map(|&i| &self.monsters[i])
    }

    pub fn skill(&self, id: u32) -> Option<&Skill> {
        self.skill_by_id.get(&id).map(|&i| &self.skills[i])
    }

    pub fn item(&self, id: u32) -> Option<&Item> {
        self.item_by_id.get(&id).map(|&i| &self.items[i])
    }

    pub fn ability(&self, id: u32) -> Option<&Ability> {
        self.ability_by_id.get(&id).map(|&i| &self.abilities[i])
    }

    pub fn obtainable(&self, id: u32) -> bool {
        self.monster(id)
            .map(|m| m.wild || !m.egg_kinds.is_empty())
            .unwrap_or(false)
    }

    /// Étapes du plan de synthèse optimal pour une cible :
    /// liste de (génération, résultat, parents).
    pub fn plan_for(&self, target: u32) -> Option<Vec<(u32, u32, Vec<u32>)>> {
        if self.min_depth.get(&target).copied().flatten().is_none() {
            return None;
        }
        let mut out: Vec<(u32, u32, Vec<u32>)> = Vec::new();
        let mut visited: HashSet<u32> = HashSet::new();
        if !self.build_plan(target, &mut visited, &mut out) {
            return None;
        }
        out.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        Some(out)
    }

    fn build_plan(
        &self,
        id: u32,
        visited: &mut HashSet<u32>,
        out: &mut Vec<(u32, u32, Vec<u32>)>,
    ) -> bool {
        if self.obtainable(id) {
            return true;
        }
        if !visited.insert(id) {
            return false;
        }
        let target_depth = self.min_depth.get(&id).copied().flatten();
        let mut found = false;
        if let Some(list) = self.recipes_by_result.get(&id) {
            for &ri in list {
                let recipe = &self.recipes[ri];
                let mut max_parent = 0u32;
                let mut ok = true;
                for &p in &recipe.p {
                    match self.min_depth.get(&p).copied().flatten() {
                        Some(d) => max_parent = max_parent.max(d),
                        None => {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok && Some(max_parent + 1) == target_depth {
                    let mut sub_ok = true;
                    for &p in &recipe.p {
                        if !self.obtainable(p) && !self.build_plan(p, visited, out) {
                            sub_ok = false;
                            break;
                        }
                    }
                    if sub_ok {
                        out.push((max_parent + 1, id, recipe.p.clone()));
                        found = true;
                        break;
                    }
                }
            }
        }
        if !found {
            visited.remove(&id);
        }
        found
    }
}

/// Profondeur minimale de synthèse (DFS avec mémo + garde anti-cycle).
fn compute_min_depth(
    recipes: &[Recipe],
    recipes_by_result: &HashMap<u32, Vec<usize>>,
    obtainable: &HashSet<u32>,
) -> HashMap<u32, Option<u32>> {
    #[derive(Clone, Copy, PartialEq)]
    enum State {
        InProgress,
        Done(Option<u32>),
    }

    fn visit(
        id: u32,
        recipes: &[Recipe],
        by_result: &HashMap<u32, Vec<usize>>,
        obtainable: &HashSet<u32>,
        memo: &mut HashMap<u32, State>,
    ) -> Option<u32> {
        if let Some(State::Done(v)) = memo.get(&id) {
            return *v;
        }
        if obtainable.contains(&id) {
            memo.insert(id, State::Done(Some(0)));
            return Some(0);
        }
        memo.insert(id, State::InProgress);
        let mut best: Option<u32> = None;
        if let Some(list) = by_result.get(&id) {
            for &ri in list {
                let parents = &recipes[ri].p;
                let mut ok = true;
                let mut d = 0u32;
                for &p in parents {
                    if matches!(memo.get(&p), Some(State::InProgress)) {
                        ok = false; // cycle sur ce chemin
                        break;
                    }
                    match visit(p, recipes, by_result, obtainable, memo) {
                        Some(v) => d = d.max(v),
                        None => {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    let cand = d + 1;
                    best = Some(match best {
                        Some(b) => b.min(cand),
                        None => cand,
                    });
                }
            }
        }
        memo.insert(id, State::Done(best));
        best
    }

    let mut memo: HashMap<u32, State> = HashMap::new();
    let mut all_ids: Vec<u32> = obtainable.iter().copied().collect();
    all_ids.extend(recipes.iter().flat_map(|r| std::iter::once(r.r).chain(r.p.iter().copied())));
    all_ids.sort_unstable();
    all_ids.dedup();
    for id in all_ids {
        visit(id, recipes, recipes_by_result, obtainable, &mut memo);
    }
    memo.into_iter()
        .map(|(k, v)| {
            let d = match v {
                State::Done(x) => x,
                State::InProgress => None,
            };
            (k, d)
        })
        .collect()
}
