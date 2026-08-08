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
    pub research: HashMap<String, ResearchDef>,
    pub research_order: Vec<String>,
    pub factions: HashMap<String, FactionDef>,
    pub faction_order: Vec<String>,
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
    pub vendors: HashMap<String, VendorDef>,
    pub vendor_order: Vec<String>,
    pub upgrades: HashMap<String, UpgradeDef>,
    pub upgrade_order: Vec<String>,
    pub starter_ship: Option<String>,
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

#[derive(Debug, Clone)]
pub struct ResearchDef {
    pub id: String,
    pub name: String,
    pub tier: u32,
    pub column: i32,
    pub row: i32,
    pub price: u32,
    pub duration_seconds: f32,
    pub requires: Vec<String>,
    pub revealed_by: Vec<String>,
    pub rewards: Vec<ResearchRewardDef>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResearchRewardDef {
    pub kind: String,
    pub target: Option<String>,
    pub amount: Option<f32>,
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
    pub purchase_price: Option<u32>,
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
    pub faction: Option<String>,
    pub behavior_tags: Vec<String>,
    pub spawn_weight: f32,
    pub spawn_count: u32,
    pub mass: f32,
    pub cargo_capacity: f32,
    pub cargo_defaults: Vec<StackDef>,
    pub credit_reward_min: u32,
    pub credit_reward_max: u32,
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
    pub ammo_item: Option<String>,
    pub ammo_per_shot: u32,
    pub tracking_degrees: f32,
    pub targeting: WeaponTargeting,
    pub effect: WeaponEffect,
    pub beam_color: [u8; 4],
    pub core_color: [u8; 4],
    pub impact_color: [u8; 4],
    pub fire_duration_seconds: f32,
    pub path_curve_strength: f32,
    pub path_wobble: f32,
    pub path_cycles: f32,
    pub trail_length: f32,
    pub burst_count: u8,
    pub travel_speed: Option<f32>,
    pub projectile_texture: Option<String>,
    pub projectile_size: f32,
    pub impact: WeaponImpact,
    pub splash_radius: f32,
    pub splash_falloff: DamageFalloff,
    pub splash_min_multiplier: f32,
    pub chain_targets: u8,
    pub chain_range: f32,
    pub chain_damage_multiplier: f32,
    pub friendly_fire: FriendlyFire,
    pub fire_audio: Option<String>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponKind {
    TurretDefense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponTargeting {
    AllHostiles,
    ShipsOnly,
    ThreatsOnly,
}

impl WeaponTargeting {
    fn from_id(id: &str) -> Option<Self> {
        match id {
            "all_hostiles" => Some(Self::AllHostiles),
            "ships_only" => Some(Self::ShipsOnly),
            "threats_only" => Some(Self::ThreatsOnly),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponEffect {
    Arc,
    Beam,
    Straight,
    Spiral,
    Zigzag,
    Homing,
    Burst,
}

impl WeaponEffect {
    fn from_id(id: &str) -> Option<Self> {
        match id {
            "arc" => Some(Self::Arc),
            "beam" => Some(Self::Beam),
            "straight" => Some(Self::Straight),
            "spiral" => Some(Self::Spiral),
            "zigzag" => Some(Self::Zigzag),
            "homing" => Some(Self::Homing),
            "burst" => Some(Self::Burst),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponImpact {
    Single,
    Chain,
    Splash,
    ChainSplash,
}

impl WeaponImpact {
    fn from_id(id: &str) -> Option<Self> {
        match id {
            "single" => Some(Self::Single),
            "chain" => Some(Self::Chain),
            "splash" => Some(Self::Splash),
            "chain_splash" => Some(Self::ChainSplash),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageFalloff {
    None,
    Linear,
    Quadratic,
}

impl DamageFalloff {
    fn from_id(id: &str) -> Option<Self> {
        match id {
            "none" => Some(Self::None),
            "linear" => Some(Self::Linear),
            "quadratic" => Some(Self::Quadratic),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendlyFire {
    HostilesOnly,
    AllExceptOwner,
    Everyone,
}

impl FriendlyFire {
    fn from_id(id: &str) -> Option<Self> {
        match id {
            "hostiles_only" => Some(Self::HostilesOnly),
            "all_except_owner" => Some(Self::AllExceptOwner),
            "everyone" => Some(Self::Everyone),
            _ => None,
        }
    }
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
pub struct FactionDef {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub default_disposition: FactionDisposition,
    pub color: [u8; 3],
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub reputation_start: i32,
    pub reputation_min: i32,
    pub reputation_max: i32,
    pub reputation_tiers: Vec<ReputationTierDef>,
}

#[derive(Debug, Clone)]
pub struct ReputationTierDef {
    pub id: String,
    pub name: String,
    pub minimum: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactionDisposition {
    Friendly,
    Neutral,
    Hostile,
    Unknown,
}

impl FactionDisposition {
    pub fn id(self) -> &'static str {
        match self {
            Self::Friendly => "friendly",
            Self::Neutral => "neutral",
            Self::Hostile => "hostile",
            Self::Unknown => "unknown",
        }
    }

    fn from_id(id: &str) -> Option<Self> {
        match id {
            "friendly" => Some(Self::Friendly),
            "neutral" => Some(Self::Neutral),
            "hostile" => Some(Self::Hostile),
            "unknown" => Some(Self::Unknown),
            _ => None,
        }
    }
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
    pub faction: Option<String>,
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
    pub faction: Option<String>,
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
    pub research: Vec<ResearchLeadDef>,
    pub recipe_unlocks: Vec<RecipeUnlockDef>,
    pub contracts: Vec<StationContractDef>,
    pub reputation_required: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct StationContractDef {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub description: Option<String>,
    pub target_station: Option<String>,
    pub target_planet: Option<String>,
    pub item: Option<String>,
    pub amount: u32,
    pub reward: u32,
    pub duration_days: f32,
    pub reputation_required: i32,
    pub reputation_reward: i32,
}

#[derive(Debug, Clone)]
pub struct VendorDef {
    pub id: String,
    pub name: String,
    pub station: String,
    pub service: String,
    pub faction: Option<String>,
    pub specialties: Vec<String>,
    pub rotation_days: f32,
    pub slots: usize,
    pub price_variance: f32,
    pub offers: Vec<VendorOfferDef>,
    pub reputation_required: i32,
    pub price_reputation_scale: f32,
}

#[derive(Debug, Clone)]
pub struct VendorOfferDef {
    pub item: String,
    pub buy_price: u32,
    pub sell_price: u32,
    pub min_stock: u32,
    pub max_stock: u32,
    pub weight: f32,
}

#[derive(Debug, Clone)]
pub struct ResearchLeadDef {
    pub research: String,
    pub unavailable: bool,
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
    unit_mass: f32,
}

#[derive(Debug, Default, Deserialize)]
struct RecipesFile {
    #[serde(default)]
    recipes: Vec<RecipeFileDef>,
}

#[derive(Debug, Default, Deserialize)]
struct ResearchFile {
    #[serde(default)]
    research: Vec<ResearchFileDef>,
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

#[derive(Debug, Deserialize)]
struct ResearchFileDef {
    id: String,
    name: String,
    tier: u32,
    column: i32,
    row: i32,
    price: u32,
    duration_seconds: f32,
    #[serde(default)]
    requires: Vec<String>,
    #[serde(default)]
    revealed_by: Vec<String>,
    #[serde(default)]
    rewards: Vec<ResearchRewardFileDef>,
    summary: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResearchRewardFileDef {
    kind: String,
    target: Option<String>,
    amount: Option<f32>,
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
    purchase_price: Option<u32>,
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
    faction: Option<String>,
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
    #[serde(default)]
    credit_reward_min: u32,
    #[serde(default)]
    credit_reward_max: u32,
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
    ammo_item: Option<String>,
    #[serde(default = "default_ammo_per_shot")]
    ammo_per_shot: u32,
    #[serde(default = "default_full_tracking_degrees")]
    tracking_degrees: f32,
    #[serde(default = "default_weapon_targeting")]
    targeting: String,
    #[serde(default = "default_weapon_effect")]
    effect: String,
    #[serde(default = "default_beam_color")]
    beam_color: String,
    #[serde(default = "default_core_color")]
    core_color: String,
    #[serde(default = "default_impact_color")]
    impact_color: String,
    #[serde(default = "default_weapon_fire_duration")]
    fire_duration_seconds: f32,
    #[serde(default = "default_path_curve_strength")]
    path_curve_strength: f32,
    #[serde(default = "default_path_wobble")]
    path_wobble: f32,
    #[serde(default = "default_path_cycles")]
    path_cycles: f32,
    #[serde(default = "default_trail_length")]
    trail_length: f32,
    #[serde(default = "default_burst_count")]
    burst_count: u8,
    travel_speed: Option<f32>,
    projectile_texture: Option<String>,
    #[serde(default = "default_projectile_size")]
    projectile_size: f32,
    #[serde(default = "default_weapon_impact")]
    impact: String,
    #[serde(default)]
    splash_radius: f32,
    #[serde(default = "default_damage_falloff")]
    splash_falloff: String,
    #[serde(default = "default_splash_min_multiplier")]
    splash_min_multiplier: f32,
    #[serde(default = "default_chain_targets")]
    chain_targets: u8,
    #[serde(default = "default_chain_range")]
    chain_range: f32,
    #[serde(default = "default_chain_damage_multiplier")]
    chain_damage_multiplier: f32,
    #[serde(default = "default_friendly_fire")]
    friendly_fire: String,
    fire_audio: Option<String>,
    summary: Option<String>,
}

fn default_weapon_targeting() -> String {
    "all_hostiles".to_string()
}
fn default_ammo_per_shot() -> u32 {
    1
}
fn default_weapon_effect() -> String {
    "arc".to_string()
}
fn default_beam_color() -> String {
    "#3db2ffff".to_string()
}
fn default_core_color() -> String {
    "#b8f5ffff".to_string()
}
fn default_impact_color() -> String {
    "#8febffff".to_string()
}
fn default_weapon_fire_duration() -> f32 {
    0.55
}
fn default_path_curve_strength() -> f32 {
    0.18
}
fn default_path_wobble() -> f32 {
    8.0
}
fn default_path_cycles() -> f32 {
    3.0
}
fn default_trail_length() -> f32 {
    0.4
}
fn default_burst_count() -> u8 {
    3
}
fn default_projectile_size() -> f32 {
    28.0
}
fn default_weapon_impact() -> String {
    "single".to_string()
}
fn default_damage_falloff() -> String {
    "linear".to_string()
}
fn default_splash_min_multiplier() -> f32 {
    0.2
}
fn default_chain_targets() -> u8 {
    3
}
fn default_chain_range() -> f32 {
    240.0
}
fn default_chain_damage_multiplier() -> f32 {
    0.75
}
fn default_friendly_fire() -> String {
    "hostiles_only".to_string()
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
struct FactionsFile {
    #[serde(default)]
    factions: Vec<FactionFileDef>,
}

#[derive(Debug, Deserialize)]
struct FactionFileDef {
    id: String,
    name: String,
    #[serde(default = "default_faction_kind")]
    kind: String,
    #[serde(default = "default_faction_disposition")]
    default_disposition: String,
    #[serde(default = "default_faction_color")]
    color: [u8; 3],
    #[serde(default)]
    tags: Vec<String>,
    summary: Option<String>,
    #[serde(default)]
    reputation_start: i32,
    #[serde(default = "default_reputation_min")]
    reputation_min: i32,
    #[serde(default = "default_reputation_max")]
    reputation_max: i32,
    #[serde(default)]
    reputation_tiers: Vec<ReputationTierFileDef>,
}

#[derive(Debug, Deserialize)]
struct ReputationTierFileDef {
    id: String,
    name: String,
    minimum: i32,
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
    faction: Option<String>,
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
struct VendorsFile {
    #[serde(default)]
    vendors: Vec<VendorFileDef>,
}

#[derive(Debug, Deserialize)]
struct VendorFileDef {
    id: String,
    name: String,
    station: String,
    service: String,
    faction: Option<String>,
    #[serde(default)]
    specialties: Vec<String>,
    rotation_days: f32,
    slots: usize,
    #[serde(default)]
    price_variance: f32,
    #[serde(default)]
    offers: Vec<VendorOfferFileDef>,
    #[serde(default)]
    reputation_required: i32,
    #[serde(default)]
    price_reputation_scale: f32,
}

#[derive(Debug, Deserialize)]
struct VendorOfferFileDef {
    item: String,
    buy_price: u32,
    sell_price: u32,
    min_stock: u32,
    max_stock: u32,
    #[serde(default = "default_vendor_offer_weight")]
    weight: f32,
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
    research: Vec<ResearchLeadFileDef>,
    #[serde(default)]
    recipe_unlocks: Vec<RecipeUnlockFileDef>,
    #[serde(default)]
    contracts: Vec<StationContractFileDef>,
    #[serde(default)]
    reputation_required: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct StationContractFileDef {
    id: String,
    name: String,
    kind: String,
    description: Option<String>,
    target_station: Option<String>,
    target_planet: Option<String>,
    item: Option<String>,
    amount: u32,
    reward: u32,
    duration_days: f32,
    #[serde(default)]
    reputation_required: i32,
    #[serde(default)]
    reputation_reward: i32,
}

#[derive(Debug, Deserialize)]
struct ResearchLeadFileDef {
    research: String,
    #[serde(default)]
    unavailable: bool,
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
    ship: Option<String>,
    #[serde(default)]
    inventory: Vec<StackFileDef>,
}

#[derive(Debug, Deserialize)]
struct PlanetFileDef {
    id: String,
    system: String,
    faction: Option<String>,
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
        if weapon.ammo_per_shot == 0 {
            errors.push(format!("Weapon `{id}` has zero ammo per shot"));
        }
        if weapon.tracking_degrees < 0.0 {
            errors.push(format!("Weapon `{id}` has negative tracking degrees"));
        }
        validate_positive(
            weapon.fire_duration_seconds,
            "Weapon",
            &id,
            "fire duration",
            errors,
        );
        if weapon.path_curve_strength < 0.0 {
            errors.push(format!("Weapon `{id}` has negative path curve strength"));
        }
        if weapon.path_wobble < 0.0 {
            errors.push(format!("Weapon `{id}` has negative path wobble"));
        }
        validate_positive(weapon.path_cycles, "Weapon", &id, "path cycles", errors);
        if !(0.01..=1.0).contains(&weapon.trail_length) {
            errors.push(format!("Weapon `{id}` has trail length outside 0.01..1.0"));
        }
        if !(1..=8).contains(&weapon.burst_count) {
            errors.push(format!("Weapon `{id}` has burst count outside 1..8"));
        }
        if weapon.travel_speed.is_some_and(|speed| speed <= 0.0) {
            errors.push(format!("Weapon `{id}` has non-positive travel speed"));
        }
        validate_positive(
            weapon.projectile_size,
            "Weapon",
            &id,
            "projectile size",
            errors,
        );
        if !(0.0..=5000.0).contains(&weapon.splash_radius) {
            errors.push(format!("Weapon `{id}` has splash radius outside 0..5000"));
        }
        validate_fraction(
            weapon.splash_min_multiplier,
            "Weapon",
            &id,
            "splash minimum multiplier",
            errors,
        );
        if !(1..=16).contains(&weapon.chain_targets) {
            errors.push(format!(
                "Weapon `{id}` has chain target count outside 1..16"
            ));
        }
        if !(0.0..=5000.0).contains(&weapon.chain_range) {
            errors.push(format!("Weapon `{id}` has chain range outside 0..5000"));
        }
        validate_fraction(
            weapon.chain_damage_multiplier,
            "Weapon",
            &id,
            "chain damage multiplier",
            errors,
        );
        let Some(kind) = WeaponKind::from_id(&weapon.kind) else {
            errors.push(format!(
                "Weapon `{id}` has unsupported kind `{}`",
                weapon.kind
            ));
            continue;
        };
        let Some(targeting) = WeaponTargeting::from_id(&weapon.targeting) else {
            errors.push(format!(
                "Weapon `{id}` has unsupported targeting `{}`",
                weapon.targeting
            ));
            continue;
        };
        let Some(effect) = WeaponEffect::from_id(&weapon.effect) else {
            errors.push(format!(
                "Weapon `{id}` has unsupported effect `{}`",
                weapon.effect
            ));
            continue;
        };
        let Some(impact) = WeaponImpact::from_id(&weapon.impact) else {
            errors.push(format!(
                "Weapon `{id}` has unsupported impact `{}`",
                weapon.impact
            ));
            continue;
        };
        let Some(splash_falloff) = DamageFalloff::from_id(&weapon.splash_falloff) else {
            errors.push(format!(
                "Weapon `{id}` has unsupported splash falloff `{}`",
                weapon.splash_falloff
            ));
            continue;
        };
        let Some(friendly_fire) = FriendlyFire::from_id(&weapon.friendly_fire) else {
            errors.push(format!(
                "Weapon `{id}` has unsupported friendly fire `{}`",
                weapon.friendly_fire
            ));
            continue;
        };
        if matches!(impact, WeaponImpact::Splash | WeaponImpact::ChainSplash)
            && weapon.splash_radius <= 0.0
        {
            errors.push(format!("Weapon `{id}` uses splash impact with zero radius"));
        }
        if matches!(impact, WeaponImpact::Chain | WeaponImpact::ChainSplash)
            && weapon.chain_range <= 0.0
        {
            errors.push(format!("Weapon `{id}` uses chain impact with zero range"));
        }
        let Some(beam_color) = parse_hex_color(&weapon.beam_color) else {
            errors.push(format!(
                "Weapon `{id}` has invalid beam color `{}`",
                weapon.beam_color
            ));
            continue;
        };
        let Some(core_color) = parse_hex_color(&weapon.core_color) else {
            errors.push(format!(
                "Weapon `{id}` has invalid core color `{}`",
                weapon.core_color
            ));
            continue;
        };
        let Some(impact_color) = parse_hex_color(&weapon.impact_color) else {
            errors.push(format!(
                "Weapon `{id}` has invalid impact color `{}`",
                weapon.impact_color
            ));
            continue;
        };
        let fire_audio = weapon
            .fire_audio
            .as_deref()
            .map(|path| resolve_asset_path(&raw_pack.path, path, &id, "audio", errors));
        let projectile_texture = weapon.projectile_texture.as_deref().map(|path| {
            resolve_asset_path(&raw_pack.path, path, &id, "projectile texture", errors)
        });
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
                    ammo_item: weapon.ammo_item.map(|item| namespaced_id(&pack_id, &item)),
                    ammo_per_shot: weapon.ammo_per_shot,
                    tracking_degrees: weapon.tracking_degrees,
                    targeting,
                    effect,
                    beam_color,
                    core_color,
                    impact_color,
                    fire_duration_seconds: weapon.fire_duration_seconds,
                    path_curve_strength: weapon.path_curve_strength,
                    path_wobble: weapon.path_wobble,
                    path_cycles: weapon.path_cycles,
                    trail_length: weapon.trail_length,
                    burst_count: weapon.burst_count,
                    travel_speed: weapon.travel_speed,
                    projectile_texture,
                    projectile_size: weapon.projectile_size,
                    impact,
                    splash_radius: weapon.splash_radius,
                    splash_falloff,
                    splash_min_multiplier: weapon.splash_min_multiplier,
                    chain_targets: weapon.chain_targets,
                    chain_range: weapon.chain_range,
                    chain_damage_multiplier: weapon.chain_damage_multiplier,
                    friendly_fire,
                    fire_audio,
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
        if ship.purchase_price == Some(0) {
            errors.push(format!("Ship `{id}` has a zero purchase price"));
        }
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
                    purchase_price: ship.purchase_price,
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
        if npc_ship.credit_reward_min > npc_ship.credit_reward_max {
            errors.push(format!(
                "NPC ship `{id}` has credit_reward_min greater than credit_reward_max"
            ));
        }
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
                    faction: npc_ship
                        .faction
                        .map(|faction| namespaced_id(&pack_id, &faction)),
                    behavior_tags: npc_ship.behavior_tags,
                    spawn_weight: npc_ship.spawn_weight,
                    spawn_count: npc_ship.spawn_count,
                    mass: npc_ship.mass,
                    cargo_capacity: npc_ship.cargo_capacity,
                    cargo_defaults,
                    credit_reward_min: npc_ship.credit_reward_min,
                    credit_reward_max: npc_ship.credit_reward_max,
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

    let research = read_optional_toml::<ResearchFile>(&raw_pack.path.join("research.toml"), errors);
    for node in research.research {
        let id = namespaced_id(&pack_id, &node.id);
        validate_local_content_id(&id, "research", errors);
        validate_required_name(&id, "Research", &node.name, errors);
        if node.price == 0 {
            errors.push(format!("Research `{id}` has zero price"));
        }
        if !node.duration_seconds.is_finite() || node.duration_seconds <= 0.0 {
            errors.push(format!("Research `{id}` has non-positive duration_seconds"));
        }
        let requires = node
            .requires
            .into_iter()
            .map(|required| namespaced_id(&pack_id, &required))
            .collect::<Vec<_>>();
        let revealed_by = node
            .revealed_by
            .into_iter()
            .map(|revealer| namespaced_id(&pack_id, &revealer))
            .collect::<Vec<_>>();
        let rewards = node
            .rewards
            .into_iter()
            .map(|reward| {
                if reward.kind.trim().is_empty() {
                    errors.push(format!("Research `{id}` has a reward with empty kind"));
                }
                ResearchRewardDef {
                    kind: reward.kind,
                    target: reward.target.map(|target| namespaced_id(&pack_id, &target)),
                    amount: reward.amount,
                }
            })
            .collect::<Vec<_>>();
        if rewards.is_empty() {
            errors.push(format!("Research `{id}` has no rewards"));
        }
        let inserted = registry
            .research
            .insert(
                id.clone(),
                ResearchDef {
                    id: id.clone(),
                    name: node.name,
                    tier: node.tier,
                    column: node.column,
                    row: node.row,
                    price: node.price,
                    duration_seconds: node.duration_seconds,
                    requires,
                    revealed_by,
                    rewards,
                    summary: node.summary,
                },
            )
            .is_none();
        if inserted {
            registry.research_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate research id `{id}`"));
        }
    }

    let factions = read_optional_toml::<FactionsFile>(&raw_pack.path.join("factions.toml"), errors);
    for faction in factions.factions {
        let id = namespaced_id(&pack_id, &faction.id);
        validate_local_content_id(&id, "faction", errors);
        validate_required_name(&id, "Faction", &faction.name, errors);
        validate_required_name(&id, "Faction kind", &faction.kind, errors);
        let default_disposition = match FactionDisposition::from_id(&faction.default_disposition) {
            Some(disposition) => disposition,
            None => {
                errors.push(format!(
                    "Faction `{id}` has unsupported default disposition `{}`",
                    faction.default_disposition
                ));
                FactionDisposition::Neutral
            }
        };
        if faction.reputation_min > faction.reputation_max {
            errors.push(format!("Faction `{id}` has inverted reputation bounds"));
        }
        if faction.reputation_start < faction.reputation_min
            || faction.reputation_start > faction.reputation_max
        {
            errors.push(format!(
                "Faction `{id}` has reputation start outside its bounds"
            ));
        }
        let mut reputation_tiers = Vec::new();
        for tier in faction.reputation_tiers {
            if tier.id.trim().is_empty() || tier.name.trim().is_empty() {
                errors.push(format!("Faction `{id}` has an empty reputation tier"));
            }
            if tier.minimum < faction.reputation_min || tier.minimum > faction.reputation_max {
                errors.push(format!(
                    "Faction `{id}` has a reputation tier outside its bounds"
                ));
            }
            reputation_tiers.push(ReputationTierDef {
                id: tier.id,
                name: tier.name,
                minimum: tier.minimum,
            });
        }
        let inserted = registry
            .factions
            .insert(
                id.clone(),
                FactionDef {
                    id: id.clone(),
                    name: faction.name,
                    kind: faction.kind,
                    default_disposition,
                    color: faction.color,
                    tags: faction.tags,
                    summary: faction.summary,
                    reputation_start: faction.reputation_start,
                    reputation_min: faction.reputation_min,
                    reputation_max: faction.reputation_max,
                    reputation_tiers,
                },
            )
            .is_none();
        if inserted {
            registry.faction_order.push(id.clone());
        } else {
            errors.push(format!("Duplicate faction id `{id}`"));
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
                    faction: system
                        .faction
                        .map(|faction| namespaced_id(&pack_id, &faction)),
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
                    faction: planet
                        .faction
                        .map(|faction| namespaced_id(&pack_id, &faction)),
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
                    base_seconds: station.base_seconds,
                    system,
                    position: station.position,
                    radius: station.radius,
                    texture,
                    icon: station.icon,
                    culture: station
                        .culture
                        .map(|culture| namespaced_id(&pack_id, &culture)),
                    faction: station
                        .faction
                        .map(|faction| namespaced_id(&pack_id, &faction)),
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

    let vendors = read_optional_toml::<VendorsFile>(&raw_pack.path.join("vendors.toml"), errors);
    for vendor in vendors.vendors {
        let id = namespaced_id(&pack_id, &vendor.id);
        validate_local_content_id(&id, "vendor", errors);
        if vendor.name.trim().is_empty() {
            errors.push(format!("Vendor `{id}` has an empty name"));
        }
        if vendor.rotation_days <= 0.0 || !vendor.rotation_days.is_finite() {
            errors.push(format!("Vendor `{id}` has non-positive rotation days"));
        }
        if vendor.slots == 0 {
            errors.push(format!(
                "Vendor `{id}` must define at least one catalog slot"
            ));
        }
        if !(0.0..=1.0).contains(&vendor.price_variance) || !vendor.price_variance.is_finite() {
            errors.push(format!(
                "Vendor `{id}` price variance must be between 0.0 and 1.0"
            ));
        }
        if !vendor.price_reputation_scale.is_finite()
            || !(-1.0..=1.0).contains(&vendor.price_reputation_scale)
        {
            errors.push(format!(
                "Vendor `{id}` price reputation scale must be between -1.0 and 1.0"
            ));
        }
        let offers = vendor
            .offers
            .into_iter()
            .map(|offer| {
                if offer.buy_price == 0 || offer.sell_price == 0 {
                    errors.push(format!(
                        "Vendor `{id}` offer `{}` must have positive prices",
                        offer.item
                    ));
                }
                if offer.min_stock > offer.max_stock {
                    errors.push(format!(
                        "Vendor `{id}` offer `{}` has min_stock above max_stock",
                        offer.item
                    ));
                }
                if offer.weight <= 0.0 || !offer.weight.is_finite() {
                    errors.push(format!(
                        "Vendor `{id}` offer `{}` has non-positive weight",
                        offer.item
                    ));
                }
                VendorOfferDef {
                    item: namespaced_id(&pack_id, &offer.item),
                    buy_price: offer.buy_price,
                    sell_price: offer.sell_price,
                    min_stock: offer.min_stock,
                    max_stock: offer.max_stock,
                    weight: offer.weight,
                }
            })
            .collect();
        let inserted = registry.vendors.insert(
            id.clone(),
            VendorDef {
                id: id.clone(),
                name: vendor.name,
                station: namespaced_id(&pack_id, &vendor.station),
                service: namespaced_id(&pack_id, &vendor.service),
                faction: vendor
                    .faction
                    .map(|faction| namespaced_id(&pack_id, &faction)),
                specialties: vendor.specialties,
                rotation_days: vendor.rotation_days,
                slots: vendor.slots,
                price_variance: vendor.price_variance,
                offers,
                reputation_required: vendor.reputation_required,
                price_reputation_scale: vendor.price_reputation_scale,
            },
        );
        if inserted.is_none() {
            registry.vendor_order.push(id);
        } else {
            errors.push(format!("Duplicate vendor id `{id}`"));
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
    if let Some(ship) = starter.ship {
        let ship = namespaced_id(&pack_id, &ship);
        if let Some(previous) = registry.starter_ship.replace(ship.clone()) {
            registry.warnings.push(format!(
                "Starter ship `{previous}` replaced by `{ship}` from pack `{pack_id}`"
            ));
        }
    }
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
        let research = service
            .research
            .into_iter()
            .map(|lead| ResearchLeadDef {
                research: namespaced_id(pack_id, &lead.research),
                unavailable: lead.unavailable,
            })
            .collect();
        let contracts = service
            .contracts
            .into_iter()
            .map(|contract| {
                let contract_id = namespaced_id(pack_id, &contract.id);
                if contract.name.trim().is_empty() {
                    errors.push(format!(
                        "Station service contract `{contract_id}` has an empty name"
                    ));
                }
                if !matches!(contract.kind.as_str(), "hauling" | "survey") {
                    errors.push(format!(
                        "Station service contract `{contract_id}` has unsupported kind `{}`",
                        contract.kind
                    ));
                }
                if contract.amount == 0 || contract.reward == 0 {
                    errors.push(format!(
                        "Station service contract `{contract_id}` has zero amount or reward"
                    ));
                }
                if contract.duration_days <= 0.0 || !contract.duration_days.is_finite() {
                    errors.push(format!(
                        "Station service contract `{contract_id}` has non-positive duration"
                    ));
                }
                if contract.target_station.is_some() == contract.target_planet.is_some() {
                    errors.push(format!(
                        "Station service contract `{contract_id}` must have exactly one target"
                    ));
                }
                if contract.kind == "hauling"
                    && (contract.item.is_none() || contract.target_station.is_none())
                {
                    errors.push(format!(
                        "Hauling contract `{contract_id}` needs an item and target station"
                    ));
                }
                if contract.kind == "survey" && contract.target_planet.is_none() {
                    errors.push(format!(
                        "Survey contract `{contract_id}` needs a target planet"
                    ));
                }
                StationContractDef {
                    id: contract_id,
                    name: contract.name,
                    kind: contract.kind,
                    description: contract.description,
                    target_station: contract
                        .target_station
                        .map(|target| namespaced_id(pack_id, &target)),
                    target_planet: contract
                        .target_planet
                        .map(|target| namespaced_id(pack_id, &target)),
                    item: contract.item.map(|item| namespaced_id(pack_id, &item)),
                    amount: contract.amount,
                    reward: contract.reward,
                    duration_days: contract.duration_days,
                    reputation_required: contract.reputation_required,
                    reputation_reward: contract.reputation_reward,
                }
            })
            .collect();
        resolved.push(StationServiceDef {
            id,
            name: service.name,
            kind: service.kind,
            description: service.description,
            trade,
            research,
            recipe_unlocks,
            contracts,
            reputation_required: service.reputation_required,
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
    if let Some(starter_ship) = registry.starter_ship.as_deref() {
        if !registry.ships.contains_key(starter_ship) {
            errors.push(format!(
                "Starter configuration references missing ship `{starter_ship}`"
            ));
        }
    }

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

    for research in registry.research.values() {
        for required in &research.requires {
            validate_reference(
                registry.research.contains_key(required),
                "Research",
                &research.id,
                "required research",
                required,
                errors,
            );
            if required == &research.id {
                errors.push(format!("Research `{}` requires itself", research.id));
            }
        }
        for revealer in &research.revealed_by {
            validate_reference(
                registry.research.contains_key(revealer),
                "Research",
                &research.id,
                "revealing research",
                revealer,
                errors,
            );
            if revealer == &research.id {
                errors.push(format!("Research `{}` reveals itself", research.id));
            }
        }
        for reward in &research.rewards {
            validate_research_reward(registry, research, reward, errors);
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
        if let Some(ammo_item) = weapon.ammo_item.as_deref() {
            validate_reference(
                registry.items.contains_key(ammo_item),
                "Weapon",
                &weapon.id,
                "ammo item",
                ammo_item,
                errors,
            );
        }
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
        if let Some(faction) = &npc_ship.faction {
            validate_reference(
                registry.factions.contains_key(faction),
                "NPC ship",
                &npc_ship.id,
                "faction",
                faction,
                errors,
            );
        }
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
        if let Some(faction) = &planet.faction {
            validate_reference(
                registry.factions.contains_key(faction),
                "Planet",
                &planet.id,
                "faction",
                faction,
                errors,
            );
        }
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
        if let Some(faction) = &station.faction {
            validate_reference(
                registry.factions.contains_key(faction),
                "Station",
                &station.id,
                "faction",
                faction,
                errors,
            );
        }
        if let Some(culture) = &station.culture {
            validate_reference(
                registry.factions.contains_key(culture),
                "Station",
                &station.id,
                "culture",
                culture,
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
            for lead in &service.research {
                validate_reference(
                    registry.research.contains_key(&lead.research),
                    "Station service",
                    &service.id,
                    "research lead",
                    &lead.research,
                    errors,
                );
            }
            for contract in &service.contracts {
                if let Some(item) = &contract.item {
                    validate_reference(
                        registry.items.contains_key(item),
                        "Station service contract",
                        &contract.id,
                        "item",
                        item,
                        errors,
                    );
                }
                if let Some(target_station) = &contract.target_station {
                    validate_reference(
                        registry.stations.contains_key(target_station),
                        "Station service contract",
                        &contract.id,
                        "target station",
                        target_station,
                        errors,
                    );
                }
                if let Some(target_planet) = &contract.target_planet {
                    validate_reference(
                        registry.planets.contains_key(target_planet),
                        "Station service contract",
                        &contract.id,
                        "target planet",
                        target_planet,
                        errors,
                    );
                }
            }
        }
    }

    for vendor in registry.vendors.values() {
        validate_reference(
            registry.stations.contains_key(&vendor.station),
            "Vendor",
            &vendor.id,
            "station",
            &vendor.station,
            errors,
        );
        let service_exists = registry
            .stations
            .get(&vendor.station)
            .is_some_and(|station| {
                station.services.iter().any(|service| {
                    format!("{}:{}", vendor.station, service.id)
                        .ends_with(&format!(":{}", vendor.service))
                        || service.id == vendor.service
                })
            });
        validate_reference(
            service_exists,
            "Vendor",
            &vendor.id,
            "service",
            &vendor.service,
            errors,
        );
        if let Some(faction) = &vendor.faction {
            validate_reference(
                registry.factions.contains_key(faction),
                "Vendor",
                &vendor.id,
                "faction",
                faction,
                errors,
            );
        }
        for offer in &vendor.offers {
            validate_reference(
                registry.items.contains_key(&offer.item),
                "Vendor",
                &vendor.id,
                "offer item",
                &offer.item,
                errors,
            );
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
        if let Some(faction) = &system.faction {
            validate_reference(
                registry.factions.contains_key(faction),
                "System",
                &system.id,
                "faction",
                faction,
                errors,
            );
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

fn validate_research_reward(
    registry: &ContentRegistry,
    research: &ResearchDef,
    reward: &ResearchRewardDef,
    errors: &mut Vec<String>,
) {
    match reward.kind.as_str() {
        "recipe_unlock" => {
            let Some(target) = reward.target.as_deref() else {
                errors.push(format!(
                    "Research `{}` recipe_unlock reward has no target",
                    research.id
                ));
                return;
            };
            validate_reference(
                registry.recipes.contains_key(target),
                "Research",
                &research.id,
                "recipe reward",
                target,
                errors,
            );
        }
        "item_visibility" => {
            let Some(target) = reward.target.as_deref() else {
                errors.push(format!(
                    "Research `{}` item_visibility reward has no target",
                    research.id
                ));
                return;
            };
            validate_reference(
                registry.items.contains_key(target),
                "Research",
                &research.id,
                "item visibility reward",
                target,
                errors,
            );
        }
        "station_visibility" => {
            let Some(target) = reward.target.as_deref() else {
                errors.push(format!(
                    "Research `{}` station_visibility reward has no target",
                    research.id
                ));
                return;
            };
            validate_reference(
                registry.stations.contains_key(target),
                "Research",
                &research.id,
                "station visibility reward",
                target,
                errors,
            );
        }
        "mining_speed_percent"
        | "smelting_speed_percent"
        | "fabrication_speed_percent"
        | "bonus_output_chance" => {
            let Some(amount) = reward.amount else {
                errors.push(format!(
                    "Research `{}` {} reward has no amount",
                    research.id, reward.kind
                ));
                return;
            };
            if !amount.is_finite() || amount <= 0.0 {
                errors.push(format!(
                    "Research `{}` {} reward has non-positive amount",
                    research.id, reward.kind
                ));
            }
        }
        _ => {
            errors.push(format!(
                "Research `{}` has unsupported reward kind `{}`",
                research.id, reward.kind
            ));
        }
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

fn resolve_asset_path(
    pack_path: &Path,
    asset: &str,
    content_id: &str,
    asset_kind: &str,
    errors: &mut Vec<String>,
) -> String {
    let path = if asset.starts_with("./") || asset.starts_with("../") {
        pack_path.join(asset)
    } else if asset.starts_with("assets/") || asset.starts_with("content/") {
        PathBuf::from(asset)
    } else {
        pack_path.join(asset)
    };
    if !path.is_file() {
        errors.push(format!(
            "Content `{content_id}` references missing {asset_kind} `{asset}`"
        ));
    }
    path.to_string_lossy().to_string()
}

fn parse_hex_color(value: &str) -> Option<[u8; 4]> {
    let hex = value.strip_prefix('#').unwrap_or(value);
    if hex.len() != 6 && hex.len() != 8 {
        return None;
    }
    let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;
    let alpha = if hex.len() == 8 {
        u8::from_str_radix(&hex[6..8], 16).ok()?
    } else {
        255
    };
    Some([red, green, blue, alpha])
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

fn default_vendor_offer_weight() -> f32 {
    1.0
}

fn default_station_radius() -> f32 {
    54.0
}

fn default_npc_ship_radius() -> f32 {
    28.0
}

fn default_faction_kind() -> String {
    "faction".to_string()
}

fn default_faction_disposition() -> String {
    "neutral".to_string()
}

fn default_faction_color() -> [u8; 3] {
    [150, 221, 226]
}

fn default_reputation_min() -> i32 {
    -100
}

fn default_reputation_max() -> i32 {
    100
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

        assert!(registry.packs.len() >= 3);
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
        assert!(registry
            .packs
            .iter()
            .any(|pack| pack.id == "turrets-galore"));
        assert!(registry.items.contains_key("core:iron_ore"));
        assert!(registry.items.contains_key("core:survey_drone"));
        assert!(registry.items.contains_key("core:point_defense_turret"));
        assert!(registry.items.contains_key("core:balanced_shield_matrix"));
        assert!(registry.items.contains_key("core:hazard_shield_matrix"));
        assert!(registry
            .factions
            .get("core:cinder_cooperative")
            .is_some_and(|faction| {
                faction.reputation_start == 0
                    && faction.reputation_min == -100
                    && faction.reputation_max == 100
            }));
        assert!(registry
            .vendors
            .get("core:cinder_yard_mara")
            .is_some_and(|vendor| {
                vendor.reputation_required == 5 && vendor.price_reputation_scale < 0.0
            }));
        assert!(registry
            .stations
            .get("core:ore_lattice_depot")
            .is_some_and(|station| {
                station.services.iter().any(|service| {
                    service.id == "core:ore_lattice_freight_lock"
                        && service.contracts.iter().any(|contract| {
                            contract.kind == "hauling"
                                && contract.target_station
                                    == Some("core:frontier_exchange".to_string())
                        })
                })
            }));
        assert!(registry
            .stations
            .get("core:pale_orbit_archive")
            .is_some_and(|station| {
                station.services.iter().any(|service| {
                    service.id == "core:pale_archive_data"
                        && service
                            .contracts
                            .iter()
                            .any(|contract| contract.kind == "survey")
                })
            }));
        assert_eq!(registry.factions.len(), 6);
        assert!(registry
            .factions
            .get("core:redwake_raiders")
            .is_some_and(|faction| {
                faction.name == "Redwake Raiders"
                    && faction.default_disposition == FactionDisposition::Hostile
                    && faction.tags.iter().any(|tag| tag == "hostile")
                    && faction.tags.iter().any(|tag| tag == "raider")
                    && faction.tags.iter().any(|tag| tag == "probe")
            }));
        assert_eq!(registry.npc_ships.len(), 5);
        assert!(registry
            .npc_ships
            .get("remote-duskfall:redwake_remote_probe")
            .is_some_and(|npc_ship| {
                npc_ship.system == "remote-duskfall:duskfall_reach"
                    && npc_ship.role == "hostile"
                    && npc_ship.behavior_tags.iter().any(|tag| tag == "pressure")
            }));
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
                    && weapon.ammo_item.as_deref() == Some("core:interceptor_round")
                    && weapon.ammo_per_shot == 1
                    && weapon.projectile_size == 34.0
                    && weapon.projectile_texture.as_deref().is_some_and(|texture| {
                        texture.ends_with("core/./assets/projectiles/point-defense.png")
                    })
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
                    && npc_ship.faction.as_deref() == Some("core:cinder_cooperative")
                    && npc_ship.archetype == "patrol-cutter"
                    && npc_ship.role == "patrol"
                    && npc_ship.spawn_count == 1
                    && npc_ship.credit_reward_min == 0
                    && npc_ship.credit_reward_max == 0
                    && npc_ship.cargo_defaults
                        == [
                            StackDef {
                                item: "core:fuel_canister".to_string(),
                                count: 1,
                            },
                            StackDef {
                                item: "core:interceptor_round".to_string(),
                                count: 80,
                            },
                        ]
                    && npc_ship.shield_slots == ["core:balanced_shield_matrix"]
                    && npc_ship.weapon_slots == ["core:point_defense_turret"]
                    && npc_ship.texture.as_deref().is_some_and(|texture| {
                        texture.contains(
                            "content/packs/core/./assets/ships/frontier-patrol-cutter.png",
                        )
                    })
            }));
        assert!(registry
            .npc_ships
            .get("core:redwake_probe")
            .and_then(|npc_ship| npc_ship.texture.as_deref())
            .is_some_and(|texture| {
                texture.contains("content/packs/core/./assets/ships/redwake-raider.png")
            }));
        assert!(registry.recipes.contains_key("core:point_defense_turret"));
        assert!(registry
            .recipes
            .get("core:interceptor_rounds")
            .is_some_and(|recipe| {
                recipe.output.item == "core:interceptor_round" && recipe.output.count == 20
            }));
        assert!(registry.recipes.contains_key("core:balanced_shield_matrix"));
        assert!(registry.recipes.contains_key("core:hazard_shield_matrix"));
        assert_eq!(
            registry.research_order.first().map(String::as_str),
            Some("core:frontier_survey_methods")
        );
        assert!(registry
            .research
            .get("core:frontier_survey_methods")
            .is_some_and(|research| {
                research.name == "Frontier Survey Methods"
                    && research.tier == 0
                    && research.column == 0
                    && research.row == 0
                    && research.price == 450
                    && research.duration_seconds == 5.0
                    && research.requires.is_empty()
                    && research.revealed_by.is_empty()
                    && research.rewards.len() == 1
                    && research.rewards[0].kind == "mining_speed_percent"
                    && research.rewards[0].amount == Some(5.0)
            }));
        assert!(registry
            .research
            .get("core:advanced_scanner_core")
            .is_some_and(|research| {
                research.requires == ["core:mining_calibration_i"]
                    && research.revealed_by == ["core:mining_calibration_i"]
                    && research.rewards.iter().any(|reward| {
                        reward.kind == "recipe_unlock"
                            && reward.target.as_deref() == Some("core:advanced_scanner_core")
                    })
            }));
        assert!(registry
            .research
            .get("remote-duskfall:duskfall_vanadium_frames")
            .is_some_and(|research| {
                research.name == "Duskfall Vanadium Frames"
                    && research.requires == ["core:jump_core"]
                    && research.revealed_by == ["core:jump_core"]
                    && research.duration_seconds == 60.0
                    && research.rewards.iter().any(|reward| {
                        reward.kind == "recipe_unlock"
                            && reward.target.as_deref() == Some("remote-duskfall:vanadium_frame")
                    })
            }));
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
        assert!(registry
            .systems
            .get("core:frontier")
            .is_some_and(|system| system.faction.as_deref() == Some("core:cinder_cooperative")));
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
            .get("core:fractured_ice_body")
            .is_some_and(|planet| planet.faction.as_deref() == Some("core:freebelt_compact")));
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
        assert_eq!(registry.vendor_order.len(), 4);
        assert!(registry
            .vendors
            .get("core:frontier_exchange_juno")
            .is_some_and(|vendor| {
                vendor.station == "core:frontier_exchange"
                    && vendor.service == "core:market"
                    && vendor.rotation_days == 5.0
                    && vendor.slots == 4
                    && vendor.offers.len() == 5
            }));
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
                    && station.faction.as_deref() == Some("core:cinder_cooperative")
                    && station.culture.as_deref() == Some("core:freebelt_compact")
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
            .is_some_and(|service| {
                service.kind == "research"
                    && service.recipe_unlocks.is_empty()
                    && service.research.len() == 3
                    && service
                        .research
                        .iter()
                        .any(|lead| lead.research == "core:advanced_scanner_core")
            }));
        assert!(registry
            .stations
            .get("core:frontier_exchange")
            .and_then(|station| station.services.iter().find(|service| {
                service.id == "core:market"
                    && service.name == "Starter Market"
                    && service.kind == "shop"
            }))
            .is_some_and(|service| {
                service
                    .trade
                    .iter()
                    .any(|stock| stock.item == "core:survey_drone")
            }));
        assert!(registry
            .stations
            .get("core:ore_lattice_depot")
            .and_then(|station| {
                station
                    .services
                    .iter()
                    .find(|service| service.id == "core:ore_lattice_bulk_market")
            })
            .is_some_and(|service| {
                service.kind == "shop"
                    && service
                        .trade
                        .iter()
                        .any(|stock| stock.item == "core:cobalt_ore")
            }));
        assert!(registry
            .stations
            .get("core:cinder_repair_yard")
            .and_then(|station| {
                station
                    .services
                    .iter()
                    .find(|service| service.id == "core:cinder_yard_parts")
            })
            .is_some_and(|service| {
                service
                    .trade
                    .iter()
                    .any(|stock| stock.item == "core:balanced_shield_matrix")
                    && service
                        .trade
                        .iter()
                        .any(|stock| stock.item == "core:point_defense_turret")
            }));
        assert!(registry
            .stations
            .get("core:freebelt_commissary")
            .and_then(|station| station.services.iter().find(|service| {
                service.id == "core:freebelt_supply" && service.name == "Supply Counter"
            }))
            .is_some_and(|service| {
                service
                    .trade
                    .iter()
                    .any(|stock| stock.item == "core:fuel_canister")
                    && service
                        .trade
                        .iter()
                        .any(|stock| stock.item == "core:improved_survey_drone")
            }));
        assert!(registry
            .stations
            .get("core:ember_watch_array")
            .is_some_and(|station| {
                station.services.iter().any(|service| {
                    service.id == "core:ember_watch_beacon"
                        && service.name == "Route Intel"
                        && service.kind == "navigation"
                }) && station.services.iter().any(|service| {
                    service.id == "core:ember_watch_listening_post" && service.kind == "signals"
                })
            }));
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
        assert!(registry
            .starter_inventory
            .iter()
            .any(|stack| stack.item == "core:reactor_pellet" && stack.count == 3));
        assert!(registry
            .starter_inventory
            .iter()
            .any(|stack| stack.item == "core:interceptor_round" && stack.count == 120));
        assert!(!registry.warnings.iter().any(|warning| {
            warning.contains("station `core:processing`")
                && warning.contains("output `core:reactor_pellet`")
                && warning.contains("core:uranium_reactor_pellet")
                && warning.contains("core:thorium_reactor_pellet")
        }));
    }

    #[test]
    fn turrets_galore_is_a_pack_owned_weapon_proof() {
        let registry = load_content_packs(Path::new("content/packs"))
            .expect("reference turret pack should load and validate");

        assert_eq!(
            registry.starter_ship.as_deref(),
            Some("turrets-galore:twinspire_gunship")
        );
        let twinspire = registry
            .ships
            .get("turrets-galore:twinspire_gunship")
            .expect("pack-owned two-bank starter ship should load");
        assert_eq!(twinspire.name, "Twinspire Gunship");
        assert_eq!(twinspire.purchase_price, Some(14_000));
        assert_eq!(
            twinspire.weapon_slots,
            [
                "turrets-galore:ember_lance_turret",
                "turrets-galore:sentinel_flak_turret"
            ]
        );
        assert!(twinspire.texture.as_deref().is_some_and(
            |path| path.ends_with("turrets-galore/./assets/ships/twinspire-gunship.png")
        ));

        let ember = registry
            .weapons
            .get("turrets-galore:ember_lance_turret")
            .expect("pack-owned anti-ship turret should load");
        assert_eq!(ember.install_item, "turrets-galore:ember_lance_turret");
        assert_eq!(ember.targeting, WeaponTargeting::ShipsOnly);
        assert_eq!(ember.effect, WeaponEffect::Beam);
        assert_eq!(ember.beam_color, [255, 90, 50, 255]);
        assert_eq!(ember.projectile_size, 54.0);
        assert!(ember.projectile_texture.as_deref().is_some_and(
            |path| path.ends_with("turrets-galore/./assets/projectiles/ember-lance.png")
        ));
        assert!(ember.fire_audio.as_deref().is_some_and(
            |path| path.ends_with("turrets-galore/./assets/audio/ember-lance-fire.wav")
        ));

        let flak = registry
            .weapons
            .get("turrets-galore:sentinel_flak_turret")
            .expect("pack-owned threat turret should load");
        assert_eq!(flak.targeting, WeaponTargeting::AllHostiles);
        assert_eq!(flak.effect, WeaponEffect::Burst);
        assert_eq!(flak.burst_count, 5);
        assert_eq!(flak.travel_speed, Some(900.0));
        assert_eq!(
            flak.ammo_item.as_deref(),
            Some("turrets-galore:sentinel_flak_canister")
        );
        assert!(flak.projectile_texture.as_deref().is_some_and(
            |path| path.ends_with("turrets-galore/./assets/projectiles/sentinel-flak.png")
        ));
        let chain = registry
            .weapons
            .get("turrets-galore:storm_chain_turret")
            .expect("pack-owned chain turret should load");
        assert_eq!(chain.impact, WeaponImpact::Chain);
        assert_eq!(chain.chain_targets, 5);
        assert_eq!(chain.chain_range, 260.0);
        assert_eq!(chain.friendly_fire, FriendlyFire::HostilesOnly);
        assert!(chain.projectile_texture.as_deref().is_some_and(
            |path| path.ends_with("turrets-galore/./assets/projectiles/storm-chain.png")
        ));

        let nuke = registry
            .weapons
            .get("turrets-galore:super_nuke_turret")
            .expect("pack-owned splash turret should load");
        assert_eq!(nuke.impact, WeaponImpact::Splash);
        assert_eq!(nuke.splash_radius, 520.0);
        assert_eq!(nuke.splash_falloff, DamageFalloff::Linear);
        assert_eq!(nuke.friendly_fire, FriendlyFire::HostilesOnly);
        assert_eq!(
            nuke.ammo_item.as_deref(),
            Some("turrets-galore:super_nuke_warhead")
        );
        assert_eq!(nuke.projectile_size, 64.0);
        assert!(nuke.projectile_texture.as_deref().is_some_and(
            |path| path.ends_with("turrets-galore/./assets/projectiles/super-nuke.png")
        ));
        assert!(registry
            .recipes
            .contains_key("turrets-galore:ember_lance_turret"));
        assert!(registry
            .npc_ships
            .get("turrets-galore:galore_proving_drone")
            .is_some_and(|ship| ship.weapon_slots == ["turrets-galore:sentinel_flak_turret"]));
        assert!(registry.starter_inventory.iter().any(|stack| {
            stack.item == "turrets-galore:ember_lance_turret" && stack.count == 1
        }));
        assert!(registry.starter_inventory.iter().any(|stack| {
            stack.item == "turrets-galore:sentinel_flak_turret" && stack.count == 1
        }));
        assert!(registry.starter_inventory.iter().any(|stack| {
            stack.item == "turrets-galore:storm_chain_turret" && stack.count == 1
        }));
        assert!(registry
            .starter_inventory
            .iter()
            .any(|stack| { stack.item == "turrets-galore:super_nuke_turret" && stack.count == 1 }));
        assert!(registry.starter_inventory.iter().any(|stack| {
            stack.item == "turrets-galore:sentinel_flak_canister" && stack.count == 150
        }));
        assert!(registry.starter_inventory.iter().any(|stack| {
            stack.item == "turrets-galore:super_nuke_warhead" && stack.count == 4
        }));

        for path_type in [
            "beam", "straight", "arc", "spiral", "zigzag", "homing", "burst",
        ] {
            assert!(
                WeaponEffect::from_id(path_type).is_some(),
                "{path_type} should be a supported content-pack path type"
            );
        }
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
    fn loads_research_nodes_and_rewards_from_content_pack() {
        let root = make_temp_content_root("research-valid");
        write_minimal_core_pack(&root);
        write_minimal_core_recipe(&root, "scanner_core");
        let pack_path = root.join("core");
        fs::write(
            pack_path.join("research.toml"),
            r#"
[[research]]
id = "survey_methods"
name = "Survey Methods"
tier = 0
column = 0
row = 1
price = 100
duration_seconds = 5.0
requires = []
revealed_by = []
summary = "Basic survey research."

[[research.rewards]]
kind = "mining_speed_percent"
amount = 5.0

[[research]]
id = "scanner_core"
name = "Scanner Core"
tier = 1
column = 1
row = 1
price = 250
duration_seconds = 12.0
requires = ["survey_methods"]
revealed_by = ["survey_methods"]
summary = "Scanner recipe research."

[[research.rewards]]
kind = "recipe_unlock"
target = "scanner_core"
"#,
        )
        .expect("research should be written");

        let registry = load_content_packs(&root).expect("research pack should load");
        assert_eq!(
            registry.research_order,
            vec![
                "core:survey_methods".to_string(),
                "core:scanner_core".to_string()
            ]
        );
        assert!(registry
            .research
            .get("core:scanner_core")
            .is_some_and(|research| {
                research.requires == ["core:survey_methods"]
                    && research.revealed_by == ["core:survey_methods"]
                    && research.duration_seconds == 12.0
                    && research.rewards.iter().any(|reward| {
                        reward.kind == "recipe_unlock"
                            && reward.target.as_deref() == Some("core:scanner_core")
                    })
            }));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rejects_research_with_invalid_references_and_rewards() {
        let root = make_temp_content_root("research-invalid");
        write_minimal_core_pack(&root);
        let pack_path = root.join("core");
        fs::write(
            pack_path.join("research.toml"),
            r#"
[[research]]
id = "bad_node"
name = "Bad Node"
tier = 0
column = 0
row = 0
price = 0
duration_seconds = 0.0
requires = ["missing_required"]
revealed_by = ["missing_revealer"]

[[research.rewards]]
kind = "recipe_unlock"
target = "missing_recipe"

[[research.rewards]]
kind = "mining_speed_percent"
amount = -1.0

[[research.rewards]]
kind = "unknown_reward"
"#,
        )
        .expect("research should be written");

        let errors = load_content_packs(&root).expect_err("invalid research should fail");
        assert!(errors
            .iter()
            .any(|error| error == "Research `core:bad_node` has zero price"));
        assert!(errors.iter().any(|error| {
            error == "Research `core:bad_node` has non-positive duration_seconds"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Research `core:bad_node` references missing required research `core:missing_required`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Research `core:bad_node` references missing revealing research `core:missing_revealer`"
        }));
        assert!(errors.iter().any(|error| {
            error == "Research `core:bad_node` references missing recipe reward `core:missing_recipe`"
        }));
        assert!(errors.iter().any(|error| {
            error == "Research `core:bad_node` mining_speed_percent reward has non-positive amount"
        }));
        assert!(errors.iter().any(|error| {
            error == "Research `core:bad_node` has unsupported reward kind `unknown_reward`"
        }));

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
unit_mass = 2.5

[[items]]
id = "point_defense"
name = "Point Defense"
tier = "weapon"
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
unit_mass = 2.5

[[items]]
id = "point_defense"
name = "Point Defense"
tier = "weapon"
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
unit_mass = 2.5

[[items]]
id = "point_defense"
name = "Point Defense"
tier = "weapon"
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
unit_mass = 2.5

[[items]]
id = "point_defense"
name = "Point Defense"
tier = "weapon"
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
ammo_item = "missing_ammo"
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
            pack_path.join("factions.toml"),
            r#"
[[factions]]
id = "bad_faction"
name = "Bad Faction"
kind = "test"
default_disposition = "furious"
"#,
        )
        .expect("factions should be written");
        fs::write(
            pack_path.join("systems.toml"),
            r#"
[[systems]]
id = "bad_system"
name = "Bad System"
faction = "missing_system_faction"
arrival = [0.0, 0.0]
"#,
        )
        .expect("systems should be written");
        fs::write(
            pack_path.join("planets.toml"),
            r#"
[[planets]]
id = "bad_planet"
system = "bad_system"
faction = "missing_planet_faction"
classification = "Bad Planet"
position = [0.0, 0.0]
radius = 64.0
mineables = ["missing_planet_mineable"]
summary = "A planet with missing faction ownership."
"#,
        )
        .expect("planets should be written");
        fs::write(
            pack_path.join("stations.toml"),
            r#"
[[stations]]
id = "bad_station"
name = "Bad Station"
system = "bad_system"
position = [0.0, 0.0]
faction = "missing_station_faction"
culture = "missing_station_culture"

[[stations.services]]
id = "bad_station_research"
name = "Bad Station Research"
kind = "research"

[[stations.services.research]]
research = "missing_research"
"#,
        )
        .expect("stations should be written");
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
faction = "missing_npc_faction"
spawn_weight = 1.0
spawn_count = 0
mass = 100.0
cargo_capacity = 50.0
cargo_defaults = [
  { item = "missing_cargo", count = 0 },
]
credit_reward_min = 20
credit_reward_max = 10
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
            error
                == "Weapon `bad-ship-pack:point_defense` references missing ammo item `bad-ship-pack:missing_ammo`"
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
                == "NPC ship `bad-ship-pack:bad_npc` has credit_reward_min greater than credit_reward_max"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "NPC ship `bad-ship-pack:bad_npc` references missing system `bad-ship-pack:missing_system`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Faction `bad-ship-pack:bad_faction` has unsupported default disposition `furious`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "System `bad-ship-pack:bad_system` references missing faction `bad-ship-pack:missing_system_faction`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Planet `bad-ship-pack:bad_planet` references missing faction `bad-ship-pack:missing_planet_faction`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Station `bad-ship-pack:bad_station` references missing faction `bad-ship-pack:missing_station_faction`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Station `bad-ship-pack:bad_station` references missing culture `bad-ship-pack:missing_station_culture`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "Station service `bad-ship-pack:bad_station_research` references missing research lead `bad-ship-pack:missing_research`"
        }));
        assert!(errors.iter().any(|error| {
            error
                == "NPC ship `bad-ship-pack:bad_npc` references missing faction `bad-ship-pack:missing_npc_faction`"
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
base_seconds = 1.0
"#,
        )
        .expect("core stations should be written");
    }

    fn write_minimal_core_recipe(root: &Path, recipe_id: &str) {
        let pack_path = root.join("core");
        fs::write(
            pack_path.join("recipes.toml"),
            format!(
                r#"
[[recipes]]
id = "{recipe_id}"
station = "crafting"
output = {{ item = "core_item", count = 1 }}
ingredients = [
  {{ item = "core_item", count = 1 }},
]
purpose = "Minimal recipe."
"#
            ),
        )
        .expect("core recipes should be written");
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
