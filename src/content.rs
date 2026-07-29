#![allow(dead_code)]

use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};
use toml::Value;

pub const MIN_ORBIT_PERIOD_DAYS: f32 = 30.0;

#[derive(Debug, Default)]
pub struct ContentRegistry {
    pub packs: Vec<PackDef>,
    pub items: HashMap<String, ItemDef>,
    pub item_order: Vec<String>,
    pub ships: HashMap<String, ShipDef>,
    pub ship_order: Vec<String>,
    pub npc_ships: HashMap<String, NpcShipDef>,
    pub npc_ship_order: Vec<String>,
    pub shields: HashMap<String, ShieldDef>,
    pub shield_order: Vec<String>,
    pub weapons: HashMap<String, WeaponDef>,
    pub weapon_order: Vec<String>,
    pub power_modules: HashMap<String, PowerModuleDef>,
    pub power_module_order: Vec<String>,
    pub recipes: HashMap<String, RecipeDef>,
    pub recipe_order: Vec<String>,
    pub universes: HashMap<String, UniverseDef>,
    pub universe_order: Vec<String>,
    pub galaxy_groups: HashMap<String, GalaxyGroupDef>,
    pub galaxy_group_order: Vec<String>,
    pub galaxy_clusters: HashMap<String, GalaxyClusterDef>,
    pub galaxy_cluster_order: Vec<String>,
    pub galaxies: HashMap<String, GalaxyDef>,
    pub galaxy_order: Vec<String>,
    pub regions: HashMap<String, RegionDef>,
    pub region_order: Vec<String>,
    pub systems: HashMap<String, SystemDef>,
    pub system_order: Vec<String>,
    pub stars: HashMap<String, StarDef>,
    pub star_order: Vec<String>,
    pub planets: HashMap<String, PlanetDef>,
    pub planet_order: Vec<String>,
    pub stations: HashMap<String, StationDef>,
    pub station_order: Vec<String>,
    pub upgrades: HashMap<String, UpgradeDef>,
    pub upgrade_order: Vec<String>,
    pub starter_inventory: Vec<StackDef>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PackDef {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub path: PathBuf,
    pub depends_on: Vec<String>,
    pub optional_depends_on: Vec<String>,
    pub options: Vec<PackOptionDef>,
}

#[derive(Debug, Clone)]
pub struct PackOptionDef {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub value_type: PackOptionValueType,
    pub default: PackOptionValue,
    pub choices: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackOptionValueType {
    Bool,
    Integer,
    Number,
    Text,
    Choice,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PackOptionValue {
    Bool(bool),
    Integer(i64),
    Number(f32),
    Text(String),
    Choice(String),
}

impl PackOptionValue {
    pub fn as_save_string(&self) -> String {
        match self {
            Self::Bool(value) => value.to_string(),
            Self::Integer(value) => value.to_string(),
            Self::Number(value) => value.to_string(),
            Self::Text(value) | Self::Choice(value) => value.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    pub tier: String,
    pub xp_value: f32,
    pub unit_mass: f32,
}

#[derive(Debug, Clone)]
pub struct RecipeDef {
    pub id: String,
    pub station: String,
    pub output: StackDef,
    pub ingredients: Vec<StackDef>,
    pub purpose: Option<String>,
    pub allow_duplicate_output: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackDef {
    pub item: String,
    pub count: u32,
}

#[derive(Debug, Clone)]
pub struct ShipDef {
    pub id: String,
    pub name: String,
    pub texture: Option<String>,
    pub mass: f32,
    pub forward_acceleration: f32,
    pub reverse_acceleration: f32,
    pub turn_acceleration: f32,
    pub energy_capacity: f32,
    pub energy_recharge: f32,
    pub linear_drag: f32,
    pub hull_capacity: f32,
    pub shield_capacity: f32,
    pub power_modules: Vec<String>,
    pub shield_slots: Vec<String>,
    pub weapon_slots: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NpcShipDef {
    pub id: String,
    pub name: String,
    pub texture: Option<String>,
    pub system: String,
    pub position: [f32; 2],
    pub radius: f32,
    pub archetype: String,
    pub role: String,
    pub behavior_tags: Vec<String>,
    pub spawn_weight: f32,
    pub spawn_count: u32,
    pub mass: f32,
    pub cargo_capacity: f32,
    pub cargo_defaults: Vec<StackDef>,
    pub hull_capacity: f32,
    pub shield_capacity: f32,
    pub energy_capacity: f32,
    pub shield_slots: Vec<String>,
    pub weapon_slots: Vec<String>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ShieldDef {
    pub id: String,
    pub name: String,
    pub install_item: String,
    pub capacity: f32,
    pub recharge_delay: f32,
    pub recharge_rate: f32,
    pub damage_resistance: f32,
    pub hazard_resistance: f32,
    pub summary: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WeaponDef {
    pub id: String,
    pub name: String,
    pub kind: WeaponKind,
    pub install_item: String,
    pub range: f32,
    pub cooldown_seconds: f32,
    pub damage: f32,
    pub energy_cost: f32,
    pub tracking_degrees: f32,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponKind {
    TurretDefense,
}

impl WeaponKind {
    pub fn id(self) -> &'static str {
        match self {
            Self::TurretDefense => "turret_defense",
        }
    }

    fn from_id(id: &str) -> Option<Self> {
        match id {
            "turret_defense" => Some(Self::TurretDefense),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PowerModuleDef {
    pub id: String,
    pub name: String,
    pub family: String,
    pub install_item: String,
    pub generation: f32,
    pub mass: f32,
    pub fuel_item: Option<String>,
    pub fuel_per_minute: f32,
    pub heat: f32,
    pub risk: f32,
    pub summary: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UniverseDef {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GalaxyGroupDef {
    pub id: String,
    pub name: String,
    pub universe: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GalaxyClusterDef {
    pub id: String,
    pub name: String,
    pub galaxy_group: Option<String>,
    pub universe: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GalaxyDef {
    pub id: String,
    pub name: String,
    pub galaxy_cluster: Option<String>,
    pub galaxy_group: Option<String>,
    pub universe: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RegionDef {
    pub id: String,
    pub name: String,
    pub galaxy: Option<String>,
    pub galaxy_cluster: Option<String>,
    pub galaxy_group: Option<String>,
    pub universe: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SystemDef {
    pub id: String,
    pub name: String,
    pub region: Option<String>,
    pub galaxy: Option<String>,
    pub universe: Option<String>,
    pub primary_star: Option<String>,
    pub arrival: [f32; 2],
    pub description: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct StarDef {
    pub id: String,
    pub name: String,
    pub system: String,
    pub classification: String,
    pub color: [u8; 3],
    pub radius: f32,
    pub position: [f32; 2],
}

#[derive(Debug, Clone)]
pub struct PlanetDef {
    pub id: String,
    pub system: String,
    pub classification: String,
    pub texture: Option<String>,
    pub position: [f32; 2],
    pub orbit: Option<OrbitDef>,
    pub radius: f32,
    pub is_poi: bool,
    pub mineables: Vec<String>,
    pub hazards: Vec<String>,
    pub hazard_effects: HazardEffectsDef,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct OrbitDef {
    pub center: Option<[f32; 2]>,
    pub around: Option<String>,
    pub radius: f32,
    pub eccentricity: f32,
    pub axis_phase: f32,
    pub period_days: f32,
    pub phase: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HazardEffectsDef {
    pub shield_drain_per_second: f32,
    pub mining_speed_multiplier: f32,
}

#[derive(Debug, Clone)]
pub struct StationDef {
    pub id: String,
    pub name: String,
    pub skill: Option<String>,
    pub base_seconds: Option<f32>,
    pub system: Option<String>,
    pub position: Option<[f32; 2]>,
    pub radius: f32,
    pub texture: Option<String>,
    pub icon: String,
    pub culture: Option<String>,
    pub faction: Option<String>,
    pub summary: Option<String>,
    pub services: Vec<StationServiceDef>,
}

#[derive(Debug, Clone)]
pub struct StationServiceDef {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub description: Option<String>,
    pub trade: Vec<TradeStockDef>,
    pub recipe_unlocks: Vec<RecipeUnlockDef>,
}

#[derive(Debug, Clone)]
pub struct TradeStockDef {
    pub item: String,
    pub buy_price: u32,
    pub sell_price: u32,
    pub stock: Option<u32>,
    pub restock_days: Option<f32>,
    pub unavailable: bool,
}

#[derive(Debug, Clone)]
pub struct RecipeUnlockDef {
    pub recipe: String,
    pub price: u32,
    pub unavailable: bool,
}

#[derive(Debug, Clone)]
pub struct UpgradeDef {
    pub id: String,
    pub costs: Vec<UpgradeCostDef>,
}

#[derive(Debug, Clone)]
pub struct UpgradeCostDef {
    pub item: String,
    pub base_count: u32,
    pub per_level: u32,
    pub per_levels: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct PackManifest {
    id: String,
    name: String,
    version: String,
    description: Option<String>,
    #[serde(default)]
    depends_on: Vec<PackDependencyFileDef>,
    #[serde(default)]
    optional_depends_on: Vec<PackDependencyFileDef>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum PackDependencyFileDef {
    Id(String),
    Detailed { id: String, version: Option<String> },
}

impl PackDependencyFileDef {
    fn id(&self) -> &str {
        match self {
            Self::Id(id) | Self::Detailed { id, .. } => id,
        }
    }

    fn version(&self) -> Option<&str> {
        match self {
            Self::Id(_) => None,
            Self::Detailed { version, .. } => version.as_deref(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct PackConfigFile {
    #[serde(default)]
    options: Vec<PackOptionFileDef>,
}

#[derive(Debug, Deserialize)]
struct PackOptionFileDef {
    id: String,
    label: String,
    description: Option<String>,
    #[serde(rename = "type")]
    value_type: String,
    default: Value,
    #[serde(default)]
    choices: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
struct ItemsFile {
    #[serde(default)]
    items: Vec<ItemFileDef>,
}

#[derive(Debug, Deserialize)]
struct ItemFileDef {
    id: String,
    name: String,
    tier: String,
    xp_value: f32,
    unit_mass: f32,
}

#[derive(Debug, Default, Deserialize)]
struct RecipesFile {
    #[serde(default)]
    recipes: Vec<RecipeFileDef>,
}

#[derive(Debug, Deserialize)]
struct RecipeFileDef {
    id: String,
    station: String,
    output: StackFileDef,
    #[serde(default)]
    ingredients: Vec<StackFileDef>,
    purpose: Option<String>,
    #[serde(default)]
    allow_duplicate_output: bool,
}

#[derive(Debug, Deserialize)]
struct StackFileDef {
    item: String,
    count: u32,
}

#[derive(Debug, Default, Deserialize)]
struct ShipsFile {
    #[serde(default)]
    ships: Vec<ShipFileDef>,
}

#[derive(Debug, Deserialize)]
struct ShipFileDef {
    id: String,
    name: String,
    texture: Option<String>,
    mass: f32,
    forward_acceleration: f32,
    reverse_acceleration: f32,
    turn_acceleration: f32,
    energy_capacity: f32,
    energy_recharge: f32,
    linear_drag: f32,
    hull_capacity: f32,
    shield_capacity: f32,
    #[serde(default)]
    power_modules: Vec<String>,
    #[serde(default)]
    shield_slots: Vec<String>,
    #[serde(default)]
    weapon_slots: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
struct NpcShipsFile {
    #[serde(default)]
    npc_ships: Vec<NpcShipFileDef>,
}

#[derive(Debug, Deserialize)]
struct NpcShipFileDef {
    id: String,
    name: String,
    texture: Option<String>,
    system: String,
    position: [f32; 2],
    #[serde(default = "default_npc_ship_radius")]
    radius: f32,
    archetype: String,
    role: String,
    #[serde(default)]
    behavior_tags: Vec<String>,
    #[serde(default = "default_spawn_weight")]
    spawn_weight: f32,
    #[serde(default = "default_spawn_count")]
    spawn_count: u32,
    mass: f32,
    cargo_capacity: f32,
    #[serde(default)]
    cargo_defaults: Vec<StackFileDef>,
    hull_capacity: f32,
    shield_capacity: f32,
    energy_capacity: f32,
    #[serde(default)]
    shield_slots: Vec<String>,
    #[serde(default)]
    weapon_slots: Vec<String>,
    summary: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct ShieldsFile {
    #[serde(default)]
    shields: Vec<ShieldFileDef>,
}

#[derive(Debug, Deserialize)]
struct ShieldFileDef {
    id: String,
    name: String,
    install_item: String,
    capacity: f32,
    recharge_delay: f32,
    recharge_rate: f32,
    #[serde(default)]
    damage_resistance: f32,
    #[serde(default)]
    hazard_resistance: f32,
    summary: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct WeaponsFile {
    #[serde(default)]
    weapons: Vec<WeaponFileDef>,
}

#[derive(Debug, Deserialize)]
struct WeaponFileDef {
    id: String,
    name: String,
    kind: String,
    install_item: String,
    range: f32,
    cooldown_seconds: f32,
    damage: f32,
    #[serde(default)]
    energy_cost: f32,
    #[serde(default = "default_full_tracking_degrees")]
    tracking_degrees: f32,
    summary: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct PowerModulesFile {
    #[serde(default)]
    power_modules: Vec<PowerModuleFileDef>,
}

#[derive(Debug, Deserialize)]
struct PowerModuleFileDef {
    id: String,
    name: String,
    family: String,
    install_item: String,
    generation: f32,
    mass: f32,
    fuel_item: Option<String>,
    #[serde(default)]
    fuel_per_minute: f32,
    #[serde(default)]
    heat: f32,
    #[serde(default)]
    risk: f32,
    summary: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct UniverseFile {
    #[serde(default)]
    universes: Vec<UniverseFileDef>,
    #[serde(default)]
    galaxy_groups: Vec<GalaxyGroupFileDef>,
    #[serde(default)]
    galaxy_clusters: Vec<GalaxyClusterFileDef>,
    #[serde(default)]
    galaxies: Vec<GalaxyFileDef>,
    #[serde(default)]
    regions: Vec<RegionFileDef>,
}

#[derive(Debug, Default, Deserialize)]
struct SystemsFile {
    #[serde(default)]
    systems: Vec<SystemFileDef>,
    #[serde(default)]
    stars: Vec<StarFileDef>,
}

#[derive(Debug, Deserialize)]
struct UniverseFileDef {
    id: String,
    name: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GalaxyGroupFileDef {
    id: String,
    name: String,
    universe: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GalaxyClusterFileDef {
    id: String,
    name: String,
    galaxy_group: Option<String>,
    universe: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GalaxyFileDef {
    id: String,
    name: String,
    galaxy_cluster: Option<String>,
    galaxy_group: Option<String>,
    universe: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RegionFileDef {
    id: String,
    name: String,
    galaxy: Option<String>,
    galaxy_cluster: Option<String>,
    galaxy_group: Option<String>,
    universe: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SystemFileDef {
    id: String,
    name: String,
    region: Option<String>,
    galaxy: Option<String>,
    universe: Option<String>,
    primary_star: Option<String>,
    arrival: [f32; 2],
    description: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct StarFileDef {
    id: String,
    name: String,
    system: String,
    classification: String,
    color: [u8; 3],
    radius: f32,
    position: [f32; 2],
}

#[derive(Debug, Default, Deserialize)]
struct PlanetsFile {
    #[serde(default)]
    planets: Vec<PlanetFileDef>,
}

#[derive(Debug, Default, Deserialize)]
struct StationsFile {
    #[serde(default)]
    stations: Vec<StationFileDef>,
}

#[derive(Debug, Default, Deserialize)]
struct UpgradesFile {
    #[serde(default)]
    upgrades: Vec<UpgradeFileDef>,
}

#[derive(Debug, Deserialize)]
struct StationFileDef {
    id: String,
    name: String,
    skill: Option<String>,
    base_seconds: Option<f32>,
    system: Option<String>,
    position: Option<[f32; 2]>,
    #[serde(default = "default_station_radius")]
    radius: f32,
    texture: Option<String>,
    #[serde(default = "default_station_icon")]
    icon: String,
    culture: Option<String>,
    faction: Option<String>,
    summary: Option<String>,
    #[serde(default)]
    services: Vec<StationServiceFileDef>,
}

#[derive(Debug, Deserialize)]
struct StationServiceFileDef {
    id: String,
    name: String,
    kind: String,
    description: Option<String>,
    #[serde(default)]
    trade: Vec<TradeStockFileDef>,
    #[serde(default)]
    recipe_unlocks: Vec<RecipeUnlockFileDef>,
}

#[derive(Debug, Deserialize)]
struct TradeStockFileDef {
    item: String,
    buy_price: u32,
    sell_price: u32,
    stock: Option<u32>,
    restock_days: Option<f32>,
    #[serde(default)]
    unavailable: bool,
}

#[derive(Debug, Deserialize)]
struct RecipeUnlockFileDef {
    recipe: String,
    price: u32,
    #[serde(default)]
    unavailable: bool,
}

#[derive(Debug, Deserialize)]
struct UpgradeFileDef {
    id: String,
    #[serde(default)]
    costs: Vec<UpgradeCostFileDef>,
}

#[derive(Debug, Deserialize)]
struct UpgradeCostFileDef {
    item: String,
    base_count: u32,
    #[serde(default)]
    per_level: u32,
    #[serde(default = "default_one")]
    per_levels: u32,
}

#[derive(Debug, Default, Deserialize)]
struct StarterFile {
    #[serde(default)]
    inventory: Vec<StackFileDef>,
}

#[derive(Debug, Deserialize)]
struct PlanetFileDef {
    id: String,
    system: String,
    classification: String,
    texture: Option<String>,
    position: [f32; 2],
    orbit: Option<OrbitFileDef>,
    radius: f32,
    #[serde(default = "default_true")]
    is_poi: bool,
    #[serde(default)]
    mineables: Vec<String>,
    #[serde(default)]
    hazards: Vec<String>,
    #[serde(default)]
    hazard_effects: HazardEffectsFileDef,
    summary: String,
}

#[derive(Debug, Clone, Deserialize)]
struct OrbitFileDef {
    center: Option<[f32; 2]>,
    around: Option<String>,
    radius: f32,
    #[serde(default)]
    eccentricity: f32,
    #[serde(default)]
    axis_phase: f32,
    period_days: f32,
    #[serde(default)]
    phase: f32,
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct HazardEffectsFileDef {
    #[serde(default)]
    shield_drain_per_second: f32,
    #[serde(default = "default_one_f32")]
    mining_speed_multiplier: f32,
}

#[derive(Clone)]
struct RawPack {
    manifest: PackManifest,
    path: PathBuf,
}

pub fn load_content_packs(root: &Path) -> Result<ContentRegistry, Vec<String>> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let raw_packs = discover_packs(root, &mut errors);
    if !errors.is_empty() {
        return Err(errors);
    }

    let raw_packs = select_loadable_packs(raw_packs, &mut warnings);
    let ordered_packs = sort_packs(raw_packs, &mut errors);
    if !errors.is_empty() {
        return Err(errors);
    }

    let mut registry = ContentRegistry::default();
    registry.warnings.extend(warnings);
    for raw_pack in ordered_packs {
        load_pack(raw_pack, &mut registry, &mut errors);
    }

    validate_references(&registry, &mut errors);
    collect_duplicate_recipe_output_warnings(&mut registry);
    if errors.is_empty() {
        Ok(registry)
    } else {
        Err(errors)
    }
}

fn discover_packs(root: &Path, errors: &mut Vec<String>) -> Vec<RawPack> {
    let Ok(entries) = fs::read_dir(root) else {
        errors.push(format!("Content pack root missing: {}", root.display()));
        return Vec::new();
    };

    let mut paths = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    paths.sort();

    let mut seen = HashSet::new();
    let mut packs = Vec::new();
    for path in paths {
        let manifest_path = path.join("pack.toml");
        let Some(manifest) = read_toml::<PackManifest>(&manifest_path, errors) else {
            continue;
        };

        if !valid_pack_id(&manifest.id) {
            errors.push(format!(
                "Invalid pack id `{}` in {}",
                manifest.id,
                manifest_path.display()
            ));
        }
        if !seen.insert(manifest.id.clone()) {
            errors.push(format!("Duplicate pack id `{}`", manifest.id));
        }
        if manifest.name.trim().is_empty() {
            errors.push(format!("Pack `{}` has an empty name", manifest.id));
        }
        if manifest.version.trim().is_empty() {
            errors.push(format!("Pack `{}` has an empty version", manifest.id));
        }
        validate_pack_dependency_declarations(&manifest, errors);

        packs.push(RawPack { manifest, path });
    }

    packs
}

fn select_loadable_packs(raw_packs: Vec<RawPack>, warnings: &mut Vec<String>) -> Vec<RawPack> {
    let versions_by_id = raw_packs
        .iter()
        .map(|pack| (pack.manifest.id.clone(), pack.manifest.version.clone()))
        .collect::<HashMap<_, _>>();
    let mut loadable_ids = versions_by_id.keys().cloned().collect::<HashSet<_>>();

    loop {
        let mut changed = false;
        for pack in &raw_packs {
            if !loadable_ids.contains(&pack.manifest.id) {
                continue;
            }
            for dependency in &pack.manifest.optional_depends_on {
                let dependency_id = dependency.id();
                let Some(installed_version) = versions_by_id.get(dependency_id) else {
                    warnings.push(format!(
                        "Skipping pack `{}` because optional dependency `{dependency_id}` is not installed",
                        pack.manifest.id
                    ));
                    loadable_ids.remove(&pack.manifest.id);
                    changed = true;
                    break;
                };
                if !loadable_ids.contains(dependency_id) {
                    warnings.push(format!(
                        "Skipping pack `{}` because optional dependency `{dependency_id}` is not loaded",
                        pack.manifest.id
                    ));
                    loadable_ids.remove(&pack.manifest.id);
                    changed = true;
                    break;
                }
                if let Some(required_version) = dependency.version() {
                    if installed_version != required_version {
                        warnings.push(format!(
                            "Skipping pack `{}` because optional dependency `{dependency_id}` requires version `{required_version}` but installed version is `{installed_version}`",
                            pack.manifest.id
                        ));
                        loadable_ids.remove(&pack.manifest.id);
                        changed = true;
                        break;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    raw_packs
        .into_iter()
        .filter(|pack| loadable_ids.contains(&pack.manifest.id))
        .collect()
}

fn sort_packs(raw_packs: Vec<RawPack>, errors: &mut Vec<String>) -> Vec<RawPack> {
    let by_id = raw_packs
        .into_iter()
        .map(|pack| (pack.manifest.id.clone(), pack))
        .collect::<HashMap<_, _>>();
    let mut ordered = Vec::new();
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    let mut ids = by_id.keys().cloned().collect::<Vec<_>>();
    ids.sort();

    for id in ids {
        visit_pack(
            &id,
            &by_id,
            &mut visiting,
            &mut visited,
            &mut ordered,
            errors,
        );
    }

    ordered
}

fn visit_pack(
    id: &str,
    by_id: &HashMap<String, RawPack>,
    visiting: &mut HashSet<String>,
    visited: &mut HashSet<String>,
    ordered: &mut Vec<RawPack>,
    errors: &mut Vec<String>,
) {
    if visited.contains(id) {
        return;
    }
    if !visiting.insert(id.to_string()) {
        errors.push(format!("Cyclic content pack dependency involving `{id}`"));
        return;
    }

    let Some(pack) = by_id.get(id) else {
        errors.push(format!("Missing content pack dependency `{id}`"));
        visiting.remove(id);
        return;
    };

    for dependency in pack
        .manifest
        .depends_on
        .iter()
        .chain(pack.manifest.optional_depends_on.iter())
    {
        let dependency_id = dependency.id();
        let Some(dependency_pack) = by_id.get(dependency_id) else {
            errors.push(format!(
                "Pack `{}` depends on missing pack `{dependency_id}`",
                pack.manifest.id
            ));
            continue;
        };
        if let Some(required_version) = dependency.version() {
            if dependency_pack.manifest.version != required_version {
                errors.push(format!(
                    "Pack `{}` depends on `{dependency_id}` version `{required_version}` but installed version is `{}`",
                    pack.manifest.id, dependency_pack.manifest.version
                ));
                continue;
            }
        }
        visit_pack(dependency_id, by_id, visiting, visited, ordered, errors);
    }

    visiting.remove(id);
    visited.insert(id.to_string());
    ordered.push(pack.clone());
}

fn load_pack(raw_pack: RawPack, registry: &mut ContentRegistry, errors: &mut Vec<String>) {
    let pack_id = raw_pack.manifest.id.clone();
    let config = read_optional_toml::<PackConfigFile>(&raw_pack.path.join("config.toml"), errors);
    let options = resolve_pack_options(&pack_id, config.options, errors);
    registry.packs.push(PackDef {
        id: pack_id.clone(),
        name: raw_pack.manifest.name,
        version: raw_pack.manifest.version,
        description: raw_pack.manifest.description,
        path: raw_pack.path.clone(),
        depends_on: raw_pack
            .manifest
            .depends_on
            .into_iter()
            .map(|dependency| dependency.id().to_string())
            .collect(),
        optional_depends_on: raw_pack
            .manifest
            .optional_depends_on
            .into_iter()
            .map(|dependency| dependency.id().to_string())
            .collect(),
        options,
    });

    let items = read_optional_toml::<ItemsFile>(&raw_pack.path.join("items.toml"), errors);
    for item in items.items {
        let id = namespaced_id(&pack_id, &item.id);
        validate_local_content_id(&id, "item", errors);
        if item.name.trim().is_empty() {
            errors.push(format!("Item `{id}` has an empty name"));
        }
        if item.tier.trim().is_empty() {
            errors.push(format!("Item `{id}` has an empty tier"));
        }
        if item.unit_mass <= 0.0 {
            errors.push(format!("Item `{id}` has non-positive unit mass"));
        }
        let inserted = registry
            .items
            .insert(
                id.clone(),
                ItemDef {
                    id: id.clone(),
                    name: item.name,
                    tier: item.tier,
                    xp_value: item.xp_value,
                    unit_mass: item.unit_mass,
                },
            )
            .is_none();
        if inserted {
            registry.item_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate item id `{id}`"));
        }
    }

    let recipes = read_optional_toml::<RecipesFile>(&raw_pack.path.join("recipes.toml"), errors);
    let power_modules =
        read_optional_toml::<PowerModulesFile>(&raw_pack.path.join("power.toml"), errors);
    for module in power_modules.power_modules {
        let id = namespaced_id(&pack_id, &module.id);
        validate_local_content_id(&id, "power module", errors);
        validate_required_name(&id, "Power module", &module.name, errors);
        validate_required_name(&id, "Power module family", &module.family, errors);
        validate_positive(module.generation, "Power module", &id, "generation", errors);
        validate_positive(module.mass, "Power module", &id, "mass", errors);
        if module.fuel_per_minute < 0.0 {
            errors.push(format!(
                "Power module `{id}` has negative fuel use per minute"
            ));
        }
        if module.heat < 0.0 {
            errors.push(format!("Power module `{id}` has negative heat"));
        }
        if module.risk < 0.0 {
            errors.push(format!("Power module `{id}` has negative risk"));
        }
        let fuel_item = module
            .fuel_item
            .map(|fuel_item| namespaced_id(&pack_id, &fuel_item));
        let inserted = registry
            .power_modules
            .insert(
                id.clone(),
                PowerModuleDef {
                    id: id.clone(),
                    name: module.name,
                    family: module.family,
                    install_item: namespaced_id(&pack_id, &module.install_item),
                    generation: module.generation,
                    mass: module.mass,
                    fuel_item,
                    fuel_per_minute: module.fuel_per_minute,
                    heat: module.heat,
                    risk: module.risk,
                    summary: module.summary,
                },
            )
            .is_none();
        if inserted {
            registry.power_module_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate power module id `{id}`"));
        }
    }

    let weapons = read_optional_toml::<WeaponsFile>(&raw_pack.path.join("weapons.toml"), errors);
    let shields = read_optional_toml::<ShieldsFile>(&raw_pack.path.join("shields.toml"), errors);
    for shield in shields.shields {
        let id = namespaced_id(&pack_id, &shield.id);
        validate_local_content_id(&id, "shield", errors);
        validate_required_name(&id, "Shield", &shield.name, errors);
        validate_positive(shield.capacity, "Shield", &id, "capacity", errors);
        validate_positive(
            shield.recharge_delay,
            "Shield",
            &id,
            "recharge delay",
            errors,
        );
        validate_positive(shield.recharge_rate, "Shield", &id, "recharge rate", errors);
        validate_fraction(
            shield.damage_resistance,
            "Shield",
            &id,
            "damage resistance",
            errors,
        );
        validate_fraction(
            shield.hazard_resistance,
            "Shield",
            &id,
            "hazard resistance",
            errors,
        );
        let inserted = registry
            .shields
            .insert(
                id.clone(),
                ShieldDef {
                    id: id.clone(),
                    name: shield.name,
                    install_item: namespaced_id(&pack_id, &shield.install_item),
                    capacity: shield.capacity,
                    recharge_delay: shield.recharge_delay,
                    recharge_rate: shield.recharge_rate,
                    damage_resistance: shield.damage_resistance,
                    hazard_resistance: shield.hazard_resistance,
                    summary: shield.summary,
                },
            )
            .is_none();
        if inserted {
            registry.shield_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate shield id `{id}`"));
        }
    }

    for weapon in weapons.weapons {
        let id = namespaced_id(&pack_id, &weapon.id);
        validate_local_content_id(&id, "weapon", errors);
        validate_required_name(&id, "Weapon", &weapon.name, errors);
        validate_positive(weapon.range, "Weapon", &id, "range", errors);
        validate_positive(weapon.cooldown_seconds, "Weapon", &id, "cooldown", errors);
        validate_positive(weapon.damage, "Weapon", &id, "damage", errors);
        if weapon.energy_cost < 0.0 {
            errors.push(format!("Weapon `{id}` has negative energy cost"));
        }
        if weapon.tracking_degrees < 0.0 {
            errors.push(format!("Weapon `{id}` has negative tracking degrees"));
        }
        let Some(kind) = WeaponKind::from_id(&weapon.kind) else {
            errors.push(format!(
                "Weapon `{id}` has unsupported kind `{}`",
                weapon.kind
            ));
            continue;
        };
        let inserted = registry
            .weapons
            .insert(
                id.clone(),
                WeaponDef {
                    id: id.clone(),
                    name: weapon.name,
                    kind,
                    install_item: namespaced_id(&pack_id, &weapon.install_item),
                    range: weapon.range,
                    cooldown_seconds: weapon.cooldown_seconds,
                    damage: weapon.damage,
                    energy_cost: weapon.energy_cost,
                    tracking_degrees: weapon.tracking_degrees,
                    summary: weapon.summary,
                },
            )
            .is_none();
        if inserted {
            registry.weapon_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate weapon id `{id}`"));
        }
    }

    let ships = read_optional_toml::<ShipsFile>(&raw_pack.path.join("ships.toml"), errors);
    for ship in ships.ships {
        let id = namespaced_id(&pack_id, &ship.id);
        validate_local_content_id(&id, "ship", errors);
        validate_required_name(&id, "Ship", &ship.name, errors);
        validate_positive(ship.mass, "Ship", &id, "mass", errors);
        validate_positive(
            ship.forward_acceleration,
            "Ship",
            &id,
            "forward acceleration",
            errors,
        );
        validate_positive(
            ship.reverse_acceleration,
            "Ship",
            &id,
            "reverse acceleration",
            errors,
        );
        validate_positive(
            ship.turn_acceleration,
            "Ship",
            &id,
            "turn acceleration",
            errors,
        );
        validate_positive(ship.energy_capacity, "Ship", &id, "energy capacity", errors);
        validate_positive(ship.energy_recharge, "Ship", &id, "energy recharge", errors);
        validate_positive(ship.linear_drag, "Ship", &id, "linear drag", errors);
        validate_positive(ship.hull_capacity, "Ship", &id, "hull capacity", errors);
        validate_positive(ship.shield_capacity, "Ship", &id, "shield capacity", errors);
        let texture = ship
            .texture
            .map(|texture| resolve_texture_path(&raw_pack.path, &texture, &id, errors));
        let inserted = registry
            .ships
            .insert(
                id.clone(),
                ShipDef {
                    id: id.clone(),
                    name: ship.name,
                    texture,
                    mass: ship.mass,
                    forward_acceleration: ship.forward_acceleration,
                    reverse_acceleration: ship.reverse_acceleration,
                    turn_acceleration: ship.turn_acceleration,
                    energy_capacity: ship.energy_capacity,
                    energy_recharge: ship.energy_recharge,
                    linear_drag: ship.linear_drag,
                    hull_capacity: ship.hull_capacity,
                    shield_capacity: ship.shield_capacity,
                    power_modules: ship
                        .power_modules
                        .into_iter()
                        .map(|module| namespaced_id(&pack_id, &module))
                        .collect(),
                    shield_slots: ship
                        .shield_slots
                        .into_iter()
                        .map(|shield| namespaced_id(&pack_id, &shield))
                        .collect(),
                    weapon_slots: ship
                        .weapon_slots
                        .into_iter()
                        .map(|weapon| namespaced_id(&pack_id, &weapon))
                        .collect(),
                },
            )
            .is_none();
        if inserted {
            registry.ship_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate ship id `{id}`"));
        }
    }

    let npc_ships =
        read_optional_toml::<NpcShipsFile>(&raw_pack.path.join("npc_ships.toml"), errors);
    for npc_ship in npc_ships.npc_ships {
        let id = namespaced_id(&pack_id, &npc_ship.id);
        validate_local_content_id(&id, "NPC ship", errors);
        validate_required_name(&id, "NPC ship", &npc_ship.name, errors);
        validate_required_name(&id, "NPC ship archetype", &npc_ship.archetype, errors);
        validate_required_name(&id, "NPC ship role", &npc_ship.role, errors);
        validate_positive(npc_ship.radius, "NPC ship", &id, "radius", errors);
        validate_positive(
            npc_ship.spawn_weight,
            "NPC ship",
            &id,
            "spawn weight",
            errors,
        );
        if npc_ship.spawn_count == 0 {
            errors.push(format!("NPC ship `{id}` has zero spawn count"));
        }
        validate_positive(npc_ship.mass, "NPC ship", &id, "mass", errors);
        validate_positive(
            npc_ship.cargo_capacity,
            "NPC ship",
            &id,
            "cargo capacity",
            errors,
        );
        validate_positive(
            npc_ship.hull_capacity,
            "NPC ship",
            &id,
            "hull capacity",
            errors,
        );
        validate_positive(
            npc_ship.shield_capacity,
            "NPC ship",
            &id,
            "shield capacity",
            errors,
        );
        validate_positive(
            npc_ship.energy_capacity,
            "NPC ship",
            &id,
            "energy capacity",
            errors,
        );
        let texture = npc_ship
            .texture
            .map(|texture| resolve_texture_path(&raw_pack.path, &texture, &id, errors));
        let cargo_defaults = npc_ship
            .cargo_defaults
            .into_iter()
            .map(|stack| {
                let stack = resolve_stack(&pack_id, stack);
                if stack.count == 0 {
                    errors.push(format!("NPC ship `{id}` has a zero-count cargo default"));
                }
                stack
            })
            .collect::<Vec<_>>();
        let inserted = registry
            .npc_ships
            .insert(
                id.clone(),
                NpcShipDef {
                    id: id.clone(),
                    name: npc_ship.name,
                    texture,
                    system: namespaced_id(&pack_id, &npc_ship.system),
                    position: npc_ship.position,
                    radius: npc_ship.radius,
                    archetype: npc_ship.archetype,
                    role: npc_ship.role,
                    behavior_tags: npc_ship.behavior_tags,
                    spawn_weight: npc_ship.spawn_weight,
                    spawn_count: npc_ship.spawn_count,
                    mass: npc_ship.mass,
                    cargo_capacity: npc_ship.cargo_capacity,
                    cargo_defaults,
                    hull_capacity: npc_ship.hull_capacity,
                    shield_capacity: npc_ship.shield_capacity,
                    energy_capacity: npc_ship.energy_capacity,
                    shield_slots: npc_ship
                        .shield_slots
                        .into_iter()
                        .map(|shield| namespaced_id(&pack_id, &shield))
                        .collect(),
                    weapon_slots: npc_ship
                        .weapon_slots
                        .into_iter()
                        .map(|weapon| namespaced_id(&pack_id, &weapon))
                        .collect(),
                    summary: npc_ship.summary,
                },
            )
            .is_none();
        if inserted {
            registry.npc_ship_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate NPC ship id `{id}`"));
        }
    }

    for recipe in recipes.recipes {
        let id = namespaced_id(&pack_id, &recipe.id);
        validate_local_content_id(&id, "recipe", errors);
        let output = resolve_stack(&pack_id, recipe.output);
        if output.count == 0 {
            errors.push(format!("Recipe `{id}` has zero output count"));
        }
        if recipe.ingredients.is_empty() {
            errors.push(format!("Recipe `{id}` has no ingredients"));
        }
        let ingredients = recipe
            .ingredients
            .into_iter()
            .map(|ingredient| {
                let ingredient = resolve_stack(&pack_id, ingredient);
                if ingredient.count == 0 {
                    errors.push(format!("Recipe `{id}` has a zero-count ingredient"));
                }
                ingredient
            })
            .collect::<Vec<_>>();
        let inserted = registry
            .recipes
            .insert(
                id.clone(),
                RecipeDef {
                    id: id.clone(),
                    station: namespaced_id(&pack_id, &recipe.station),
                    output,
                    ingredients,
                    purpose: recipe.purpose,
                    allow_duplicate_output: recipe.allow_duplicate_output,
                },
            )
            .is_none();
        if inserted {
            registry.recipe_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate recipe id `{id}`"));
        }
    }

    let universe = read_optional_toml::<UniverseFile>(&raw_pack.path.join("universe.toml"), errors);
    for universe_def in universe.universes {
        let id = namespaced_id(&pack_id, &universe_def.id);
        validate_local_content_id(&id, "universe", errors);
        validate_required_name(&id, "Universe", &universe_def.name, errors);
        let inserted = registry
            .universes
            .insert(
                id.clone(),
                UniverseDef {
                    id: id.clone(),
                    name: universe_def.name,
                    description: universe_def.description,
                },
            )
            .is_none();
        if inserted {
            registry.universe_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate universe id `{id}`"));
        }
    }
    for group in universe.galaxy_groups {
        let id = namespaced_id(&pack_id, &group.id);
        validate_local_content_id(&id, "galaxy group", errors);
        validate_required_name(&id, "Galaxy group", &group.name, errors);
        let inserted = registry
            .galaxy_groups
            .insert(
                id.clone(),
                GalaxyGroupDef {
                    id: id.clone(),
                    name: group.name,
                    universe: group
                        .universe
                        .map(|universe| namespaced_id(&pack_id, &universe)),
                    description: group.description,
                },
            )
            .is_none();
        if inserted {
            registry.galaxy_group_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate galaxy group id `{id}`"));
        }
    }
    for cluster in universe.galaxy_clusters {
        let id = namespaced_id(&pack_id, &cluster.id);
        validate_local_content_id(&id, "galaxy cluster", errors);
        validate_required_name(&id, "Galaxy cluster", &cluster.name, errors);
        let inserted = registry
            .galaxy_clusters
            .insert(
                id.clone(),
                GalaxyClusterDef {
                    id: id.clone(),
                    name: cluster.name,
                    galaxy_group: cluster
                        .galaxy_group
                        .map(|galaxy_group| namespaced_id(&pack_id, &galaxy_group)),
                    universe: cluster
                        .universe
                        .map(|universe| namespaced_id(&pack_id, &universe)),
                    description: cluster.description,
                },
            )
            .is_none();
        if inserted {
            registry.galaxy_cluster_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate galaxy cluster id `{id}`"));
        }
    }
    for galaxy in universe.galaxies {
        let id = namespaced_id(&pack_id, &galaxy.id);
        validate_local_content_id(&id, "galaxy", errors);
        validate_required_name(&id, "Galaxy", &galaxy.name, errors);
        let inserted = registry
            .galaxies
            .insert(
                id.clone(),
                GalaxyDef {
                    id: id.clone(),
                    name: galaxy.name,
                    galaxy_cluster: galaxy
                        .galaxy_cluster
                        .map(|galaxy_cluster| namespaced_id(&pack_id, &galaxy_cluster)),
                    galaxy_group: galaxy
                        .galaxy_group
                        .map(|galaxy_group| namespaced_id(&pack_id, &galaxy_group)),
                    universe: galaxy
                        .universe
                        .map(|universe| namespaced_id(&pack_id, &universe)),
                    description: galaxy.description,
                },
            )
            .is_none();
        if inserted {
            registry.galaxy_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate galaxy id `{id}`"));
        }
    }
    for region in universe.regions {
        let id = namespaced_id(&pack_id, &region.id);
        validate_local_content_id(&id, "region", errors);
        validate_required_name(&id, "Region", &region.name, errors);
        let inserted = registry
            .regions
            .insert(
                id.clone(),
                RegionDef {
                    id: id.clone(),
                    name: region.name,
                    galaxy: region.galaxy.map(|galaxy| namespaced_id(&pack_id, &galaxy)),
                    galaxy_cluster: region
                        .galaxy_cluster
                        .map(|galaxy_cluster| namespaced_id(&pack_id, &galaxy_cluster)),
                    galaxy_group: region
                        .galaxy_group
                        .map(|galaxy_group| namespaced_id(&pack_id, &galaxy_group)),
                    universe: region
                        .universe
                        .map(|universe| namespaced_id(&pack_id, &universe)),
                    description: region.description,
                },
            )
            .is_none();
        if inserted {
            registry.region_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate region id `{id}`"));
        }
    }
    let systems = read_optional_toml::<SystemsFile>(&raw_pack.path.join("systems.toml"), errors);
    for system in systems.systems {
        let id = namespaced_id(&pack_id, &system.id);
        validate_local_content_id(&id, "system", errors);
        validate_required_name(&id, "System", &system.name, errors);
        let inserted = registry
            .systems
            .insert(
                id.clone(),
                SystemDef {
                    id: id.clone(),
                    name: system.name,
                    region: system.region.map(|region| namespaced_id(&pack_id, &region)),
                    galaxy: system.galaxy.map(|galaxy| namespaced_id(&pack_id, &galaxy)),
                    universe: system
                        .universe
                        .map(|universe| namespaced_id(&pack_id, &universe)),
                    primary_star: system
                        .primary_star
                        .map(|primary_star| namespaced_id(&pack_id, &primary_star)),
                    arrival: system.arrival,
                    description: system.description,
                    tags: system.tags,
                },
            )
            .is_none();
        if inserted {
            registry.system_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate system id `{id}`"));
        }
    }
    for star in systems.stars {
        let id = namespaced_id(&pack_id, &star.id);
        validate_local_content_id(&id, "star", errors);
        validate_required_name(&id, "Star", &star.name, errors);
        if star.classification.trim().is_empty() {
            errors.push(format!("Star `{id}` has an empty classification"));
        }
        if star.radius <= 0.0 {
            errors.push(format!("Star `{id}` has a non-positive radius"));
        }
        let inserted = registry
            .stars
            .insert(
                id.clone(),
                StarDef {
                    id: id.clone(),
                    name: star.name,
                    system: namespaced_id(&pack_id, &star.system),
                    classification: star.classification,
                    color: star.color,
                    radius: star.radius,
                    position: star.position,
                },
            )
            .is_none();
        if inserted {
            registry.star_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate star id `{id}`"));
        }
    }

    let planets = read_optional_toml::<PlanetsFile>(&raw_pack.path.join("planets.toml"), errors);
    for planet in planets.planets {
        let id = namespaced_id(&pack_id, &planet.id);
        validate_local_content_id(&id, "planet", errors);
        if planet.classification.trim().is_empty() {
            errors.push(format!("Planet `{id}` has an empty classification"));
        }
        if planet.summary.trim().is_empty() {
            errors.push(format!("Planet `{id}` has an empty summary"));
        }
        if planet.radius <= 0.0 {
            errors.push(format!("Planet `{id}` has a non-positive radius"));
        }
        if let Some(orbit) = &planet.orbit {
            if orbit.radius <= 0.0 {
                errors.push(format!("Planet `{id}` has a non-positive orbit radius"));
            }
            if !(0.0..0.85).contains(&orbit.eccentricity) {
                errors.push(format!(
                    "Planet `{id}` has orbit eccentricity {:.2} outside the supported range 0.00..0.85",
                    orbit.eccentricity
                ));
            }
            if orbit.period_days <= 0.0 {
                errors.push(format!("Planet `{id}` has a non-positive orbit period"));
            } else if orbit.period_days < MIN_ORBIT_PERIOD_DAYS {
                errors.push(format!(
                    "Planet `{id}` has orbit period {:.1} days below the minimum {:.1} days",
                    orbit.period_days, MIN_ORBIT_PERIOD_DAYS
                ));
            }
        }
        let texture = planet
            .texture
            .map(|texture| resolve_texture_path(&raw_pack.path, &texture, &id, errors));
        let mineables = planet
            .mineables
            .into_iter()
            .map(|mineable| namespaced_id(&pack_id, &mineable))
            .collect::<Vec<_>>();
        let inserted = registry
            .planets
            .insert(
                id.clone(),
                PlanetDef {
                    id: id.clone(),
                    system: namespaced_id(&pack_id, &planet.system),
                    classification: planet.classification,
                    texture,
                    position: planet.position,
                    orbit: planet.orbit.map(|orbit| OrbitDef {
                        center: orbit.center,
                        around: orbit
                            .around
                            .map(|around| resolve_orbit_anchor_id(&pack_id, &around)),
                        radius: orbit.radius,
                        eccentricity: orbit.eccentricity,
                        axis_phase: orbit.axis_phase,
                        period_days: orbit.period_days,
                        phase: orbit.phase,
                    }),
                    radius: planet.radius,
                    is_poi: planet.is_poi,
                    mineables,
                    hazards: planet.hazards,
                    hazard_effects: HazardEffectsDef {
                        shield_drain_per_second: planet
                            .hazard_effects
                            .shield_drain_per_second
                            .max(0.0),
                        mining_speed_multiplier: planet
                            .hazard_effects
                            .mining_speed_multiplier
                            .max(1.0),
                    },
                    summary: planet.summary,
                },
            )
            .is_none();
        if inserted {
            registry.planet_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate planet id `{id}`"));
        }
    }

    let stations = read_optional_toml::<StationsFile>(&raw_pack.path.join("stations.toml"), errors);
    for station in stations.stations {
        let id = namespaced_id(&pack_id, &station.id);
        validate_local_content_id(&id, "station", errors);
        if station.name.trim().is_empty() {
            errors.push(format!("Station `{id}` has an empty name"));
        }
        if station.base_seconds.is_some_and(|seconds| seconds <= 0.0) {
            errors.push(format!("Station `{id}` has non-positive base seconds"));
        }
        if station.radius <= 0.0 {
            errors.push(format!("Station `{id}` has non-positive radius"));
        }
        if station.system.is_some() != station.position.is_some() {
            errors.push(format!(
                "Station `{id}` must define both system and position to become a destination"
            ));
        }
        let texture = station
            .texture
            .map(|texture| resolve_texture_path(&raw_pack.path, &texture, &id, errors));
        let services = resolve_station_services(&pack_id, &id, station.services, errors);
        let system = station
            .system
            .map(|system| namespaced_id(&pack_id, &system));
        let inserted = registry
            .stations
            .insert(
                id.clone(),
                StationDef {
                    id: id.clone(),
                    name: station.name,
                    skill: station.skill,
                    base_seconds: station.base_seconds,
                    system,
                    position: station.position,
                    radius: station.radius,
                    texture,
                    icon: station.icon,
                    culture: station.culture,
                    faction: station.faction,
                    summary: station.summary,
                    services,
                },
            )
            .is_none();
        if inserted {
            registry.station_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate station id `{id}`"));
        }
    }

    let upgrades = read_optional_toml::<UpgradesFile>(&raw_pack.path.join("upgrades.toml"), errors);
    for upgrade in upgrades.upgrades {
        let id = namespaced_id(&pack_id, &upgrade.id);
        validate_local_content_id(&id, "upgrade", errors);
        if upgrade.costs.is_empty() {
            errors.push(format!("Upgrade `{id}` has no costs"));
        }
        let costs = upgrade
            .costs
            .into_iter()
            .map(|cost| {
                let item = namespaced_id(&pack_id, &cost.item);
                if cost.base_count == 0 && cost.per_level == 0 {
                    errors.push(format!("Upgrade `{id}` has a zero-count cost"));
                }
                if cost.per_levels == 0 {
                    errors.push(format!("Upgrade `{id}` has a zero per-level interval"));
                }
                UpgradeCostDef {
                    item,
                    base_count: cost.base_count,
                    per_level: cost.per_level,
                    per_levels: cost.per_levels.max(1),
                }
            })
            .collect::<Vec<_>>();
        let inserted = registry
            .upgrades
            .insert(
                id.clone(),
                UpgradeDef {
                    id: id.clone(),
                    costs,
                },
            )
            .is_none();
        if inserted {
            registry.upgrade_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate upgrade id `{id}`"));
        }
    }

    let starter = read_optional_toml::<StarterFile>(&raw_pack.path.join("starter.toml"), errors);
    for stack in starter.inventory {
        let stack = resolve_stack(&pack_id, stack);
        if stack.count == 0 {
            errors.push(format!(
                "Starter inventory stack `{}` has zero count",
                stack.item
            ));
        }
        registry.starter_inventory.push(stack);
    }
}

fn resolve_station_services(
    pack_id: &str,
    station_id: &str,
    services: Vec<StationServiceFileDef>,
    errors: &mut Vec<String>,
) -> Vec<StationServiceDef> {
    let mut resolved = Vec::new();
    let mut seen = HashSet::new();
    for service in services {
        let id = namespaced_id(pack_id, &service.id);
        validate_local_content_id(&id, "station service", errors);
        if service.name.trim().is_empty() {
            errors.push(format!(
                "Station service `{id}` on `{station_id}` has an empty name"
            ));
        }
        if service.kind.trim().is_empty() {
            errors.push(format!(
                "Station service `{id}` on `{station_id}` has an empty kind"
            ));
        }
        if !seen.insert(id.clone()) {
            errors.push(format!(
                "Duplicate station service id `{id}` on `{station_id}`"
            ));
        }
        let trade = service
            .trade
            .into_iter()
            .map(|stock| {
                let item = namespaced_id(pack_id, &stock.item);
                if stock.buy_price == 0 {
                    errors.push(format!(
                        "Station service `{id}` trade item `{item}` has zero buy price"
                    ));
                }
                if stock.sell_price == 0 {
                    errors.push(format!(
                        "Station service `{id}` trade item `{item}` has zero sell price"
                    ));
                }
                if stock.restock_days.is_some_and(|days| days <= 0.0) {
                    errors.push(format!(
                        "Station service `{id}` trade item `{item}` has non-positive restock days"
                    ));
                }
                TradeStockDef {
                    item,
                    buy_price: stock.buy_price,
                    sell_price: stock.sell_price,
                    stock: stock.stock,
                    restock_days: stock.restock_days,
                    unavailable: stock.unavailable,
                }
            })
            .collect();
        let recipe_unlocks = service
            .recipe_unlocks
            .into_iter()
            .map(|unlock| {
                let recipe = namespaced_id(pack_id, &unlock.recipe);
                if unlock.price == 0 {
                    errors.push(format!(
                        "Station service `{id}` recipe unlock `{recipe}` has zero price"
                    ));
                }
                RecipeUnlockDef {
                    recipe,
                    price: unlock.price,
                    unavailable: unlock.unavailable,
                }
            })
            .collect();
        resolved.push(StationServiceDef {
            id,
            name: service.name,
            kind: service.kind,
            description: service.description,
            trade,
            recipe_unlocks,
        });
    }
    resolved
}

fn resolve_pack_options(
    pack_id: &str,
    options: Vec<PackOptionFileDef>,
    errors: &mut Vec<String>,
) -> Vec<PackOptionDef> {
    let mut resolved = Vec::new();
    let mut seen = HashSet::new();
    for option in options {
        let id = namespaced_id(pack_id, &option.id);
        validate_local_content_id(&id, "pack option", errors);
        if option.label.trim().is_empty() {
            errors.push(format!("Pack option `{id}` has an empty label"));
        }
        if !seen.insert(id.clone()) {
            errors.push(format!("Duplicate pack option id `{id}`"));
        }

        let Some(value_type) = parse_pack_option_value_type(&id, &option.value_type, errors) else {
            continue;
        };
        let choices = option
            .choices
            .into_iter()
            .filter(|choice| !choice.trim().is_empty())
            .collect::<Vec<_>>();
        let Some(default) =
            parse_pack_option_default(&id, value_type, &option.default, &choices, errors)
        else {
            continue;
        };

        resolved.push(PackOptionDef {
            id,
            label: option.label,
            description: option.description,
            value_type,
            default,
            choices,
        });
    }
    resolved
}

fn parse_pack_option_value_type(
    id: &str,
    value_type: &str,
    errors: &mut Vec<String>,
) -> Option<PackOptionValueType> {
    match value_type {
        "bool" | "boolean" => Some(PackOptionValueType::Bool),
        "integer" | "int" => Some(PackOptionValueType::Integer),
        "number" | "float" => Some(PackOptionValueType::Number),
        "text" | "string" => Some(PackOptionValueType::Text),
        "choice" => Some(PackOptionValueType::Choice),
        _ => {
            errors.push(format!(
                "Pack option `{id}` has unsupported type `{value_type}`"
            ));
            None
        }
    }
}

fn parse_pack_option_default(
    id: &str,
    value_type: PackOptionValueType,
    default: &Value,
    choices: &[String],
    errors: &mut Vec<String>,
) -> Option<PackOptionValue> {
    match value_type {
        PackOptionValueType::Bool => default.as_bool().map(PackOptionValue::Bool).or_else(|| {
            errors.push(format!("Pack option `{id}` default must be a bool"));
            None
        }),
        PackOptionValueType::Integer => {
            default
                .as_integer()
                .map(PackOptionValue::Integer)
                .or_else(|| {
                    errors.push(format!("Pack option `{id}` default must be an integer"));
                    None
                })
        }
        PackOptionValueType::Number => {
            if let Some(value) = default.as_float() {
                Some(PackOptionValue::Number(value as f32))
            } else if let Some(value) = default.as_integer() {
                Some(PackOptionValue::Number(value as f32))
            } else {
                errors.push(format!("Pack option `{id}` default must be a number"));
                None
            }
        }
        PackOptionValueType::Text => default
            .as_str()
            .map(|value| PackOptionValue::Text(value.to_string()))
            .or_else(|| {
                errors.push(format!("Pack option `{id}` default must be text"));
                None
            }),
        PackOptionValueType::Choice => {
            let Some(value) = default.as_str() else {
                errors.push(format!(
                    "Pack option `{id}` default must be a choice string"
                ));
                return None;
            };
            if choices.is_empty() {
                errors.push(format!("Pack option `{id}` choice list cannot be empty"));
                return None;
            }
            if !choices.iter().any(|choice| choice == value) {
                errors.push(format!(
                    "Pack option `{id}` default `{value}` is not in its choices"
                ));
                return None;
            }
            Some(PackOptionValue::Choice(value.to_string()))
        }
    }
}

fn validate_references(registry: &ContentRegistry, errors: &mut Vec<String>) {
    for recipe in registry.recipes.values() {
        if !registry.stations.contains_key(&recipe.station) {
            errors.push(format!(
                "Recipe `{}` references missing station `{}`",
                recipe.id, recipe.station
            ));
        }
        if !registry.items.contains_key(&recipe.output.item) {
            errors.push(format!(
                "Recipe `{}` outputs missing item `{}`",
                recipe.id, recipe.output.item
            ));
        }
        for ingredient in &recipe.ingredients {
            if !registry.items.contains_key(&ingredient.item) {
                errors.push(format!(
                    "Recipe `{}` uses missing item `{}`",
                    recipe.id, ingredient.item
                ));
            }
        }
    }

    for stack in &registry.starter_inventory {
        if !registry.items.contains_key(&stack.item) {
            errors.push(format!(
                "Starter inventory references missing item `{}`",
                stack.item
            ));
        }
    }

    for module in registry.power_modules.values() {
        validate_reference(
            registry.items.contains_key(&module.install_item),
            "Power module",
            &module.id,
            "install item",
            &module.install_item,
            errors,
        );
        if let Some(fuel_item) = module.fuel_item.as_deref() {
            validate_reference(
                registry.items.contains_key(fuel_item),
                "Power module",
                &module.id,
                "fuel item",
                fuel_item,
                errors,
            );
        }
    }

    for weapon in registry.weapons.values() {
        validate_reference(
            registry.items.contains_key(&weapon.install_item),
            "Weapon",
            &weapon.id,
            "install item",
            &weapon.install_item,
            errors,
        );
    }

    for shield in registry.shields.values() {
        validate_reference(
            registry.items.contains_key(&shield.install_item),
            "Shield",
            &shield.id,
            "install item",
            &shield.install_item,
            errors,
        );
    }

    for ship in registry.ships.values() {
        for module in &ship.power_modules {
            validate_reference(
                registry.power_modules.contains_key(module),
                "Ship",
                &ship.id,
                "power module",
                module,
                errors,
            );
        }
        for shield in &ship.shield_slots {
            validate_reference(
                registry.shields.contains_key(shield),
                "Ship",
                &ship.id,
                "shield",
                shield,
                errors,
            );
        }
        for weapon in &ship.weapon_slots {
            validate_reference(
                registry.weapons.contains_key(weapon),
                "Ship",
                &ship.id,
                "weapon",
                weapon,
                errors,
            );
        }
    }

    for npc_ship in registry.npc_ships.values() {
        validate_reference(
            registry.systems.contains_key(&npc_ship.system),
            "NPC ship",
            &npc_ship.id,
            "system",
            &npc_ship.system,
            errors,
        );
        for cargo in &npc_ship.cargo_defaults {
            validate_reference(
                registry.items.contains_key(&cargo.item),
                "NPC ship",
                &npc_ship.id,
                "cargo item",
                &cargo.item,
                errors,
            );
        }
        for shield in &npc_ship.shield_slots {
            validate_reference(
                registry.shields.contains_key(shield),
                "NPC ship",
                &npc_ship.id,
                "shield",
                shield,
                errors,
            );
        }
        for weapon in &npc_ship.weapon_slots {
            validate_reference(
                registry.weapons.contains_key(weapon),
                "NPC ship",
                &npc_ship.id,
                "weapon",
                weapon,
                errors,
            );
        }
    }

    for planet in registry.planets.values() {
        validate_reference(
            registry.systems.contains_key(&planet.system),
            "Planet",
            &planet.id,
            "system",
            &planet.system,
            errors,
        );
        for mineable in &planet.mineables {
            if !registry.items.contains_key(mineable) {
                errors.push(format!(
                    "Planet `{}` references missing mineable `{mineable}`",
                    planet.id
                ));
            }
        }
        if let Some(orbit) = &planet.orbit {
            validate_planet_orbit_anchor(registry, planet, orbit, errors);
        }
    }

    for station in registry.stations.values() {
        if let Some(system) = station.system.as_deref() {
            validate_reference(
                registry.systems.contains_key(system),
                "Station",
                &station.id,
                "system",
                system,
                errors,
            );
        }
        for service in &station.services {
            for stock in &service.trade {
                validate_reference(
                    registry.items.contains_key(&stock.item),
                    "Station service",
                    &service.id,
                    "trade item",
                    &stock.item,
                    errors,
                );
            }
            for unlock in &service.recipe_unlocks {
                validate_reference(
                    registry.recipes.contains_key(&unlock.recipe),
                    "Station service",
                    &service.id,
                    "recipe unlock",
                    &unlock.recipe,
                    errors,
                );
            }
        }
    }

    for group in registry.galaxy_groups.values() {
        if let Some(universe) = &group.universe {
            validate_reference(
                registry.universes.contains_key(universe),
                "Galaxy group",
                &group.id,
                "universe",
                universe,
                errors,
            );
        }
    }

    for cluster in registry.galaxy_clusters.values() {
        if let Some(galaxy_group) = &cluster.galaxy_group {
            validate_reference(
                registry.galaxy_groups.contains_key(galaxy_group),
                "Galaxy cluster",
                &cluster.id,
                "galaxy group",
                galaxy_group,
                errors,
            );
        }
        if let Some(universe) = &cluster.universe {
            validate_reference(
                registry.universes.contains_key(universe),
                "Galaxy cluster",
                &cluster.id,
                "universe",
                universe,
                errors,
            );
        }
    }

    for galaxy in registry.galaxies.values() {
        if let Some(galaxy_cluster) = &galaxy.galaxy_cluster {
            validate_reference(
                registry.galaxy_clusters.contains_key(galaxy_cluster),
                "Galaxy",
                &galaxy.id,
                "galaxy cluster",
                galaxy_cluster,
                errors,
            );
        }
        if let Some(galaxy_group) = &galaxy.galaxy_group {
            validate_reference(
                registry.galaxy_groups.contains_key(galaxy_group),
                "Galaxy",
                &galaxy.id,
                "galaxy group",
                galaxy_group,
                errors,
            );
        }
        if let Some(universe) = &galaxy.universe {
            validate_reference(
                registry.universes.contains_key(universe),
                "Galaxy",
                &galaxy.id,
                "universe",
                universe,
                errors,
            );
        }
    }

    for region in registry.regions.values() {
        if let Some(galaxy) = &region.galaxy {
            validate_reference(
                registry.galaxies.contains_key(galaxy),
                "Region",
                &region.id,
                "galaxy",
                galaxy,
                errors,
            );
        }
        if let Some(galaxy_cluster) = &region.galaxy_cluster {
            validate_reference(
                registry.galaxy_clusters.contains_key(galaxy_cluster),
                "Region",
                &region.id,
                "galaxy cluster",
                galaxy_cluster,
                errors,
            );
        }
        if let Some(galaxy_group) = &region.galaxy_group {
            validate_reference(
                registry.galaxy_groups.contains_key(galaxy_group),
                "Region",
                &region.id,
                "galaxy group",
                galaxy_group,
                errors,
            );
        }
        if let Some(universe) = &region.universe {
            validate_reference(
                registry.universes.contains_key(universe),
                "Region",
                &region.id,
                "universe",
                universe,
                errors,
            );
        }
    }

    for system in registry.systems.values() {
        if let Some(region) = &system.region {
            validate_reference(
                registry.regions.contains_key(region),
                "System",
                &system.id,
                "region",
                region,
                errors,
            );
        }
        if let Some(galaxy) = &system.galaxy {
            validate_reference(
                registry.galaxies.contains_key(galaxy),
                "System",
                &system.id,
                "galaxy",
                galaxy,
                errors,
            );
        }
        if let Some(universe) = &system.universe {
            validate_reference(
                registry.universes.contains_key(universe),
                "System",
                &system.id,
                "universe",
                universe,
                errors,
            );
        }
        if let Some(primary_star) = &system.primary_star {
            validate_reference(
                registry.stars.contains_key(primary_star),
                "System",
                &system.id,
                "primary star",
                primary_star,
                errors,
            );
            if registry
                .stars
                .get(primary_star)
                .is_some_and(|star| star.system != system.id)
            {
                errors.push(format!(
                    "System `{}` primary star `{primary_star}` belongs to another system",
                    system.id
                ));
            }
        }
    }

    for star in registry.stars.values() {
        validate_reference(
            registry.systems.contains_key(&star.system),
            "Star",
            &star.id,
            "system",
            &star.system,
            errors,
        );
    }

    for upgrade in registry.upgrades.values() {
        for cost in &upgrade.costs {
            if !registry.items.contains_key(&cost.item) {
                errors.push(format!(
                    "Upgrade `{}` references missing cost item `{}`",
                    upgrade.id, cost.item
                ));
            }
        }
    }
}

fn collect_duplicate_recipe_output_warnings(registry: &mut ContentRegistry) {
    let mut recipes_by_station_output: HashMap<(String, String), Vec<String>> = HashMap::new();
    for recipe_id in &registry.recipe_order {
        let Some(recipe) = registry.recipes.get(recipe_id) else {
            continue;
        };
        recipes_by_station_output
            .entry((recipe.station.clone(), recipe.output.item.clone()))
            .or_default()
            .push(recipe.id.clone());
    }

    let mut duplicate_outputs = recipes_by_station_output
        .into_iter()
        .filter(|(_, recipe_ids)| recipe_ids.len() > 1)
        .collect::<Vec<_>>();
    duplicate_outputs.sort_by(|((station_a, output_a), _), ((station_b, output_b), _)| {
        station_a.cmp(station_b).then(output_a.cmp(output_b))
    });

    for ((station, output), recipe_ids) in duplicate_outputs {
        if recipe_ids.iter().all(|recipe_id| {
            registry
                .recipes
                .get(recipe_id)
                .is_some_and(|recipe| recipe.allow_duplicate_output)
        }) {
            continue;
        }
        registry.warnings.push(format!(
            "Multiple recipes in station `{station}` output `{output}`: {}",
            recipe_ids.join(", ")
        ));
    }
}

fn validate_pack_dependency_declarations(manifest: &PackManifest, errors: &mut Vec<String>) {
    for dependency in manifest
        .depends_on
        .iter()
        .chain(manifest.optional_depends_on.iter())
    {
        let dependency_id = dependency.id();
        if !valid_pack_id(dependency_id) {
            errors.push(format!(
                "Pack `{}` has invalid dependency id `{dependency_id}`",
                manifest.id
            ));
        }
        if dependency_id == manifest.id {
            errors.push(format!("Pack `{}` cannot depend on itself", manifest.id));
        }
        if dependency
            .version()
            .is_some_and(|version| version.trim().is_empty())
        {
            errors.push(format!(
                "Pack `{}` dependency `{dependency_id}` has an empty version",
                manifest.id
            ));
        }
    }
}

fn validate_reference(
    exists: bool,
    source_kind: &str,
    source_id: &str,
    target_kind: &str,
    target_id: &str,
    errors: &mut Vec<String>,
) {
    if !exists {
        errors.push(format!(
            "{source_kind} `{source_id}` references missing {target_kind} `{target_id}`"
        ));
    }
}

fn validate_planet_orbit_anchor(
    registry: &ContentRegistry,
    planet: &PlanetDef,
    orbit: &OrbitDef,
    errors: &mut Vec<String>,
) {
    let Some(anchor) = orbit.around.as_deref() else {
        if orbit.center.is_none() {
            let has_primary_star = registry
                .systems
                .get(&planet.system)
                .and_then(|system| system.primary_star.as_deref())
                .is_some();
            validate_reference(
                has_primary_star,
                "Planet",
                &planet.id,
                "primary star orbit anchor",
                "primary_star",
                errors,
            );
        }
        return;
    };

    if anchor == "primary_star" {
        let has_primary_star = registry
            .systems
            .get(&planet.system)
            .and_then(|system| system.primary_star.as_deref())
            .is_some();
        validate_reference(
            has_primary_star,
            "Planet",
            &planet.id,
            "primary star orbit anchor",
            anchor,
            errors,
        );
        return;
    }

    let star_match = registry
        .stars
        .get(anchor)
        .is_some_and(|star| star.system == planet.system);
    let planet_match = registry
        .planets
        .get(anchor)
        .is_some_and(|anchor_planet| anchor_planet.system == planet.system && anchor != planet.id);
    validate_reference(
        star_match || planet_match,
        "Planet",
        &planet.id,
        "orbit anchor",
        anchor,
        errors,
    );
}

fn read_optional_toml<T>(path: &Path, errors: &mut Vec<String>) -> T
where
    T: Default + for<'de> Deserialize<'de>,
{
    if !path.exists() {
        return T::default();
    }
    read_toml(path, errors).unwrap_or_default()
}

fn read_toml<T>(path: &Path, errors: &mut Vec<String>) -> Option<T>
where
    T: for<'de> Deserialize<'de>,
{
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            errors.push(format!("Failed to read {}: {error}", path.display()));
            return None;
        }
    };

    match toml::from_str(&source) {
        Ok(parsed) => Some(parsed),
        Err(error) => {
            errors.push(format!("Failed to parse {}: {error}", path.display()));
            None
        }
    }
}

fn resolve_stack(pack_id: &str, stack: StackFileDef) -> StackDef {
    StackDef {
        item: namespaced_id(pack_id, &stack.item),
        count: stack.count,
    }
}

fn resolve_orbit_anchor_id(pack_id: &str, anchor: &str) -> String {
    if anchor == "primary_star" {
        anchor.to_string()
    } else {
        namespaced_id(pack_id, anchor)
    }
}

fn namespaced_id(pack_id: &str, id: &str) -> String {
    if id.contains(':') {
        id.to_string()
    } else {
        format!("{pack_id}:{id}")
    }
}

fn validate_local_content_id(id: &str, kind: &str, errors: &mut Vec<String>) {
    let Some((pack_id, local_id)) = id.split_once(':') else {
        errors.push(format!("{kind} id `{id}` is missing a namespace"));
        return;
    };

    if !valid_pack_id(pack_id) || !valid_local_id(local_id) {
        errors.push(format!("Invalid {kind} id `{id}`"));
    }
}

fn validate_required_name(id: &str, kind: &str, name: &str, errors: &mut Vec<String>) {
    if name.trim().is_empty() {
        errors.push(format!("{kind} `{id}` has an empty name"));
    }
}

fn validate_positive(value: f32, kind: &str, id: &str, field: &str, errors: &mut Vec<String>) {
    if value <= 0.0 {
        errors.push(format!("{kind} `{id}` has non-positive {field}"));
    }
}

fn validate_fraction(value: f32, kind: &str, id: &str, field: &str, errors: &mut Vec<String>) {
    if !(0.0..=1.0).contains(&value) {
        errors.push(format!("{kind} `{id}` has {field} outside 0.0..1.0"));
    }
}

fn valid_pack_id(id: &str) -> bool {
    !id.is_empty()
        && id.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}

fn valid_local_id(id: &str) -> bool {
    !id.is_empty()
        && id.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        })
}

fn resolve_texture_path(
    pack_path: &Path,
    texture: &str,
    content_id: &str,
    errors: &mut Vec<String>,
) -> String {
    let path = if texture.starts_with("./") || texture.starts_with("../") {
        pack_path.join(texture)
    } else if texture.starts_with("assets/") || texture.starts_with("content/") {
        PathBuf::from(texture)
    } else {
        pack_path.join(texture)
    };

    if !path.exists() {
        errors.push(format!(
            "Content `{content_id}` references missing texture `{texture}`"
        ));
    }
    path.to_string_lossy().to_string()
}

fn default_true() -> bool {
    true
}

fn default_one_f32() -> f32 {
    1.0
}

fn default_one() -> u32 {
    1
}

fn default_station_radius() -> f32 {
    54.0
}

fn default_npc_ship_radius() -> f32 {
    28.0
}

fn default_spawn_weight() -> f32 {
    1.0
}

fn default_spawn_count() -> u32 {
    1
}

fn default_station_icon() -> String {
    "station".to_string()
}

fn default_full_tracking_degrees() -> f32 {
    360.0
}

impl Default for HazardEffectsFileDef {
    fn default() -> Self {
        Self {
            shield_drain_per_second: 0.0,
            mining_speed_multiplier: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn loads_core_content_pack() {
        let registry = load_content_packs(Path::new("content/packs"))
            .expect("core content pack should load and validate");

        assert_eq!(registry.packs.len(), 2);
        assert!(registry
            .packs
            .iter()
            .any(|pack| pack.id == "core" && pack.options.len() == 2));
        assert!(registry.packs.iter().any(|pack| {
            pack.id == "remote-duskfall"
                && pack
                    .options
                    .iter()
                    .any(|option| option.id == "remote-duskfall:redwake_hostility")
        }));
        assert!(registry.items.contains_key("core:iron_ore"));
        assert!(registry.items.contains_key("core:survey_drone"));
        assert!(registry.items.contains_key("core:point_defense_turret"));
        assert!(registry.items.contains_key("core:balanced_shield_matrix"));
        assert!(registry.items.contains_key("core:hazard_shield_matrix"));
        assert_eq!(registry.npc_ships.len(), 3);
        assert!(registry
            .ships
            .get("core:frontier_cargo_ship_01")
            .is_some_and(|ship| {
                ship.name == "Frontier Cargo Ship"
                    && ship.mass == 85000.0
                    && ship.forward_acceleration == 420.0
                    && ship.power_modules == ["core:compact_fission_cell"]
                    && ship.shield_slots == ["core:balanced_shield_matrix"]
                    && ship.weapon_slots == ["core:point_defense_turret"]
                    && ship.texture.as_deref().is_some_and(|texture| {
                        texture.contains(
                            "content/packs/core/./assets/ships/frontier-cargo-ship-01.png",
                        )
                    })
            }));
        assert!(registry
            .weapons
            .get("core:point_defense_turret")
            .is_some_and(|weapon| {
                weapon.kind == WeaponKind::TurretDefense
                    && weapon.install_item == "core:point_defense_turret"
                    && weapon.range == 460.0
                    && weapon.cooldown_seconds == 1.4
                    && weapon.damage == 18.0
                    && weapon.energy_cost == 7.0
            }));
        assert!(registry
            .shields
            .get("core:hazard_shield_matrix")
            .is_some_and(|shield| {
                shield.install_item == "core:hazard_shield_matrix"
                    && shield.capacity == 85.0
                    && shield.recharge_delay == 3.0
                    && shield.recharge_rate == 6.0
                    && shield.damage_resistance == 0.05
                    && shield.hazard_resistance == 0.55
            }));
        assert!(registry
            .npc_ships
            .get("core:frontier_patrol_cutter")
            .is_some_and(|npc_ship| {
                npc_ship.name == "Frontier Patrol Cutter"
                    && npc_ship.system == "core:frontier"
                    && npc_ship.archetype == "patrol-cutter"
                    && npc_ship.role == "patrol"
                    && npc_ship.spawn_count == 1
                    && npc_ship.cargo_defaults
                        == [StackDef {
                            item: "core:fuel_canister".to_string(),
                            count: 1,
                        }]
                    && npc_ship.shield_slots == ["core:balanced_shield_matrix"]
                    && npc_ship.weapon_slots == ["core:point_defense_turret"]
                    && npc_ship.texture.as_deref().is_some_and(|texture| {
                        texture.contains("content/packs/core/./assets/ships/npc-scout-01.png")
                    })
            }));
        assert!(registry.recipes.contains_key("core:point_defense_turret"));
        assert!(registry.recipes.contains_key("core:balanced_shield_matrix"));
        assert!(registry.recipes.contains_key("core:hazard_shield_matrix"));
        assert!(registry.power_modules.contains_key("core:film_solar_sail"));
        assert!(registry
            .power_modules
            .contains_key("core:plasma_torch_cell"));
        assert!(registry
            .power_modules
            .get("core:compact_fission_cell")
            .is_some_and(|module| {
                module.family == "Nuclear"
                    && module.install_item == "core:compact_fission_cell"
                    && module.generation == 14.0
                    && module.fuel_item.as_deref() == Some("core:reactor_pellet")
            }));
        for module_id in [
            "core:film_solar_sail",
            "core:plasma_torch_cell",
            "core:compact_fission_cell",
        ] {
            let module = registry
                .power_modules
                .get(module_id)
                .expect("core power module should load");
            assert!(registry.items.contains_key(&module.install_item));
            assert!(registry
                .recipes
                .values()
                .any(|recipe| recipe.output.item == module.install_item));
        }
        assert!(registry.recipes.contains_key("core:survey_drone"));
        for upgrade_id in [
            "core:engine",
            "core:thrusters",
            "core:energy_core",
            "core:shields",
            "core:drone_bay",
            "core:fuel_systems",
            "core:scanner_array",
            "core:cargo_hold",
        ] {
            assert!(registry.upgrades.contains_key(upgrade_id));
        }
        assert!(registry.universes.contains_key("core:frontier_universe"));
        assert!(registry.galaxy_groups.contains_key("core:local_group"));
        assert!(registry
            .galaxy_clusters
            .contains_key("core:frontier_cluster"));
        assert!(registry.galaxies.contains_key("core:ember_spiral"));
        assert!(registry.regions.contains_key("core:cinder_reaches"));
        assert!(registry.systems.contains_key("core:frontier"));
        assert!(registry.stars.contains_key("core:frontier_primary"));
        assert!(registry
            .systems
            .contains_key("remote-duskfall:duskfall_reach"));
        assert!(registry
            .stars
            .contains_key("remote-duskfall:duskfall_primary"));
        assert!(registry.items.contains_key("remote-duskfall:vanadium_ore"));
        assert!(registry
            .items
            .contains_key("remote-duskfall:vanadium_plate"));
        assert!(registry
            .items
            .contains_key("remote-duskfall:vanadium_frame"));
        assert!(registry
            .recipes
            .contains_key("remote-duskfall:vanadium_plate"));
        assert!(registry
            .recipes
            .contains_key("remote-duskfall:vanadium_frame"));
        assert!(registry.planets.contains_key("core:fractured_ice_body"));
        assert!(registry
            .planets
            .values()
            .filter(|planet| planet.id.starts_with("core:"))
            .all(|planet| planet.system == "core:frontier"));
        assert!(registry
            .planets
            .get("remote-duskfall:duskfall_vanadium_shard")
            .is_some_and(|planet| planet.system == "remote-duskfall:duskfall_reach"));
        assert!(registry
            .planets
            .get("remote-duskfall:redwake_reactor_moon")
            .is_some_and(|planet| planet.system == "remote-duskfall:duskfall_reach"));
        assert!(registry
            .planets
            .values()
            .filter(|planet| planet.id.starts_with("core:"))
            .all(|planet| {
                planet.orbit.as_ref().is_some_and(|orbit| {
                    orbit.center.is_none()
                        && orbit.around.as_deref() == Some("primary_star")
                        && orbit.period_days >= MIN_ORBIT_PERIOD_DAYS
                })
            }));
        let mut core_orbits = registry
            .planets
            .values()
            .filter(|planet| planet.id.starts_with("core:"))
            .filter_map(|planet| planet.orbit.as_ref())
            .collect::<Vec<_>>();
        core_orbits.sort_by(|a, b| a.radius.total_cmp(&b.radius));
        assert_eq!(core_orbits.len(), 20);
        assert!(core_orbits
            .windows(2)
            .all(|window| window[1].radius - window[0].radius >= 600.0));
        assert!(core_orbits.iter().all(|orbit| orbit.period_days >= 5400.0));
        assert!(core_orbits.iter().any(|orbit| orbit.eccentricity > 0.0));
        assert!(core_orbits.iter().any(|orbit| orbit.axis_phase > 0.0));
        assert!(registry
            .planets
            .values()
            .filter(|planet| planet.id.starts_with("remote-duskfall:"))
            .all(|planet| planet.orbit.is_none()));
        assert!(registry.stations.contains_key("core:smelting"));
        let core_destination_stations = registry
            .stations
            .values()
            .filter(|station| {
                station
                    .system
                    .as_deref()
                    .is_some_and(|system| system == "core:frontier")
            })
            .collect::<Vec<_>>();
        assert_eq!(core_destination_stations.len(), 6);
        assert!(registry
            .stations
            .get("core:frontier_exchange")
            .is_some_and(|station| {
                station.system.as_deref() == Some("core:frontier")
                    && station.position == Some([760.0, -420.0])
                    && station.texture.as_deref().is_some_and(|texture| {
                        texture
                            .contains("content/packs/core/./assets/stations/frontier-exchange.png")
                    })
                    && station.faction.as_deref() == Some("Cinder Cooperative")
                    && station.services.len() == 3
                    && station
                        .services
                        .iter()
                        .find(|service| service.id == "core:market")
                        .is_some_and(|service| service.trade.len() == 3)
            }));
        for station_id in [
            "core:ore_lattice_depot",
            "core:cinder_repair_yard",
            "core:pale_orbit_archive",
            "core:freebelt_commissary",
            "core:ember_watch_array",
        ] {
            assert!(registry.stations.get(station_id).is_some_and(|station| {
                station.system.as_deref() == Some("core:frontier")
                    && station.position.is_some()
                    && !station.services.is_empty()
            }));
        }
        assert!(registry
            .stations
            .get("core:pale_orbit_archive")
            .and_then(|station| {
                station
                    .services
                    .iter()
                    .find(|service| service.id == "core:pale_archive_recipes")
            })
            .is_some_and(|service| service.recipe_unlocks.len() == 3));
        assert!(registry.upgrades.contains_key("core:engine"));
        assert_eq!(
            registry.recipe_order.first().map(String::as_str),
            Some("core:iron_plate")
        );
        assert_eq!(
            registry.planet_order.first().map(String::as_str),
            Some("core:bleached_azure_world")
        );
        assert!(registry
            .starter_inventory
            .iter()
            .any(|stack| stack.item == "core:survey_drone" && stack.count == 25));
        assert!(!registry.warnings.iter().any(|warning| {
            warning.contains("station `core:processing`")
                && warning.contains("output `core:reactor_pellet`")
                && warning.contains("core:uranium_reactor_pellet")
                && warning.contains("core:thorium_reactor_pellet")
        }));
    }

    #[test]
    fn loads_optional_compatibility_pack_when_dependencies_match() {
        let root = make_temp_content_root("compat-enabled");
        write_minimal_core_pack(&root);
        write_addon_pack(&root, "0.1.0", "addon_item");
        write_compat_pack(
            &root,
            r#"
depends_on = ["core"]
optional_depends_on = [
  { id = "addon-pack", version = "0.1.0" },
]
"#,
            "addon-pack:addon_item",
        );

        let registry = load_content_packs(&root).expect("compatibility pack should load");
        assert!(registry.packs.iter().any(|pack| pack.id == "compat-pack"));
        assert!(registry.items.contains_key("compat-pack:hybrid_item"));
        assert!(registry.recipes.contains_key("compat-pack:hybrid_item"));
        assert!(registry.warnings.is_empty());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn skips_optional_compatibility_pack_when_dependency_is_missing() {
        let root = make_temp_content_root("compat-missing");
        write_minimal_core_pack(&root);
        write_compat_pack(
            &root,
            r#"
depends_on = ["core"]
optional_depends_on = [
  { id = "addon-pack", version = "0.1.0" },
]
"#,
            "addon-pack:addon_item",
        );

        let registry =
            load_content_packs(&root).expect("missing optional dependency should not fail startup");
        assert!(!registry.packs.iter().any(|pack| pack.id == "compat-pack"));
        assert!(!registry.items.contains_key("compat-pack:hybrid_item"));
        assert!(!registry.recipes.contains_key("compat-pack:hybrid_item"));
        assert!(registry.warnings.iter().any(|warning| {
            warning == "Skipping pack `compat-pack` because optional dependency `addon-pack` is not installed"
        }));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn skips_optional_compatibility_pack_when_dependency_version_mismatches() {
        let root = make_temp_content_root("compat-version");
        write_minimal_core_pack(&root);
        write_addon_pack(&root, "0.2.0", "addon_item");
        write_compat_pack(
            &root,
            r#"
depends_on = ["core"]
optional_depends_on = [
  { id = "addon-pack", version = "0.1.0" },
]
"#,
            "addon-pack:addon_item",
        );

        let registry =
            load_content_packs(&root).expect("optional version mismatch should not fail startup");
        assert!(!registry.packs.iter().any(|pack| pack.id == "compat-pack"));
        assert!(registry.warnings.iter().any(|warning| {
            warning == "Skipping pack `compat-pack` because optional dependency `addon-pack` requires version `0.1.0` but installed version is `0.2.0`"
        }));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn validates_enabled_compatibility_pack_references() {
        let root = make_temp_content_root("compat-invalid");
        write_minimal_core_pack(&root);
        write_addon_pack(&root, "0.1.0", "addon_item");
        write_compat_pack(
            &root,
            r#"
depends_on = ["core"]
optional_depends_on = [
  { id = "addon-pack", version = "0.1.0" },
]
"#,
            "addon-pack:missing_item",
        );

        let errors =
            load_content_packs(&root).expect_err("enabled invalid compat pack should fail");
        assert!(errors.iter().any(|error| {
            error == "Recipe `compat-pack:hybrid_item` uses missing item `addon-pack:missing_item`"
        }));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_planet_with_missing_system() {
        let root = make_temp_content_root("missing-system");
        let pack_path = root.join("bad-pack");
        fs::create_dir_all(&pack_path).expect("temp pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            r#"
id = "bad-pack"
name = "Bad Pack"
version = "0.1.0"
depends_on = []
optional_depends_on = []
"#,
        )
        .expect("pack manifest should be written");
        fs::write(
            pack_path.join("items.toml"),
            r#"
[[items]]
id = "iron_ore"
name = "Iron ore"
tier = "raw"
xp_value = 1.0
unit_mass = 2.5

[[items]]
id = "point_defense"
name = "Point Defense"
tier = "weapon"
xp_value = 4.0
unit_mass = 20.0
"#,
        )
        .expect("items should be written");
        fs::write(
            pack_path.join("planets.toml"),
            r#"
[[planets]]
id = "orphan_world"
system = "missing_system"
classification = "Orphan World"
position = [0.0, 0.0]
radius = 64.0
mineables = ["iron_ore"]
hazards = []
summary = "A planet that points at a missing system."
"#,
        )
        .expect("planets should be written");

        let errors = load_content_packs(&root).expect_err("missing system should fail validation");
        assert!(errors.iter().any(|error| {
            error == "Planet `bad-pack:orphan_world` references missing system `bad-pack:missing_system`"
        }));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_planet_with_invalid_orbit_metadata() {
        let root = make_temp_content_root("invalid-orbit");
        let pack_path = root.join("bad-pack");
        fs::create_dir_all(&pack_path).expect("temp pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            r#"
id = "bad-pack"
name = "Bad Pack"
version = "0.1.0"
depends_on = []
optional_depends_on = []
"#,
        )
        .expect("pack manifest should be written");
        fs::write(
            pack_path.join("systems.toml"),
            r#"
[[systems]]
id = "test_system"
name = "Test System"
arrival = [0.0, 0.0]
"#,
        )
        .expect("systems should be written");
        fs::write(
            pack_path.join("items.toml"),
            r#"
[[items]]
id = "iron_ore"
name = "Iron ore"
tier = "raw"
xp_value = 1.0
unit_mass = 2.5

[[items]]
id = "point_defense"
name = "Point Defense"
tier = "weapon"
xp_value = 4.0
unit_mass = 20.0
"#,
        )
        .expect("items should be written");
        fs::write(
            pack_path.join("planets.toml"),
            r#"
[[planets]]
id = "bad_orbit"
system = "test_system"
classification = "Bad Orbit"
position = [0.0, 0.0]
orbit = { center = [0.0, 0.0], radius = 0.0, period_days = 2.0 }
radius = 64.0
mineables = ["iron_ore"]
hazards = []
summary = "A planet with invalid orbit metadata."
"#,
        )
        .expect("planets should be written");

        let errors = load_content_packs(&root).expect_err("invalid orbit should fail validation");
        assert!(errors
            .iter()
            .any(|error| error == "Planet `bad-pack:bad_orbit` has a non-positive orbit radius"));
        assert!(errors
            .iter()
            .any(|error| error
                == "Planet `bad-pack:bad_orbit` has orbit period 2.0 days below the minimum 30.0 days"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_planet_with_non_positive_orbit_period() {
        let root = make_temp_content_root("bad-orbit-period");
        let pack_path = root.join("bad-pack");
        fs::create_dir_all(&pack_path).expect("temp pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            r#"
id = "bad-pack"
name = "Bad Pack"
version = "0.1.0"
depends_on = []
optional_depends_on = []
"#,
        )
        .expect("pack manifest should be written");
        fs::write(
            pack_path.join("systems.toml"),
            r#"
[[systems]]
id = "test_system"
name = "Test System"
arrival = [0.0, 0.0]
"#,
        )
        .expect("systems should be written");
        fs::write(
            pack_path.join("items.toml"),
            r#"
[[items]]
id = "iron_ore"
name = "Iron ore"
tier = "raw"
xp_value = 1.0
unit_mass = 2.5

[[items]]
id = "point_defense"
name = "Point Defense"
tier = "weapon"
xp_value = 4.0
unit_mass = 20.0
"#,
        )
        .expect("items should be written");
        fs::write(
            pack_path.join("planets.toml"),
            r#"
[[planets]]
id = "bad_period"
system = "test_system"
classification = "Bad Period"
position = [0.0, 0.0]
orbit = { center = [0.0, 0.0], radius = 100.0, period_days = 0.0 }
radius = 64.0
mineables = ["iron_ore"]
hazards = []
summary = "A planet with a non-positive orbit period."
"#,
        )
        .expect("planets should be written");

        let errors = load_content_packs(&root)
            .expect_err("non-positive orbit period should fail validation");
        assert!(errors
            .iter()
            .any(|error| error == "Planet `bad-pack:bad_period` has a non-positive orbit period"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn loads_planet_orbit_metadata() {
        let root = make_temp_content_root("valid-orbit");
        let pack_path = root.join("orbit-pack");
        fs::create_dir_all(&pack_path).expect("temp pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            r#"
id = "orbit-pack"
name = "Orbit Pack"
version = "0.1.0"
depends_on = []
optional_depends_on = []
"#,
        )
        .expect("pack manifest should be written");
        fs::write(
            pack_path.join("systems.toml"),
            r#"
[[systems]]
id = "test_system"
name = "Test System"
arrival = [0.0, 0.0]
"#,
        )
        .expect("systems should be written");
        fs::write(
            pack_path.join("items.toml"),
            r#"
[[items]]
id = "iron_ore"
name = "Iron ore"
tier = "raw"
xp_value = 1.0
unit_mass = 2.5
"#,
        )
        .expect("items should be written");
        fs::write(
            pack_path.join("planets.toml"),
            r#"
[[planets]]
id = "orbiter"
system = "test_system"
classification = "Orbiter"
position = [100.0, 0.0]
orbit = { center = [10.0, -20.0], radius = 100.0, period_days = 40.0, phase = 0.25 }
radius = 64.0
mineables = ["iron_ore"]
hazards = []
summary = "A planet with valid orbit metadata."
"#,
        )
        .expect("planets should be written");

        let registry = load_content_packs(&root).expect("valid orbit metadata should load");
        let orbit = registry
            .planets
            .get("orbit-pack:orbiter")
            .and_then(|planet| planet.orbit.clone())
            .expect("planet should keep orbit metadata");
        assert_eq!(orbit.center, Some([10.0, -20.0]));
        assert_eq!(orbit.around, None);
        assert_eq!(orbit.radius, 100.0);
        assert_eq!(orbit.period_days, 40.0);
        assert_eq!(orbit.phase, 0.25);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn loads_primary_star_orbit_anchor_metadata() {
        let root = make_temp_content_root("primary-star-orbit");
        let pack_path = root.join("orbit-pack");
        fs::create_dir_all(&pack_path).expect("temp pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            r#"
id = "orbit-pack"
name = "Orbit Pack"
version = "0.1.0"
depends_on = []
optional_depends_on = []
"#,
        )
        .expect("pack manifest should be written");
        fs::write(
            pack_path.join("systems.toml"),
            r#"
[[systems]]
id = "test_system"
name = "Test System"
primary_star = "test_star"
arrival = [0.0, 0.0]

[[stars]]
id = "test_star"
name = "Test Star"
system = "test_system"
classification = "G-type main sequence"
color = [255, 230, 180]
radius = 180.0
position = [25.0, -50.0]
"#,
        )
        .expect("systems should be written");
        fs::write(
            pack_path.join("items.toml"),
            r#"
[[items]]
id = "iron_ore"
name = "Iron ore"
tier = "raw"
xp_value = 1.0
unit_mass = 2.5
"#,
        )
        .expect("items should be written");
        fs::write(
            pack_path.join("planets.toml"),
            r#"
[[planets]]
id = "orbiter"
system = "test_system"
classification = "Orbiter"
position = [100.0, 0.0]
orbit = { around = "primary_star", radius = 100.0, period_days = 40.0, phase = 0.25 }
radius = 64.0
mineables = ["iron_ore"]
hazards = []
summary = "A planet orbiting the primary star."
"#,
        )
        .expect("planets should be written");

        let registry = load_content_packs(&root).expect("primary star orbit should validate");
        let orbit = registry
            .planets
            .get("orbit-pack:orbiter")
            .and_then(|planet| planet.orbit.clone())
            .expect("planet should keep orbit metadata");
        assert_eq!(orbit.center, None);
        assert_eq!(orbit.around.as_deref(), Some("primary_star"));
        assert_eq!(orbit.radius, 100.0);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_planet_with_missing_orbit_anchor() {
        let root = make_temp_content_root("missing-orbit-anchor");
        let pack_path = root.join("bad-pack");
        fs::create_dir_all(&pack_path).expect("temp pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            r#"
id = "bad-pack"
name = "Bad Pack"
version = "0.1.0"
depends_on = []
optional_depends_on = []
"#,
        )
        .expect("pack manifest should be written");
        fs::write(
            pack_path.join("systems.toml"),
            r#"
[[systems]]
id = "test_system"
name = "Test System"
arrival = [0.0, 0.0]
"#,
        )
        .expect("systems should be written");
        fs::write(
            pack_path.join("items.toml"),
            r#"
[[items]]
id = "iron_ore"
name = "Iron ore"
tier = "raw"
xp_value = 1.0
unit_mass = 2.5
"#,
        )
        .expect("items should be written");
        fs::write(
            pack_path.join("planets.toml"),
            r#"
[[planets]]
id = "bad_anchor"
system = "test_system"
classification = "Bad Anchor"
position = [0.0, 0.0]
orbit = { around = "missing_star", radius = 100.0, period_days = 40.0 }
radius = 64.0
mineables = ["iron_ore"]
hazards = []
summary = "A planet with a missing orbit anchor."
"#,
        )
        .expect("planets should be written");

        let errors = load_content_packs(&root).expect_err("missing orbit anchor should fail");
        assert!(errors.iter().any(|error| {
            error == "Planet `bad-pack:bad_anchor` references missing orbit anchor `bad-pack:missing_star`"
        }));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn loads_pack_configuration_options() {
        let root = make_temp_content_root("pack-config");
        let pack_path = root.join("config-pack");
        fs::create_dir_all(&pack_path).expect("temp pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            r#"
id = "config-pack"
name = "Config Pack"
version = "0.1.0"
"#,
        )
        .expect("pack manifest should be written");
        fs::write(
            pack_path.join("config.toml"),
            r#"
[[options]]
id = "enabled"
label = "Enabled"
description = "Toggle this pack's extra rules."
type = "bool"
default = true

[[options]]
id = "density"
label = "Density"
type = "choice"
default = "standard"
choices = ["lean", "standard", "rich"]
"#,
        )
        .expect("config should be written");

        let registry = load_content_packs(&root).expect("valid pack config should load");
        let pack = registry
            .packs
            .iter()
            .find(|pack| pack.id == "config-pack")
            .expect("pack should be present");
        assert_eq!(pack.options.len(), 2);
        assert_eq!(pack.options[0].id, "config-pack:enabled");
        assert_eq!(pack.options[0].value_type, PackOptionValueType::Bool);
        assert_eq!(pack.options[0].default, PackOptionValue::Bool(true));
        assert_eq!(pack.options[1].choices, ["lean", "standard", "rich"]);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn resolves_plugin_local_planet_and_station_assets() {
        let root = make_temp_content_root("local-assets");
        let pack_path = root.join("asset-pack");
        fs::create_dir_all(pack_path.join("assets/planets"))
            .expect("planet asset directory should be created");
        fs::create_dir_all(pack_path.join("assets/stations"))
            .expect("station asset directory should be created");
        fs::create_dir_all(pack_path.join("assets/ships"))
            .expect("ship asset directory should be created");
        fs::write(pack_path.join("assets/planets/local-world.png"), b"fake")
            .expect("planet asset should be written");
        fs::write(pack_path.join("assets/stations/local-station.png"), b"fake")
            .expect("station asset should be written");
        fs::write(pack_path.join("assets/ships/local-ship.png"), b"fake")
            .expect("ship asset should be written");
        fs::write(pack_path.join("assets/ships/local-npc.png"), b"fake")
            .expect("npc ship asset should be written");
        fs::write(
            pack_path.join("pack.toml"),
            r#"
id = "asset-pack"
name = "Asset Pack"
version = "0.1.0"
"#,
        )
        .expect("pack manifest should be written");
        fs::write(
            pack_path.join("systems.toml"),
            r#"
[[systems]]
id = "test_system"
name = "Test System"
arrival = [0.0, 0.0]
"#,
        )
        .expect("systems should be written");
        fs::write(
            pack_path.join("items.toml"),
            r#"
[[items]]
id = "iron_ore"
name = "Iron ore"
tier = "raw"
xp_value = 1.0
unit_mass = 2.5

[[items]]
id = "point_defense"
name = "Point Defense"
tier = "weapon"
xp_value = 4.0
unit_mass = 20.0
"#,
        )
        .expect("items should be written");
        fs::write(
            pack_path.join("weapons.toml"),
            r#"
[[weapons]]
id = "point_defense"
name = "Point Defense"
kind = "turret_defense"
install_item = "point_defense"
range = 300.0
cooldown_seconds = 1.0
damage = 12.0
energy_cost = 5.0
"#,
        )
        .expect("weapons should be written");
        fs::write(
            pack_path.join("ships.toml"),
            r#"
[[ships]]
id = "local_ship"
name = "Local Ship"
texture = "./assets/ships/local-ship.png"
mass = 1000.0
forward_acceleration = 10.0
reverse_acceleration = 5.0
turn_acceleration = 2.0
energy_capacity = 100.0
energy_recharge = 10.0
linear_drag = 0.9
hull_capacity = 100.0
shield_capacity = 50.0
"#,
        )
        .expect("ships should be written");
        fs::write(
            pack_path.join("npc_ships.toml"),
            r#"
[[npc_ships]]
id = "local_npc"
name = "Local NPC"
texture = "./assets/ships/local-npc.png"
system = "test_system"
position = [40.0, -80.0]
archetype = "test-courier"
role = "hauler"
behavior_tags = ["traffic"]
mass = 500.0
cargo_capacity = 100.0
cargo_defaults = [
  { item = "iron_ore", count = 1 },
]
hull_capacity = 25.0
shield_capacity = 10.0
energy_capacity = 20.0
"#,
        )
        .expect("npc ships should be written");
        fs::write(
            pack_path.join("planets.toml"),
            r#"
[[planets]]
id = "local_world"
system = "test_system"
classification = "Local World"
texture = "./assets/planets/local-world.png"
position = [120.0, 0.0]
radius = 64.0
mineables = ["iron_ore"]
hazards = []
summary = "A planet using a plugin-local asset."
"#,
        )
        .expect("planets should be written");
        fs::write(
            pack_path.join("stations.toml"),
            r#"
[[stations]]
id = "local_station"
name = "Local Station"
system = "test_system"
position = [-120.0, 0.0]
radius = 48.0
texture = "./assets/stations/local-station.png"
summary = "A station using a plugin-local asset."
"#,
        )
        .expect("stations should be written");

        let registry = load_content_packs(&root).expect("plugin-local assets should validate");
        assert!(
            registry
                .planets
                .get("asset-pack:local_world")
                .and_then(|planet| planet.texture.as_deref())
                .is_some_and(
                    |texture| texture.contains("asset-pack/./assets/planets/local-world.png")
                )
        );
        assert!(registry
            .stations
            .get("asset-pack:local_station")
            .and_then(|station| station.texture.as_deref())
            .is_some_and(
                |texture| texture.contains("asset-pack/./assets/stations/local-station.png")
            ));
        assert!(registry
            .ships
            .get("asset-pack:local_ship")
            .and_then(|ship| ship.texture.as_deref())
            .is_some_and(|texture| texture.contains("asset-pack/./assets/ships/local-ship.png")));
        assert!(registry
            .npc_ships
            .get("asset-pack:local_npc")
            .and_then(|npc_ship| npc_ship.texture.as_deref())
            .is_some_and(|texture| texture.contains("asset-pack/./assets/ships/local-npc.png")));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_invalid_pack_configuration_options() {
        let root = make_temp_content_root("bad-pack-config");
        let pack_path = root.join("bad-config-pack");
        fs::create_dir_all(&pack_path).expect("temp pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            r#"
id = "bad-config-pack"
name = "Bad Config Pack"
version = "0.1.0"
"#,
        )
        .expect("pack manifest should be written");
        fs::write(
            pack_path.join("config.toml"),
            r#"
[[options]]
id = "bad_bool"
label = "Bad Bool"
type = "bool"
default = "yes"

[[options]]
id = "bad_choice"
label = "Bad Choice"
type = "choice"
default = "extreme"
choices = ["standard"]
"#,
        )
        .expect("config should be written");

        let errors = load_content_packs(&root).expect_err("invalid pack config should fail");
        assert!(errors
            .iter()
            .any(|error| error == "Pack option `bad-config-pack:bad_bool` default must be a bool"));
        assert!(errors.iter().any(|error| {
            error == "Pack option `bad-config-pack:bad_choice` default `extreme` is not in its choices"
        }));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_station_destination_without_position_pair() {
        let root = make_temp_content_root("bad-station-destination");
        let pack_path = root.join("bad-station-pack");
        fs::create_dir_all(&pack_path).expect("temp pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            r#"
id = "bad-station-pack"
name = "Bad Station Pack"
version = "0.1.0"
"#,
        )
        .expect("pack manifest should be written");
        fs::write(
            pack_path.join("systems.toml"),
            r#"
[[systems]]
id = "test_system"
name = "Test System"
arrival = [0.0, 0.0]
"#,
        )
        .expect("systems should be written");
        fs::write(
            pack_path.join("stations.toml"),
            r#"
[[stations]]
id = "partial_destination"
name = "Partial Destination"
system = "test_system"
radius = 48.0
"#,
        )
        .expect("stations should be written");

        let errors = load_content_packs(&root)
            .expect_err("station with only system or position should fail validation");
        assert!(errors.iter().any(|error| {
            error == "Station `bad-station-pack:partial_destination` must define both system and position to become a destination"
        }));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_missing_ship_and_power_module_references() {
        let root = make_temp_content_root("bad-ship-power-refs");
        let pack_path = root.join("bad-ship-pack");
        fs::create_dir_all(&pack_path).expect("temp pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            r#"
id = "bad-ship-pack"
name = "Bad Ship Pack"
version = "0.1.0"
"#,
        )
        .expect("pack manifest should be written");
        fs::write(
            pack_path.join("power.toml"),
            r#"
[[power_modules]]
id = "bad_reactor"
name = "Bad Reactor"
family = "Test"
install_item = "missing_reactor"
generation = 10.0
mass = 100.0
fuel_item = "missing_fuel"
"#,
        )
        .expect("power modules should be written");
        fs::write(
            pack_path.join("weapons.toml"),
            r#"
[[weapons]]
id = "point_defense"
name = "Point Defense"
kind = "turret_defense"
install_item = "missing_turret_item"
range = 300.0
cooldown_seconds = 1.0
damage = 12.0
"#,
        )
        .expect("weapons should be written");
        fs::write(
            pack_path.join("shields.toml"),
            r#"
[[shields]]
id = "bad_shield"
name = "Bad Shield"
install_item = "missing_shield_item"
capacity = 50.0
recharge_delay = 2.0
recharge_rate = 3.0
damage_resistance = 1.5
hazard_resistance = -0.1
"#,
        )
        .expect("shields should be written");
        fs::write(
            pack_path.join("ships.toml"),
            r#"
[[ships]]
id = "bad_ship"
name = "Bad Ship"
mass = 1000.0
forward_acceleration = 10.0
reverse_acceleration = 5.0
turn_acceleration = 2.0
energy_capacity = 100.0
energy_recharge = 10.0
linear_drag = 0.9
hull_capacity = 100.0
shield_capacity = 50.0
power_modules = ["bad_reactor", "missing_module"]
shield_slots = ["bad_shield", "missing_shield"]
weapon_slots = ["point_defense", "missing_weapon"]
"#,
        )
        .expect("ships should be written");
        fs::write(
            pack_path.join("npc_ships.toml"),
            r#"
[[npc_ships]]
id = "bad_npc"
name = "Bad NPC"
system = "missing_system"
position = [0.0, 0.0]
radius = 12.0
archetype = "bad-archetype"
role = "hostile"
spawn_weight = 1.0
spawn_count = 0
mass = 100.0
cargo_capacity = 50.0
cargo_defaults = [
  { item = "missing_cargo", count = 0 },
]
hull_capacity = 25.0
shield_capacity = 10.0
energy_capacity = 20.0
shield_slots = ["missing_npc_shield"]
weapon_slots = ["missing_npc_weapon"]
"#,
        )
        .expect("npc ships should be written");

        let errors =
            load_content_packs(&root).expect_err("missing ship and power refs should fail");
        assert!(errors.iter().any(|error| {
            error
                == "Power module `bad-ship-pack:bad_reactor` references missing install item `bad-ship-pack:missing_reactor`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Power module `bad-ship-pack:bad_reactor` references missing fuel item `bad-ship-pack:missing_fuel`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Weapon `bad-ship-pack:point_defense` references missing install item `bad-ship-pack:missing_turret_item`"
        }));
        assert!(errors.iter().any(|error| {
            error == "Shield `bad-ship-pack:bad_shield` has damage resistance outside 0.0..1.0"
        }));
        assert!(errors.iter().any(|error| {
            error == "Shield `bad-ship-pack:bad_shield` has hazard resistance outside 0.0..1.0"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Shield `bad-ship-pack:bad_shield` references missing install item `bad-ship-pack:missing_shield_item`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Ship `bad-ship-pack:bad_ship` references missing power module `bad-ship-pack:missing_module`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Ship `bad-ship-pack:bad_ship` references missing shield `bad-ship-pack:missing_shield`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Ship `bad-ship-pack:bad_ship` references missing weapon `bad-ship-pack:missing_weapon`"
        }));
        assert!(errors
            .iter()
            .any(|error| error == "NPC ship `bad-ship-pack:bad_npc` has zero spawn count"));
        assert!(errors.iter().any(|error| {
            error == "NPC ship `bad-ship-pack:bad_npc` has a zero-count cargo default"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "NPC ship `bad-ship-pack:bad_npc` references missing system `bad-ship-pack:missing_system`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "NPC ship `bad-ship-pack:bad_npc` references missing cargo item `bad-ship-pack:missing_cargo`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "NPC ship `bad-ship-pack:bad_npc` references missing shield `bad-ship-pack:missing_npc_shield`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "NPC ship `bad-ship-pack:bad_npc` references missing weapon `bad-ship-pack:missing_npc_weapon`"
        }));

        fs::remove_dir_all(root).ok();
    }

    fn make_temp_content_root(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("some-frontier-{label}-{nanos}"))
    }

    fn write_minimal_core_pack(root: &Path) {
        let pack_path = root.join("core");
        fs::create_dir_all(&pack_path).expect("core pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            r#"
id = "core"
name = "Core"
version = "0.1.0"
"#,
        )
        .expect("core manifest should be written");
        fs::write(
            pack_path.join("items.toml"),
            r#"
[[items]]
id = "core_item"
name = "Core item"
tier = "component"
xp_value = 1.0
unit_mass = 1.0
"#,
        )
        .expect("core items should be written");
        fs::write(
            pack_path.join("stations.toml"),
            r#"
[[stations]]
id = "crafting"
name = "Crafting"
skill = "crafting"
base_seconds = 1.0
"#,
        )
        .expect("core stations should be written");
    }

    fn write_addon_pack(root: &Path, version: &str, item_id: &str) {
        let pack_path = root.join("addon-pack");
        fs::create_dir_all(&pack_path).expect("addon pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            format!(
                r#"
id = "addon-pack"
name = "Addon Pack"
version = "{version}"
depends_on = ["core"]
"#
            ),
        )
        .expect("addon manifest should be written");
        fs::write(
            pack_path.join("items.toml"),
            format!(
                r#"
[[items]]
id = "{item_id}"
name = "Addon item"
tier = "component"
xp_value = 1.0
unit_mass = 1.0
"#
            ),
        )
        .expect("addon items should be written");
    }

    fn write_compat_pack(root: &Path, dependencies: &str, addon_item: &str) {
        let pack_path = root.join("compat-pack");
        fs::create_dir_all(&pack_path).expect("compat pack directory should be created");
        fs::write(
            pack_path.join("pack.toml"),
            format!(
                r#"
id = "compat-pack"
name = "Compatibility Pack"
version = "0.1.0"
{dependencies}
"#
            ),
        )
        .expect("compat manifest should be written");
        fs::write(
            pack_path.join("items.toml"),
            r#"
[[items]]
id = "hybrid_item"
name = "Hybrid item"
tier = "component"
xp_value = 1.0
unit_mass = 1.0
"#,
        )
        .expect("compat items should be written");
        fs::write(
            pack_path.join("recipes.toml"),
            format!(
                r#"
[[recipes]]
id = "hybrid_item"
station = "core:crafting"
output = {{ item = "hybrid_item", count = 1 }}
ingredients = [
  {{ item = "core:core_item", count = 1 }},
  {{ item = "{addon_item}", count = 1 }},
]
purpose = "Compatibility recipe."
"#
            ),
        )
        .expect("compat recipes should be written");
    }
}
