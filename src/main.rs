mod branding_icon;
mod content;

use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const DEFAULT_WINDOW_WIDTH: i32 = 1100;
const DEFAULT_WINDOW_HEIGHT: i32 = 760;
const BRANDING_LOGO_PATH: &str = "assets/branding/some-frontier-logo.png";
const STARFIELD_RADIUS: f32 = 9000.0;
const SHIP_RADIUS: f32 = 22.0;
const SHIP_SPRITE_SIZE: f32 = 72.0;
const DEFENSE_THREAT_RADIUS: f32 = 18.0;
const WEAPON_FIRE_EVENT_SECONDS: f32 = 0.18;
const NPC_PATROL_SPEED: f32 = 34.0;
const NPC_TRAFFIC_SPEED: f32 = 26.0;
const NPC_FOLLOW_SPEED: f32 = 46.0;
const NPC_FLEE_SPEED: f32 = 58.0;
const NPC_HOSTILE_SPEED: f32 = 54.0;
const NPC_ACCELERATION: f32 = 46.0;
const NPC_SEPARATION_PADDING: f32 = 54.0;
const NPC_STATION_CLEARANCE: f32 = 72.0;
const NPC_PLANET_CLEARANCE: f32 = 96.0;
const NPC_FOLLOW_DISTANCE: f32 = 420.0;
const NPC_HOSTILE_STANDOFF_DISTANCE: f32 = 360.0;
const NPC_ROUTE_RADIUS: f32 = 520.0;
const NPC_ROUTE_POINTS: [[f32; 2]; 4] = [[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0]];
const NPC_INTERACTION_PADDING: f32 = 126.0;
const PLANET_INTERACTION_PADDING: f32 = 96.0;
const PLANET_ORBIT_CLEARANCE: f32 = 48.0;
const STATION_INTERACTION_PADDING: f32 = 86.0;
const BASE_MINING_SECONDS: f32 = 3.0;
const BASE_SMELTING_SECONDS: f32 = 2.0;
const BASE_CRAFTING_SECONDS: f32 = 1.5;
const BASE_PROCESSING_SECONDS: f32 = 2.0;
const STARTER_SHIP_DRY_MASS: f32 = 85_000.0;
const STARTER_FORWARD_ACCELERATION: f32 = 420.0;
const STARTER_REVERSE_ACCELERATION: f32 = 280.0;
const STARTER_TURN_ACCELERATION: f32 = 4.8;
const STARTER_SHIP_ID: &str = "core:frontier_cargo_ship_01";
const STARMAP_SCALE: f32 = 0.055;
const STARMAP_PANEL_SCREEN_FRACTION: f32 = 0.80;
const STARMAP_ZOOM_MIN: f32 = 0.45;
const STARMAP_ZOOM_MAX: f32 = 3.5;
const STARMAP_ZOOM_STEP: f32 = 1.15;
const STARMAP_PAN_PIXELS_TO_WORLD: f32 = 1.35;
const ARC_SEGMENTS: usize = 72;
const INVENTORY_SLOTS: usize = 200;
const SKILL_COUNT: usize = 3;
const SHIP_UPGRADE_COUNT: usize = 8;
const OBJECT_ACTION_RAIL_MIN_WIDTH: f32 = 280.0;
const OBJECT_ACTION_RAIL_MAX_SCREEN_FRACTION: f32 = 0.55;
const OBJECT_ACTION_RAIL_GAP: f32 = 8.0;
const ACTION_RAIL_RESIZE_HITBOX_WIDTH: f32 = 28.0;
const WORK_ROW_HEIGHT: f32 = 30.0;
const INVENTORY_ROW_HEIGHT: f32 = 30.0;
const SHIP_UPGRADE_ROW_HEIGHT: f32 = 54.0;
const SAVE_VERSION: u32 = 1;
const AUTOSAVE_SECONDS: f32 = 60.0;
const GAME_DAY_SECONDS: f32 = 120.0;
const PLANET_SEED_JITTER: f32 = 180.0;
const PLANET_SEED_ROTATION: f32 = 0.22;
const CAMERA_ZOOM_MIN: f32 = 0.35;
const CAMERA_ZOOM_MAX: f32 = 2.0;
const CAMERA_ZOOM_STEP: f32 = 1.12;
const STARTUP_FADE_SECONDS: f32 = 0.8;
const TRANSITION_FADE_IN_SECONDS: f32 = 0.75;
const TRANSITION_HOLD_SECONDS: f32 = 0.9;
const TRANSITION_FADE_OUT_SECONDS: f32 = 0.85;
const KNOWN_SYSTEMS_PANEL_WIDTH: f32 = 280.0;
const KNOWN_SYSTEM_ROW_HEIGHT: f32 = 54.0;
const WARP_CHARGE_SECONDS: f32 = 2.0;
const MAX_SCAN_LEVEL: u8 = 3;
const STARTER_SYSTEM_ID: &str = "core:frontier";
const UI_FONT_PATH: &str = "assets/fonts/Junicode.ttf";

thread_local! {
    static UI_FONT: RefCell<Option<Font>> = const { RefCell::new(None) };
}

#[derive(Clone, Copy)]
struct Star {
    position: Vec2,
    size: f32,
    brightness: f32,
}

struct StarLayer {
    stars: Vec<Star>,
    depth: f32,
    trail_scale: f32,
    color: Color,
}

struct UniverseBackground {
    star_layers: Vec<StarLayer>,
}

struct SystemStar {
    system: String,
    name: String,
    classification: String,
    position: Vec2,
    radius: f32,
    color: Color,
    is_primary: bool,
}

struct TransitionAsset {
    path: String,
    texture: Texture2D,
}

struct SceneTransition {
    texture: Option<Texture2D>,
    label: String,
    timer: f32,
    fade_in_seconds: f32,
    hold_seconds: f32,
    fade_out_seconds: f32,
    pending_action: TransitionAction,
    midpoint_applied: bool,
}

#[derive(Clone)]
enum TransitionAction {
    SwitchSystem(String),
}

#[derive(Clone, Copy)]
enum TransitionPhase {
    FadeIn,
    Hold,
    FadeOut,
}

struct GameState {
    content_registry: content::ContentRegistry,
    content_pack_options: Vec<PackOptionSelection>,
    transition_assets: Vec<TransitionAsset>,
    scene_transition: Option<SceneTransition>,
    current_system_id: String,
    save_path: PathBuf,
    world_seed: u64,
    world_elapsed_days: f32,
    credits: u32,
    ship: Ship,
    installed_power_modules: Vec<PowerModule>,
    equipped_shields: Vec<ShieldSystem>,
    equipped_weapons: Vec<WeaponSystem>,
    npc_ships: Vec<NpcShip>,
    defense_threats: Vec<DefenseThreat>,
    weapon_fire_events: Vec<WeaponFireEvent>,
    ship_texture: Option<Texture2D>,
    system_light_haze_texture: Option<Texture2D>,
    system_stars: Vec<SystemStar>,
    planets: Vec<Planet>,
    stations: Vec<StationDestination>,
    recipe_vendor_locked_recipes: Vec<String>,
    purchased_recipe_unlocks: Vec<String>,
    selected_planet: Option<usize>,
    selected_station: Option<usize>,
    selected_npc_ship: Option<usize>,
    selected_station_service: Option<usize>,
    destination_planet: Option<usize>,
    orbiting_planet: Option<usize>,
    system_destinations: HashMap<String, String>,
    pending_warp: Option<PendingWarp>,
    camera_zoom: f32,
    starmap_zoom: f32,
    starmap_pan: Vec2,
    starmap_drag_previous_mouse: Option<Vec2>,
    action_rail_width_override: Option<f32>,
    action_rail_resize_previous_mouse: Option<Vec2>,
    inventory: Inventory,
    smelt_recipes: Vec<Recipe>,
    smelt_settings: Vec<CraftSetting>,
    craft_recipes: Vec<Recipe>,
    craft_settings: Vec<CraftSetting>,
    processing_recipes: Vec<Recipe>,
    processing_settings: Vec<CraftSetting>,
    production_mode: ProductionMode,
    skills: [Skill; SKILL_COUNT],
    ship_upgrades: [ShipUpgrade; SHIP_UPGRADE_COUNT],
    inventory_open: bool,
    map_open: bool,
    skills_open: bool,
    upgrades_open: bool,
    content_open: bool,
    content_browser: ContentBrowserState,
    escape_dialog_open: bool,
    quit_to_title_requested: bool,
    starmap_filter: StarmapFilter,
    starmap_resource_filter_index: usize,
    work_scroll: f32,
    inventory_scroll: f32,
    upgrades_scroll: f32,
    shield_recharge_delay_remaining: f32,
    last_window_size: (i32, i32),
    window_save_delay: Option<f32>,
    save_delay: Option<f32>,
    save_dirty: bool,
    save_status_timer: f32,
    save_status_manual: bool,
}

#[derive(Clone, Copy, Default)]
struct ContentBrowserState {
    selected_pack_index: Option<usize>,
    packs_scroll: f32,
    items_scroll: f32,
    recipes_scroll: f32,
    npc_ships_scroll: f32,
    planets_scroll: f32,
}

#[derive(Clone, Copy)]
enum SaveFeedback {
    Auto,
    Manual,
}

enum AppState {
    Title(TitleMenu),
    Playing(Box<GameState>),
}

struct TitleMenu {
    view: TitleView,
    new_game_seed_text: String,
    save_slots: Vec<TitleSaveSlot>,
    selected_save_index: usize,
    last_save_click_index: Option<usize>,
    last_save_click_time: f64,
    content_packs: Vec<TitleContentPack>,
    selected_pack_index: usize,
    settings: AppSettings,
    selected_settings_category: SettingsCategory,
}

struct TitleSaveSlot {
    path: PathBuf,
    label: String,
    world_seed: u64,
    current_system_id: String,
    world_elapsed_days: f32,
    modified_unix_seconds: u64,
    is_legacy: bool,
}

struct TitleContentPack {
    id: String,
    name: String,
    version: String,
    description: Option<String>,
    options: Vec<TitlePackOption>,
}

#[derive(Clone)]
struct TitlePackOption {
    id: String,
    label: String,
    description: Option<String>,
    value_type: content::PackOptionValueType,
    default_value: String,
    current_value: String,
    choices: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TitleView {
    Main,
    NewGame,
    LoadGame,
    ContentPacks,
    Settings,
}

enum GameStartMode {
    NewGame {
        seed: u64,
        pack_options: Vec<PackOptionSelection>,
    },
    LoadGame {
        path: PathBuf,
    },
}

enum TitleAction {
    NewGame {
        seed: u64,
        pack_options: Vec<PackOptionSelection>,
    },
    LoadGame {
        path: PathBuf,
    },
    QuitDesktop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EscapeDialogAction {
    Resume,
    SaveNow,
    SaveToTitle,
    QuitDesktop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EscapeDialogResult {
    Continue,
    QuitDesktop,
}

#[derive(Clone, Serialize, Deserialize)]
struct AppSettings {
    #[serde(default = "default_ui_scale")]
    ui_scale: f32,
    #[serde(default = "default_master_volume")]
    master_volume: f32,
    #[serde(default = "default_controls_profile")]
    controls_profile: String,
    #[serde(default = "default_gameplay_autosave_minutes")]
    gameplay_autosave_minutes: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SettingsCategory {
    Display,
    Audio,
    Controls,
    Gameplay,
}

#[derive(Clone, Copy)]
struct Ship {
    position: Vec2,
    velocity: Vec2,
    angle: f32,
    angular_velocity: f32,
    attributes: ShipAttributes,
    systems: ShipSystems,
}

#[derive(Clone)]
struct PowerModule {
    id: String,
    name: String,
    family: String,
    generation: f32,
    mass: f32,
    fuel_item: Option<String>,
    fuel_per_minute: f32,
    heat: f32,
    risk: f32,
}

#[derive(Clone)]
struct ShieldSystem {
    id: String,
    name: String,
    install_item: String,
    capacity: f32,
    recharge_delay: f32,
    recharge_rate: f32,
    damage_resistance: f32,
    hazard_resistance: f32,
}

#[derive(Clone)]
struct WeaponSystem {
    id: String,
    name: String,
    kind: content::WeaponKind,
    install_item: String,
    range: f32,
    cooldown_seconds: f32,
    damage: f32,
    energy_cost: f32,
    tracking_degrees: f32,
    cooldown_remaining: f32,
    status: WeaponStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WeaponStatus {
    Ready,
    NoThreat,
    Cooldown,
    InsufficientEnergy,
    Fired,
}

struct DefenseThreat {
    id: String,
    name: String,
    system: String,
    position: Vec2,
    radius: f32,
    disposition: ThreatDisposition,
    hull: ShipResource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThreatDisposition {
    Hostile,
    Neutral,
    Owned,
    Environmental,
}

struct WeaponFireEvent {
    from: Vec2,
    to: Vec2,
    timer: f32,
}

#[derive(Clone, Copy)]
struct ShipAttributes {
    mass: f32,
    engine_strength: f32,
    reverse_engine_strength: f32,
    turn_thruster_strength: f32,
    energy_capacity: f32,
    energy_recharge: f32,
    linear_drag: f32,
}

#[derive(Clone, Copy)]
struct ShipSystems {
    hull: ShipResource,
    shields: ShipResource,
    energy: ShipResource,
}

#[derive(Clone, Copy)]
struct ShipResource {
    current: f32,
    max: f32,
}

struct Planet {
    id: String,
    system: String,
    faction: Option<String>,
    base_position: Vec2,
    position: Vec2,
    motion: PlanetMotion,
    radius: f32,
    is_poi: bool,
    texture: Option<Texture2D>,
    info: PlanetInfo,
    mining: Vec<MiningSetting>,
    scan_level: u8,
}

struct StationDestination {
    id: String,
    system: String,
    name: String,
    position: Vec2,
    radius: f32,
    texture: Option<Texture2D>,
    icon: String,
    culture: Option<String>,
    faction: Option<String>,
    summary: String,
    services: Vec<StationService>,
}

struct NpcShip {
    id: String,
    name: String,
    system: String,
    position: Vec2,
    velocity: Vec2,
    angle: f32,
    radius: f32,
    texture: Option<Texture2D>,
    archetype: String,
    role: String,
    faction: Option<String>,
    behavior_tags: Vec<String>,
    behavior: NpcBehaviorMode,
    route_index: usize,
    anchor: Vec2,
    identified: bool,
    cargo_capacity: f32,
    cargo_defaults: Vec<ItemStack>,
    hull: ShipResource,
    shields: ShipResource,
    energy: ShipResource,
    shield_slots: Vec<String>,
    weapon_slots: Vec<String>,
    summary: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NpcBehaviorMode {
    Patrol,
    Follow,
    Flee,
    TradeRoute,
    StationTraffic,
    HostileIntercept,
}

impl NpcBehaviorMode {
    fn label(self) -> &'static str {
        match self {
            Self::Patrol => "patrol",
            Self::Follow => "follow",
            Self::Flee => "flee",
            Self::TradeRoute => "trade route",
            Self::StationTraffic => "traffic",
            Self::HostileIntercept => "intercept",
        }
    }

    fn max_speed(self) -> f32 {
        match self {
            Self::Patrol => NPC_PATROL_SPEED,
            Self::Follow => NPC_FOLLOW_SPEED,
            Self::Flee => NPC_FLEE_SPEED,
            Self::TradeRoute => NPC_TRAFFIC_SPEED,
            Self::StationTraffic => NPC_TRAFFIC_SPEED,
            Self::HostileIntercept => NPC_HOSTILE_SPEED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NpcInteractionAction {
    Identify,
    Hail,
    Dock,
    Trade,
    Conflict,
}

impl NpcInteractionAction {
    fn label(self) -> &'static str {
        match self {
            Self::Identify => "Identify",
            Self::Hail => "Hail",
            Self::Dock => "Dock",
            Self::Trade => "Trade",
            Self::Conflict => "Conflict",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NpcInteractionState {
    Available,
    Complete,
    Unavailable,
}

struct NpcInteractionRow {
    action: NpcInteractionAction,
    state: NpcInteractionState,
    status: &'static str,
}

struct StationService {
    id: String,
    name: String,
    kind: String,
    description: Option<String>,
    trade: Vec<TradeOffer>,
    recipe_unlocks: Vec<RecipeUnlockOffer>,
}

#[derive(Clone)]
struct TradeOffer {
    item: ItemRef,
    buy_price: u32,
    sell_price: u32,
    stock: Option<u32>,
    restock_days: Option<f32>,
    unavailable: bool,
}

#[derive(Clone)]
struct RecipeUnlockOffer {
    recipe: String,
    price: u32,
    unavailable: bool,
}

#[derive(Clone, Copy)]
enum PlanetMotion {
    Static,
    Orbit(OrbitMotion),
}

#[derive(Clone, Copy)]
struct OrbitMotion {
    center: Vec2,
    anchor_planet: Option<usize>,
    radius: f32,
    semi_minor: f32,
    axis_rotation: f32,
    period_days: f32,
    phase: f32,
}

#[derive(Clone, Copy)]
struct OrbitGuide {
    center: Vec2,
    radius: f32,
    semi_minor: f32,
    axis_rotation: f32,
}

struct PlanetInfo {
    classification: String,
    mineables: Vec<Mineable>,
    hazards: Vec<String>,
    hazard_effects: HazardEffects,
    summary: String,
}

#[derive(Clone, Copy)]
struct HazardEffects {
    shield_drain_per_second: f32,
    mining_speed_multiplier: f32,
}

#[derive(Clone)]
struct Mineable {
    item: ItemRef,
}

#[derive(Clone, PartialEq)]
struct ItemRef {
    id: String,
    name: String,
    xp_value: f32,
    unit_mass: f32,
}

#[derive(Clone)]
struct ItemStack {
    item: ItemRef,
    count: u32,
}

struct Inventory {
    slots: [Option<ItemStack>; INVENTORY_SLOTS],
}

#[derive(Serialize, Deserialize)]
struct SaveData {
    version: u32,
    world_seed: u64,
    #[serde(default)]
    world_elapsed_days: f32,
    #[serde(default = "default_current_system_id")]
    current_system_id: String,
    #[serde(default = "default_camera_zoom")]
    camera_zoom: f32,
    #[serde(default = "default_credits")]
    credits: u32,
    ship: SaveShip,
    inventory: Vec<SaveStack>,
    skills: Vec<SaveSkill>,
    upgrades: Vec<SaveUpgrade>,
    destination_planet: Option<String>,
    #[serde(default)]
    orbiting_planet: Option<String>,
    #[serde(default)]
    installed_power_modules: Vec<String>,
    #[serde(default)]
    shield_slots: Vec<String>,
    #[serde(default)]
    shield_recharge_delay_remaining: f32,
    #[serde(default)]
    weapon_slots: Vec<String>,
    #[serde(default)]
    system_destinations: Vec<SaveSystemDestination>,
    #[serde(default)]
    content_pack_options: Vec<PackOptionSelection>,
    #[serde(default)]
    purchased_recipe_unlocks: Vec<String>,
    production_mode: String,
    smelt_settings: Vec<SaveWorkSetting>,
    craft_settings: Vec<SaveWorkSetting>,
    processing_settings: Vec<SaveWorkSetting>,
    planets: Vec<SavePlanet>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct PackOptionSelection {
    pack_id: String,
    option_id: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct SaveShip {
    position: [f32; 2],
    velocity: [f32; 2],
    angle: f32,
    angular_velocity: f32,
    hull: SaveResource,
    shields: SaveResource,
    energy: SaveResource,
}

#[derive(Serialize, Deserialize)]
struct SaveResource {
    current: f32,
    max: f32,
}

#[derive(Serialize, Deserialize)]
struct SaveStack {
    item: String,
    count: u32,
}

#[derive(Serialize, Deserialize)]
struct SaveSkill {
    kind: String,
    level: u32,
    xp: f32,
}

#[derive(Serialize, Deserialize)]
struct SaveUpgrade {
    kind: String,
    level: u32,
}

#[derive(Serialize, Deserialize)]
struct SaveWorkSetting {
    id: String,
    keep: u32,
    queued: u32,
    progress: f32,
}

#[derive(Serialize, Deserialize)]
struct SavePlanet {
    id: String,
    #[serde(default)]
    scanned: bool,
    #[serde(default)]
    scan_level: u8,
    mining: Vec<SaveWorkSetting>,
}

#[derive(Serialize, Deserialize)]
struct SaveSystemDestination {
    system: String,
    planet: String,
}

struct PendingWarp {
    target_system_id: String,
    timer: f32,
    cost: Vec<ItemStack>,
}

struct Recipe {
    id: String,
    output: ItemStack,
    ingredients: Vec<ItemStack>,
    base_seconds: f32,
}

#[derive(Clone, Copy)]
struct Skill {
    kind: SkillKind,
    level: u32,
    xp: f32,
}

#[derive(Clone, Copy)]
struct ShipUpgrade {
    kind: ShipUpgradeKind,
    level: u32,
}

#[derive(Clone, Copy)]
struct CraftSetting {
    keep: u32,
    queued: u32,
    progress: f32,
}

#[derive(Clone, Copy)]
struct MiningSetting {
    keep: u32,
    queued: u32,
    progress: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorkColumn {
    Item,
    Keep,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum UiColumnSizing {
    Fixed(f32),
    Content { measured: f32, min: f32, max: f32 },
    Flex { min: f32, weight: f32 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct UiColumnSpec {
    sizing: UiColumnSizing,
}

#[derive(Clone, Debug, PartialEq)]
struct UiTableLayout {
    bounds: Rect,
    viewport: Rect,
    columns: Vec<Rect>,
    row_height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UiTableCell {
    row: usize,
    column: usize,
}

struct UiTableBottomLayout<'a> {
    x: f32,
    y: f32,
    width: f32,
    row_start_offset: f32,
    viewport_bottom: f32,
    row_height: f32,
    column_gap: f32,
    columns: &'a [UiColumnSpec],
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ProductionMode {
    Smelting,
    Crafting,
    Processing,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StarmapFilter {
    All,
    Scanned,
    Unscanned,
    Destination,
    Resource,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SkillKind {
    Mining,
    Smelting,
    Fabrication,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ShipUpgradeKind {
    Engine,
    Thrusters,
    EnergyCore,
    Shields,
    DroneBay,
    FuelSystems,
    ScannerArray,
    CargoHold,
}

impl GameState {
    async fn new(start_mode: GameStartMode) -> Self {
        draw_startup_transition(None, "Loading content packs ... core", 1.0);
        next_frame().await;
        let start_pack_options = match &start_mode {
            GameStartMode::NewGame { pack_options, .. } => pack_options.clone(),
            GameStartMode::LoadGame { .. } => Vec::new(),
        };
        let content_registry = load_game_content_registry_with_options(&start_pack_options);

        draw_startup_transition(None, "Loading save data ...", 1.0);
        next_frame().await;
        let save_path = match &start_mode {
            GameStartMode::NewGame { seed, .. } => new_save_slot_path(*seed),
            GameStartMode::LoadGame { path } => path.clone(),
        };
        let save_data = match &start_mode {
            GameStartMode::NewGame { .. } => None,
            GameStartMode::LoadGame { path } => read_save_data_at(path),
        };
        let world_seed = match (&start_mode, save_data.as_ref()) {
            (GameStartMode::NewGame { seed, .. }, _) => *seed,
            (GameStartMode::LoadGame { .. }, Some(save)) => save.world_seed,
            (GameStartMode::LoadGame { .. }, None) => new_world_seed(),
        };
        let credits = save_data
            .as_ref()
            .map(|save| save.credits)
            .unwrap_or_else(default_credits);
        let content_pack_options = match (&start_mode, save_data.as_ref()) {
            (GameStartMode::NewGame { .. }, _) => {
                validated_pack_option_selections(&content_registry, start_pack_options)
            }
            (GameStartMode::LoadGame { .. }, Some(save)) => validated_pack_option_selections(
                &content_registry,
                save.content_pack_options.clone(),
            ),
            (GameStartMode::LoadGame { .. }, None) => {
                default_pack_option_selections_from_registry(&content_registry)
            }
        };

        draw_startup_transition(None, "Preparing production chains ...", 1.0);
        next_frame().await;
        let smelt_recipes = make_smelting_recipes(&content_registry);
        let craft_recipes = make_crafting_recipes(&content_registry);
        let processing_recipes = make_processing_recipes(&content_registry);
        let inventory = Inventory::starter(&content_registry);
        let smelt_settings = vec![CraftSetting::starter(); smelt_recipes.len()];
        let craft_settings = vec![CraftSetting::starter(); craft_recipes.len()];
        let processing_settings = vec![CraftSetting::starter(); processing_recipes.len()];

        let transition_assets = load_transition_assets(Path::new("assets/transitions")).await;
        let startup_background_texture = select_transition_texture(&transition_assets);
        let startup_background = startup_background_texture.as_ref();

        draw_startup_transition(
            startup_background,
            "Loading ship asset ... frontier_cargo_ship_01",
            1.0,
        );
        next_frame().await;
        let starter_ship_def = content_registry.ships.get(STARTER_SHIP_ID).cloned();
        let ship_texture_path = starter_ship_def
            .as_ref()
            .and_then(|ship| ship.texture.as_deref());
        let ship_texture = if let Some(texture_path) = ship_texture_path {
            load_asset_texture(texture_path).await
        } else {
            eprintln!("Starter ship `{STARTER_SHIP_ID}` has no texture");
            None
        };
        let system_light_haze_texture = Some(make_system_light_haze_texture());
        let system_stars = make_system_stars(&content_registry);
        let planets = make_planets(&content_registry, world_seed, startup_background).await;
        let stations = make_station_destinations(&content_registry, startup_background).await;
        let npc_ships = make_npc_ships(&content_registry, startup_background).await;
        let recipe_vendor_locked_recipes = recipe_vendor_locked_recipes(&stations);
        let purchased_recipe_unlocks = save_data
            .as_ref()
            .map(|save| save.purchased_recipe_unlocks.clone())
            .unwrap_or_default();
        let installed_power_modules = starter_ship_def
            .as_ref()
            .map(|ship| installed_power_modules_from_ids(&content_registry, &ship.power_modules))
            .unwrap_or_default();
        let equipped_shields = starter_ship_def
            .as_ref()
            .map(|ship| equipped_shields_from_ids(&content_registry, &ship.shield_slots))
            .unwrap_or_default();
        let equipped_weapons = starter_ship_def
            .as_ref()
            .map(|ship| equipped_weapons_from_ids(&content_registry, &ship.weapon_slots))
            .unwrap_or_default();
        let defense_threats = make_defense_threats();

        let mut game = Self {
            content_registry,
            content_pack_options,
            transition_assets,
            scene_transition: None,
            current_system_id: STARTER_SYSTEM_ID.to_string(),
            save_path,
            world_seed,
            world_elapsed_days: 0.0,
            credits,
            ship: starter_ship_def
                .as_ref()
                .map(Ship::from_content)
                .unwrap_or_else(Ship::starter),
            installed_power_modules,
            equipped_shields,
            equipped_weapons,
            npc_ships,
            defense_threats,
            weapon_fire_events: Vec::new(),
            ship_texture,
            system_light_haze_texture,
            system_stars,
            planets,
            stations,
            recipe_vendor_locked_recipes,
            purchased_recipe_unlocks,
            selected_planet: None,
            selected_station: None,
            selected_npc_ship: None,
            selected_station_service: None,
            destination_planet: Some(1),
            orbiting_planet: None,
            system_destinations: HashMap::new(),
            pending_warp: None,
            camera_zoom: 1.0,
            starmap_zoom: 1.0,
            starmap_pan: Vec2::ZERO,
            starmap_drag_previous_mouse: None,
            action_rail_width_override: None,
            action_rail_resize_previous_mouse: None,
            inventory,
            smelt_recipes,
            smelt_settings,
            craft_recipes,
            craft_settings,
            processing_recipes,
            processing_settings,
            production_mode: ProductionMode::Smelting,
            skills: make_skills(),
            ship_upgrades: make_ship_upgrades(),
            inventory_open: true,
            map_open: false,
            skills_open: false,
            upgrades_open: false,
            content_open: false,
            content_browser: ContentBrowserState::default(),
            escape_dialog_open: false,
            quit_to_title_requested: false,
            starmap_filter: StarmapFilter::All,
            starmap_resource_filter_index: 0,
            work_scroll: 0.0,
            inventory_scroll: 0.0,
            upgrades_scroll: 0.0,
            shield_recharge_delay_remaining: 0.0,
            last_window_size: current_window_size(),
            window_save_delay: None,
            save_delay: Some(AUTOSAVE_SECONDS),
            save_dirty: true,
            save_status_timer: 0.0,
            save_status_manual: false,
        };
        if let Some(save_data) = save_data {
            draw_startup_transition(startup_background, "Restoring saved flight state ...", 1.0);
            next_frame().await;
            game.apply_save(save_data);
            game.save_dirty = false;
        }
        run_startup_transition_out(startup_background).await;
        game
    }

    fn apply_save(&mut self, save: SaveData) {
        self.world_seed = save.world_seed;
        self.world_elapsed_days = finite_nonnegative_or(save.world_elapsed_days, 0.0);
        self.credits = save.credits;
        self.purchased_recipe_unlocks = save
            .purchased_recipe_unlocks
            .into_iter()
            .filter(|recipe| self.recipe_vendor_locked_recipes.contains(recipe))
            .collect();
        update_planet_runtime_positions(&mut self.planets, self.world_elapsed_days);
        self.current_system_id = if self
            .content_registry
            .systems
            .contains_key(&save.current_system_id)
        {
            save.current_system_id
        } else {
            STARTER_SYSTEM_ID.to_string()
        };
        self.camera_zoom = finite_or(save.camera_zoom, default_camera_zoom())
            .clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX);
        self.ship.position = vec2(
            finite_or(save.ship.position[0], 0.0),
            finite_or(save.ship.position[1], 0.0),
        );
        self.ship.velocity = vec2(
            finite_or(save.ship.velocity[0], 0.0),
            finite_or(save.ship.velocity[1], 0.0),
        );
        self.ship.angle = finite_or(save.ship.angle, 0.0);
        self.ship.angular_velocity = finite_or(save.ship.angular_velocity, 0.0);
        self.ship.systems.hull = ShipResource::from_save(save.ship.hull);
        self.ship.systems.shields = ShipResource::from_save(save.ship.shields);
        self.ship.systems.energy = ShipResource::from_save(save.ship.energy);
        self.installed_power_modules = if save.installed_power_modules.is_empty() {
            default_installed_power_modules(&self.content_registry)
        } else {
            installed_power_modules_from_ids(&self.content_registry, &save.installed_power_modules)
        };
        self.equipped_shields = if save.shield_slots.is_empty() {
            default_equipped_shields(&self.content_registry)
        } else {
            equipped_shields_from_ids(&self.content_registry, &save.shield_slots)
        };
        self.shield_recharge_delay_remaining =
            finite_nonnegative_or(save.shield_recharge_delay_remaining, 0.0);
        self.equipped_weapons = if save.weapon_slots.is_empty() {
            default_equipped_weapons(&self.content_registry)
        } else {
            equipped_weapons_from_ids(&self.content_registry, &save.weapon_slots)
        };

        self.inventory = Inventory::from_save(&self.content_registry, &save.inventory);
        apply_skill_save(&mut self.skills, &save.skills);
        apply_upgrade_save(&mut self.ship_upgrades, &save.upgrades);
        self.rebuild_ship_from_upgrades();

        self.system_destinations = save
            .system_destinations
            .into_iter()
            .filter(|destination| {
                self.content_registry
                    .systems
                    .contains_key(&destination.system)
            })
            .filter(|destination| {
                self.planets.iter().any(|planet| {
                    planet.id == destination.planet && planet.system == destination.system
                })
            })
            .map(|destination| (destination.system, destination.planet))
            .collect();
        if let Some(destination_planet) = save.destination_planet {
            if let Some(planet) = self.planets.iter().find(|planet| {
                planet.id == destination_planet && planet.system == self.current_system_id
            }) {
                self.system_destinations
                    .insert(self.current_system_id.clone(), planet.id.clone());
            }
        }
        self.destination_planet = destination_planet_for_system(
            &self.planets,
            &self.system_destinations,
            &self.current_system_id,
        );
        self.orbiting_planet = save.orbiting_planet.as_deref().and_then(|planet_id| {
            self.planets.iter().position(|planet| {
                planet.id == planet_id && planet.system == self.current_system_id
            })
        });
        self.production_mode = ProductionMode::from_id(&save.production_mode);
        apply_work_settings(
            &mut self.smelt_settings,
            &self.smelt_recipes,
            &save.smelt_settings,
            |recipe| recipe.id.as_str(),
            |recipe| recipe.output.item.id.as_str(),
        );
        apply_work_settings(
            &mut self.craft_settings,
            &self.craft_recipes,
            &save.craft_settings,
            |recipe| recipe.id.as_str(),
            |recipe| recipe.output.item.id.as_str(),
        );
        apply_work_settings(
            &mut self.processing_settings,
            &self.processing_recipes,
            &save.processing_settings,
            |recipe| recipe.id.as_str(),
            |recipe| recipe.output.item.id.as_str(),
        );
        apply_planet_save(&mut self.planets, &save.planets);
    }

    fn to_save(&self) -> SaveData {
        SaveData {
            version: SAVE_VERSION,
            world_seed: self.world_seed,
            world_elapsed_days: finite_nonnegative_or(self.world_elapsed_days, 0.0),
            current_system_id: self.current_system_id.clone(),
            camera_zoom: finite_or(self.camera_zoom, default_camera_zoom())
                .clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX),
            credits: self.credits,
            ship: SaveShip {
                position: [
                    finite_or(self.ship.position.x, 0.0),
                    finite_or(self.ship.position.y, 0.0),
                ],
                velocity: [
                    finite_or(self.ship.velocity.x, 0.0),
                    finite_or(self.ship.velocity.y, 0.0),
                ],
                angle: finite_or(self.ship.angle, 0.0),
                angular_velocity: finite_or(self.ship.angular_velocity, 0.0),
                hull: self.ship.systems.hull.to_save(),
                shields: self.ship.systems.shields.to_save(),
                energy: self.ship.systems.energy.to_save(),
            },
            inventory: self.inventory.to_save(),
            skills: self
                .skills
                .iter()
                .map(|skill| SaveSkill {
                    kind: skill.kind.id().to_string(),
                    level: skill.level,
                    xp: finite_nonnegative_or(skill.xp, 0.0),
                })
                .collect(),
            upgrades: self
                .ship_upgrades
                .iter()
                .map(|upgrade| SaveUpgrade {
                    kind: upgrade.kind.id().to_string(),
                    level: upgrade.level,
                })
                .collect(),
            destination_planet: self
                .destination_planet
                .and_then(|index| self.planets.get(index))
                .map(|planet| planet.id.clone()),
            orbiting_planet: self
                .orbiting_planet
                .and_then(|index| self.planets.get(index))
                .filter(|planet| planet.system == self.current_system_id)
                .map(|planet| planet.id.clone()),
            installed_power_modules: self
                .installed_power_modules
                .iter()
                .map(|module| module.id.clone())
                .collect(),
            shield_slots: self
                .equipped_shields
                .iter()
                .map(|shield| shield.id.clone())
                .collect(),
            shield_recharge_delay_remaining: finite_nonnegative_or(
                self.shield_recharge_delay_remaining,
                0.0,
            ),
            weapon_slots: self
                .equipped_weapons
                .iter()
                .map(|weapon| weapon.id.clone())
                .collect(),
            system_destinations: save_system_destinations(self),
            content_pack_options: self.content_pack_options.clone(),
            purchased_recipe_unlocks: self.purchased_recipe_unlocks.clone(),
            production_mode: self.production_mode.id().to_string(),
            smelt_settings: save_work_settings(
                &self.smelt_recipes,
                &self.smelt_settings,
                |recipe| recipe.id.as_str(),
            ),
            craft_settings: save_work_settings(
                &self.craft_recipes,
                &self.craft_settings,
                |recipe| recipe.id.as_str(),
            ),
            processing_settings: save_work_settings(
                &self.processing_recipes,
                &self.processing_settings,
                |recipe| recipe.id.as_str(),
            ),
            planets: self
                .planets
                .iter()
                .map(|planet| SavePlanet {
                    id: planet.id.clone(),
                    scanned: planet_has_composition_scan(planet),
                    scan_level: planet.scan_level,
                    mining: save_work_settings(
                        &planet.info.mineables,
                        &planet.mining,
                        |mineable| mineable.item.id.as_str(),
                    ),
                })
                .collect(),
        }
    }

    fn rebuild_ship_from_upgrades(&mut self) {
        let systems = self.ship.systems;
        if let Some(starter_ship) = self.content_registry.ships.get(STARTER_SHIP_ID) {
            self.ship.attributes = ShipAttributes::from_content(starter_ship);
            self.ship.systems = ShipSystems::from_content(starter_ship, self.ship.attributes);
        } else {
            self.ship.attributes = ShipAttributes::starter();
            self.ship.systems = ShipSystems::starter(self.ship.attributes);
        }
        if self.equipped_shields.is_empty() {
            self.equipped_shields = default_equipped_shields(&self.content_registry);
        }
        self.ship.systems.shields.max = active_shield_capacity(self);
        for upgrade in self.ship_upgrades {
            for _ in 0..upgrade.level {
                apply_ship_upgrade(&mut self.ship, upgrade.kind);
            }
        }
        self.ship.systems.hull.current = systems
            .hull
            .current
            .min(self.ship.systems.hull.max)
            .max(0.0);
        self.ship.systems.shields.current = systems
            .shields
            .current
            .min(self.ship.systems.shields.max)
            .max(0.0);
        self.ship.systems.energy.current = systems
            .energy
            .current
            .min(self.ship.systems.energy.max)
            .max(0.0);
        if self.equipped_weapons.is_empty() {
            self.equipped_weapons = default_equipped_weapons(&self.content_registry);
        }
    }
}

fn load_game_content_registry() -> content::ContentRegistry {
    match content::load_content_packs(Path::new("content/packs")) {
        Ok(registry) => {
            println!(
                "Loaded {} content pack(s), {} item(s), {} ship(s), {} faction(s), {} NPC ship(s), {} shield(s), {} weapon(s), {} recipe(s), {} system(s), {} planet(s)",
                registry.packs.len(),
                registry.items.len(),
                registry.ships.len(),
                registry.factions.len(),
                registry.npc_ships.len(),
                registry.shields.len(),
                registry.weapons.len(),
                registry.recipes.len(),
                registry.systems.len(),
                registry.planets.len()
            );
            for warning in &registry.warnings {
                eprintln!("Content warning: {warning}");
            }
            registry
        }
        Err(errors) => {
            panic!("Failed to load content packs:\n{}", errors.join("\n"));
        }
    }
}

async fn make_planets(
    content_registry: &content::ContentRegistry,
    world_seed: u64,
    startup_background: Option<&Texture2D>,
) -> Vec<Planet> {
    let mut planets = Vec::new();
    for planet_id in &content_registry.planet_order {
        let Some(planet_def) = content_registry.planets.get(planet_id) else {
            continue;
        };
        let mineables = planet_def
            .mineables
            .iter()
            .filter_map(|item_id| {
                let item = registry_item(content_registry, item_id);
                if item.is_none() {
                    eprintln!(
                        "Planet `{}` mineable `{}` is missing from the content item registry",
                        planet_def.id, item_id
                    );
                }
                item.map(|item| Mineable { item })
            })
            .collect::<Vec<_>>();
        if mineables.is_empty() {
            eprintln!(
                "Skipping planet `{}` because it has no runtime-supported mineables",
                planet_def.id
            );
            continue;
        }

        let texture = match planet_def.texture.as_deref() {
            Some(path) => {
                draw_startup_transition(
                    startup_background,
                    &format!("Loading planet asset ... {}", asset_file_name(path)),
                    1.0,
                );
                next_frame().await;
                load_asset_texture(path).await
            }
            None => None,
        };
        let base_position = vec2(planet_def.position[0], planet_def.position[1]);
        let seeded_position = seeded_planet_position(base_position, world_seed, &planet_def.id);
        let motion = planet_motion_from_def(content_registry, planet_def, world_seed);
        planets.push(Planet {
            id: planet_def.id.clone(),
            system: planet_def.system.clone(),
            faction: planet_def.faction.clone(),
            base_position: seeded_position,
            position: runtime_position_from_motion(seeded_position, motion, 0.0),
            motion,
            radius: planet_def.radius,
            is_poi: planet_def.is_poi,
            texture,
            info: PlanetInfo {
                classification: planet_def.classification.clone(),
                mineables: mineables.clone(),
                hazards: planet_def.hazards.clone(),
                hazard_effects: HazardEffects {
                    shield_drain_per_second: planet_def.hazard_effects.shield_drain_per_second,
                    mining_speed_multiplier: planet_def.hazard_effects.mining_speed_multiplier,
                },
                summary: planet_def.summary.clone(),
            },
            mining: vec![MiningSetting::starter(); mineables.len()],
            scan_level: 0,
        });
    }

    resolve_planet_orbit_anchor_indices(&mut planets, content_registry);

    if planets.is_empty() {
        panic!("No planets loaded from content registry");
    }
    planets
}

fn make_system_stars(content_registry: &content::ContentRegistry) -> Vec<SystemStar> {
    content_registry
        .star_order
        .iter()
        .filter_map(|star_id| {
            let star_def = content_registry.stars.get(star_id)?;
            let is_primary = content_registry
                .systems
                .get(&star_def.system)
                .and_then(|system| system.primary_star.as_deref())
                == Some(star_def.id.as_str());
            Some(SystemStar {
                system: star_def.system.clone(),
                name: star_def.name.clone(),
                classification: star_def.classification.clone(),
                position: vec2(star_def.position[0], star_def.position[1]),
                radius: star_def.radius,
                color: Color::from_rgba(
                    star_def.color[0],
                    star_def.color[1],
                    star_def.color[2],
                    255,
                ),
                is_primary,
            })
        })
        .collect()
}

async fn make_station_destinations(
    content_registry: &content::ContentRegistry,
    startup_background: Option<&Texture2D>,
) -> Vec<StationDestination> {
    let mut stations = Vec::new();
    for station_id in &content_registry.station_order {
        let Some(station_def) = content_registry.stations.get(station_id) else {
            continue;
        };
        let (Some(system), Some(position)) = (&station_def.system, station_def.position) else {
            continue;
        };
        let texture = match station_def.texture.as_deref() {
            Some(path) => {
                draw_startup_transition(
                    startup_background,
                    &format!("Loading station asset ... {}", asset_file_name(path)),
                    1.0,
                );
                next_frame().await;
                load_asset_texture(path).await
            }
            None => None,
        };
        stations.push(StationDestination {
            id: station_def.id.clone(),
            system: system.clone(),
            name: station_def.name.clone(),
            position: vec2(position[0], position[1]),
            radius: station_def.radius,
            texture,
            icon: station_def.icon.clone(),
            culture: station_def.culture.clone(),
            faction: station_def.faction.clone(),
            summary: station_def
                .summary
                .clone()
                .unwrap_or_else(|| "A local station destination.".to_string()),
            services: station_def
                .services
                .iter()
                .map(|service| StationService {
                    id: service.id.clone(),
                    name: service.name.clone(),
                    kind: service.kind.clone(),
                    description: service.description.clone(),
                    trade: service
                        .trade
                        .iter()
                        .filter_map(|offer| {
                            registry_item(content_registry, &offer.item).map(|item| TradeOffer {
                                item,
                                buy_price: offer.buy_price,
                                sell_price: offer.sell_price,
                                stock: offer.stock,
                                restock_days: offer.restock_days,
                                unavailable: offer.unavailable,
                            })
                        })
                        .collect(),
                    recipe_unlocks: service
                        .recipe_unlocks
                        .iter()
                        .map(|unlock| RecipeUnlockOffer {
                            recipe: unlock.recipe.clone(),
                            price: unlock.price,
                            unavailable: unlock.unavailable,
                        })
                        .collect(),
                })
                .collect(),
        });
    }
    stations
}

async fn make_npc_ships(
    content_registry: &content::ContentRegistry,
    startup_background: Option<&Texture2D>,
) -> Vec<NpcShip> {
    let mut npc_ships = Vec::new();
    for npc_ship_id in &content_registry.npc_ship_order {
        let Some(npc_ship_def) = content_registry.npc_ships.get(npc_ship_id) else {
            continue;
        };
        let texture = match npc_ship_def.texture.as_deref() {
            Some(path) => {
                draw_startup_transition(
                    startup_background,
                    &format!("Loading NPC ship asset ... {}", asset_file_name(path)),
                    1.0,
                );
                next_frame().await;
                load_asset_texture(path).await
            }
            None => None,
        };
        let cargo_defaults = npc_ship_def
            .cargo_defaults
            .iter()
            .filter_map(|stack| {
                registry_item(content_registry, &stack.item).map(|item| ItemStack {
                    item,
                    count: stack.count,
                })
            })
            .collect::<Vec<_>>();
        let position = vec2(npc_ship_def.position[0], npc_ship_def.position[1]);
        let behavior = npc_behavior_mode(content_registry, npc_ship_def);

        npc_ships.push(NpcShip {
            id: npc_ship_def.id.clone(),
            name: npc_ship_def.name.clone(),
            system: npc_ship_def.system.clone(),
            position,
            velocity: Vec2::ZERO,
            angle: 0.0,
            radius: npc_ship_def.radius,
            texture,
            archetype: npc_ship_def.archetype.clone(),
            role: npc_ship_def.role.clone(),
            faction: npc_ship_def.faction.clone(),
            behavior_tags: npc_ship_def.behavior_tags.clone(),
            behavior,
            route_index: npc_initial_route_index(&npc_ship_def.id),
            anchor: position,
            identified: false,
            cargo_capacity: npc_ship_def.cargo_capacity,
            cargo_defaults,
            hull: ShipResource::full(npc_ship_def.hull_capacity),
            shields: ShipResource::full(npc_ship_def.shield_capacity),
            energy: ShipResource::full(npc_ship_def.energy_capacity),
            shield_slots: npc_ship_def.shield_slots.clone(),
            weapon_slots: npc_ship_def.weapon_slots.clone(),
            summary: npc_ship_def
                .summary
                .clone()
                .unwrap_or_else(|| "Non-player ship contact.".to_string()),
        });
    }
    npc_ships
}

fn npc_behavior_mode(
    content_registry: &content::ContentRegistry,
    npc_ship: &content::NpcShipDef,
) -> NpcBehaviorMode {
    if npc_ship_has_behavior_tag(npc_ship, "flee") {
        return NpcBehaviorMode::Flee;
    }
    if npc_ship_has_behavior_tag(npc_ship, "follow") {
        return NpcBehaviorMode::Follow;
    }
    if npc_ship_has_behavior_tag(npc_ship, "trade-route") || npc_ship.role == "trader" {
        return NpcBehaviorMode::TradeRoute;
    }
    if npc_ship_has_behavior_tag(npc_ship, "traffic") || npc_ship.role == "traffic" {
        return NpcBehaviorMode::StationTraffic;
    }
    if npc_ship_has_behavior_tag(npc_ship, "hostile")
        || npc_ship.role == "hostile"
        || npc_ship.faction.as_deref().is_some_and(|faction_id| {
            content_registry
                .factions
                .get(faction_id)
                .is_some_and(|faction| {
                    faction.default_disposition == content::FactionDisposition::Hostile
                })
        })
    {
        return NpcBehaviorMode::HostileIntercept;
    }
    NpcBehaviorMode::Patrol
}

fn npc_ship_has_behavior_tag(npc_ship: &content::NpcShipDef, tag: &str) -> bool {
    npc_ship
        .behavior_tags
        .iter()
        .any(|behavior_tag| behavior_tag == tag)
}

fn npc_initial_route_index(id: &str) -> usize {
    id.bytes().fold(0usize, |hash, byte| {
        hash.wrapping_mul(31).wrapping_add(byte as usize)
    }) % NPC_ROUTE_POINTS.len()
}

fn recipe_vendor_locked_recipes(stations: &[StationDestination]) -> Vec<String> {
    let mut recipes = Vec::new();
    for station in stations {
        for service in &station.services {
            for unlock in &service.recipe_unlocks {
                if !recipes.contains(&unlock.recipe) {
                    recipes.push(unlock.recipe.clone());
                }
            }
        }
    }
    recipes
}

fn recipe_is_unlocked(game: &GameState, recipe_id: &str) -> bool {
    !game
        .recipe_vendor_locked_recipes
        .iter()
        .any(|locked| locked == recipe_id)
        || game
            .purchased_recipe_unlocks
            .iter()
            .any(|unlocked| unlocked == recipe_id)
}

async fn load_asset_texture(path: &str) -> Option<Texture2D> {
    match load_texture(path).await {
        Ok(texture) => {
            texture.set_filter(FilterMode::Linear);
            Some(texture)
        }
        Err(_) => None,
    }
}

fn make_system_light_haze_texture() -> Texture2D {
    let size = 512_u16;
    let center = (size as f32 - 1.0) * 0.5;
    let mut bytes = Vec::with_capacity(size as usize * size as usize * 4);

    for y in 0..size {
        for x in 0..size {
            let dx = (x as f32 - center) / center;
            let dy = (y as f32 - center) / center;
            let distance = (dx * dx + dy * dy).sqrt().clamp(0.0, 1.0);
            let alpha = radial_haze_alpha(distance);
            bytes.extend_from_slice(&[255, 255, 255, alpha]);
        }
    }

    let texture = Texture2D::from_rgba8(size, size, &bytes);
    texture.set_filter(FilterMode::Linear);
    texture
}

fn radial_haze_alpha(distance: f32) -> u8 {
    let falloff = 1.0 - smoothstep(0.0, 1.0, distance);
    (falloff.powf(2.2) * 255.0).round().clamp(0.0, 255.0) as u8
}

fn smoothstep(edge0: f32, edge1: f32, value: f32) -> f32 {
    let t = ((value - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

async fn load_ui_font() -> Option<Font> {
    match load_ttf_font(UI_FONT_PATH).await {
        Ok(font) => Some(font),
        Err(error) => {
            eprintln!("Failed to load UI font `{UI_FONT_PATH}`: {error}");
            None
        }
    }
}

fn set_ui_font(font: Option<Font>) {
    UI_FONT.with(|ui_font| {
        *ui_font.borrow_mut() = font;
    });
}

fn draw_text(text: &str, x: f32, y: f32, font_size: f32, color: Color) -> TextDimensions {
    UI_FONT.with(|ui_font| {
        let ui_font = ui_font.borrow();
        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: ui_font.as_ref(),
                font_size: font_size.round().max(1.0) as u16,
                font_scale: 1.0,
                color,
                ..Default::default()
            },
        )
    })
}

fn measure_text(
    text: &str,
    font: Option<&Font>,
    font_size: u16,
    font_scale: f32,
) -> TextDimensions {
    if font.is_some() {
        return macroquad::prelude::measure_text(text, font, font_size, font_scale);
    }

    UI_FONT.with(|ui_font| {
        let ui_font = ui_font.borrow();
        macroquad::prelude::measure_text(text, ui_font.as_ref(), font_size, font_scale)
    })
}

async fn load_transition_assets(root: &Path) -> Vec<TransitionAsset> {
    let _ = fs::create_dir_all(root);
    let Ok(entries) = fs::read_dir(root) else {
        eprintln!("Transition asset directory unavailable: {}", root.display());
        return Vec::new();
    };

    let mut paths = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && is_supported_transition_image(path))
        .collect::<Vec<_>>();
    paths.sort();

    let mut assets = Vec::new();
    for path in paths {
        let path_string = path.to_string_lossy().to_string();
        let startup_background = assets.first().map(|asset: &TransitionAsset| &asset.texture);
        draw_startup_transition(
            startup_background,
            &format!(
                "Loading transition asset ... {}",
                asset_file_name(&path_string)
            ),
            1.0,
        );
        next_frame().await;
        let Some(texture) = load_asset_texture(&path_string).await else {
            eprintln!("Skipping transition asset `{path_string}` because it failed to load");
            continue;
        };
        assets.push(TransitionAsset {
            path: path_string,
            texture,
        });
    }

    assets
}

fn is_supported_transition_image(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "png" | "jpg" | "jpeg"
            )
        })
        .unwrap_or(false)
}

fn asset_file_name(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .unwrap_or(path)
}

fn select_transition_texture(assets: &[TransitionAsset]) -> Option<Texture2D> {
    if assets.is_empty() {
        return None;
    }

    let index = (rand::gen_range(0.0, assets.len() as f32) as usize).min(assets.len() - 1);
    Some(assets[index].texture.clone())
}

impl SceneTransition {
    fn total_seconds(&self) -> f32 {
        self.fade_in_seconds + self.hold_seconds + self.fade_out_seconds
    }

    fn phase(&self) -> TransitionPhase {
        if self.timer < self.fade_in_seconds {
            TransitionPhase::FadeIn
        } else if self.timer < self.fade_in_seconds + self.hold_seconds {
            TransitionPhase::Hold
        } else {
            TransitionPhase::FadeOut
        }
    }

    fn opacity(&self) -> f32 {
        match self.phase() {
            TransitionPhase::FadeIn => (self.timer / self.fade_in_seconds).clamp(0.0, 1.0),
            TransitionPhase::Hold => 1.0,
            TransitionPhase::FadeOut => {
                let fade_out_elapsed = self.timer - self.fade_in_seconds - self.hold_seconds;
                1.0 - (fade_out_elapsed / self.fade_out_seconds).clamp(0.0, 1.0)
            }
        }
    }
}

impl ShipAttributes {
    fn starter() -> Self {
        Self {
            mass: STARTER_SHIP_DRY_MASS,
            engine_strength: STARTER_SHIP_DRY_MASS * STARTER_FORWARD_ACCELERATION,
            reverse_engine_strength: STARTER_SHIP_DRY_MASS * STARTER_REVERSE_ACCELERATION,
            turn_thruster_strength: STARTER_SHIP_DRY_MASS * STARTER_TURN_ACCELERATION,
            energy_capacity: 100.0,
            energy_recharge: 22.0,
            linear_drag: 0.985,
        }
    }

    fn from_content(ship: &content::ShipDef) -> Self {
        Self {
            mass: ship.mass,
            engine_strength: ship.mass * ship.forward_acceleration,
            reverse_engine_strength: ship.mass * ship.reverse_acceleration,
            turn_thruster_strength: ship.mass * ship.turn_acceleration,
            energy_capacity: ship.energy_capacity,
            energy_recharge: ship.energy_recharge,
            linear_drag: ship.linear_drag,
        }
    }
}

impl ShipSystems {
    fn starter(attributes: ShipAttributes) -> Self {
        Self {
            hull: ShipResource::full(100.0),
            shields: ShipResource::full(100.0),
            energy: ShipResource::full(attributes.energy_capacity),
        }
    }

    fn from_content(ship: &content::ShipDef, attributes: ShipAttributes) -> Self {
        Self {
            hull: ShipResource::full(ship.hull_capacity),
            shields: ShipResource::full(ship.shield_capacity),
            energy: ShipResource::full(attributes.energy_capacity),
        }
    }
}

impl ShipResource {
    fn full(max: f32) -> Self {
        Self { current: max, max }
    }

    fn from_save(save: SaveResource) -> Self {
        let max = finite_nonnegative_or(save.max, 1.0).max(1.0);
        Self {
            current: finite_nonnegative_or(save.current, max).min(max),
            max,
        }
    }

    fn to_save(self) -> SaveResource {
        let max = finite_nonnegative_or(self.max, 1.0).max(1.0);
        SaveResource {
            current: finite_nonnegative_or(self.current, max).min(max),
            max,
        }
    }

    fn fraction(self) -> f32 {
        (self.current / self.max).clamp(0.0, 1.0)
    }

    fn spend(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    fn restore(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }
}

impl Ship {
    fn starter() -> Self {
        let attributes = ShipAttributes::starter();
        Self {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            angle: 0.0,
            angular_velocity: 0.0,
            attributes,
            systems: ShipSystems::starter(attributes),
        }
    }

    fn from_content(ship: &content::ShipDef) -> Self {
        let attributes = ShipAttributes::from_content(ship);
        Self {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            angle: 0.0,
            angular_velocity: 0.0,
            attributes,
            systems: ShipSystems::from_content(ship, attributes),
        }
    }

    fn forward_acceleration(&self) -> f32 {
        self.attributes.engine_strength / self.attributes.mass
    }

    fn reverse_acceleration(&self) -> f32 {
        self.attributes.reverse_engine_strength / self.attributes.mass
    }

    fn turn_acceleration(&self) -> f32 {
        self.attributes.turn_thruster_strength / self.attributes.mass
    }

    fn max_turn_rate(&self) -> f32 {
        self.turn_acceleration() * 0.18
    }
}

impl ItemRef {
    fn from_def(item: &content::ItemDef) -> Self {
        Self {
            id: item.id.clone(),
            name: item.name.clone(),
            xp_value: item.xp_value,
            unit_mass: item.unit_mass,
        }
    }

    fn is_id(&self, id: &str) -> bool {
        self.id == id
    }
}

impl PowerModule {
    fn from_def(module: &content::PowerModuleDef) -> Self {
        Self {
            id: module.id.clone(),
            name: module.name.clone(),
            family: module.family.clone(),
            generation: module.generation,
            mass: module.mass,
            fuel_item: module.fuel_item.clone(),
            fuel_per_minute: module.fuel_per_minute,
            heat: module.heat,
            risk: module.risk,
        }
    }
}

impl ShieldSystem {
    fn from_def(shield: &content::ShieldDef) -> Self {
        Self {
            id: shield.id.clone(),
            name: shield.name.clone(),
            install_item: shield.install_item.clone(),
            capacity: shield.capacity,
            recharge_delay: shield.recharge_delay,
            recharge_rate: shield.recharge_rate,
            damage_resistance: shield.damage_resistance,
            hazard_resistance: shield.hazard_resistance,
        }
    }
}

impl WeaponSystem {
    fn from_def(weapon: &content::WeaponDef) -> Self {
        Self {
            id: weapon.id.clone(),
            name: weapon.name.clone(),
            kind: weapon.kind,
            install_item: weapon.install_item.clone(),
            range: weapon.range,
            cooldown_seconds: weapon.cooldown_seconds,
            damage: weapon.damage,
            energy_cost: weapon.energy_cost,
            tracking_degrees: weapon.tracking_degrees,
            cooldown_remaining: 0.0,
            status: WeaponStatus::Ready,
        }
    }

    fn readiness_label(&self) -> &'static str {
        match self.status {
            WeaponStatus::Ready => "ready",
            WeaponStatus::NoThreat => "no threats",
            WeaponStatus::Cooldown => "cooldown",
            WeaponStatus::InsufficientEnergy => "low energy",
            WeaponStatus::Fired => "fired",
        }
    }
}

impl ThreatDisposition {
    fn label(self) -> &'static str {
        match self {
            Self::Hostile => "hostile",
            Self::Neutral => "neutral",
            Self::Owned => "owned",
            Self::Environmental => "environmental",
        }
    }
}

fn registry_item(content_registry: &content::ContentRegistry, item_id: &str) -> Option<ItemRef> {
    content_registry.items.get(item_id).map(ItemRef::from_def)
}

fn core_item(content_registry: &content::ContentRegistry, local_id: &str) -> Option<ItemRef> {
    registry_item(content_registry, &format!("core:{local_id}"))
}

fn required_item(content_registry: &content::ContentRegistry, item_id: &str) -> ItemRef {
    registry_item(content_registry, item_id)
        .unwrap_or_else(|| panic!("Required item `{item_id}` is missing from the content registry"))
}

fn installed_power_modules_from_ids(
    content_registry: &content::ContentRegistry,
    module_ids: &[String],
) -> Vec<PowerModule> {
    module_ids
        .iter()
        .filter_map(|module_id| {
            content_registry
                .power_modules
                .get(module_id)
                .map(PowerModule::from_def)
        })
        .collect()
}

fn default_installed_power_modules(
    content_registry: &content::ContentRegistry,
) -> Vec<PowerModule> {
    content_registry
        .ships
        .get(STARTER_SHIP_ID)
        .map(|ship| installed_power_modules_from_ids(content_registry, &ship.power_modules))
        .unwrap_or_default()
}

fn equipped_shields_from_ids(
    content_registry: &content::ContentRegistry,
    shield_ids: &[String],
) -> Vec<ShieldSystem> {
    shield_ids
        .iter()
        .filter_map(|shield_id| {
            content_registry
                .shields
                .get(shield_id)
                .map(ShieldSystem::from_def)
        })
        .collect()
}

fn default_equipped_shields(content_registry: &content::ContentRegistry) -> Vec<ShieldSystem> {
    content_registry
        .ships
        .get(STARTER_SHIP_ID)
        .map(|ship| equipped_shields_from_ids(content_registry, &ship.shield_slots))
        .unwrap_or_default()
}

fn equipped_weapons_from_ids(
    content_registry: &content::ContentRegistry,
    weapon_ids: &[String],
) -> Vec<WeaponSystem> {
    weapon_ids
        .iter()
        .filter_map(|weapon_id| {
            content_registry
                .weapons
                .get(weapon_id)
                .map(WeaponSystem::from_def)
        })
        .collect()
}

fn default_equipped_weapons(content_registry: &content::ContentRegistry) -> Vec<WeaponSystem> {
    content_registry
        .ships
        .get(STARTER_SHIP_ID)
        .map(|ship| equipped_weapons_from_ids(content_registry, &ship.weapon_slots))
        .unwrap_or_default()
}

#[derive(Debug, PartialEq, Eq)]
enum WeaponInstallError {
    InvalidSlot,
    UnknownWeapon,
    MissingInstallItem,
}

#[derive(Debug, PartialEq, Eq)]
enum ShieldInstallError {
    InvalidSlot,
    UnknownShield,
    MissingInstallItem,
}

fn shield_slot_capacity(game: &GameState) -> usize {
    game.content_registry
        .ships
        .get(STARTER_SHIP_ID)
        .map(|ship| ship.shield_slots.len())
        .unwrap_or(0)
        .max(game.equipped_shields.len())
}

fn install_shield_in_slot(
    game: &mut GameState,
    slot_index: usize,
    shield_id: &str,
) -> Result<(), ShieldInstallError> {
    if slot_index >= shield_slot_capacity(game) {
        return Err(ShieldInstallError::InvalidSlot);
    }

    if game
        .equipped_shields
        .get(slot_index)
        .is_some_and(|shield| shield.id == shield_id)
    {
        return Ok(());
    }

    let Some(shield_def) = game.content_registry.shields.get(shield_id).cloned() else {
        return Err(ShieldInstallError::UnknownShield);
    };
    let install_item = required_item(&game.content_registry, &shield_def.install_item);
    if game.inventory.count(&install_item) == 0 {
        return Err(ShieldInstallError::MissingInstallItem);
    }

    game.inventory.remove_item(&install_item, 1);
    if let Some(previous_shield) = game.equipped_shields.get(slot_index) {
        let previous_item = required_item(&game.content_registry, &previous_shield.install_item);
        game.inventory.add_item(previous_item, 1);
    }

    let installed_shield = ShieldSystem::from_def(&shield_def);
    if slot_index < game.equipped_shields.len() {
        game.equipped_shields[slot_index] = installed_shield;
    } else {
        game.equipped_shields.push(installed_shield);
    }
    game.rebuild_ship_from_upgrades();
    game.save_dirty = true;
    Ok(())
}

fn weapon_slot_capacity(game: &GameState) -> usize {
    game.content_registry
        .ships
        .get(STARTER_SHIP_ID)
        .map(|ship| ship.weapon_slots.len())
        .unwrap_or(0)
        .max(game.equipped_weapons.len())
}

fn install_weapon_in_slot(
    game: &mut GameState,
    slot_index: usize,
    weapon_id: &str,
) -> Result<(), WeaponInstallError> {
    if slot_index >= weapon_slot_capacity(game) {
        return Err(WeaponInstallError::InvalidSlot);
    }

    if game
        .equipped_weapons
        .get(slot_index)
        .is_some_and(|weapon| weapon.id == weapon_id)
    {
        return Ok(());
    }

    let Some(weapon_def) = game.content_registry.weapons.get(weapon_id).cloned() else {
        return Err(WeaponInstallError::UnknownWeapon);
    };
    let install_item = required_item(&game.content_registry, &weapon_def.install_item);
    if game.inventory.count(&install_item) == 0 {
        return Err(WeaponInstallError::MissingInstallItem);
    }

    game.inventory.remove_item(&install_item, 1);
    if let Some(previous_weapon) = game.equipped_weapons.get(slot_index) {
        let previous_item = required_item(&game.content_registry, &previous_weapon.install_item);
        game.inventory.add_item(previous_item, 1);
    }

    let installed_weapon = WeaponSystem::from_def(&weapon_def);
    if slot_index < game.equipped_weapons.len() {
        game.equipped_weapons[slot_index] = installed_weapon;
    } else {
        game.equipped_weapons.push(installed_weapon);
    }
    game.save_dirty = true;
    Ok(())
}

fn active_shield(game: &GameState) -> Option<&ShieldSystem> {
    game.equipped_shields.first()
}

fn active_shield_capacity(game: &GameState) -> f32 {
    active_shield(game)
        .map(|shield| shield.capacity)
        .unwrap_or(game.ship.systems.shields.max)
}

fn active_shield_recharge_delay(game: &GameState) -> f32 {
    active_shield(game)
        .map(|shield| shield.recharge_delay)
        .unwrap_or(4.0)
}

fn active_shield_recharge_rate(game: &GameState) -> f32 {
    active_shield(game)
        .map(|shield| shield.recharge_rate)
        .unwrap_or(0.0)
}

fn active_shield_hazard_resistance(game: &GameState) -> f32 {
    active_shield(game)
        .map(|shield| shield.hazard_resistance)
        .unwrap_or(0.0)
}

fn shield_hazard_drain_after_resistance(game: &GameState, drain: f32) -> f32 {
    drain.max(0.0) * (1.0 - active_shield_hazard_resistance(game)).clamp(0.0, 1.0)
}

fn apply_shield_hazard_drain(game: &mut GameState, amount: f32) {
    let damage = shield_hazard_drain_after_resistance(game, amount);
    if damage <= 0.0 {
        return;
    }
    game.ship.systems.shields.spend(damage);
    game.shield_recharge_delay_remaining = active_shield_recharge_delay(game);
}

fn make_defense_threats() -> Vec<DefenseThreat> {
    vec![
        DefenseThreat {
            id: "core:raider_probe_alpha".to_string(),
            name: "Raider probe alpha".to_string(),
            system: STARTER_SYSTEM_ID.to_string(),
            position: vec2(620.0, -220.0),
            radius: DEFENSE_THREAT_RADIUS,
            disposition: ThreatDisposition::Hostile,
            hull: ShipResource::full(36.0),
        },
        DefenseThreat {
            id: "core:frontier_beacon_drone".to_string(),
            name: "Frontier beacon drone".to_string(),
            system: STARTER_SYSTEM_ID.to_string(),
            position: vec2(-360.0, 260.0),
            radius: DEFENSE_THREAT_RADIUS,
            disposition: ThreatDisposition::Neutral,
            hull: ShipResource::full(24.0),
        },
        DefenseThreat {
            id: "core:owned_survey_drone".to_string(),
            name: "Owned survey drone".to_string(),
            system: STARTER_SYSTEM_ID.to_string(),
            position: vec2(-520.0, -260.0),
            radius: DEFENSE_THREAT_RADIUS,
            disposition: ThreatDisposition::Owned,
            hull: ShipResource::full(18.0),
        },
        DefenseThreat {
            id: "core:static_hazard_echo".to_string(),
            name: "Static hazard echo".to_string(),
            system: STARTER_SYSTEM_ID.to_string(),
            position: vec2(240.0, 520.0),
            radius: DEFENSE_THREAT_RADIUS,
            disposition: ThreatDisposition::Environmental,
            hull: ShipResource::full(18.0),
        },
    ]
}

fn ship_energy_recharge(ship: &Ship, installed_power_modules: &[PowerModule]) -> f32 {
    ship.attributes.energy_recharge
        + installed_power_modules
            .iter()
            .map(|module| module.generation)
            .sum::<f32>()
}

impl Skill {
    fn new(kind: SkillKind) -> Self {
        Self {
            kind,
            level: 0,
            xp: 0.0,
        }
    }

    fn cost_to_next_level(&self) -> f32 {
        35.0 + self.level as f32 * 22.0
    }

    fn can_level_up(&self) -> bool {
        self.xp >= self.cost_to_next_level()
    }

    fn speed_bonus_percent(&self) -> f32 {
        self.level.min(10) as f32
    }

    fn bonus_output_chance_percent(&self) -> f32 {
        self.level.saturating_sub(10).min(10) as f32 * 0.05
    }

    fn buy_level(&mut self) -> bool {
        let cost = self.cost_to_next_level();
        if self.xp < cost {
            return false;
        }

        self.xp -= cost;
        self.level += 1;
        true
    }
}

impl SkillKind {
    fn id(self) -> &'static str {
        match self {
            Self::Mining => "mining",
            Self::Smelting => "smelting",
            Self::Fabrication => "fabrication",
        }
    }

    fn from_id(id: &str) -> Option<Self> {
        match id {
            "mining" => Some(Self::Mining),
            "smelting" => Some(Self::Smelting),
            "fabrication" => Some(Self::Fabrication),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Mining => "Mining",
            Self::Smelting => "Smelting",
            Self::Fabrication => "Fabrication",
        }
    }

    fn action_name(self) -> &'static str {
        match self {
            Self::Mining => "mining",
            Self::Smelting => "smelting",
            Self::Fabrication => "fabrication",
        }
    }
}

impl ShipUpgrade {
    fn new(kind: ShipUpgradeKind) -> Self {
        Self { kind, level: 0 }
    }

    fn next_cost(&self, content_registry: &content::ContentRegistry) -> Vec<ItemStack> {
        self.kind.cost_for_level(self.level + 1, content_registry)
    }
}

impl ShipUpgradeKind {
    fn id(self) -> &'static str {
        match self {
            Self::Engine => "core:engine",
            Self::Thrusters => "core:thrusters",
            Self::EnergyCore => "core:energy_core",
            Self::Shields => "core:shields",
            Self::DroneBay => "core:drone_bay",
            Self::FuelSystems => "core:fuel_systems",
            Self::ScannerArray => "core:scanner_array",
            Self::CargoHold => "core:cargo_hold",
        }
    }

    fn from_id(id: &str) -> Option<Self> {
        match id {
            "core:engine" => Some(Self::Engine),
            "core:thrusters" => Some(Self::Thrusters),
            "core:energy_core" => Some(Self::EnergyCore),
            "core:shields" => Some(Self::Shields),
            "core:drone_bay" => Some(Self::DroneBay),
            "core:fuel_systems" => Some(Self::FuelSystems),
            "core:scanner_array" => Some(Self::ScannerArray),
            "core:cargo_hold" => Some(Self::CargoHold),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Engine => "Engine tuning",
            Self::Thrusters => "Maneuver thrusters",
            Self::EnergyCore => "Energy core",
            Self::Shields => "Shield matrix",
            Self::DroneBay => "Drone bay",
            Self::FuelSystems => "Fuel systems",
            Self::ScannerArray => "Scanner array",
            Self::CargoHold => "Cargo hold",
        }
    }

    fn effect_text(self) -> &'static str {
        match self {
            Self::Engine => "+8% forward thrust, +6% reverse thrust",
            Self::Thrusters => "+8% turn authority",
            Self::EnergyCore => "+15 energy capacity, +3 recharge",
            Self::Shields => "+15 shield capacity",
            Self::DroneBay => "+10% survey drone return chance",
            Self::FuelSystems => "-10% warp charge time",
            Self::ScannerArray => "+1 survey depth every 2 levels",
            Self::CargoHold => "+10 t cargo rating",
        }
    }

    fn cost_for_level(
        self,
        level: u32,
        content_registry: &content::ContentRegistry,
    ) -> Vec<ItemStack> {
        let upgrade = content_registry
            .upgrades
            .get(self.id())
            .unwrap_or_else(|| panic!("Required upgrade `{}` is missing from content", self.id()));

        upgrade
            .costs
            .iter()
            .filter_map(|cost| {
                let count =
                    cost.base_count + level.saturating_mul(cost.per_level) / cost.per_levels;
                (count > 0).then(|| ItemStack {
                    item: required_item(content_registry, &cost.item),
                    count,
                })
            })
            .collect()
    }
}

impl ProductionMode {
    fn id(self) -> &'static str {
        match self {
            Self::Smelting => "smelting",
            Self::Crafting => "crafting",
            Self::Processing => "processing",
        }
    }

    fn from_id(id: &str) -> Self {
        match id {
            "crafting" => Self::Crafting,
            "processing" => Self::Processing,
            _ => Self::Smelting,
        }
    }
}

impl StarmapFilter {
    fn next(self) -> Self {
        match self {
            Self::All => Self::Scanned,
            Self::Scanned => Self::Unscanned,
            Self::Unscanned => Self::Destination,
            Self::Destination => Self::Resource,
            Self::Resource => Self::All,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Scanned => "Scanned",
            Self::Unscanned => "Unscanned",
            Self::Destination => "Destination",
            Self::Resource => "Resource",
        }
    }
}

fn make_skills() -> [Skill; SKILL_COUNT] {
    [
        Skill::new(SkillKind::Mining),
        Skill::new(SkillKind::Smelting),
        Skill::new(SkillKind::Fabrication),
    ]
}

fn make_ship_upgrades() -> [ShipUpgrade; SHIP_UPGRADE_COUNT] {
    [
        ShipUpgrade::new(ShipUpgradeKind::Engine),
        ShipUpgrade::new(ShipUpgradeKind::Thrusters),
        ShipUpgrade::new(ShipUpgradeKind::EnergyCore),
        ShipUpgrade::new(ShipUpgradeKind::Shields),
        ShipUpgrade::new(ShipUpgradeKind::DroneBay),
        ShipUpgrade::new(ShipUpgradeKind::FuelSystems),
        ShipUpgrade::new(ShipUpgradeKind::ScannerArray),
        ShipUpgrade::new(ShipUpgradeKind::CargoHold),
    ]
}

fn award_skill_xp(skills: &mut [Skill; SKILL_COUNT], kind: SkillKind, item: &ItemRef, count: u32) {
    let Some(skill) = skills.iter_mut().find(|skill| skill.kind == kind) else {
        return;
    };
    let diminishing = 1.0 / (1.0 + skill.level as f32 * 0.32);
    let minimum = 0.08;
    let xp = item.xp_value * count as f32 * diminishing.max(minimum);
    skill.xp += xp;
}

fn bonus_output_count(skills: &[Skill; SKILL_COUNT], kind: SkillKind, count: u32) -> u32 {
    let Some(skill) = skills.iter().find(|skill| skill.kind == kind) else {
        return 0;
    };
    let chance = skill.bonus_output_chance_percent() / 100.0;
    if chance <= 0.0 {
        return 0;
    }

    (0..count)
        .filter(|_| rand::gen_range(0.0, 1.0) < chance)
        .count() as u32
}

fn ship_upgrade_level(upgrades: &[ShipUpgrade; SHIP_UPGRADE_COUNT], kind: ShipUpgradeKind) -> u32 {
    upgrades
        .iter()
        .find(|upgrade| upgrade.kind == kind)
        .map(|upgrade| upgrade.level)
        .unwrap_or(0)
}

fn survey_drone_return_chance(upgrades: &[ShipUpgrade; SHIP_UPGRADE_COUNT]) -> f32 {
    (ship_upgrade_level(upgrades, ShipUpgradeKind::DroneBay) as f32 * 0.10).min(0.65)
}

fn scanner_survey_bonus(upgrades: &[ShipUpgrade; SHIP_UPGRADE_COUNT]) -> u8 {
    (ship_upgrade_level(upgrades, ShipUpgradeKind::ScannerArray) / 2).min(2) as u8
}

fn warp_charge_seconds(upgrades: &[ShipUpgrade; SHIP_UPGRADE_COUNT]) -> f32 {
    let fuel_level = ship_upgrade_level(upgrades, ShipUpgradeKind::FuelSystems) as f32;
    WARP_CHARGE_SECONDS * (1.0 - fuel_level * 0.10).max(0.45)
}

fn cargo_rating_kg(upgrades: &[ShipUpgrade; SHIP_UPGRADE_COUNT]) -> f32 {
    20_000.0 + ship_upgrade_level(upgrades, ShipUpgradeKind::CargoHold) as f32 * 10_000.0
}

fn can_afford_cost(inventory: &Inventory, cost: &[ItemStack]) -> bool {
    cost.iter()
        .all(|stack| inventory.count(&stack.item) >= stack.count)
}

fn pay_cost(inventory: &mut Inventory, cost: &[ItemStack]) {
    for stack in cost {
        inventory.remove_item(&stack.item, stack.count);
    }
}

fn buy_ship_upgrade(game: &mut GameState, upgrade_index: usize) -> bool {
    let Some(upgrade) = game.ship_upgrades.get(upgrade_index).copied() else {
        return false;
    };
    let cost = upgrade.next_cost(&game.content_registry);
    if !can_afford_cost(&game.inventory, &cost) {
        return false;
    }

    pay_cost(&mut game.inventory, &cost);
    apply_ship_upgrade(&mut game.ship, upgrade.kind);
    game.ship_upgrades[upgrade_index].level += 1;
    true
}

fn apply_ship_upgrade(ship: &mut Ship, kind: ShipUpgradeKind) {
    match kind {
        ShipUpgradeKind::Engine => {
            ship.attributes.engine_strength *= 1.08;
            ship.attributes.reverse_engine_strength *= 1.06;
        }
        ShipUpgradeKind::Thrusters => {
            ship.attributes.turn_thruster_strength *= 1.08;
        }
        ShipUpgradeKind::EnergyCore => {
            ship.attributes.energy_capacity += 15.0;
            ship.attributes.energy_recharge += 3.0;
            ship.systems.energy.max += 15.0;
            ship.systems.energy.restore(15.0);
        }
        ShipUpgradeKind::Shields => {
            ship.systems.shields.max += 15.0;
            ship.systems.shields.restore(15.0);
        }
        ShipUpgradeKind::DroneBay
        | ShipUpgradeKind::FuelSystems
        | ShipUpgradeKind::ScannerArray
        | ShipUpgradeKind::CargoHold => {}
    }
}

fn skill_speed_bonus_percent(skills: &[Skill; SKILL_COUNT], kind: SkillKind) -> f32 {
    skills
        .iter()
        .find(|skill| skill.kind == kind)
        .map(|skill| skill.speed_bonus_percent())
        .unwrap_or(0.0)
}

fn mining_operation_seconds(skills: &[Skill; SKILL_COUNT]) -> f32 {
    BASE_MINING_SECONDS / (1.0 + skill_speed_bonus_percent(skills, SkillKind::Mining) / 100.0)
}

fn recipe_operation_seconds(
    skills: &[Skill; SKILL_COUNT],
    skill_kind: SkillKind,
    recipe: &Recipe,
) -> f32 {
    let base_seconds = if skill_kind == SkillKind::Mining {
        BASE_MINING_SECONDS
    } else if skill_kind == SkillKind::Fabrication && recipe.output.item.is_id("core:circuit") {
        recipe.base_seconds * 2.0
    } else {
        recipe.base_seconds
    };
    base_seconds / (1.0 + skill_speed_bonus_percent(skills, skill_kind) / 100.0)
}

impl CraftSetting {
    fn starter() -> Self {
        Self {
            keep: 0,
            queued: 0,
            progress: 0.0,
        }
    }
}

impl MiningSetting {
    fn starter() -> Self {
        Self {
            keep: 0,
            queued: 0,
            progress: 0.0,
        }
    }
}

fn scan_level_from_save(saved_planet: &SavePlanet) -> u8 {
    if saved_planet.scan_level > 0 {
        saved_planet.scan_level.min(MAX_SCAN_LEVEL)
    } else if saved_planet.scanned {
        2
    } else {
        0
    }
}

fn planet_has_surface_scan(planet: &Planet) -> bool {
    planet.scan_level >= 1
}

fn planet_has_composition_scan(planet: &Planet) -> bool {
    planet.scan_level >= 2
}

fn planet_has_richness_scan(planet: &Planet) -> bool {
    planet.scan_level >= 3
}

fn planet_richness_multiplier(planet: &Planet) -> f32 {
    if planet_has_richness_scan(planet) {
        let richest = planet
            .info
            .mineables
            .iter()
            .enumerate()
            .map(|(index, _)| mineable_richness_multiplier(planet, index))
            .fold(1.0_f32, f32::max);
        richest
    } else {
        1.0
    }
}

fn mineable_richness_multiplier(planet: &Planet, mineable_index: usize) -> f32 {
    if !planet_has_richness_scan(planet) {
        return 1.0;
    }
    let Some(mineable) = planet.info.mineables.get(mineable_index) else {
        return 1.0;
    };
    let key = format!("{}:{}", planet.id, mineable.item.id);
    0.85 + stable_unit_noise(&key, 0xA117_5EED) * 0.75
}

fn mineable_bonus_yield_chance(planet: &Planet, mineable_index: usize) -> f32 {
    if !planet_has_richness_scan(planet) {
        return 0.0;
    }
    (mineable_richness_multiplier(planet, mineable_index) - 1.0).max(0.0) * 0.35
}

fn planet_hazard_mining_slowdown(planet: &Planet) -> f32 {
    planet.info.hazard_effects.mining_speed_multiplier.max(1.0)
}

fn planet_hazard_shield_drain_per_second(planet: &Planet) -> f32 {
    planet.info.hazard_effects.shield_drain_per_second.max(0.0)
}

fn stable_unit_noise(id: &str, salt: u64) -> f32 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64 ^ salt;
    for byte in id.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x1000_0000_01b3);
    }
    (hash as f64 / u64::MAX as f64) as f32
}

impl Inventory {
    fn starter(content_registry: &content::ContentRegistry) -> Self {
        let mut inventory = Self {
            slots: std::array::from_fn(|_| None),
        };
        for stack in &content_registry.starter_inventory {
            inventory.add_item(required_item(content_registry, &stack.item), stack.count);
        }
        inventory
    }

    fn add_item(&mut self, item: ItemRef, count: u32) {
        for slot in &mut self.slots {
            if count == 0 {
                return;
            }

            if let Some(stack) = slot {
                if stack.item.id == item.id {
                    stack.count += count;
                    return;
                }
            }
        }

        for slot in &mut self.slots {
            if count == 0 {
                return;
            }

            if slot.is_none() {
                *slot = Some(ItemStack { item, count });
                return;
            }
        }
    }

    fn count(&self, item: &ItemRef) -> u32 {
        self.slots
            .iter()
            .filter_map(|slot| slot.as_ref())
            .filter(|stack| stack.item.id == item.id)
            .map(|stack| stack.count)
            .sum()
    }

    fn total_mass(&self) -> f32 {
        self.slots
            .iter()
            .filter_map(|slot| slot.as_ref())
            .map(|stack| stack.item.unit_mass * stack.count as f32)
            .sum()
    }

    fn can_craft(&self, recipe: &Recipe) -> bool {
        recipe
            .ingredients
            .iter()
            .all(|ingredient| self.count(&ingredient.item) >= ingredient.count)
    }

    fn craft(&mut self, recipe: &Recipe) -> bool {
        if !self.can_craft(recipe) {
            return false;
        }

        for ingredient in &recipe.ingredients {
            self.remove_item(&ingredient.item, ingredient.count);
        }
        self.add_item(recipe.output.item.clone(), recipe.output.count);
        true
    }

    fn remove_item(&mut self, item: &ItemRef, mut count: u32) {
        for slot in &mut self.slots {
            if count == 0 {
                return;
            }

            if let Some(stack) = slot {
                if stack.item.id == item.id {
                    let removed = stack.count.min(count);
                    stack.count -= removed;
                    count -= removed;

                    if stack.count == 0 {
                        *slot = None;
                    }
                }
            }
        }
    }

    fn from_save(content_registry: &content::ContentRegistry, saved_stacks: &[SaveStack]) -> Self {
        let mut inventory = Self {
            slots: std::array::from_fn(|_| None),
        };
        for stack in saved_stacks {
            let Some(item) = registry_item(content_registry, &stack.item) else {
                eprintln!(
                    "Skipping saved inventory item `{}` because it is not loaded",
                    stack.item
                );
                continue;
            };
            inventory.add_item(item, stack.count);
        }
        inventory
    }

    fn to_save(&self) -> Vec<SaveStack> {
        self.slots
            .iter()
            .filter_map(|slot| slot.as_ref())
            .map(|stack| SaveStack {
                item: stack.item.id.clone(),
                count: stack.count,
            })
            .collect()
    }
}

fn apply_skill_save(skills: &mut [Skill; SKILL_COUNT], saved_skills: &[SaveSkill]) {
    for saved_skill in saved_skills {
        let Some(kind) = SkillKind::from_id(&saved_skill.kind) else {
            continue;
        };
        if let Some(skill) = skills.iter_mut().find(|skill| skill.kind == kind) {
            skill.level = saved_skill.level.min(20);
            skill.xp = finite_nonnegative_or(saved_skill.xp, 0.0);
        }
    }
}

fn apply_upgrade_save(
    upgrades: &mut [ShipUpgrade; SHIP_UPGRADE_COUNT],
    saved_upgrades: &[SaveUpgrade],
) {
    for saved_upgrade in saved_upgrades {
        let Some(kind) = ShipUpgradeKind::from_id(&saved_upgrade.kind) else {
            continue;
        };
        if let Some(upgrade) = upgrades.iter_mut().find(|upgrade| upgrade.kind == kind) {
            upgrade.level = saved_upgrade.level;
        }
    }
}

fn save_work_settings<T>(
    rows: &[T],
    settings: &[impl WorkSettingSnapshot],
    id_for: impl Fn(&T) -> &str,
) -> Vec<SaveWorkSetting> {
    rows.iter()
        .zip(settings.iter())
        .map(|(row, setting)| SaveWorkSetting {
            id: id_for(row).to_string(),
            keep: setting.keep(),
            queued: setting.queued(),
            progress: finite_or(setting.progress(), 0.0).clamp(0.0, 0.99),
        })
        .collect()
}

fn apply_work_settings<T, S>(
    settings: &mut [S],
    rows: &[T],
    saved_settings: &[SaveWorkSetting],
    id_for: impl Fn(&T) -> &str,
    legacy_id_for: impl Fn(&T) -> &str,
) where
    S: WorkSettingSnapshot + WorkSettingApply,
{
    for saved_setting in saved_settings {
        let Some(index) = rows.iter().position(|row| {
            id_for(row) == saved_setting.id.as_str()
                || legacy_id_for(row) == saved_setting.id.as_str()
        }) else {
            continue;
        };
        if let Some(setting) = settings.get_mut(index) {
            setting.apply_save(saved_setting);
        }
    }
}

fn apply_planet_save(planets: &mut [Planet], saved_planets: &[SavePlanet]) {
    for saved_planet in saved_planets {
        let Some(planet) = planets
            .iter_mut()
            .find(|planet| planet.id == saved_planet.id)
        else {
            continue;
        };
        planet.scan_level = scan_level_from_save(saved_planet);
        apply_work_settings(
            &mut planet.mining,
            &planet.info.mineables,
            &saved_planet.mining,
            |mineable| mineable.item.id.as_str(),
            |mineable| mineable.item.id.as_str(),
        );
    }
}

trait WorkSettingSnapshot {
    fn keep(&self) -> u32;
    fn queued(&self) -> u32;
    fn progress(&self) -> f32;
}

trait WorkSettingApply {
    fn apply_save(&mut self, saved: &SaveWorkSetting);
}

impl WorkSettingSnapshot for CraftSetting {
    fn keep(&self) -> u32 {
        self.keep
    }

    fn queued(&self) -> u32 {
        self.queued
    }

    fn progress(&self) -> f32 {
        self.progress
    }
}

impl WorkSettingApply for CraftSetting {
    fn apply_save(&mut self, saved: &SaveWorkSetting) {
        self.keep = saved.keep.min(999);
        self.queued = saved.queued.min(999);
        self.progress = finite_or(saved.progress, 0.0).clamp(0.0, 0.99);
    }
}

impl WorkSettingSnapshot for MiningSetting {
    fn keep(&self) -> u32 {
        self.keep
    }

    fn queued(&self) -> u32 {
        self.queued
    }

    fn progress(&self) -> f32 {
        self.progress
    }
}

impl WorkSettingApply for MiningSetting {
    fn apply_save(&mut self, saved: &SaveWorkSetting) {
        self.keep = saved.keep.min(999);
        self.queued = saved.queued.min(999);
        self.progress = finite_or(saved.progress, 0.0).clamp(0.0, 0.99);
    }
}

fn make_smelting_recipes(content_registry: &content::ContentRegistry) -> Vec<Recipe> {
    let recipes = make_recipes_for_station(content_registry, "core:smelting");
    if recipes.is_empty() {
        panic!("No smelting recipes loaded from content registry");
    }
    recipes
}

fn make_crafting_recipes(content_registry: &content::ContentRegistry) -> Vec<Recipe> {
    let recipes = make_recipes_for_station(content_registry, "core:crafting");
    if recipes.is_empty() {
        panic!("No crafting recipes loaded from content registry");
    }
    recipes
}

fn make_processing_recipes(content_registry: &content::ContentRegistry) -> Vec<Recipe> {
    let recipes = make_recipes_for_station(content_registry, "core:processing");
    if recipes.is_empty() {
        panic!("No processing recipes loaded from content registry");
    }
    recipes
}

fn make_recipes_for_station(
    content_registry: &content::ContentRegistry,
    station_id: &str,
) -> Vec<Recipe> {
    let recipe_ids = content_registry
        .recipe_order
        .iter()
        .filter_map(|recipe_id| {
            let recipe = content_registry.recipes.get(recipe_id)?;
            (recipe.station == station_id).then_some(recipe_id.clone())
        })
        .collect::<Vec<_>>();

    recipe_ids
        .into_iter()
        .filter_map(|recipe_id| {
            let recipe_def = content_registry.recipes.get(&recipe_id)?;
            let base_seconds = content_registry
                .stations
                .get(&recipe_def.station)
                .and_then(|station| station.base_seconds)
                .unwrap_or_else(|| default_station_seconds(station_id));
            recipe_from_content(content_registry, recipe_def, base_seconds)
        })
        .collect()
}

fn default_station_seconds(station_id: &str) -> f32 {
    match station_id {
        "core:smelting" => BASE_SMELTING_SECONDS,
        "core:processing" => BASE_PROCESSING_SECONDS,
        _ => BASE_CRAFTING_SECONDS,
    }
}

fn recipe_from_content(
    content_registry: &content::ContentRegistry,
    recipe_def: &content::RecipeDef,
    base_seconds: f32,
) -> Option<Recipe> {
    let Some(output) = item_stack_from_content(content_registry, &recipe_def.output) else {
        eprintln!(
            "Recipe `{}` output `{}` is missing from the content item registry",
            recipe_def.id, recipe_def.output.item
        );
        return None;
    };

    let mut ingredients = Vec::new();
    for ingredient in &recipe_def.ingredients {
        let Some(stack) = item_stack_from_content(content_registry, ingredient) else {
            eprintln!(
                "Recipe `{}` ingredient `{}` is missing from the content item registry",
                recipe_def.id, ingredient.item
            );
            return None;
        };
        ingredients.push(stack);
    }

    Some(Recipe {
        id: recipe_def.id.clone(),
        output,
        ingredients,
        base_seconds,
    })
}

fn item_stack_from_content(
    content_registry: &content::ContentRegistry,
    stack: &content::StackDef,
) -> Option<ItemStack> {
    Some(ItemStack {
        item: registry_item(content_registry, &stack.item)?,
        count: stack.count,
    })
}

fn seeded_planet_position(base: Vec2, world_seed: u64, planet_id: &str) -> Vec2 {
    let hash = hash_seeded_id(world_seed, planet_id);
    let rotation = seeded_unit(hash) * PLANET_SEED_ROTATION * 2.0 - PLANET_SEED_ROTATION;
    let rotated = vec2(
        base.x * rotation.cos() - base.y * rotation.sin(),
        base.x * rotation.sin() + base.y * rotation.cos(),
    );
    let jitter_angle = seeded_unit(hash.rotate_left(17)) * std::f32::consts::TAU;
    let jitter_radius = seeded_unit(hash.rotate_left(31)) * PLANET_SEED_JITTER;

    rotated + vec2(jitter_angle.cos(), jitter_angle.sin()) * jitter_radius
}

fn planet_motion_from_def(
    content_registry: &content::ContentRegistry,
    planet_def: &content::PlanetDef,
    world_seed: u64,
) -> PlanetMotion {
    planet_def
        .orbit
        .as_ref()
        .map(|orbit| {
            let seeded_phase = seeded_unit(hash_seeded_id(world_seed, &planet_def.id));
            PlanetMotion::Orbit(OrbitMotion {
                center: orbit_anchor_center(content_registry, planet_def, orbit)
                    .unwrap_or_else(|| vec2(planet_def.position[0], planet_def.position[1])),
                anchor_planet: None,
                radius: orbit.radius,
                semi_minor: orbit_semi_minor(orbit.radius, orbit.eccentricity),
                axis_rotation: orbit.axis_phase * std::f32::consts::TAU,
                period_days: orbit.period_days,
                phase: (orbit.phase + seeded_phase).fract(),
            })
        })
        .unwrap_or(PlanetMotion::Static)
}

fn orbit_semi_minor(radius: f32, eccentricity: f32) -> f32 {
    radius * (1.0 - eccentricity * eccentricity).max(0.0).sqrt()
}

fn resolve_planet_orbit_anchor_indices(
    planets: &mut [Planet],
    content_registry: &content::ContentRegistry,
) {
    let planet_ids = planets
        .iter()
        .map(|planet| planet.id.clone())
        .collect::<Vec<_>>();

    for index in 0..planets.len() {
        let Some(anchor) = content_registry
            .planets
            .get(&planets[index].id)
            .and_then(|planet| planet.orbit.as_ref())
            .and_then(|orbit| orbit.around.as_deref())
        else {
            continue;
        };
        if anchor == "primary_star" || content_registry.stars.contains_key(anchor) {
            continue;
        }
        let Some(anchor_index) = planet_ids
            .iter()
            .position(|id| id == anchor && planets[index].id != *id)
        else {
            continue;
        };
        if planets[anchor_index].system != planets[index].system {
            continue;
        }
        if let PlanetMotion::Orbit(orbit) = &mut planets[index].motion {
            orbit.anchor_planet = Some(anchor_index);
        }
    }
}

fn orbit_anchor_center(
    content_registry: &content::ContentRegistry,
    planet_def: &content::PlanetDef,
    orbit: &content::OrbitDef,
) -> Option<Vec2> {
    if let Some(anchor) = orbit.around.as_deref() {
        return orbit_anchor_position(content_registry, &planet_def.system, anchor);
    }

    if let Some(center) = orbit.center {
        return Some(vec2(center[0], center[1]));
    }

    orbit_anchor_position(content_registry, &planet_def.system, "primary_star")
}

fn orbit_anchor_position(
    content_registry: &content::ContentRegistry,
    system_id: &str,
    anchor: &str,
) -> Option<Vec2> {
    if anchor == "primary_star" {
        let primary_star = content_registry
            .systems
            .get(system_id)
            .and_then(|system| system.primary_star.as_deref())?;
        return orbit_anchor_position(content_registry, system_id, primary_star);
    }

    if let Some(star) = content_registry
        .stars
        .get(anchor)
        .filter(|star| star.system == system_id)
    {
        return Some(vec2(star.position[0], star.position[1]));
    }

    content_registry
        .planets
        .get(anchor)
        .filter(|planet| planet.system == system_id)
        .map(|planet| vec2(planet.position[0], planet.position[1]))
}

fn runtime_planet_position(planet: &Planet, elapsed_days: f32) -> Vec2 {
    runtime_position_from_motion(planet.base_position, planet.motion, elapsed_days)
}

fn runtime_position_from_motion(
    static_position: Vec2,
    motion: PlanetMotion,
    elapsed_days: f32,
) -> Vec2 {
    match motion {
        PlanetMotion::Static => static_position,
        PlanetMotion::Orbit(orbit) => orbit_position(orbit, elapsed_days),
    }
}

fn update_planet_runtime_positions(planets: &mut [Planet], elapsed_days: f32) {
    let mut positions = vec![None; planets.len()];
    let mut visiting = vec![false; planets.len()];
    for index in 0..planets.len() {
        let position =
            runtime_planet_position_at(index, planets, elapsed_days, &mut positions, &mut visiting);
        positions[index] = Some(position);
    }
    for (planet, position) in planets.iter_mut().zip(positions) {
        if let Some(position) = position {
            planet.position = position;
        }
    }
}

fn runtime_planet_position_at(
    index: usize,
    planets: &[Planet],
    elapsed_days: f32,
    positions: &mut [Option<Vec2>],
    visiting: &mut [bool],
) -> Vec2 {
    if let Some(position) = positions[index] {
        return position;
    }
    if visiting[index] {
        return runtime_planet_position(&planets[index], elapsed_days);
    }

    visiting[index] = true;
    let position = match planets[index].motion {
        PlanetMotion::Static => planets[index].base_position,
        PlanetMotion::Orbit(orbit) => {
            let center = orbit
                .anchor_planet
                .and_then(|anchor_index| {
                    (anchor_index < planets.len() && anchor_index != index).then(|| {
                        runtime_planet_position_at(
                            anchor_index,
                            planets,
                            elapsed_days,
                            positions,
                            visiting,
                        )
                    })
                })
                .unwrap_or(orbit.center);
            orbit_position_around(orbit, center, elapsed_days)
        }
    };
    visiting[index] = false;
    positions[index] = Some(position);
    position
}

fn advance_world_time_and_planets(game: &mut GameState, dt: f32) {
    game.world_elapsed_days = (game.world_elapsed_days + dt / GAME_DAY_SECONDS).max(0.0);
    update_planet_runtime_positions(&mut game.planets, game.world_elapsed_days);
}

fn orbit_position(orbit: OrbitMotion, elapsed_days: f32) -> Vec2 {
    orbit_position_around(orbit, orbit.center, elapsed_days)
}

fn orbit_position_around(orbit: OrbitMotion, center: Vec2, elapsed_days: f32) -> Vec2 {
    let orbit_fraction = elapsed_days / orbit.period_days.max(content::MIN_ORBIT_PERIOD_DAYS);
    let angle = (orbit.phase + orbit_fraction) * std::f32::consts::TAU;
    center
        + rotate(
            vec2(angle.cos() * orbit.radius, angle.sin() * orbit.semi_minor),
            orbit.axis_rotation,
        )
}

fn new_world_seed() -> u64 {
    let time_bits = get_time().to_bits();
    let screen_bits = ((screen_width().round() as u64) << 32) ^ screen_height().round() as u64;
    hash_u64(time_bits ^ screen_bits ^ 0x5177_5eed_cafe_babe)
}

fn hash_seeded_id(seed: u64, id: &str) -> u64 {
    let mut hash = hash_u64(seed ^ 0xcbf2_9ce4_8422_2325);
    for byte in id.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x1000_0000_01b3);
    }
    hash_u64(hash)
}

fn hash_u64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn seeded_unit(value: u64) -> f32 {
    ((value >> 40) as f32) / ((1_u64 << 24) as f32)
}

fn window_conf() -> Conf {
    let (window_width, window_height) =
        read_saved_window_size().unwrap_or((DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT));

    Conf {
        window_title: "Some Frontier".to_string(),
        window_width,
        window_height,
        high_dpi: true,
        sample_count: 4,
        icon: Some(macroquad::miniquad::conf::Icon {
            small: branding_icon::ICON_SMALL,
            medium: branding_icon::ICON_MEDIUM,
            big: branding_icon::ICON_BIG,
        }),
        ..Default::default()
    }
}

fn current_window_size() -> (i32, i32) {
    (
        screen_width().round() as i32,
        screen_height().round() as i32,
    )
}

fn window_state_path() -> PathBuf {
    config_dir().join("window-size.txt")
}

fn save_state_path() -> PathBuf {
    config_dir().join("save.toml")
}

fn settings_path() -> PathBuf {
    config_dir().join("settings.toml")
}

fn save_slots_dir() -> PathBuf {
    config_dir().join("saves")
}

fn new_save_slot_path(seed: u64) -> PathBuf {
    let timestamp = current_unix_seconds();
    save_slots_dir().join(format!("frontier-{timestamp}-{seed}.toml"))
}

fn config_dir() -> PathBuf {
    if let Some(config_home) = env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(config_home).join("some-frontier");
    }

    if let Some(home) = env::var_os("HOME") {
        return PathBuf::from(home).join(".config/some-frontier");
    }

    PathBuf::from(".some-frontier")
}

fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() {
        value
    } else {
        fallback
    }
}

fn finite_nonnegative_or(value: f32, fallback: f32) -> f32 {
    finite_or(value, fallback).max(0.0)
}

fn read_saved_window_size() -> Option<(i32, i32)> {
    let saved = fs::read_to_string(window_state_path()).ok()?;
    let mut values = saved.split_whitespace();
    let width = values.next()?.parse::<i32>().ok()?;
    let height = values.next()?.parse::<i32>().ok()?;

    valid_window_size(width, height).then_some((width, height))
}

fn read_save_data_at(path: &Path) -> Option<SaveData> {
    let source = fs::read_to_string(path).ok()?;
    match toml::from_str::<SaveData>(&source) {
        Ok(save) if save.version == SAVE_VERSION => Some(save),
        Ok(save) => {
            eprintln!(
                "Ignoring save file {} with unsupported version {}",
                path.display(),
                save.version
            );
            None
        }
        Err(error) => {
            eprintln!("Failed to parse save file {}: {error}", path.display());
            None
        }
    }
}

fn read_app_settings() -> AppSettings {
    let path = settings_path();
    let Some(source) = fs::read_to_string(&path).ok() else {
        return AppSettings::default();
    };

    match toml::from_str::<AppSettings>(&source) {
        Ok(settings) => settings.clamped(),
        Err(error) => {
            eprintln!("Failed to parse settings file {}: {error}", path.display());
            AppSettings::default()
        }
    }
}

fn save_app_settings(settings: &AppSettings) {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let Ok(serialized) = toml::to_string_pretty(&settings.clamped()) else {
        eprintln!("Failed to serialize app settings");
        return;
    };
    if let Err(error) = fs::write(&path, serialized) {
        eprintln!("Failed to write settings file {}: {error}", path.display());
    }
}

fn save_game_state(game: &GameState) {
    let path = game.save_path.clone();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let serialized = match toml::to_string_pretty(&game.to_save()) {
        Ok(serialized) => serialized,
        Err(error) => {
            eprintln!("Failed to serialize save data: {error}");
            return;
        }
    };
    if let Err(error) = fs::write(&path, serialized) {
        eprintln!("Failed to write save file {}: {error}", path.display());
    };
}

fn save_window_size((width, height): (i32, i32)) {
    if !valid_window_size(width, height) {
        return;
    }

    let path = window_state_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, format!("{width} {height}\n"));
}

fn valid_window_size(width: i32, height: i32) -> bool {
    (640..=7680).contains(&width) && (420..=4320).contains(&height)
}

fn draw_startup_transition(background: Option<&Texture2D>, label: &str, opacity: f32) {
    let opacity = opacity.clamp(0.0, 1.0);
    let screen_w = screen_width();
    let screen_h = screen_height();

    clear_background(Color::from_rgba(5, 8, 18, 255));

    if let Some(texture) = background {
        draw_fullscreen_texture_cover(texture, opacity);
    }

    draw_rectangle(
        0.0,
        0.0,
        screen_w,
        screen_h,
        Color::new(0.01, 0.02, 0.04, 0.34 + 0.28 * opacity),
    );
    draw_rectangle(
        0.0,
        screen_h * 0.72,
        screen_w,
        screen_h * 0.28,
        Color::new(0.01, 0.02, 0.04, 0.46 + 0.28 * opacity),
    );

    let title = "Some Frontier";
    let title_size = 34.0;
    let label_size = 20.0;
    let label = fit_debug_text(label, screen_w - 80.0, label_size as u16);
    let title_width = measure_text(title, None, title_size as u16, 1.0).width;
    let label_width = measure_text(&label, None, label_size as u16, 1.0).width;
    let text_x = (screen_w - title_width).max(0.0) * 0.5;
    let label_x = (screen_w - label_width).max(0.0) * 0.5;
    let base_y = screen_h * 0.8;

    draw_text(
        title,
        text_x,
        base_y,
        title_size,
        Color::new(0.92, 0.95, 0.89, opacity),
    );
    draw_text(
        &label,
        label_x,
        base_y + 34.0,
        label_size,
        Color::new(0.59, 0.87, 0.89, opacity),
    );

    let pulse = (get_time() as f32 * 4.0).sin() * 0.5 + 0.5;
    let bar_width = 220.0;
    let bar_x = (screen_w - bar_width) * 0.5;
    let bar_y = base_y + 58.0;
    draw_rectangle_lines(
        bar_x,
        bar_y,
        bar_width,
        4.0,
        1.0,
        Color::new(0.33, 0.47, 0.51, 0.55 * opacity),
    );
    draw_rectangle(
        bar_x,
        bar_y,
        bar_width * (0.32 + 0.68 * pulse),
        4.0,
        Color::new(0.59, 0.87, 0.89, 0.82 * opacity),
    );
}

async fn run_startup_transition_out(background: Option<&Texture2D>) {
    let mut elapsed = 0.0;
    while elapsed < STARTUP_FADE_SECONDS {
        let dt = get_frame_time().min(1.0 / 30.0);
        elapsed += dt;
        let opacity = 1.0 - (elapsed / STARTUP_FADE_SECONDS).clamp(0.0, 1.0);
        draw_startup_transition(background, "Launching ...", opacity);
        next_frame().await;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_ui_font(load_ui_font().await);
    let branding_logo = load_asset_texture(BRANDING_LOGO_PATH).await;
    let background = make_background();
    let mut app = if fast_start_enabled() {
        let start_mode = latest_save_path()
            .map(|path| GameStartMode::LoadGame { path })
            .unwrap_or_else(|| GameStartMode::NewGame {
                seed: new_world_seed(),
                pack_options: default_content_pack_option_selections(),
            });
        AppState::Playing(Box::new(GameState::new(start_mode).await))
    } else {
        AppState::Title(TitleMenu::default())
    };

    loop {
        let dt = get_frame_time().min(1.0 / 30.0);
        match &mut app {
            AppState::Title(menu) => {
                if let Some(action) = update_title_menu(menu) {
                    match action {
                        TitleAction::NewGame { seed, pack_options } => {
                            let mut game =
                                GameState::new(GameStartMode::NewGame { seed, pack_options }).await;
                            save_game_now(&mut game, SaveFeedback::Manual);
                            app = AppState::Playing(Box::new(game));
                        }
                        TitleAction::LoadGame { path } => {
                            app = AppState::Playing(Box::new(
                                GameState::new(GameStartMode::LoadGame { path }).await,
                            ));
                        }
                        TitleAction::QuitDesktop => {
                            macroquad::miniquad::window::quit();
                        }
                    }
                } else {
                    draw_title_menu(menu, &background, branding_logo.as_ref());
                }
            }
            AppState::Playing(game) => {
                update_game(game, dt);
                if game.quit_to_title_requested {
                    app = AppState::Title(TitleMenu::default());
                } else {
                    draw_scene(game, &background, branding_logo.as_ref());
                }
            }
        }
        next_frame().await;
    }
}

fn fast_start_enabled() -> bool {
    env::var("SOME_FRONTIER_FAST_START")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

impl Default for TitleMenu {
    fn default() -> Self {
        Self {
            view: TitleView::Main,
            new_game_seed_text: new_world_seed().to_string(),
            save_slots: title_save_slots(),
            selected_save_index: 0,
            last_save_click_index: None,
            last_save_click_time: 0.0,
            content_packs: title_content_packs(),
            selected_pack_index: 0,
            settings: read_app_settings(),
            selected_settings_category: SettingsCategory::Display,
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ui_scale: default_ui_scale(),
            master_volume: default_master_volume(),
            controls_profile: default_controls_profile(),
            gameplay_autosave_minutes: default_gameplay_autosave_minutes(),
        }
    }
}

impl AppSettings {
    fn clamped(&self) -> Self {
        Self {
            ui_scale: self.ui_scale.clamp(0.85, 1.25),
            master_volume: self.master_volume.clamp(0.0, 1.0),
            controls_profile: if self.controls_profile == "precision" {
                "precision".to_string()
            } else {
                "standard".to_string()
            },
            gameplay_autosave_minutes: self.gameplay_autosave_minutes.clamp(1, 10),
        }
    }

    fn adjust_selected(&mut self, category: SettingsCategory, direction: i32) {
        match category {
            SettingsCategory::Display => {
                self.ui_scale = (self.ui_scale + direction as f32 * 0.05).clamp(0.85, 1.25);
            }
            SettingsCategory::Audio => {
                self.master_volume = (self.master_volume + direction as f32 * 0.05).clamp(0.0, 1.0);
            }
            SettingsCategory::Controls => {
                self.controls_profile = if self.controls_profile == "standard" {
                    "precision".to_string()
                } else {
                    "standard".to_string()
                };
            }
            SettingsCategory::Gameplay => {
                if direction > 0 {
                    self.gameplay_autosave_minutes = (self.gameplay_autosave_minutes + 1).min(10);
                } else {
                    self.gameplay_autosave_minutes =
                        self.gameplay_autosave_minutes.saturating_sub(1).max(1);
                }
            }
        }
    }
}

impl SettingsCategory {
    const ALL: [Self; 4] = [Self::Display, Self::Audio, Self::Controls, Self::Gameplay];

    fn label(self) -> &'static str {
        match self {
            Self::Display => "Display",
            Self::Audio => "Audio",
            Self::Controls => "Controls",
            Self::Gameplay => "Gameplay",
        }
    }
}

fn default_ui_scale() -> f32 {
    1.0
}

fn default_master_volume() -> f32 {
    1.0
}

fn default_controls_profile() -> String {
    "standard".to_string()
}

fn default_gameplay_autosave_minutes() -> u32 {
    1
}

fn latest_save_path() -> Option<PathBuf> {
    title_save_slots().into_iter().next().map(|slot| slot.path)
}

fn title_save_slots() -> Vec<TitleSaveSlot> {
    let mut slots = Vec::new();
    let legacy_path = save_state_path();
    if let Some(slot) = title_save_slot_from_path(legacy_path, true) {
        slots.push(slot);
    }

    if let Ok(entries) = fs::read_dir(save_slots_dir()) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path
                .extension()
                .is_some_and(|extension| extension == "toml")
            {
                if let Some(slot) = title_save_slot_from_path(path, false) {
                    slots.push(slot);
                }
            }
        }
    }

    slots.sort_by_key(|slot| std::cmp::Reverse(slot.modified_unix_seconds));
    slots
}

fn title_save_slot_from_path(path: PathBuf, is_legacy: bool) -> Option<TitleSaveSlot> {
    let save = read_save_data_at(&path)?;
    let modified_unix_seconds = fs::metadata(&path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let label = if is_legacy {
        "Legacy Save".to_string()
    } else {
        path.file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("Saved Game")
            .to_string()
    };

    Some(TitleSaveSlot {
        path,
        label,
        world_seed: save.world_seed,
        current_system_id: save.current_system_id,
        world_elapsed_days: save.world_elapsed_days,
        modified_unix_seconds,
        is_legacy,
    })
}

fn title_content_packs() -> Vec<TitleContentPack> {
    load_game_content_registry()
        .packs
        .into_iter()
        .map(|pack| TitleContentPack {
            id: pack.id,
            name: pack.name,
            version: pack.version,
            description: pack.description,
            options: pack
                .options
                .into_iter()
                .map(|option| {
                    let default_value = option.default.as_save_string();
                    TitlePackOption {
                        id: option.id,
                        label: option.label,
                        description: option.description,
                        value_type: option.value_type,
                        default_value: default_value.clone(),
                        current_value: default_value,
                        choices: option.choices,
                    }
                })
                .collect(),
        })
        .collect()
}

fn default_content_pack_option_selections() -> Vec<PackOptionSelection> {
    let registry = load_game_content_registry();
    default_pack_option_selections_from_registry(&registry)
}

fn default_pack_option_selections_from_registry(
    registry: &content::ContentRegistry,
) -> Vec<PackOptionSelection> {
    registry
        .packs
        .iter()
        .flat_map(|pack| {
            pack.options.iter().map(|option| PackOptionSelection {
                pack_id: pack.id.clone(),
                option_id: option.id.clone(),
                value: option.default.as_save_string(),
            })
        })
        .collect()
}

fn selected_title_pack_options(packs: &[TitleContentPack]) -> Vec<PackOptionSelection> {
    packs
        .iter()
        .flat_map(|pack| {
            pack.options.iter().map(|option| PackOptionSelection {
                pack_id: pack.id.clone(),
                option_id: option.id.clone(),
                value: option.current_value.clone(),
            })
        })
        .collect()
}

fn validated_pack_option_selections(
    registry: &content::ContentRegistry,
    selections: Vec<PackOptionSelection>,
) -> Vec<PackOptionSelection> {
    let selected_by_option = selections
        .into_iter()
        .map(|selection| (selection.option_id.clone(), selection))
        .collect::<HashMap<_, _>>();

    registry
        .packs
        .iter()
        .flat_map(|pack| {
            pack.options.iter().map(|option| {
                selected_by_option
                    .get(&option.id)
                    .filter(|selection| {
                        selection.pack_id == pack.id
                            && pack_option_value_is_valid(option, &selection.value)
                    })
                    .cloned()
                    .unwrap_or_else(|| PackOptionSelection {
                        pack_id: pack.id.clone(),
                        option_id: option.id.clone(),
                        value: option.default.as_save_string(),
                    })
            })
        })
        .collect()
}

fn pack_option_value_is_valid(option: &content::PackOptionDef, value: &str) -> bool {
    match option.value_type {
        content::PackOptionValueType::Bool => value == "true" || value == "false",
        content::PackOptionValueType::Integer => value.parse::<i64>().is_ok(),
        content::PackOptionValueType::Number => value.parse::<f32>().is_ok(),
        content::PackOptionValueType::Text => true,
        content::PackOptionValueType::Choice => option.choices.iter().any(|choice| choice == value),
    }
}

fn load_game_content_registry_with_options(
    _pack_options: &[PackOptionSelection],
) -> content::ContentRegistry {
    load_game_content_registry()
}

fn update_title_menu(menu: &mut TitleMenu) -> Option<TitleAction> {
    if is_key_pressed(KeyCode::Escape) {
        if menu.view == TitleView::Main {
            return Some(TitleAction::QuitDesktop);
        }
        menu.view = TitleView::Main;
        return None;
    }

    match menu.view {
        TitleView::Main => update_title_main_menu(menu),
        TitleView::NewGame => {
            let action = update_title_new_game(menu);
            if action.is_some() {
                return action;
            }
            if is_mouse_button_pressed(MouseButton::Left)
                && title_back_button_rect().contains(mouse_vec2())
            {
                menu.view = TitleView::Main;
            }
            None
        }
        TitleView::LoadGame => {
            let action = update_title_load_game(menu);
            if action.is_some() {
                return action;
            }
            if is_key_pressed(KeyCode::Backspace)
                || (is_mouse_button_pressed(MouseButton::Left)
                    && title_load_back_button_rect().contains(mouse_vec2()))
            {
                menu.view = TitleView::Main;
            }
            None
        }
        TitleView::ContentPacks => {
            update_title_content_packs(menu);
            if is_key_pressed(KeyCode::Backspace)
                || (is_mouse_button_pressed(MouseButton::Left)
                    && title_back_button_rect().contains(mouse_vec2()))
            {
                menu.view = TitleView::Main;
            }
            None
        }
        TitleView::Settings => {
            update_title_settings(menu);
            if is_key_pressed(KeyCode::Backspace)
                || (is_mouse_button_pressed(MouseButton::Left)
                    && title_back_button_rect().contains(mouse_vec2()))
            {
                menu.view = TitleView::Main;
            }
            None
        }
    }
}

fn update_title_settings(menu: &mut TitleMenu) {
    let mut changed = false;
    if is_key_pressed(KeyCode::Up) {
        menu.selected_settings_category =
            previous_settings_category(menu.selected_settings_category);
    }
    if is_key_pressed(KeyCode::Down) {
        menu.selected_settings_category = next_settings_category(menu.selected_settings_category);
    }
    if is_key_pressed(KeyCode::Left) {
        menu.settings
            .adjust_selected(menu.selected_settings_category, -1);
        changed = true;
    }
    if is_key_pressed(KeyCode::Right) {
        menu.settings
            .adjust_selected(menu.selected_settings_category, 1);
        changed = true;
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse = mouse_vec2();
        for (index, category) in SettingsCategory::ALL.iter().copied().enumerate() {
            if title_settings_category_row_rect(index).contains(mouse) {
                menu.selected_settings_category = category;
            }
        }
        if title_settings_decrement_button_rect().contains(mouse) {
            menu.settings
                .adjust_selected(menu.selected_settings_category, -1);
            changed = true;
        } else if title_settings_increment_button_rect().contains(mouse) {
            menu.settings
                .adjust_selected(menu.selected_settings_category, 1);
            changed = true;
        }
    }

    if changed {
        save_app_settings(&menu.settings);
    }
}

fn previous_settings_category(category: SettingsCategory) -> SettingsCategory {
    let index = SettingsCategory::ALL
        .iter()
        .position(|candidate| *candidate == category)
        .unwrap_or(0);
    SettingsCategory::ALL[index.saturating_sub(1)]
}

fn next_settings_category(category: SettingsCategory) -> SettingsCategory {
    let index = SettingsCategory::ALL
        .iter()
        .position(|candidate| *candidate == category)
        .unwrap_or(0);
    SettingsCategory::ALL[(index + 1).min(SettingsCategory::ALL.len() - 1)]
}

fn update_title_load_game(menu: &mut TitleMenu) -> Option<TitleAction> {
    if menu.save_slots.is_empty() {
        return None;
    }

    if is_key_pressed(KeyCode::Up) {
        menu.selected_save_index = menu.selected_save_index.saturating_sub(1);
    }
    if is_key_pressed(KeyCode::Down) {
        menu.selected_save_index =
            (menu.selected_save_index + 1).min(menu.save_slots.len().saturating_sub(1));
    }
    if is_key_pressed(KeyCode::Enter) {
        return menu
            .save_slots
            .get(menu.selected_save_index)
            .map(|slot| TitleAction::LoadGame {
                path: slot.path.clone(),
            });
    }

    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    let mouse = mouse_vec2();
    for index in 0..menu.save_slots.len() {
        if title_save_row_rect(index).contains(mouse) {
            let clicked_at = get_time();
            let is_double_click = title_save_row_double_clicked(
                menu.last_save_click_index,
                menu.last_save_click_time,
                index,
                clicked_at,
            );
            menu.selected_save_index = index;
            menu.last_save_click_index = Some(index);
            menu.last_save_click_time = clicked_at;
            return if is_double_click {
                menu.save_slots
                    .get(index)
                    .map(|slot| TitleAction::LoadGame {
                        path: slot.path.clone(),
                    })
            } else {
                None
            };
        }
    }

    if title_load_game_button_rect().contains(mouse) {
        return menu
            .save_slots
            .get(menu.selected_save_index)
            .map(|slot| TitleAction::LoadGame {
                path: slot.path.clone(),
            });
    }

    None
}

fn title_save_row_double_clicked(
    previous_index: Option<usize>,
    previous_time: f64,
    clicked_index: usize,
    clicked_time: f64,
) -> bool {
    const DOUBLE_CLICK_SECONDS: f64 = 0.42;

    previous_index == Some(clicked_index)
        && clicked_time >= previous_time
        && clicked_time - previous_time <= DOUBLE_CLICK_SECONDS
}

fn update_title_content_packs(menu: &mut TitleMenu) {
    if menu.content_packs.is_empty() {
        return;
    }

    if is_key_pressed(KeyCode::Up) {
        menu.selected_pack_index = menu.selected_pack_index.saturating_sub(1);
    }
    if is_key_pressed(KeyCode::Down) {
        menu.selected_pack_index =
            (menu.selected_pack_index + 1).min(menu.content_packs.len().saturating_sub(1));
    }
    if !is_mouse_button_pressed(MouseButton::Left) {
        return;
    }

    let mouse = mouse_vec2();
    for index in 0..menu.content_packs.len() {
        if title_pack_row_rect(index).contains(mouse) {
            menu.selected_pack_index = index;
            return;
        }
    }

    if let Some(pack) = menu.content_packs.get_mut(menu.selected_pack_index) {
        for option_index in 0..pack.options.len() {
            if title_pack_option_row_rect(option_index).contains(mouse) {
                cycle_title_pack_option(pack, option_index);
                return;
            }
        }
    }
}

fn cycle_title_pack_option(pack: &mut TitleContentPack, option_index: usize) {
    let Some(option) = pack.options.get_mut(option_index) else {
        return;
    };

    match option.value_type {
        content::PackOptionValueType::Bool => {
            option.current_value = if option.current_value == "true" {
                "false".to_string()
            } else {
                "true".to_string()
            };
        }
        content::PackOptionValueType::Choice => {
            if option.choices.is_empty() {
                return;
            }
            let current_index = option
                .choices
                .iter()
                .position(|choice| choice == &option.current_value)
                .unwrap_or(0);
            option.current_value =
                option.choices[(current_index + 1) % option.choices.len()].clone();
        }
        content::PackOptionValueType::Integer
        | content::PackOptionValueType::Number
        | content::PackOptionValueType::Text => {}
    }
}

fn title_pack_option_is_interactive(option: &TitlePackOption) -> bool {
    matches!(
        option.value_type,
        content::PackOptionValueType::Bool | content::PackOptionValueType::Choice
    )
}

fn update_title_new_game(menu: &mut TitleMenu) -> Option<TitleAction> {
    while let Some(character) = get_char_pressed() {
        if character.is_ascii_digit() && menu.new_game_seed_text.len() < 20 {
            menu.new_game_seed_text.push(character);
        }
    }

    if is_key_pressed(KeyCode::Backspace) {
        menu.new_game_seed_text.pop();
    }
    if is_key_pressed(KeyCode::R) {
        menu.new_game_seed_text = new_world_seed().to_string();
    }

    let parsed_seed = parse_title_seed(&menu.new_game_seed_text);
    if is_key_pressed(KeyCode::Enter) {
        if let Some(seed) = parsed_seed {
            return Some(TitleAction::NewGame {
                seed,
                pack_options: selected_title_pack_options(&menu.content_packs),
            });
        }
    }

    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    let mouse = mouse_vec2();
    if title_seed_randomize_button_rect().contains(mouse) {
        menu.new_game_seed_text = new_world_seed().to_string();
        None
    } else if title_new_game_start_button_rect().contains(mouse) {
        parsed_seed.map(|seed| TitleAction::NewGame {
            seed,
            pack_options: selected_title_pack_options(&menu.content_packs),
        })
    } else {
        None
    }
}

fn parse_title_seed(seed_text: &str) -> Option<u64> {
    let trimmed = seed_text.trim();
    if trimmed.is_empty() {
        return None;
    }

    trimmed.parse::<u64>().ok()
}

fn update_title_main_menu(menu: &mut TitleMenu) -> Option<TitleAction> {
    if is_key_pressed(KeyCode::N) {
        menu.view = TitleView::NewGame;
        return None;
    }
    if is_key_pressed(KeyCode::L) && !menu.save_slots.is_empty() {
        menu.view = TitleView::LoadGame;
        return None;
    }
    if is_key_pressed(KeyCode::C) {
        menu.view = TitleView::ContentPacks;
        return None;
    }
    if is_key_pressed(KeyCode::S) {
        menu.view = TitleView::Settings;
        return None;
    }
    if is_key_pressed(KeyCode::Q) {
        return Some(TitleAction::QuitDesktop);
    }

    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    let mouse = mouse_vec2();
    if title_menu_button_rect(0).contains(mouse) {
        menu.view = TitleView::NewGame;
        None
    } else if title_menu_button_rect(1).contains(mouse) && !menu.save_slots.is_empty() {
        menu.view = TitleView::LoadGame;
        None
    } else if title_menu_button_rect(2).contains(mouse) {
        menu.view = TitleView::ContentPacks;
        None
    } else if title_menu_button_rect(3).contains(mouse) {
        menu.view = TitleView::Settings;
        None
    } else if title_menu_button_rect(4).contains(mouse) {
        Some(TitleAction::QuitDesktop)
    } else {
        None
    }
}

fn mouse_vec2() -> Vec2 {
    vec2(mouse_position().0, mouse_position().1)
}

fn draw_title_menu(menu: &TitleMenu, background: &UniverseBackground, logo: Option<&Texture2D>) {
    draw_title_background(background);
    match menu.view {
        TitleView::Main => draw_title_main_menu(menu, logo),
        TitleView::NewGame => draw_title_new_game(menu),
        TitleView::LoadGame => draw_title_load_game(menu),
        TitleView::ContentPacks => draw_title_content_packs(menu),
        TitleView::Settings => draw_title_settings(menu),
    }
}

fn draw_title_background(background: &UniverseBackground) {
    clear_background(Color::from_rgba(5, 8, 18, 255));
    let center = vec2(screen_width() * 0.5, screen_height() * 0.5);
    for layer in &background.star_layers {
        for star in &layer.stars {
            let position = center + star.position * layer.depth * 0.48;
            if position.x < -20.0
                || position.x > screen_width() + 20.0
                || position.y < -20.0
                || position.y > screen_height() + 20.0
            {
                continue;
            }
            draw_circle(
                position.x,
                position.y,
                star.size,
                Color {
                    a: layer.color.a * star.brightness,
                    ..layer.color
                },
            );
        }
    }
}

fn title_panel_rect() -> Rect {
    let width = 470.0;
    let height = 430.0;
    Rect::new(
        (screen_width() - width) * 0.5,
        (screen_height() - height) * 0.5,
        width,
        height,
    )
}

fn title_load_panel_rect() -> Rect {
    title_load_panel_rect_for_screen(screen_width(), screen_height())
}

fn title_load_panel_rect_for_screen(screen_width: f32, screen_height: f32) -> Rect {
    let width = (screen_width - 96.0).clamp(720.0, 980.0);
    let height = (screen_height - 96.0).clamp(480.0, 620.0);
    Rect::new(
        (screen_width - width) * 0.5,
        (screen_height - height) * 0.5,
        width,
        height,
    )
}

fn title_main_panel_rect() -> Rect {
    let width = (screen_width() - 64.0).clamp(640.0, 920.0);
    let height = (screen_height() - 64.0).clamp(520.0, 680.0);
    Rect::new(
        (screen_width() - width) * 0.5,
        (screen_height() - height) * 0.5,
        width,
        height,
    )
}

fn title_menu_button_rect(index: usize) -> Rect {
    let panel = title_main_panel_rect();
    Rect::new(
        panel.x + panel.w * 0.5 - 178.0,
        panel.y + panel.h - 270.0 + index as f32 * 52.0,
        356.0,
        38.0,
    )
}

fn title_back_button_rect() -> Rect {
    let panel = title_panel_rect();
    Rect::new(panel.x + 28.0, panel.y + panel.h - 64.0, 126.0, 38.0)
}

fn title_load_back_button_rect() -> Rect {
    let panel = title_load_panel_rect();
    Rect::new(panel.x + 28.0, panel.y + panel.h - 64.0, 126.0, 38.0)
}

fn title_pack_list_rect() -> Rect {
    let panel = title_panel_rect();
    Rect::new(panel.x + 28.0, panel.y + 88.0, 176.0, panel.h - 170.0)
}

fn title_settings_category_list_rect() -> Rect {
    let panel = title_panel_rect();
    Rect::new(panel.x + 28.0, panel.y + 88.0, 166.0, panel.h - 170.0)
}

fn title_settings_category_row_rect(index: usize) -> Rect {
    let list = title_settings_category_list_rect();
    Rect::new(list.x, list.y + index as f32 * 44.0, list.w, 38.0)
}

fn title_settings_decrement_button_rect() -> Rect {
    let panel = title_panel_rect();
    Rect::new(panel.x + 230.0, panel.y + 180.0, 44.0, 38.0)
}

fn title_settings_increment_button_rect() -> Rect {
    let panel = title_panel_rect();
    Rect::new(panel.x + panel.w - 72.0, panel.y + 180.0, 44.0, 38.0)
}

fn title_save_list_rect() -> Rect {
    title_save_list_rect_for_panel(title_load_panel_rect())
}

fn title_save_list_rect_for_panel(panel: Rect) -> Rect {
    Rect::new(
        panel.x + 28.0,
        panel.y + 102.0,
        panel.w * 0.42,
        panel.h - 186.0,
    )
}

fn title_save_row_rect(index: usize) -> Rect {
    let list = title_save_list_rect();
    Rect::new(list.x, list.y + index as f32 * 62.0, list.w, 56.0)
}

fn title_load_game_button_rect() -> Rect {
    let panel = title_load_panel_rect();
    Rect::new(
        panel.x + panel.w - 176.0,
        panel.y + panel.h - 64.0,
        148.0,
        38.0,
    )
}

fn title_pack_row_rect(index: usize) -> Rect {
    let list = title_pack_list_rect();
    Rect::new(list.x, list.y + index as f32 * 42.0, list.w, 36.0)
}

fn title_pack_option_row_rect(index: usize) -> Rect {
    let panel = title_panel_rect();
    Rect::new(
        panel.x + 226.0,
        panel.y + 218.0 + index as f32 * 52.0,
        panel.w - 254.0,
        44.0,
    )
}

fn title_seed_input_rect() -> Rect {
    let panel = title_panel_rect();
    Rect::new(panel.x + 28.0, panel.y + 126.0, panel.w - 196.0, 40.0)
}

fn title_seed_randomize_button_rect() -> Rect {
    let panel = title_panel_rect();
    Rect::new(panel.x + panel.w - 154.0, panel.y + 126.0, 126.0, 40.0)
}

fn title_new_game_start_button_rect() -> Rect {
    let panel = title_panel_rect();
    Rect::new(
        panel.x + panel.w - 176.0,
        panel.y + panel.h - 64.0,
        148.0,
        38.0,
    )
}

fn draw_title_main_menu(menu: &TitleMenu, logo: Option<&Texture2D>) {
    let panel = title_main_panel_rect();
    draw_title_panel(panel);
    if let Some(logo) = logo {
        let logo_source = Rect::new(
            logo.width() * 0.20,
            logo.height() * 0.18,
            logo.width() * 0.61,
            logo.height() * 0.52,
        );
        draw_texture_source_contain(
            logo,
            logo_source,
            Rect::new(
                panel.x + 18.0,
                panel.y + 18.0,
                panel.w - 36.0,
                panel.h - 278.0,
            ),
            1.0,
        );
    } else {
        let title = "Some Frontier";
        let title_measure = measure_text(title, None, 34, 1.0);
        draw_text(
            title,
            panel.x + (panel.w - title_measure.width) * 0.5,
            panel.y + 56.0,
            34.0,
            Color::from_rgba(235, 242, 226, 255),
        );
    }
    let subtitle = "Local systems, rough ore, quiet engines";
    let subtitle_measure = measure_text(subtitle, None, 18, 1.0);
    draw_text(
        subtitle,
        panel.x + (panel.w - subtitle_measure.width) * 0.5,
        panel.y + panel.h - 282.0,
        18.0,
        Color::from_rgba(168, 204, 210, 255),
    );

    let load_enabled = !menu.save_slots.is_empty();
    draw_title_button(title_menu_button_rect(0), "New Game", true, "N");
    draw_title_button(title_menu_button_rect(1), "Load Game", load_enabled, "L");
    draw_title_button(title_menu_button_rect(2), "Content Packs", true, "C");
    draw_title_button(title_menu_button_rect(3), "Settings", true, "S");
    draw_title_button(title_menu_button_rect(4), "Quit", true, "Q");

    if fast_start_enabled() {
        draw_text(
            "Fast start enabled by SOME_FRONTIER_FAST_START",
            panel.x + 28.0,
            panel.y + panel.h - 22.0,
            14.0,
            Color::from_rgba(226, 190, 150, 255),
        );
    }
}

fn draw_title_new_game(menu: &TitleMenu) {
    let panel = title_panel_rect();
    draw_title_panel(panel);
    draw_text(
        "New Game",
        panel.x + 28.0,
        panel.y + 54.0,
        30.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_text(
        "Choose a world seed before launching a fresh run.",
        panel.x + 28.0,
        panel.y + 84.0,
        17.0,
        Color::from_rgba(168, 204, 210, 255),
    );

    draw_text(
        "World seed",
        panel.x + 28.0,
        panel.y + 116.0,
        16.0,
        Color::from_rgba(168, 204, 210, 255),
    );
    let input = title_seed_input_rect();
    let seed_valid = parse_title_seed(&menu.new_game_seed_text).is_some();
    draw_rectangle(
        input.x,
        input.y,
        input.w,
        input.h,
        Color::from_rgba(8, 18, 24, 245),
    );
    draw_rectangle_lines(
        input.x,
        input.y,
        input.w,
        input.h,
        1.0,
        if seed_valid {
            Color::from_rgba(150, 221, 226, 220)
        } else {
            Color::from_rgba(226, 190, 150, 220)
        },
    );
    draw_text(
        &fit_debug_text(&menu.new_game_seed_text, input.w - 26.0, 19),
        input.x + 12.0,
        input.y + 26.0,
        19.0,
        if seed_valid {
            Color::from_rgba(235, 242, 226, 255)
        } else {
            Color::from_rgba(226, 190, 150, 255)
        },
    );
    draw_title_button(title_seed_randomize_button_rect(), "Randomize", true, "R");

    let options_y = panel.y + 206.0;
    draw_text(
        "Initial options",
        panel.x + 28.0,
        options_y,
        16.0,
        Color::from_rgba(168, 204, 210, 255),
    );
    draw_title_option_row(
        panel.x + 28.0,
        options_y + 28.0,
        panel.w - 56.0,
        "Start",
        "Frontier cargo ship",
    );
    draw_title_option_row(
        panel.x + 28.0,
        options_y + 72.0,
        panel.w - 56.0,
        "Packs",
        &format!(
            "{} active, {} configured",
            menu.content_packs.len(),
            selected_title_pack_options(&menu.content_packs).len()
        ),
    );
    draw_title_option_row(
        panel.x + 28.0,
        options_y + 116.0,
        panel.w - 56.0,
        "Difficulty",
        "Standard",
    );

    draw_text(
        "Pack selections are saved with the new run.",
        panel.x + 28.0,
        panel.y + panel.h - 88.0,
        15.0,
        Color::from_rgba(178, 197, 203, 255),
    );
    draw_title_button(title_back_button_rect(), "Back", true, "Esc");
    draw_title_button(
        title_new_game_start_button_rect(),
        "Start Game",
        seed_valid,
        "Enter",
    );
}

fn draw_title_load_game(menu: &TitleMenu) {
    let panel = title_load_panel_rect();
    draw_title_panel(panel);
    draw_text(
        "Load Game",
        panel.x + 28.0,
        panel.y + 54.0,
        30.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_text(
        "Last played saves appear first.",
        panel.x + 28.0,
        panel.y + 82.0,
        16.0,
        Color::from_rgba(168, 204, 210, 255),
    );

    let list = title_save_list_rect();
    draw_rectangle_lines(
        list.x,
        list.y,
        list.w,
        list.h,
        1.0,
        Color::from_rgba(96, 137, 150, 205),
    );

    for (index, slot) in menu.save_slots.iter().enumerate() {
        let row = title_save_row_rect(index);
        if row.y + row.h > list.y + list.h {
            break;
        }
        let selected = index == menu.selected_save_index;
        let hovered = row.contains(mouse_vec2());
        if selected || hovered {
            draw_rectangle(
                row.x,
                row.y,
                row.w,
                row.h,
                if selected {
                    Color::from_rgba(24, 58, 66, 230)
                } else {
                    Color::from_rgba(10, 18, 24, 150)
                },
            );
        }
        draw_text(
            &fit_debug_text(&slot.label, row.w - 20.0, 19),
            row.x + 10.0,
            row.y + 23.0,
            19.0,
            if selected {
                Color::from_rgba(235, 242, 226, 255)
            } else {
                Color::from_rgba(205, 226, 230, 255)
            },
        );
        draw_text(
            &fit_debug_text(
                &format!(
                    "{}  /  {}",
                    slot.current_system_id,
                    format_last_played(slot.modified_unix_seconds)
                ),
                row.w - 20.0,
                14,
            ),
            row.x + 10.0,
            row.y + 43.0,
            14.0,
            Color::from_rgba(150, 221, 226, 255),
        );
    }

    let detail_x = list.x + list.w + 28.0;
    let detail_y = panel.y + 102.0;
    let detail_width = panel.x + panel.w - detail_x - 28.0;
    if let Some(slot) = menu.save_slots.get(menu.selected_save_index) {
        draw_text(
            &fit_debug_text(&slot.label, detail_width, 22),
            detail_x,
            detail_y,
            22.0,
            Color::from_rgba(235, 242, 226, 255),
        );
        draw_text(
            if slot.is_legacy {
                "Legacy save file"
            } else {
                "Save slot"
            },
            detail_x,
            detail_y + 28.0,
            16.0,
            Color::from_rgba(150, 221, 226, 255),
        );
        draw_title_save_detail_row(
            detail_x,
            detail_y + 68.0,
            detail_width,
            "Seed",
            &slot.world_seed.to_string(),
        );
        draw_title_save_detail_row(
            detail_x,
            detail_y + 102.0,
            detail_width,
            "System",
            &slot.current_system_id,
        );
        draw_title_save_detail_row(
            detail_x,
            detail_y + 136.0,
            detail_width,
            "Elapsed",
            &format!("{:.1} days", slot.world_elapsed_days),
        );
        draw_title_save_detail_row(
            detail_x,
            detail_y + 170.0,
            detail_width,
            "Played",
            &format_last_played(slot.modified_unix_seconds),
        );
    } else {
        draw_text(
            "No saved games found.",
            detail_x,
            detail_y,
            18.0,
            Color::from_rgba(226, 190, 150, 255),
        );
    }

    draw_title_button(title_load_back_button_rect(), "Back", true, "Esc");
    draw_title_button(
        title_load_game_button_rect(),
        "Load",
        !menu.save_slots.is_empty(),
        "Enter",
    );
}

fn draw_title_save_detail_row(x: f32, y: f32, width: f32, label: &str, value: &str) {
    draw_text(label, x, y, 15.0, Color::from_rgba(168, 204, 210, 255));
    draw_text(
        &fit_debug_text(value, width - 92.0, 17),
        x + 78.0,
        y,
        17.0,
        Color::from_rgba(235, 242, 226, 255),
    );
}

fn format_last_played(modified_unix_seconds: u64) -> String {
    let now = current_unix_seconds();
    if modified_unix_seconds == 0 || modified_unix_seconds > now {
        return "unknown".to_string();
    }

    let elapsed = now - modified_unix_seconds;
    if elapsed < 60 {
        "just now".to_string()
    } else if elapsed < 3_600 {
        format!("{} min ago", elapsed / 60)
    } else if elapsed < 86_400 {
        format!("{} hr ago", elapsed / 3_600)
    } else {
        format!("{} days ago", elapsed / 86_400)
    }
}

fn draw_title_settings(menu: &TitleMenu) {
    let panel = title_panel_rect();
    draw_title_panel(panel);
    draw_text(
        "Settings",
        panel.x + 28.0,
        panel.y + 54.0,
        30.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_text(
        "Saved separately from game saves.",
        panel.x + 28.0,
        panel.y + 82.0,
        16.0,
        Color::from_rgba(168, 204, 210, 255),
    );

    let list = title_settings_category_list_rect();
    draw_rectangle_lines(
        list.x,
        list.y,
        list.w,
        list.h,
        1.0,
        Color::from_rgba(96, 137, 150, 205),
    );
    for (index, category) in SettingsCategory::ALL.iter().copied().enumerate() {
        let row = title_settings_category_row_rect(index);
        let selected = category == menu.selected_settings_category;
        let hovered = row.contains(mouse_vec2());
        if selected || hovered {
            draw_rectangle(
                row.x,
                row.y,
                row.w,
                row.h,
                if selected {
                    Color::from_rgba(24, 58, 66, 230)
                } else {
                    Color::from_rgba(10, 18, 24, 150)
                },
            );
        }
        draw_text(
            category.label(),
            row.x + 8.0,
            row.y + 24.0,
            18.0,
            if selected {
                Color::from_rgba(235, 242, 226, 255)
            } else {
                Color::from_rgba(205, 226, 230, 255)
            },
        );
    }

    let detail_x = panel.x + 226.0;
    draw_text(
        menu.selected_settings_category.label(),
        detail_x,
        panel.y + 116.0,
        24.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    let (setting_label, setting_value, setting_note) =
        title_selected_setting_text(&menu.settings, menu.selected_settings_category);
    draw_text(
        setting_label,
        detail_x,
        panel.y + 158.0,
        16.0,
        Color::from_rgba(168, 204, 210, 255),
    );
    draw_title_button(title_settings_decrement_button_rect(), "-", true, "Left");
    let value_measure = measure_text(&setting_value, None, 22, 1.0);
    let value_x =
        title_settings_decrement_button_rect().x + title_settings_decrement_button_rect().w + 18.0;
    draw_text(
        &setting_value,
        value_x,
        panel.y + 205.0,
        22.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_title_button(title_settings_increment_button_rect(), "+", true, "Right");
    draw_line(
        value_x,
        panel.y + 214.0,
        value_x + value_measure.width,
        panel.y + 214.0,
        1.0,
        Color::from_rgba(150, 221, 226, 160),
    );
    draw_wrapped_text(
        setting_note,
        detail_x,
        panel.y + 250.0,
        panel.w - 254.0,
        16,
        Color::from_rgba(178, 197, 203, 255),
    );
    draw_text(
        "Use Up/Down to choose a category and Left/Right to change it.",
        detail_x,
        panel.y + panel.h - 84.0,
        15.0,
        Color::from_rgba(150, 221, 226, 255),
    );

    draw_title_button(title_back_button_rect(), "Back", true, "Esc");
}

fn title_selected_setting_text(
    settings: &AppSettings,
    category: SettingsCategory,
) -> (&'static str, String, &'static str) {
    match category {
        SettingsCategory::Display => (
            "UI scale",
            format!("{:.0}%", settings.ui_scale * 100.0),
            "Prepared for future UI scaling. The saved value is available to the interface layer.",
        ),
        SettingsCategory::Audio => (
            "Master volume",
            format!("{:.0}%", settings.master_volume * 100.0),
            "Prepared for future audio cues and music.",
        ),
        SettingsCategory::Controls => (
            "Control profile",
            settings.controls_profile.clone(),
            "Standard keeps current controls. Precision is reserved for future input tuning.",
        ),
        SettingsCategory::Gameplay => (
            "Autosave interval",
            format!("{} min", settings.gameplay_autosave_minutes),
            "Saved as a gameplay preference. Runtime autosave currently uses the fixed one-minute cadence.",
        ),
    }
}

fn draw_title_option_row(x: f32, y: f32, width: f32, label: &str, value: &str) {
    draw_rectangle(x, y - 22.0, width, 34.0, Color::from_rgba(10, 18, 24, 130));
    draw_text(
        label,
        x + 10.0,
        y,
        16.0,
        Color::from_rgba(168, 204, 210, 255),
    );
    let value_width = measure_text(value, None, 17, 1.0).width;
    draw_text(
        value,
        x + width - value_width - 10.0,
        y,
        17.0,
        Color::from_rgba(235, 242, 226, 255),
    );
}

fn draw_title_content_packs(menu: &TitleMenu) {
    let panel = title_panel_rect();
    draw_title_panel(panel);
    draw_text(
        "Content Packs",
        panel.x + 28.0,
        panel.y + 54.0,
        30.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_text(
        "Installed",
        panel.x + 28.0,
        panel.y + 82.0,
        16.0,
        Color::from_rgba(168, 204, 210, 255),
    );
    draw_text(
        "Configuration",
        panel.x + 226.0,
        panel.y + 82.0,
        16.0,
        Color::from_rgba(168, 204, 210, 255),
    );

    let list = title_pack_list_rect();
    draw_rectangle_lines(
        list.x,
        list.y,
        list.w,
        list.h,
        1.0,
        Color::from_rgba(96, 137, 150, 205),
    );

    for (index, pack) in menu.content_packs.iter().enumerate() {
        let row = title_pack_row_rect(index);
        if row.y + row.h > list.y + list.h {
            break;
        }
        let selected = index == menu.selected_pack_index;
        let hovered = row.contains(mouse_vec2());
        if selected || hovered {
            draw_rectangle(
                row.x,
                row.y,
                row.w,
                row.h,
                if selected {
                    Color::from_rgba(24, 58, 66, 230)
                } else {
                    Color::from_rgba(10, 18, 24, 150)
                },
            );
        }
        draw_text(
            &fit_debug_text(&pack.name, row.w - 14.0, 17),
            row.x + 8.0,
            row.y + 23.0,
            17.0,
            if selected {
                Color::from_rgba(235, 242, 226, 255)
            } else {
                Color::from_rgba(205, 226, 230, 255)
            },
        );
    }

    let detail_x = panel.x + 226.0;
    let detail_y = panel.y + 98.0;
    let detail_width = panel.w - 254.0;
    if let Some(pack) = menu.content_packs.get(menu.selected_pack_index) {
        draw_text(
            &fit_debug_text(&pack.name, detail_width, 22),
            detail_x,
            detail_y,
            22.0,
            Color::from_rgba(235, 242, 226, 255),
        );
        draw_text(
            &format!("{}  v{}", pack.id, pack.version),
            detail_x,
            detail_y + 30.0,
            16.0,
            Color::from_rgba(150, 221, 226, 255),
        );
        let after_description = draw_wrapped_text(
            pack.description
                .as_deref()
                .unwrap_or("No description provided."),
            detail_x,
            detail_y + 64.0,
            detail_width,
            17,
            Color::from_rgba(205, 226, 230, 255),
        );
        draw_text(
            "Options",
            detail_x,
            (after_description + 22.0).min(panel.y + 202.0),
            16.0,
            Color::from_rgba(168, 204, 210, 255),
        );
        if pack.options.is_empty() {
            draw_text(
                "No configurable options declared.",
                detail_x,
                panel.y + 236.0,
                16.0,
                Color::from_rgba(226, 190, 150, 255),
            );
        }
        for (option_index, option) in pack.options.iter().enumerate() {
            let row = title_pack_option_row_rect(option_index);
            if row.y + row.h > panel.y + panel.h - 78.0 {
                break;
            }
            let interactive = title_pack_option_is_interactive(option);
            let hovered = interactive && row.contains(mouse_vec2());
            draw_rectangle(
                row.x,
                row.y,
                row.w,
                row.h,
                if hovered {
                    Color::from_rgba(24, 58, 66, 230)
                } else {
                    Color::from_rgba(8, 18, 24, 210)
                },
            );
            draw_rectangle_lines(
                row.x,
                row.y,
                row.w,
                row.h,
                1.0,
                Color::from_rgba(96, 137, 150, 220),
            );
            draw_text(
                &fit_debug_text(&option.label, row.w - 112.0, 16),
                row.x + 10.0,
                row.y + 18.0,
                16.0,
                Color::from_rgba(235, 242, 226, 255),
            );
            draw_text(
                &fit_debug_text(
                    option
                        .description
                        .as_deref()
                        .unwrap_or("Saved with new games; application pending."),
                    row.w - 118.0,
                    13,
                ),
                row.x + 10.0,
                row.y + 36.0,
                13.0,
                Color::from_rgba(178, 197, 203, 255),
            );
            let value_label = if interactive {
                option.current_value.clone()
            } else {
                format!("{} pending", option.default_value)
            };
            draw_text(
                &fit_debug_text(&value_label, 94.0, 15),
                row.x + row.w - 102.0,
                row.y + 27.0,
                15.0,
                if interactive {
                    Color::from_rgba(150, 221, 226, 255)
                } else {
                    Color::from_rgba(226, 190, 150, 255)
                },
            );
        }
    } else {
        draw_text(
            "No content packs found.",
            detail_x,
            detail_y,
            18.0,
            Color::from_rgba(226, 190, 150, 255),
        );
    }

    draw_title_button(title_back_button_rect(), "Back", true, "Esc");
}

fn draw_title_panel(rect: Rect) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::from_rgba(3, 8, 13, 244),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::from_rgba(112, 151, 163, 220),
    );
}

fn draw_title_button(rect: Rect, label: &str, enabled: bool, shortcut: &str) {
    let hovered = enabled && rect.contains(mouse_vec2());
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if enabled {
            if hovered {
                Color::from_rgba(32, 74, 80, 245)
            } else {
                Color::from_rgba(13, 32, 40, 235)
            }
        } else {
            Color::from_rgba(12, 18, 22, 220)
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        if enabled {
            Color::from_rgba(150, 221, 226, 210)
        } else {
            Color::from_rgba(96, 112, 116, 170)
        },
    );
    draw_text(
        label,
        rect.x + 16.0,
        rect.y + 25.0,
        19.0,
        if enabled {
            Color::from_rgba(235, 242, 226, 255)
        } else {
            Color::from_rgba(126, 143, 148, 255)
        },
    );
    if hovered && !shortcut.is_empty() {
        draw_title_button_shortcut_tooltip(rect, shortcut);
    }
}

fn draw_title_button_shortcut_tooltip(rect: Rect, shortcut: &str) {
    let label = format!("Key {shortcut}");
    let measure = measure_text(&label, None, 14, 1.0);
    let width = measure.width + 20.0;
    let height = 28.0;
    let x = (rect.x + rect.w - width).clamp(12.0, screen_width() - width - 12.0);
    let y = (rect.y - height - 8.0).max(12.0);

    draw_rectangle(x, y, width, height, Color::from_rgba(2, 6, 10, 245));
    draw_rectangle_lines(
        x,
        y,
        width,
        height,
        1.0,
        Color::from_rgba(112, 151, 163, 170),
    );
    draw_text(
        &label,
        x + 10.0,
        y + 19.0,
        14.0,
        Color::from_rgba(150, 221, 226, 255),
    );
}

fn make_background() -> UniverseBackground {
    UniverseBackground {
        star_layers: vec![
            make_star_layer(
                260,
                0.08,
                0.25,
                0.6..1.5,
                Color::from_rgba(136, 168, 205, 160),
            ),
            make_star_layer(
                230,
                0.32,
                0.7,
                1.0..2.4,
                Color::from_rgba(186, 220, 232, 210),
            ),
            make_star_layer(
                95,
                0.72,
                1.35,
                1.8..3.6,
                Color::from_rgba(235, 242, 226, 230),
            ),
        ],
    }
}

fn make_star_layer(
    count: usize,
    depth: f32,
    trail_scale: f32,
    size_range: std::ops::Range<f32>,
    color: Color,
) -> StarLayer {
    let stars = (0..count)
        .map(|_| {
            let radius = rand::gen_range(80.0, STARFIELD_RADIUS);
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            Star {
                position: vec2(angle.cos(), angle.sin()) * radius,
                size: rand::gen_range(size_range.start, size_range.end),
                brightness: rand::gen_range(0.35, 1.0),
            }
        })
        .collect();

    StarLayer {
        stars,
        depth,
        trail_scale,
        color,
    }
}

fn update_game(game: &mut GameState, dt: f32) {
    update_window_size_memory(game, dt);
    update_save_state(game, dt);
    clamp_inventory_scrolls(game);
    if game.escape_dialog_open {
        handle_escape_dialog_input(game);
        return;
    }

    update_scene_transition(game, dt);
    update_pending_warp(game, dt);
    advance_world_time_and_planets(game, dt);
    if game.orbiting_planet.is_some() {
        if orbit_break_input_down() {
            break_planet_orbit(game);
        } else {
            update_ship_orbit(game);
        }
    }
    let save_snapshot = game_save_snapshot(game);

    if is_key_pressed(KeyCode::T) {
        if let Some(target_system_id) = debug_transition_target_system_id(game) {
            let label = format!(
                "Loading local space ... {}",
                system_display_name(&game.content_registry, &target_system_id)
            );
            start_scene_transition_with_action(
                game,
                &label,
                TransitionAction::SwitchSystem(target_system_id),
            );
        }
    }

    if is_key_pressed(KeyCode::M) {
        game.map_open = !game.map_open;
        if game.map_open {
            game.starmap_pan = Vec2::ZERO;
            game.starmap_drag_previous_mouse = None;
            game.inventory_open = false;
            game.skills_open = false;
            game.upgrades_open = false;
            game.content_open = false;
            game.selected_planet = None;
            game.selected_station = None;
            game.selected_npc_ship = None;
            game.selected_station_service = None;
        }
    }
    if game.map_open && is_key_pressed(KeyCode::F) {
        game.starmap_filter = game.starmap_filter.next();
        game.starmap_resource_filter_index = 0;
    }
    if game.map_open && is_key_pressed(KeyCode::R) {
        let resource_count = starmap_resource_filters(game).len();
        if resource_count > 0 {
            game.starmap_filter = StarmapFilter::Resource;
            game.starmap_resource_filter_index =
                (game.starmap_resource_filter_index + 1) % resource_count;
        }
    }
    if is_key_pressed(KeyCode::Tab) || is_key_pressed(KeyCode::E) {
        game.map_open = false;
        game.skills_open = false;
        game.upgrades_open = false;
        game.content_open = false;
        game.selected_planet = None;
        game.selected_station = None;
        game.selected_npc_ship = None;
        game.selected_station_service = None;
        game.inventory_open = !game.inventory_open;
    }
    if is_key_pressed(KeyCode::K) {
        game.skills_open = !game.skills_open;
        if game.skills_open {
            game.map_open = false;
            game.inventory_open = false;
            game.upgrades_open = false;
            game.content_open = false;
            game.selected_planet = None;
            game.selected_station = None;
            game.selected_npc_ship = None;
            game.selected_station_service = None;
        }
    }
    if is_key_pressed(KeyCode::C) {
        game.content_open = !game.content_open;
        if game.content_open {
            game.map_open = false;
            game.inventory_open = false;
            game.skills_open = false;
            game.upgrades_open = false;
            game.selected_planet = None;
            game.selected_station = None;
            game.selected_npc_ship = None;
            game.selected_station_service = None;
        }
    }
    if is_key_pressed(KeyCode::PageUp) {
        adjust_camera_zoom(game, 1);
    }
    if is_key_pressed(KeyCode::PageDown) {
        adjust_camera_zoom(game, -1);
    }
    if is_key_pressed(KeyCode::Escape) {
        handle_escape_pressed(game);
    }
    if is_key_pressed(KeyCode::Space)
        && !game.map_open
        && !game.skills_open
        && !game.upgrades_open
        && !game.content_open
    {
        select_nearby_destination(game);
        identify_selected_npc_ship(game);
        if game.selected_planet.is_some()
            || game.selected_station.is_some()
            || game.selected_npc_ship.is_some()
        {
            game.inventory_open = true;
        }
    }

    update_production(game, dt);
    update_mining(game, dt);
    update_orbital_hazards(game, dt);
    update_shield_recharge(game, dt);
    update_npc_ships(game, dt);
    update_weapon_systems(game, dt);

    let wheel = mouse_wheel().1;
    if game.map_open {
        update_starmap_view_input(game, dt, wheel);
    }
    if wheel != 0.0
        && !game.inventory_open
        && !game.map_open
        && !game.skills_open
        && !game.upgrades_open
        && !game.content_open
    {
        adjust_camera_zoom(game, wheel.signum() as i32);
    }

    let mut click_handled = false;
    if game.skills_open {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        click_handled = handle_skills_table_input(game, mouse);
    }

    if game.upgrades_open {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        if wheel != 0.0 {
            handle_ship_upgrades_scroll(game, mouse, wheel);
        }
        click_handled = handle_ship_upgrades_input(game, mouse);
    }

    if game.content_open {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        if wheel != 0.0 {
            handle_content_browser_scroll(game, mouse, wheel);
        }
        click_handled = handle_content_browser_input(game, mouse);
    }

    if game.inventory_open
        && !game.map_open
        && !game.skills_open
        && !game.upgrades_open
        && !game.content_open
    {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        if wheel != 0.0 {
            handle_inventory_overlay_scroll(game, mouse, wheel);
        }

        if handle_action_rail_resize_input(game, mouse) {
            click_handled = true;
        } else if let Some(planet_index) = game.selected_planet {
            click_handled = handle_planet_orbit_input(game, planet_index, mouse)
                || handle_planet_scan_input(game, planet_index, mouse)
                || handle_mining_table_input(game, planet_index, mouse, wheel)
                || handle_production_table_input(game, mouse, wheel);
        } else if let Some(station_index) = game.selected_station {
            click_handled = handle_station_service_input(game, station_index, mouse)
                || handle_production_table_input(game, mouse, wheel);
        } else if let Some(npc_ship_index) = game.selected_npc_ship {
            click_handled = handle_npc_ship_interaction_input(game, npc_ship_index, mouse)
                || handle_production_table_input(game, mouse, wheel);
        } else {
            if handle_ship_shield_slot_input(game, mouse)
                || handle_ship_weapon_slot_input(game, mouse)
            {
                click_handled = true;
            } else if is_mouse_button_pressed(MouseButton::Left)
                && ship_detail_preview_rect().contains(mouse)
            {
                game.upgrades_open = true;
                game.inventory_open = false;
                click_handled = true;
            } else {
                click_handled = handle_production_table_input(game, mouse, wheel);
            }
        }

        if !click_handled && action_rail_consumes_pointer_click(game, mouse) {
            click_handled = true;
        }
    }

    if game.map_open && is_mouse_button_pressed(MouseButton::Left) {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        if let Some(system_id) = clicked_known_system_id(mouse, game) {
            if system_id != game.current_system_id && game.scene_transition.is_none() {
                start_player_warp_charge(game, system_id);
            }
            click_handled = true;
        } else if let Some(planet_index) = clicked_starmap_planet_index(mouse, game) {
            set_destination_planet(game, Some(planet_index));
            game.selected_planet = None;
            game.selected_station = None;
            game.selected_npc_ship = None;
            game.selected_station_service = None;
            click_handled = true;
        }
    }

    if is_mouse_button_pressed(MouseButton::Left)
        && !click_handled
        && !game.map_open
        && !game.skills_open
        && !game.upgrades_open
        && !game.content_open
    {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        if clicked_player_ship(mouse) {
            game.selected_planet = None;
            game.selected_station = None;
            game.selected_npc_ship = None;
            game.selected_station_service = None;
            game.inventory_open = true;
        } else {
            select_clicked_destination(game, mouse);
            identify_selected_npc_ship(game);
            if game.selected_planet.is_some()
                || game.selected_station.is_some()
                || game.selected_npc_ship.is_some()
            {
                game.inventory_open = true;
            }
        }
    }

    if game.orbiting_planet.is_some() {
        update_ship_orbit(game);
    } else {
        let energy_recharge = ship_energy_recharge(&game.ship, &game.installed_power_modules);
        update_ship(&mut game.ship, dt, energy_recharge);
    }
    if game_save_snapshot(game) != save_snapshot {
        game.save_dirty = true;
    }
}

fn handle_escape_pressed(game: &mut GameState) {
    if close_topmost_gameplay_overlay(game) {
        return;
    }

    game.escape_dialog_open = true;
}

fn close_topmost_gameplay_overlay(game: &mut GameState) -> bool {
    if game.content_open {
        game.content_open = false;
        true
    } else if game.upgrades_open {
        game.upgrades_open = false;
        true
    } else if game.skills_open {
        game.skills_open = false;
        true
    } else if game.map_open {
        game.map_open = false;
        true
    } else if game.inventory_open {
        game.inventory_open = false;
        game.selected_planet = None;
        game.selected_station = None;
        game.selected_npc_ship = None;
        game.selected_station_service = None;
        true
    } else {
        false
    }
}

fn handle_escape_dialog_input(game: &mut GameState) {
    if is_key_pressed(KeyCode::Escape) {
        apply_escape_dialog_action(game, EscapeDialogAction::Resume);
        return;
    }

    if is_key_pressed(KeyCode::S) {
        apply_escape_dialog_action(game, EscapeDialogAction::SaveNow);
        return;
    }

    if is_key_pressed(KeyCode::T) {
        apply_escape_dialog_action(game, EscapeDialogAction::SaveToTitle);
        return;
    }

    if is_key_pressed(KeyCode::Q) {
        if apply_escape_dialog_action(game, EscapeDialogAction::QuitDesktop)
            == EscapeDialogResult::QuitDesktop
        {
            macroquad::miniquad::window::quit();
        }
        return;
    }

    if !is_mouse_button_pressed(MouseButton::Left) {
        return;
    }

    let mouse = vec2(mouse_position().0, mouse_position().1);
    if escape_dialog_resume_button_rect().contains(mouse) {
        apply_escape_dialog_action(game, EscapeDialogAction::Resume);
    } else if escape_dialog_save_button_rect().contains(mouse) {
        apply_escape_dialog_action(game, EscapeDialogAction::SaveNow);
    } else if escape_dialog_title_button_rect().contains(mouse) {
        apply_escape_dialog_action(game, EscapeDialogAction::SaveToTitle);
    } else if escape_dialog_quit_button_rect().contains(mouse)
        && apply_escape_dialog_action(game, EscapeDialogAction::QuitDesktop)
            == EscapeDialogResult::QuitDesktop
    {
        macroquad::miniquad::window::quit();
    }
}

fn apply_escape_dialog_action(
    game: &mut GameState,
    action: EscapeDialogAction,
) -> EscapeDialogResult {
    match action {
        EscapeDialogAction::Resume => {
            game.escape_dialog_open = false;
            EscapeDialogResult::Continue
        }
        EscapeDialogAction::SaveNow => {
            save_game_now(game, SaveFeedback::Manual);
            EscapeDialogResult::Continue
        }
        EscapeDialogAction::SaveToTitle => {
            save_game_now(game, SaveFeedback::Manual);
            game.escape_dialog_open = false;
            game.quit_to_title_requested = true;
            EscapeDialogResult::Continue
        }
        EscapeDialogAction::QuitDesktop => {
            save_game_now(game, SaveFeedback::Manual);
            EscapeDialogResult::QuitDesktop
        }
    }
}

fn start_scene_transition_with_action(
    game: &mut GameState,
    label: &str,
    pending_action: TransitionAction,
) {
    game.scene_transition = Some(SceneTransition {
        texture: select_transition_texture(&game.transition_assets),
        label: label.to_string(),
        timer: 0.0,
        fade_in_seconds: TRANSITION_FADE_IN_SECONDS,
        hold_seconds: TRANSITION_HOLD_SECONDS,
        fade_out_seconds: TRANSITION_FADE_OUT_SECONDS,
        pending_action,
        midpoint_applied: false,
    });
}

fn update_scene_transition(game: &mut GameState, dt: f32) {
    let mut pending_action = None;
    let finished = {
        let Some(transition) = &mut game.scene_transition else {
            return;
        };

        transition.timer += dt;
        if !transition.midpoint_applied && transition.timer >= transition.fade_in_seconds {
            transition.midpoint_applied = true;
            pending_action = Some(transition.pending_action.clone());
        }

        transition.timer >= transition.total_seconds()
    };

    if let Some(action) = pending_action {
        apply_transition_action(game, action);
    }
    if finished {
        game.scene_transition = None;
    }
}

fn apply_transition_action(game: &mut GameState, action: TransitionAction) {
    match action {
        TransitionAction::SwitchSystem(system_id) => {
            switch_current_system(game, &system_id);
        }
    }
}

fn start_player_warp_charge(game: &mut GameState, target_system_id: String) {
    if !system_is_known(&game.content_registry, &target_system_id) {
        return;
    }
    if game
        .pending_warp
        .as_ref()
        .is_some_and(|warp| warp.target_system_id == target_system_id)
    {
        return;
    }

    let cost = warp_cost(
        &game.content_registry,
        &game.current_system_id,
        &target_system_id,
    );
    if !can_afford_cost(&game.inventory, &cost) {
        return;
    }

    break_planet_orbit(game);
    game.pending_warp = Some(PendingWarp {
        target_system_id,
        timer: warp_charge_seconds(&game.ship_upgrades),
        cost,
    });
}

fn update_pending_warp(game: &mut GameState, dt: f32) {
    if game.scene_transition.is_some() {
        return;
    }

    let Some(warp) = &mut game.pending_warp else {
        return;
    };
    warp.timer = (warp.timer - dt).max(0.0);
    if warp.timer > 0.0 {
        return;
    }

    let Some(warp) = game.pending_warp.take() else {
        return;
    };
    if !can_afford_cost(&game.inventory, &warp.cost) {
        return;
    }

    pay_cost(&mut game.inventory, &warp.cost);
    game.save_dirty = true;
    let label = format!(
        "Loading local space ... {}",
        system_display_name(&game.content_registry, &warp.target_system_id)
    );
    start_scene_transition_with_action(
        game,
        &label,
        TransitionAction::SwitchSystem(warp.target_system_id),
    );
}

fn debug_transition_target_system_id(game: &GameState) -> Option<String> {
    transition_target_system_id(&game.content_registry, &game.current_system_id)
}

fn transition_target_system_id(
    registry: &content::ContentRegistry,
    current_system_id: &str,
) -> Option<String> {
    if current_system_id != STARTER_SYSTEM_ID && registry.systems.contains_key(STARTER_SYSTEM_ID) {
        return Some(STARTER_SYSTEM_ID.to_string());
    }

    registry
        .system_order
        .iter()
        .find(|system_id| {
            system_id.as_str() != current_system_id
                && registry
                    .systems
                    .get(system_id.as_str())
                    .is_some_and(|system| system.tags.iter().any(|tag| tag == "remote"))
        })
        .or_else(|| {
            registry
                .system_order
                .iter()
                .find(|system_id| system_id.as_str() != current_system_id)
        })
        .cloned()
}

struct ContentBrowserLayout {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    column_y: f32,
    column_width: f32,
    column_gap: f32,
    row_height: f32,
    viewport_height: f32,
}

struct ContentColumnRender<'a> {
    title: &'a str,
    x: f32,
    y: f32,
    width: f32,
    row_height: f32,
    viewport_height: f32,
    scroll: f32,
    rows: &'a [String],
    selected_row: Option<usize>,
}

struct StationDetailRender<'a> {
    content_registry: &'a content::ContentRegistry,
    station: &'a StationDestination,
    selected_service: Option<usize>,
    in_range: bool,
    distance: f32,
    x: f32,
    y: f32,
    width: f32,
}

struct RecipeTableInput<'a> {
    recipes: &'a [Recipe],
    settings: &'a mut [CraftSetting],
    locked_recipes: &'a [String],
    purchased_unlocks: &'a [String],
    mouse: Vec2,
    wheel: f32,
    scroll: f32,
    action_rail_width: Option<f32>,
}

struct PlanetActionRailRender<'a> {
    content_registry: &'a content::ContentRegistry,
    planet: &'a Planet,
    inventory: &'a Inventory,
    ship_upgrades: &'a [ShipUpgrade; SHIP_UPGRADE_COUNT],
    action_rail_width: f32,
    is_orbiting: bool,
    in_range: bool,
    scroll: f32,
    mouse: Vec2,
}

struct StationActionRailRender<'a> {
    station: &'a StationDestination,
    selected_service: Option<usize>,
    in_range: bool,
    credits: u32,
    inventory: &'a Inventory,
    purchased_unlocks: &'a [String],
    action_rail_width: f32,
}

struct StationTradeTableRender<'a> {
    station: &'a StationDestination,
    service: &'a StationService,
    in_range: bool,
    credits: u32,
    inventory: &'a Inventory,
    action_rail_width: f32,
    x: f32,
    width: f32,
}

struct RecipeUnlockTableRender<'a> {
    station: &'a StationDestination,
    service: &'a StationService,
    in_range: bool,
    credits: u32,
    purchased_unlocks: &'a [String],
    action_rail_width: f32,
    x: f32,
    width: f32,
}

fn content_browser_layout() -> ContentBrowserLayout {
    let width = screen_width() * 0.8;
    let height = screen_height() * 0.8;
    let x = (screen_width() - width) * 0.5;
    let y = (screen_height() - height) * 0.5;
    let column_gap = 14.0;
    let column_width = (width - 48.0 - column_gap * 4.0) / 5.0;
    let column_y = y + 110.0;
    let row_height = 23.0;
    let viewport_height = (height - 150.0).max(row_height);

    ContentBrowserLayout {
        x,
        y,
        width,
        height,
        column_y,
        column_width,
        column_gap,
        row_height,
        viewport_height,
    }
}

fn content_browser_column_rect(layout: &ContentBrowserLayout, column: usize) -> Rect {
    Rect::new(
        layout.x + 24.0 + (layout.column_width + layout.column_gap) * column as f32,
        layout.column_y + 18.0,
        layout.column_width,
        layout.viewport_height,
    )
}

fn handle_content_browser_scroll(game: &mut GameState, mouse: Vec2, wheel: f32) {
    let layout = content_browser_layout();
    let selected_pack_id = selected_content_pack_id(game).map(str::to_string);
    let item_count = filtered_content_item_rows(game, selected_pack_id.as_deref()).len();
    let recipe_count = filtered_content_recipe_rows(game, selected_pack_id.as_deref()).len();
    let npc_ship_count = filtered_content_npc_ship_rows(game, selected_pack_id.as_deref()).len();
    let planet_count = filtered_content_planet_rows(game, selected_pack_id.as_deref()).len();

    if content_browser_column_rect(&layout, 0).contains(mouse) {
        game.content_browser.packs_scroll = content_scrolled_offset(
            game.content_browser.packs_scroll,
            wheel,
            game.content_registry.packs.len() + 1,
            layout.row_height,
            layout.viewport_height,
        );
    } else if content_browser_column_rect(&layout, 1).contains(mouse) {
        game.content_browser.items_scroll = content_scrolled_offset(
            game.content_browser.items_scroll,
            wheel,
            item_count,
            layout.row_height,
            layout.viewport_height,
        );
    } else if content_browser_column_rect(&layout, 2).contains(mouse) {
        game.content_browser.recipes_scroll = content_scrolled_offset(
            game.content_browser.recipes_scroll,
            wheel,
            recipe_count,
            layout.row_height,
            layout.viewport_height,
        );
    } else if content_browser_column_rect(&layout, 3).contains(mouse) {
        game.content_browser.npc_ships_scroll = content_scrolled_offset(
            game.content_browser.npc_ships_scroll,
            wheel,
            npc_ship_count,
            layout.row_height,
            layout.viewport_height,
        );
    } else if content_browser_column_rect(&layout, 4).contains(mouse) {
        game.content_browser.planets_scroll = content_scrolled_offset(
            game.content_browser.planets_scroll,
            wheel,
            planet_count,
            layout.row_height,
            layout.viewport_height,
        );
    }
}

fn handle_content_browser_input(game: &mut GameState, mouse: Vec2) -> bool {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }

    let layout = content_browser_layout();
    let packs_rect = content_browser_column_rect(&layout, 0);
    if !packs_rect.contains(mouse) {
        return false;
    }

    let row = ((mouse.y - packs_rect.y + game.content_browser.packs_scroll) / layout.row_height)
        .floor() as isize;
    if row < 0 {
        return false;
    }

    let row = row as usize;
    if row == 0 {
        game.content_browser.selected_pack_index = None;
    } else if row - 1 < game.content_registry.packs.len() {
        game.content_browser.selected_pack_index = Some(row - 1);
    } else {
        return false;
    }

    game.content_browser.items_scroll = 0.0;
    game.content_browser.recipes_scroll = 0.0;
    game.content_browser.npc_ships_scroll = 0.0;
    game.content_browser.planets_scroll = 0.0;
    true
}

fn content_scrolled_offset(
    current: f32,
    wheel: f32,
    row_count: usize,
    row_height: f32,
    viewport_height: f32,
) -> f32 {
    let max_scroll = max_scroll_offset(row_count, row_height, viewport_height);
    (current - wheel * row_height * 2.0).clamp(0.0, max_scroll)
}

fn selected_content_pack_id(game: &GameState) -> Option<&str> {
    game.content_browser
        .selected_pack_index
        .and_then(|index| game.content_registry.packs.get(index))
        .map(|pack| pack.id.as_str())
}

fn content_id_belongs_to_pack(id: &str, pack_id: &str) -> bool {
    id.split_once(':')
        .is_some_and(|(id_pack, _)| id_pack == pack_id)
}

fn system_is_known(registry: &content::ContentRegistry, system_id: &str) -> bool {
    registry.systems.get(system_id).is_some_and(|system| {
        system.tags.iter().any(|tag| {
            tag == "starter" || tag == "surveyed-route" || tag == "known" || tag == "remote"
        })
    })
}

fn known_system_ids(registry: &content::ContentRegistry) -> Vec<String> {
    registry
        .system_order
        .iter()
        .filter(|system_id| system_is_known(registry, system_id))
        .cloned()
        .collect()
}

fn warp_cost(
    registry: &content::ContentRegistry,
    current_system_id: &str,
    target_system_id: &str,
) -> Vec<ItemStack> {
    if current_system_id == target_system_id || target_system_id == STARTER_SYSTEM_ID {
        return Vec::new();
    }

    core_item(registry, "fuel_canister")
        .map(|item| vec![ItemStack { item, count: 1 }])
        .unwrap_or_default()
}

fn format_warp_cost(cost: &[ItemStack]) -> String {
    if cost.is_empty() {
        return "free".to_string();
    }

    cost.iter()
        .map(|stack| format!("{} x{}", stack.item.name, stack.count))
        .collect::<Vec<_>>()
        .join(", ")
}

fn switch_current_system(game: &mut GameState, target_system_id: &str) {
    let Some(target_system) = game.content_registry.systems.get(target_system_id) else {
        eprintln!("Cannot switch to missing system `{target_system_id}`");
        return;
    };
    let arrival = target_system.arrival;

    remember_current_system_destination(game);

    game.current_system_id = target_system_id.to_string();
    game.selected_planet = None;
    game.selected_station = None;
    game.selected_npc_ship = None;
    game.selected_station_service = None;
    game.orbiting_planet = None;
    game.destination_planet = destination_planet_for_system(
        &game.planets,
        &game.system_destinations,
        &game.current_system_id,
    );
    game.ship.position = vec2(arrival[0], arrival[1]);
    game.ship.velocity = Vec2::ZERO;
    game.ship.angular_velocity = 0.0;
    game.save_dirty = true;
}

fn set_destination_planet(game: &mut GameState, destination_planet: Option<usize>) {
    if destination_planet.is_some() {
        break_planet_orbit(game);
    }
    game.destination_planet =
        destination_planet.filter(|index| planet_in_active_system(game, *index));
    game.selected_station = None;
    game.selected_npc_ship = None;
    game.selected_station_service = None;
    remember_current_system_destination(game);
}

fn select_nearby_destination(game: &mut GameState) {
    game.selected_planet = ship_over_planet_index(game);
    game.selected_station = if game.selected_planet.is_none() {
        ship_over_station_index(game)
    } else {
        None
    };
    game.selected_npc_ship = if game.selected_planet.is_none() && game.selected_station.is_none() {
        ship_over_npc_ship_index(game)
    } else {
        None
    };
    game.selected_station_service = None;
}

fn select_clicked_destination(game: &mut GameState, mouse: Vec2) {
    game.selected_planet = clicked_planet_index(
        mouse,
        &game.ship,
        &game.planets,
        &game.current_system_id,
        game.camera_zoom,
    );
    game.selected_station = if game.selected_planet.is_none() {
        clicked_station_index(
            mouse,
            &game.ship,
            &game.stations,
            &game.current_system_id,
            game.camera_zoom,
        )
    } else {
        None
    };
    game.selected_npc_ship = if game.selected_planet.is_none() && game.selected_station.is_none() {
        clicked_npc_ship_index(
            mouse,
            &game.ship,
            &game.npc_ships,
            &game.current_system_id,
            game.camera_zoom,
        )
    } else {
        None
    };
    game.selected_station_service = None;
}

fn clicked_player_ship(mouse: Vec2) -> bool {
    let center = vec2(screen_width() * 0.5, screen_height() * 0.5);
    mouse.distance(center) <= SHIP_SPRITE_SIZE * 0.5
}

fn identify_selected_npc_ship(game: &mut GameState) -> bool {
    let Some(npc_ship_index) = game.selected_npc_ship else {
        return false;
    };
    let Some(npc_ship) = game.npc_ships.get_mut(npc_ship_index) else {
        return false;
    };
    if !npc_ship_is_in_system(npc_ship, &game.current_system_id)
        || !npc_ship_in_interaction_range(&game.ship, npc_ship)
    {
        return false;
    }
    npc_ship.identified = true;
    true
}

fn remember_current_system_destination(game: &mut GameState) {
    if let Some(planet_id) = game
        .destination_planet
        .and_then(|index| game.planets.get(index))
        .filter(|planet| planet.system == game.current_system_id)
        .map(|planet| planet.id.clone())
    {
        game.system_destinations
            .insert(game.current_system_id.clone(), planet_id);
    } else {
        game.system_destinations.remove(&game.current_system_id);
    }
}

fn destination_planet_for_system(
    planets: &[Planet],
    system_destinations: &HashMap<String, String>,
    system_id: &str,
) -> Option<usize> {
    system_destinations.get(system_id).and_then(|planet_id| {
        planets
            .iter()
            .position(|planet| planet.id == *planet_id && planet.system == system_id)
    })
}

fn save_system_destinations(game: &GameState) -> Vec<SaveSystemDestination> {
    let mut destinations = game.system_destinations.clone();
    if let Some(planet_id) = game
        .destination_planet
        .and_then(|index| game.planets.get(index))
        .filter(|planet| planet.system == game.current_system_id)
        .map(|planet| planet.id.clone())
    {
        destinations.insert(game.current_system_id.clone(), planet_id);
    } else {
        destinations.remove(&game.current_system_id);
    }

    let mut destinations = destinations
        .into_iter()
        .map(|(system, planet)| SaveSystemDestination { system, planet })
        .collect::<Vec<_>>();
    destinations.sort_by(|a, b| a.system.cmp(&b.system).then(a.planet.cmp(&b.planet)));
    destinations
}

fn system_display_name<'a>(registry: &'a content::ContentRegistry, system_id: &'a str) -> &'a str {
    registry
        .systems
        .get(system_id)
        .map(|system| system.name.as_str())
        .unwrap_or(system_id)
}

fn clamp_inventory_scrolls(game: &mut GameState) {
    let table_height = work_table_height();
    game.work_scroll = game.work_scroll.clamp(
        0.0,
        max_scroll_offset(
            active_production_row_count(game),
            WORK_ROW_HEIGHT,
            table_height,
        ),
    );
    let inventory_rows = game
        .inventory
        .slots
        .iter()
        .filter(|slot| slot.is_some())
        .count();
    game.inventory_scroll = game.inventory_scroll.clamp(
        0.0,
        max_scroll_offset(inventory_rows, INVENTORY_ROW_HEIGHT, table_height),
    );
    game.upgrades_scroll = game.upgrades_scroll.clamp(
        0.0,
        max_scroll_offset(
            game.ship_upgrades.len(),
            SHIP_UPGRADE_ROW_HEIGHT,
            ship_upgrades_table_viewport_height(),
        ),
    );
}

fn update_save_state(game: &mut GameState, dt: f32) {
    game.save_status_timer = (game.save_status_timer - dt).max(0.0);
    if !game.save_dirty {
        game.save_delay = Some(AUTOSAVE_SECONDS);
        return;
    }
    let Some(delay) = game.save_delay.as_mut() else {
        return;
    };

    *delay -= dt;
    if *delay <= 0.0 {
        save_game_now(game, SaveFeedback::Auto);
    }
}

fn save_game_now(game: &mut GameState, feedback: SaveFeedback) {
    save_game_state(game);
    game.save_dirty = false;
    game.save_delay = Some(AUTOSAVE_SECONDS);
    game.save_status_manual = matches!(feedback, SaveFeedback::Manual);
    game.save_status_timer = if game.save_status_manual { 2.6 } else { 1.6 };
}

type GameSaveSnapshot = (
    u64,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    i32,
    i32,
    Option<usize>,
    Option<usize>,
);

fn game_save_snapshot(game: &GameState) -> GameSaveSnapshot {
    (
        game.world_seed,
        (game.camera_zoom * 1_000.0).round() as u32,
        game.inventory
            .slots
            .iter()
            .filter_map(|slot| slot.as_ref())
            .map(|stack| stack.count)
            .sum(),
        game.skills.iter().map(|skill| skill.level).sum(),
        game.ship_upgrades.iter().map(|upgrade| upgrade.level).sum(),
        game.planets
            .iter()
            .map(|planet| planet.scan_level as u32)
            .sum(),
        work_settings_signal(game),
        game.ship.position.x.round() as i32,
        game.ship.position.y.round() as i32,
        game.destination_planet,
        game.orbiting_planet,
    )
}

fn work_settings_signal(game: &GameState) -> u32 {
    let production = game
        .smelt_settings
        .iter()
        .chain(game.craft_settings.iter())
        .chain(game.processing_settings.iter())
        .fold(0_u32, |signal, setting| {
            signal
                .wrapping_add(setting.keep.rotate_left(3))
                .wrapping_add(setting.queued.rotate_left(7))
        });

    game.planets
        .iter()
        .flat_map(|planet| planet.mining.iter())
        .fold(production, |signal, setting| {
            signal
                .wrapping_add(setting.keep.rotate_left(11))
                .wrapping_add(setting.queued.rotate_left(17))
        })
}

fn update_window_size_memory(game: &mut GameState, dt: f32) {
    let window_size = current_window_size();
    if window_size != game.last_window_size {
        game.last_window_size = window_size;
        game.window_save_delay = Some(0.75);
        return;
    }

    let Some(delay) = game.window_save_delay.as_mut() else {
        return;
    };

    *delay -= dt;
    if *delay <= 0.0 {
        save_window_size(game.last_window_size);
        game.window_save_delay = None;
    }
}

fn handle_skills_table_input(game: &mut GameState, mouse: Vec2) -> bool {
    let Some(skill_index) = hovered_skill_plus(mouse, game.skills.len()) else {
        return false;
    };

    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }

    let levels_to_buy = work_setting_step() as u32;
    for _ in 0..levels_to_buy {
        if !game.skills[skill_index].buy_level() {
            break;
        }
    }
    true
}

fn handle_ship_upgrades_input(game: &mut GameState, mouse: Vec2) -> bool {
    let Some(upgrade_index) =
        hovered_ship_upgrade_plus(mouse, game.ship_upgrades.len(), game.upgrades_scroll)
    else {
        return false;
    };
    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }

    let levels_to_buy = work_setting_step();
    for _ in 0..levels_to_buy {
        if !buy_ship_upgrade(game, upgrade_index) {
            break;
        }
    }
    true
}

fn handle_ship_upgrades_scroll(game: &mut GameState, mouse: Vec2, wheel: f32) {
    let origin = ship_upgrade_table_origin();
    let (_, _, panel_width, _) = ship_upgrades_panel_rect();
    let viewport_top = ship_upgrades_table_viewport_top();
    let viewport_height = ship_upgrades_table_viewport_height();
    let viewport = Rect::new(origin.x, viewport_top, panel_width - 56.0, viewport_height);

    if viewport.contains(mouse) {
        game.upgrades_scroll = content_scrolled_offset(
            game.upgrades_scroll,
            wheel,
            game.ship_upgrades.len(),
            SHIP_UPGRADE_ROW_HEIGHT,
            viewport_height,
        );
    }
}

fn handle_ship_shield_slot_input(game: &mut GameState, mouse: Vec2) -> bool {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }

    (0..shield_slot_capacity(game))
        .find(|slot_index| ship_shield_slot_rect(*slot_index).contains(mouse))
        .is_some_and(|slot_index| install_first_available_shield_for_slot(game, slot_index))
}

fn install_first_available_shield_for_slot(game: &mut GameState, slot_index: usize) -> bool {
    let current_shield_id = game
        .equipped_shields
        .get(slot_index)
        .map(|shield| shield.id.as_str());
    let Some(shield_id) = game
        .content_registry
        .shield_order
        .iter()
        .find_map(|shield_id| {
            if current_shield_id == Some(shield_id.as_str()) {
                return None;
            }
            let shield = game.content_registry.shields.get(shield_id)?;
            let install_item = registry_item(&game.content_registry, &shield.install_item)?;
            (game.inventory.count(&install_item) > 0).then(|| shield_id.clone())
        })
    else {
        return false;
    };

    install_shield_in_slot(game, slot_index, &shield_id).is_ok()
}

fn handle_ship_weapon_slot_input(game: &mut GameState, mouse: Vec2) -> bool {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }

    (0..weapon_slot_capacity(game))
        .find(|slot_index| ship_weapon_slot_rect(*slot_index).contains(mouse))
        .is_some_and(|slot_index| install_first_available_weapon_for_slot(game, slot_index))
}

fn install_first_available_weapon_for_slot(game: &mut GameState, slot_index: usize) -> bool {
    let current_weapon_id = game
        .equipped_weapons
        .get(slot_index)
        .map(|weapon| weapon.id.as_str());
    let Some(weapon_id) = game
        .content_registry
        .weapon_order
        .iter()
        .find_map(|weapon_id| {
            if current_weapon_id == Some(weapon_id.as_str()) {
                return None;
            }
            let weapon = game.content_registry.weapons.get(weapon_id)?;
            let install_item = registry_item(&game.content_registry, &weapon.install_item)?;
            (game.inventory.count(&install_item) > 0).then(|| weapon_id.clone())
        })
    else {
        return false;
    };

    install_weapon_in_slot(game, slot_index, &weapon_id).is_ok()
}

fn handle_production_table_input(game: &mut GameState, mouse: Vec2, wheel: f32) -> bool {
    let action_rail_width = selected_action_rail_width(game);
    if let Some(mode) = clicked_production_mode(mouse, action_rail_width) {
        if is_mouse_button_pressed(MouseButton::Left) {
            game.production_mode = mode;
            game.work_scroll = 0.0;
            return true;
        }
    }

    match game.production_mode {
        ProductionMode::Smelting => handle_recipe_table_input(RecipeTableInput {
            recipes: &game.smelt_recipes,
            settings: &mut game.smelt_settings,
            locked_recipes: &game.recipe_vendor_locked_recipes,
            purchased_unlocks: &game.purchased_recipe_unlocks,
            mouse,
            wheel,
            scroll: game.work_scroll,
            action_rail_width,
        }),
        ProductionMode::Crafting => handle_recipe_table_input(RecipeTableInput {
            recipes: &game.craft_recipes,
            settings: &mut game.craft_settings,
            locked_recipes: &game.recipe_vendor_locked_recipes,
            purchased_unlocks: &game.purchased_recipe_unlocks,
            mouse,
            wheel,
            scroll: game.work_scroll,
            action_rail_width,
        }),
        ProductionMode::Processing => handle_recipe_table_input(RecipeTableInput {
            recipes: &game.processing_recipes,
            settings: &mut game.processing_settings,
            locked_recipes: &game.recipe_vendor_locked_recipes,
            purchased_unlocks: &game.purchased_recipe_unlocks,
            mouse,
            wheel,
            scroll: game.work_scroll,
            action_rail_width,
        }),
    }
}

fn handle_recipe_table_input(input: RecipeTableInput<'_>) -> bool {
    let RecipeTableInput {
        recipes,
        settings,
        locked_recipes,
        purchased_unlocks,
        mouse,
        wheel,
        scroll,
        action_rail_width,
    } = input;
    let Some((recipe_index, column)) =
        hovered_work_cell(mouse, recipes.len(), scroll, action_rail_width)
    else {
        return false;
    };
    if !recipe_is_unlocked_from_sets(&recipes[recipe_index].id, locked_recipes, purchased_unlocks) {
        return true;
    }

    let setting = &mut settings[recipe_index];
    let step = work_setting_step();

    if wheel > 0.0 {
        adjust_work_setting(&mut setting.keep, column, step);
    } else if wheel < 0.0 {
        adjust_work_setting(&mut setting.keep, column, -step);
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        match column {
            WorkColumn::Item => {
                setting.queued = setting.queued.saturating_add(1).min(999);
            }
            WorkColumn::Keep => adjust_work_setting(&mut setting.keep, column, step),
        }
        return true;
    }

    if is_mouse_button_pressed(MouseButton::Right) {
        adjust_work_setting(&mut setting.keep, column, -step);
        return true;
    }

    false
}

fn recipe_is_unlocked_from_sets(
    recipe_id: &str,
    locked_recipes: &[String],
    purchased_unlocks: &[String],
) -> bool {
    !locked_recipes.iter().any(|locked| locked == recipe_id)
        || purchased_unlocks
            .iter()
            .any(|unlocked| unlocked == recipe_id)
}

fn handle_planet_scan_input(game: &mut GameState, planet_index: usize, mouse: Vec2) -> bool {
    let Some(planet) = game.planets.get(planet_index) else {
        return false;
    };
    let rail_width = action_rail_width_with_override(planet_action_rail_width(planet), game);
    if !is_mouse_button_pressed(MouseButton::Left)
        || !planet_scan_button_rect(rail_width).contains(mouse)
    {
        return false;
    }
    if !planet_in_active_system(game, planet_index) {
        return false;
    }

    launch_planet_scan(game, planet_index);
    true
}

fn handle_planet_orbit_input(game: &mut GameState, planet_index: usize, mouse: Vec2) -> bool {
    let Some(planet) = game.planets.get(planet_index) else {
        return false;
    };
    let rail_width = action_rail_width_with_override(planet_action_rail_width(planet), game);
    if !is_mouse_button_pressed(MouseButton::Left)
        || !planet_orbit_button_rect(rail_width).contains(mouse)
    {
        return false;
    }

    enter_planet_orbit(game, planet_index);
    true
}

fn handle_station_service_input(game: &mut GameState, station_index: usize, mouse: Vec2) -> bool {
    if handle_station_recipe_unlock_input(game, station_index, mouse) {
        return true;
    }
    if handle_station_trade_input(game, station_index, mouse) {
        return true;
    }
    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }
    let Some(station) = game.stations.get(station_index) else {
        return false;
    };
    if !station_is_in_system(station, &game.current_system_id) {
        return false;
    }
    let rail_width = action_rail_width_with_override(station_action_rail_width(station), game);
    let Some(service_index) = hovered_station_service_index(station, mouse, rail_width) else {
        return false;
    };

    select_station_service(game, station_index, service_index)
}

fn handle_npc_ship_interaction_input(
    game: &mut GameState,
    npc_ship_index: usize,
    mouse: Vec2,
) -> bool {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }
    let Some(npc_ship) = game.npc_ships.get(npc_ship_index) else {
        return false;
    };
    if !npc_ship_is_in_system(npc_ship, &game.current_system_id) {
        return false;
    }
    let rows = npc_interaction_rows(&game.content_registry, &game.ship, npc_ship);
    let rail_width = npc_ship_action_rail_width(&game.content_registry, &game.ship, npc_ship);
    let Some(row_index) = hovered_npc_interaction_row_index(mouse, rows.len(), rail_width) else {
        return false;
    };
    let Some(row) = rows.get(row_index) else {
        return false;
    };
    if row.action == NpcInteractionAction::Identify && row.state == NpcInteractionState::Available {
        identify_selected_npc_ship(game);
    }
    true
}

fn npc_interaction_rows(
    content_registry: &content::ContentRegistry,
    ship: &Ship,
    npc_ship: &NpcShip,
) -> Vec<NpcInteractionRow> {
    let in_range = npc_ship_in_interaction_range(ship, npc_ship);
    let identified = npc_ship.identified;
    let hostile = npc_ship_is_hostile(content_registry, npc_ship);
    let trade_role = matches!(npc_ship.role.as_str(), "hauler" | "trader")
        || npc_ship
            .behavior_tags
            .iter()
            .any(|tag| tag == "trade-route");

    vec![
        NpcInteractionRow {
            action: NpcInteractionAction::Identify,
            state: if identified {
                NpcInteractionState::Complete
            } else if in_range {
                NpcInteractionState::Available
            } else {
                NpcInteractionState::Unavailable
            },
            status: if identified {
                "Known"
            } else if in_range {
                "Ready"
            } else {
                "Approach"
            },
        },
        NpcInteractionRow {
            action: NpcInteractionAction::Hail,
            state: if in_range && identified && !hostile {
                NpcInteractionState::Available
            } else {
                NpcInteractionState::Unavailable
            },
            status: if hostile {
                "Hostile"
            } else if !identified {
                "Identify"
            } else if in_range {
                "Channel"
            } else {
                "Approach"
            },
        },
        NpcInteractionRow {
            action: NpcInteractionAction::Dock,
            state: NpcInteractionState::Unavailable,
            status: if !identified {
                "Identify"
            } else if hostile {
                "Hostile"
            } else {
                "No dock"
            },
        },
        NpcInteractionRow {
            action: NpcInteractionAction::Trade,
            state: NpcInteractionState::Unavailable,
            status: if !identified {
                "Identify"
            } else if hostile {
                "Hostile"
            } else if trade_role {
                "No exchange"
            } else {
                "No stock"
            },
        },
        NpcInteractionRow {
            action: NpcInteractionAction::Conflict,
            state: NpcInteractionState::Unavailable,
            status: if hostile {
                "Auto defense"
            } else {
                "Unavailable"
            },
        },
    ]
}

fn npc_ship_is_hostile(content_registry: &content::ContentRegistry, npc_ship: &NpcShip) -> bool {
    npc_ship.role == "hostile"
        || npc_ship.behavior_tags.iter().any(|tag| tag == "hostile")
        || npc_ship.faction.as_deref().is_some_and(|faction_id| {
            content_registry
                .factions
                .get(faction_id)
                .is_some_and(|faction| {
                    faction.default_disposition == content::FactionDisposition::Hostile
                })
        })
}

fn hovered_npc_interaction_row_index(
    mouse: Vec2,
    row_count: usize,
    action_rail_width: f32,
) -> Option<usize> {
    let overlay = inventory_overlay_layout(Some(action_rail_width));
    let rail = overlay.action_rail?;
    let table = npc_interaction_table_layout(rail.x + 12.0, rail.y + 48.0, rail.w - 24.0);
    ui_hovered_table_cell(mouse, &table, row_count, 0.0).map(|cell| cell.row)
}

fn handle_station_recipe_unlock_input(
    game: &mut GameState,
    station_index: usize,
    mouse: Vec2,
) -> bool {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }
    let Some(service_index) = game.selected_station_service else {
        return false;
    };
    let Some(station) = game.stations.get(station_index) else {
        return false;
    };
    let Some(service) = station.services.get(service_index) else {
        return false;
    };
    let rail_width = action_rail_width_with_override(station_action_rail_width(station), game);
    let Some(unlock_index) = hovered_recipe_unlock_index(station, service, mouse, rail_width)
    else {
        return false;
    };

    purchase_recipe_unlock(game, station_index, service_index, unlock_index)
}

fn purchase_recipe_unlock(
    game: &mut GameState,
    station_index: usize,
    service_index: usize,
    unlock_index: usize,
) -> bool {
    let in_range = game
        .stations
        .get(station_index)
        .is_some_and(|station| station_in_interaction_range(&game.ship, station));
    if !in_range {
        return true;
    }
    let Some(unlock) = game
        .stations
        .get(station_index)
        .and_then(|station| station.services.get(service_index))
        .and_then(|service| service.recipe_unlocks.get(unlock_index))
    else {
        return false;
    };
    if unlock.unavailable
        || game.credits < unlock.price
        || game.purchased_recipe_unlocks.contains(&unlock.recipe)
    {
        return true;
    }

    game.credits -= unlock.price;
    game.purchased_recipe_unlocks.push(unlock.recipe.clone());
    game.purchased_recipe_unlocks.sort();
    game.purchased_recipe_unlocks.dedup();
    game.save_dirty = true;
    true
}

fn handle_station_trade_input(game: &mut GameState, station_index: usize, mouse: Vec2) -> bool {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }
    let Some(service_index) = game.selected_station_service else {
        return false;
    };
    let Some(station) = game.stations.get(station_index) else {
        return false;
    };
    if !station_is_in_system(station, &game.current_system_id) {
        return false;
    }
    let Some(service) = station.services.get(service_index) else {
        return false;
    };
    let rail_width = action_rail_width_with_override(station_action_rail_width(station), game);
    let Some((offer_index, action)) =
        hovered_station_trade_action(station, service, mouse, rail_width)
    else {
        return false;
    };

    match action {
        StationTradeAction::Buy => {
            buy_station_trade_offer(game, station_index, service_index, offer_index)
        }
        StationTradeAction::Sell => {
            sell_station_trade_offer(game, station_index, service_index, offer_index)
        }
    }
}

fn select_station_service(
    game: &mut GameState,
    station_index: usize,
    service_index: usize,
) -> bool {
    let Some(station) = game.stations.get(station_index) else {
        return false;
    };
    if !station_is_in_system(station, &game.current_system_id) {
        return false;
    }
    if service_index >= station.services.len() {
        return false;
    }

    game.selected_station = Some(station_index);
    game.selected_planet = None;
    game.selected_npc_ship = None;
    game.selected_station_service = Some(service_index);
    true
}

fn buy_station_trade_offer(
    game: &mut GameState,
    station_index: usize,
    service_index: usize,
    offer_index: usize,
) -> bool {
    let in_range = game
        .stations
        .get(station_index)
        .is_some_and(|station| station_in_interaction_range(&game.ship, station));
    if !in_range {
        return true;
    }
    let Some(offer) = game
        .stations
        .get_mut(station_index)
        .and_then(|station| station.services.get_mut(service_index))
        .and_then(|service| service.trade.get_mut(offer_index))
    else {
        return false;
    };
    if offer.unavailable || offer.stock == Some(0) || game.credits < offer.buy_price {
        return true;
    }

    game.credits -= offer.buy_price;
    if let Some(stock) = offer.stock.as_mut() {
        *stock = stock.saturating_sub(1);
    }
    game.inventory.add_item(offer.item.clone(), 1);
    game.save_dirty = true;
    true
}

fn sell_station_trade_offer(
    game: &mut GameState,
    station_index: usize,
    service_index: usize,
    offer_index: usize,
) -> bool {
    let in_range = game
        .stations
        .get(station_index)
        .is_some_and(|station| station_in_interaction_range(&game.ship, station));
    if !in_range {
        return true;
    }
    let Some(offer) = game
        .stations
        .get_mut(station_index)
        .and_then(|station| station.services.get_mut(service_index))
        .and_then(|service| service.trade.get_mut(offer_index))
    else {
        return false;
    };
    if game.inventory.count(&offer.item) == 0 {
        return true;
    }

    game.inventory.remove_item(&offer.item, 1);
    game.credits = game.credits.saturating_add(offer.sell_price);
    if let Some(stock) = offer.stock.as_mut() {
        *stock = stock.saturating_add(1);
    }
    game.save_dirty = true;
    true
}

fn hovered_station_service_index(
    station: &StationDestination,
    mouse: Vec2,
    action_rail_width: f32,
) -> Option<usize> {
    station
        .services
        .iter()
        .enumerate()
        .find(|(index, _)| {
            station_service_button_rect(station, *index, action_rail_width).contains(mouse)
        })
        .map(|(index, _)| index)
}

#[derive(Clone, Copy)]
enum StationTradeAction {
    Buy,
    Sell,
}

fn hovered_station_trade_action(
    station: &StationDestination,
    service: &StationService,
    mouse: Vec2,
    action_rail_width: f32,
) -> Option<(usize, StationTradeAction)> {
    let layout = station_action_layout(station, action_rail_width);
    let y = station_trade_table_y(station, action_rail_width);

    if layout.detail.w <= 0.0 {
        return None;
    }

    let table = station_trade_table_layout(layout.detail.x, y, layout.detail.w);
    ui_hovered_table_cell(mouse, &table, service.trade.len(), 0.0).and_then(|cell| {
        let action = match cell.column {
            1 => StationTradeAction::Buy,
            2 => StationTradeAction::Sell,
            _ => return None,
        };
        Some((cell.row, action))
    })
}

fn hovered_recipe_unlock_index(
    station: &StationDestination,
    service: &StationService,
    mouse: Vec2,
    action_rail_width: f32,
) -> Option<usize> {
    let layout = station_action_layout(station, action_rail_width);
    let y = recipe_unlock_table_y(station, service, action_rail_width);
    if layout.detail.w <= 0.0 {
        return None;
    }

    let table = recipe_unlock_table_layout(layout.detail.x, y, layout.detail.w);
    ui_hovered_table_cell(mouse, &table, service.recipe_unlocks.len(), 0.0).map(|cell| cell.row)
}

fn launch_planet_scan(game: &mut GameState, planet_index: usize) -> bool {
    let Some(planet) = game.planets.get(planet_index) else {
        return false;
    };
    if planet.scan_level >= MAX_SCAN_LEVEL {
        return false;
    }

    let survey_drone = core_item(&game.content_registry, "survey_drone");
    let improved_survey_drone = core_item(&game.content_registry, "improved_survey_drone");
    let scan_item = improved_survey_drone
        .as_ref()
        .filter(|item| game.inventory.count(item) > 0)
        .map(|item| (item.clone(), 2_u8))
        .or_else(|| {
            survey_drone
                .as_ref()
                .filter(|item| game.inventory.count(item) > 0)
                .map(|item| (item.clone(), 1_u8))
        });
    let Some((scan_item, scan_steps)) = scan_item else {
        return true;
    };

    if planet_in_interaction_range(&game.ship, planet) {
        let drone_returns =
            rand::gen_range(0.0, 1.0) < survey_drone_return_chance(&game.ship_upgrades);
        if !drone_returns {
            game.inventory.remove_item(&scan_item, 1);
        }
        if let Some(planet) = game.planets.get_mut(planet_index) {
            let scan_steps = scan_steps.saturating_add(scanner_survey_bonus(&game.ship_upgrades));
            planet.scan_level = planet
                .scan_level
                .saturating_add(scan_steps)
                .min(MAX_SCAN_LEVEL);
        }
        return true;
    }

    false
}

fn handle_mining_table_input(
    game: &mut GameState,
    planet_index: usize,
    mouse: Vec2,
    wheel: f32,
) -> bool {
    if !planet_in_active_system(game, planet_index) {
        return false;
    }
    let Some(planet) = game.planets.get(planet_index) else {
        return false;
    };
    if !planet_has_composition_scan(planet) {
        return false;
    }
    let rail_width = action_rail_width_with_override(planet_action_rail_width(planet), game);
    let Some((mineable_index, column)) = hovered_work_cell_with_action_rail(
        mouse,
        planet.info.mineables.len(),
        game.work_scroll,
        rail_width,
    ) else {
        return false;
    };

    let in_range = planet_in_interaction_range(&game.ship, planet);
    let Some(planet) = game.planets.get_mut(planet_index) else {
        return false;
    };
    let setting = &mut planet.mining[mineable_index];
    let step = work_setting_step();

    if wheel > 0.0 {
        adjust_work_setting(&mut setting.keep, column, step);
    } else if wheel < 0.0 {
        adjust_work_setting(&mut setting.keep, column, -step);
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        match column {
            WorkColumn::Item if in_range => {
                setting.queued = setting.queued.saturating_add(1).min(999);
            }
            WorkColumn::Item => {}
            WorkColumn::Keep => adjust_work_setting(&mut setting.keep, column, step),
        }
        return true;
    }

    if is_mouse_button_pressed(MouseButton::Right) {
        adjust_work_setting(&mut setting.keep, column, -step);
        return true;
    }

    false
}

fn handle_inventory_overlay_scroll(game: &mut GameState, mouse: Vec2, wheel: f32) {
    let layout = inventory_overlay_layout(selected_action_rail_width(game));
    let table_y = work_table_y();
    let table_height = work_table_height();

    if let Some(rail) = layout.action_rail.filter(|rail| rail.contains(mouse)) {
        let row_count = game
            .selected_planet
            .and_then(|planet_index| game.planets.get(planet_index))
            .filter(|planet| planet_has_composition_scan(planet))
            .map(|planet| planet.info.mineables.len())
            .unwrap_or(0);
        if row_count > 0 {
            game.work_scroll = scrolled_offset(
                game.work_scroll,
                wheel,
                row_count,
                (rail.h - 252.0).max(WORK_ROW_HEIGHT),
            );
        }
    } else if Rect::new(
        layout.production_x,
        table_y,
        layout.production_width,
        table_height,
    )
    .contains(mouse)
    {
        let row_count = active_production_row_count(game);
        let hovering_keep = hovered_work_cell(
            mouse,
            row_count,
            game.work_scroll,
            selected_action_rail_width(game),
        )
        .is_some_and(|(_, column)| column == WorkColumn::Keep);
        if !hovering_keep {
            game.work_scroll = scrolled_offset(game.work_scroll, wheel, row_count, table_height);
        }
    } else if Rect::new(
        layout.inventory_x,
        table_y,
        layout.inventory_width,
        table_height,
    )
    .contains(mouse)
    {
        let row_count = game
            .inventory
            .slots
            .iter()
            .filter(|slot| slot.is_some())
            .count();
        game.inventory_scroll =
            scrolled_offset(game.inventory_scroll, wheel, row_count, table_height);
    }
}

fn handle_action_rail_resize_input(game: &mut GameState, mouse: Vec2) -> bool {
    let Some(width) = selected_action_rail_width(game).filter(|_| selected_world_object(game))
    else {
        game.action_rail_resize_previous_mouse = None;
        return false;
    };
    let rail = action_rail_rect(width);
    let handle = action_rail_resize_handle_rect(rail);

    if is_mouse_button_down(MouseButton::Left) {
        if let Some(previous_mouse) = game.action_rail_resize_previous_mouse {
            let delta_x = mouse.x - previous_mouse.x;
            let resized_width = clamp_action_rail_width(width - delta_x);
            game.action_rail_width_override = Some(resized_width);
            game.action_rail_resize_previous_mouse = Some(mouse);
            return true;
        }
        if handle.contains(mouse) {
            game.action_rail_resize_previous_mouse = Some(mouse);
            return true;
        }
    } else {
        game.action_rail_resize_previous_mouse = None;
    }

    false
}

fn action_rail_consumes_pointer_click(game: &GameState, mouse: Vec2) -> bool {
    if !is_mouse_button_pressed(MouseButton::Left) && !is_mouse_button_pressed(MouseButton::Right) {
        return false;
    }
    let Some(width) = selected_action_rail_width(game).filter(|_| selected_world_object(game))
    else {
        return false;
    };

    action_rail_blocks_pointer(action_rail_rect(width), mouse)
}

fn action_rail_blocks_pointer(rail: Rect, mouse: Vec2) -> bool {
    rail.contains(mouse) || action_rail_resize_handle_rect(rail).contains(mouse)
}

fn active_production_row_count(game: &GameState) -> usize {
    match game.production_mode {
        ProductionMode::Smelting => game.smelt_recipes.len(),
        ProductionMode::Crafting => game.craft_recipes.len(),
        ProductionMode::Processing => game.processing_recipes.len(),
    }
}

fn scrolled_offset(current: f32, wheel: f32, row_count: usize, viewport_height: f32) -> f32 {
    let max_scroll = max_scroll_offset(row_count, WORK_ROW_HEIGHT, viewport_height);
    (current - wheel * WORK_ROW_HEIGHT * 2.0).clamp(0.0, max_scroll)
}

fn max_scroll_offset(row_count: usize, row_height: f32, viewport_height: f32) -> f32 {
    let content_height = row_count as f32 * row_height;
    (content_height - viewport_height).max(0.0)
}

fn update_production(game: &mut GameState, dt: f32) {
    update_recipes(
        RecipeUpdate {
            inventory: &mut game.inventory,
            recipes: &game.smelt_recipes,
            settings: &mut game.smelt_settings,
            skills: &mut game.skills,
            locked_recipes: &game.recipe_vendor_locked_recipes,
            purchased_unlocks: &game.purchased_recipe_unlocks,
            skill_kind: SkillKind::Smelting,
        },
        dt,
    );
    update_recipes(
        RecipeUpdate {
            inventory: &mut game.inventory,
            recipes: &game.craft_recipes,
            settings: &mut game.craft_settings,
            skills: &mut game.skills,
            locked_recipes: &game.recipe_vendor_locked_recipes,
            purchased_unlocks: &game.purchased_recipe_unlocks,
            skill_kind: SkillKind::Fabrication,
        },
        dt,
    );
    update_recipes(
        RecipeUpdate {
            inventory: &mut game.inventory,
            recipes: &game.processing_recipes,
            settings: &mut game.processing_settings,
            skills: &mut game.skills,
            locked_recipes: &game.recipe_vendor_locked_recipes,
            purchased_unlocks: &game.purchased_recipe_unlocks,
            skill_kind: SkillKind::Fabrication,
        },
        dt,
    );
}

struct RecipeUpdate<'a> {
    inventory: &'a mut Inventory,
    recipes: &'a [Recipe],
    settings: &'a mut [CraftSetting],
    skills: &'a mut [Skill; SKILL_COUNT],
    locked_recipes: &'a [String],
    purchased_unlocks: &'a [String],
    skill_kind: SkillKind,
}

fn update_recipes(update: RecipeUpdate<'_>, dt: f32) {
    let RecipeUpdate {
        inventory,
        recipes,
        settings,
        skills,
        locked_recipes,
        purchased_unlocks,
        skill_kind,
    } = update;

    clear_blocked_recipe_progress(
        inventory,
        recipes,
        settings,
        locked_recipes,
        purchased_unlocks,
    );
    let Some(recipe_index) = next_recipe_bill_index_for_sets(
        inventory,
        recipes,
        settings,
        locked_recipes,
        purchased_unlocks,
    ) else {
        return;
    };
    let recipe = &recipes[recipe_index];
    let operation_seconds = recipe_operation_seconds(skills, skill_kind, recipe);
    let setting = &mut settings[recipe_index];
    setting.progress += dt / operation_seconds;
    if setting.progress < 1.0 {
        return;
    }
    if !inventory.can_craft(recipe) {
        setting.progress = 0.0;
        return;
    }

    setting.progress -= 1.0;
    if setting.queued > 0 {
        setting.queued = setting.queued.saturating_sub(recipe.output.count);
    }
    if inventory.craft(recipe) {
        let bonus = bonus_output_count(skills, skill_kind, recipe.output.count);
        if bonus > 0 {
            inventory.add_item(recipe.output.item.clone(), bonus);
        }
        award_skill_xp(skills, skill_kind, &recipe.output.item, recipe.output.count);
    }
}

fn clear_blocked_recipe_progress(
    inventory: &Inventory,
    recipes: &[Recipe],
    settings: &mut [CraftSetting],
    locked_recipes: &[String],
    purchased_unlocks: &[String],
) {
    for (recipe, setting) in recipes.iter().zip(settings.iter_mut()) {
        if recipe_has_bill(inventory, recipe, setting)
            && (!recipe_is_unlocked_from_sets(&recipe.id, locked_recipes, purchased_unlocks)
                || !inventory.can_craft(recipe))
        {
            setting.progress = 0.0;
        }
    }
}

fn next_recipe_bill_index(
    game: &GameState,
    recipes: &[Recipe],
    settings: &[CraftSetting],
) -> Option<usize> {
    next_recipe_bill_index_for_sets(
        &game.inventory,
        recipes,
        settings,
        &game.recipe_vendor_locked_recipes,
        &game.purchased_recipe_unlocks,
    )
}

fn next_recipe_bill_index_for_sets(
    inventory: &Inventory,
    recipes: &[Recipe],
    settings: &[CraftSetting],
    locked_recipes: &[String],
    purchased_unlocks: &[String],
) -> Option<usize> {
    recipes
        .iter()
        .zip(settings.iter())
        .enumerate()
        .find_map(|(index, (recipe, setting))| {
            (recipe_is_unlocked_from_sets(&recipe.id, locked_recipes, purchased_unlocks)
                && recipe_has_bill(inventory, recipe, setting)
                && inventory.can_craft(recipe))
            .then_some(index)
        })
}

fn recipe_has_bill(inventory: &Inventory, recipe: &Recipe, setting: &CraftSetting) -> bool {
    setting.queued > 0 || (setting.keep > 0 && inventory.count(&recipe.output.item) < setting.keep)
}

fn update_mining(game: &mut GameState, dt: f32) {
    let base_operation_seconds = mining_operation_seconds(&game.skills);

    for planet_index in 0..game.planets.len() {
        let Some(planet) = game.planets.get(planet_index) else {
            continue;
        };
        if !planet_in_active_system(game, planet_index)
            || !planet_has_composition_scan(planet)
            || !planet_in_interaction_range(&game.ship, planet)
        {
            continue;
        }

        let Some(mineable_index) = next_mining_bill_index(&game.inventory, planet) else {
            continue;
        };
        let hazard_slowdown = planet_hazard_mining_slowdown(planet);
        let operation_seconds = base_operation_seconds * hazard_slowdown
            / mineable_richness_multiplier(planet, mineable_index);
        let mineable = game.planets[planet_index].info.mineables[mineable_index].clone();
        let richness_bonus_chance =
            mineable_bonus_yield_chance(&game.planets[planet_index], mineable_index);
        let mined = {
            let setting = &mut game.planets[planet_index].mining[mineable_index];
            setting.progress += dt / operation_seconds;
            if setting.progress < 1.0 {
                continue;
            }

            setting.progress -= 1.0;
            if setting.queued > 0 {
                let mined = setting.queued.min(1);
                setting.queued -= mined;
                mined
            } else {
                let current = game.inventory.count(&mineable.item);
                (setting.keep - current).min(1)
            }
        };

        if mined == 0 {
            continue;
        }

        game.inventory.add_item(mineable.item.clone(), mined);
        let bonus = bonus_output_count(&game.skills, SkillKind::Mining, mined);
        if bonus > 0 {
            game.inventory.add_item(mineable.item.clone(), bonus);
        }
        if rand::gen_range(0.0, 1.0) < richness_bonus_chance {
            game.inventory.add_item(mineable.item.clone(), mined);
        }
        award_skill_xp(&mut game.skills, SkillKind::Mining, &mineable.item, mined);
    }
}

fn update_orbital_hazards(game: &mut GameState, dt: f32) {
    let shield_drain = game
        .planets
        .iter()
        .enumerate()
        .filter(|(planet_index, _)| game.orbiting_planet != Some(*planet_index))
        .map(|(_, planet)| planet)
        .filter(|planet| planet_is_in_system(planet, &game.current_system_id))
        .filter(|planet| planet_in_interaction_range(&game.ship, planet))
        .map(planet_hazard_shield_drain_per_second)
        .sum::<f32>();

    if shield_drain > 0.0 {
        apply_shield_hazard_drain(game, shield_drain * dt);
    }
}

fn update_shield_recharge(game: &mut GameState, dt: f32) {
    if game.ship.systems.shields.current >= game.ship.systems.shields.max {
        game.shield_recharge_delay_remaining = 0.0;
        return;
    }

    if game.shield_recharge_delay_remaining > 0.0 {
        game.shield_recharge_delay_remaining = (game.shield_recharge_delay_remaining - dt).max(0.0);
        return;
    }

    let recharge = active_shield_recharge_rate(game) * dt;
    if recharge > 0.0 {
        game.ship.systems.shields.restore(recharge);
    }
}

#[derive(Clone, Copy)]
struct NpcAvoidanceBody {
    position: Vec2,
    radius: f32,
}

struct NpcMotionContext<'a> {
    target: Vec2,
    player_position: Vec2,
    stations: &'a [NpcAvoidanceBody],
    planets: &'a [NpcAvoidanceBody],
    npc_snapshots: &'a [(usize, NpcAvoidanceBody)],
    npc_index: usize,
    dt: f32,
}

fn update_npc_ships(game: &mut GameState, dt: f32) {
    if dt <= 0.0 {
        return;
    }

    let current_system_id = game.current_system_id.clone();
    let player_position = game.ship.position;
    let stations = game
        .stations
        .iter()
        .filter(|station| station.system == current_system_id)
        .map(|station| NpcAvoidanceBody {
            position: station.position,
            radius: station.radius + NPC_STATION_CLEARANCE,
        })
        .collect::<Vec<_>>();
    let planets = game
        .planets
        .iter()
        .filter(|planet| planet_is_in_system(planet, &current_system_id))
        .map(|planet| NpcAvoidanceBody {
            position: planet.position,
            radius: planet.radius + NPC_PLANET_CLEARANCE,
        })
        .collect::<Vec<_>>();
    let npc_snapshots = game
        .npc_ships
        .iter()
        .enumerate()
        .filter(|(_, npc_ship)| npc_ship.system == current_system_id)
        .map(|(index, npc_ship)| {
            (
                index,
                NpcAvoidanceBody {
                    position: npc_ship.position,
                    radius: npc_ship.radius + NPC_SEPARATION_PADDING,
                },
            )
        })
        .collect::<Vec<_>>();

    for index in 0..game.npc_ships.len() {
        if game.npc_ships[index].system != current_system_id {
            continue;
        }
        let target = npc_behavior_target(&game.npc_ships[index], player_position, &stations);
        update_npc_route_progress(&mut game.npc_ships[index], target);
        update_npc_ship_motion(
            &mut game.npc_ships[index],
            NpcMotionContext {
                target,
                player_position,
                stations: &stations,
                planets: &planets,
                npc_snapshots: &npc_snapshots,
                npc_index: index,
                dt,
            },
        );
    }
}

fn npc_behavior_target(
    npc_ship: &NpcShip,
    player_position: Vec2,
    stations: &[NpcAvoidanceBody],
) -> Vec2 {
    match npc_ship.behavior {
        NpcBehaviorMode::Follow => {
            player_position
                + safe_direction(npc_ship.position - player_position, vec2(1.0, 0.0))
                    * NPC_FOLLOW_DISTANCE
        }
        NpcBehaviorMode::Flee => {
            npc_ship.position
                + safe_direction(npc_ship.position - player_position, vec2(1.0, 0.0))
                    * NPC_ROUTE_RADIUS
        }
        NpcBehaviorMode::HostileIntercept => {
            player_position
                + safe_direction(npc_ship.position - player_position, vec2(1.0, 0.0))
                    * NPC_HOSTILE_STANDOFF_DISTANCE
        }
        NpcBehaviorMode::TradeRoute | NpcBehaviorMode::StationTraffic if !stations.is_empty() => {
            let station = stations[npc_ship.route_index % stations.len()];
            station.position + npc_route_offset(npc_ship.route_index) * (station.radius + 120.0)
        }
        NpcBehaviorMode::Patrol | NpcBehaviorMode::TradeRoute | NpcBehaviorMode::StationTraffic => {
            npc_ship.anchor + npc_route_offset(npc_ship.route_index) * NPC_ROUTE_RADIUS
        }
    }
}

fn update_npc_route_progress(npc_ship: &mut NpcShip, target: Vec2) {
    if npc_ship.position.distance(target) <= npc_ship.radius + 42.0 {
        npc_ship.route_index = npc_ship.route_index.wrapping_add(1);
    }
}

fn update_npc_ship_motion(npc_ship: &mut NpcShip, context: NpcMotionContext<'_>) {
    let desired_velocity = npc_desired_velocity(npc_ship, context.target, context.player_position);
    let mut steering = desired_velocity - npc_ship.velocity;
    steering += avoidance_steering(
        npc_ship.position,
        npc_ship.radius,
        NPC_SEPARATION_PADDING + SHIP_RADIUS,
        &[NpcAvoidanceBody {
            position: context.player_position,
            radius: SHIP_RADIUS,
        }],
    ) * NPC_ACCELERATION;
    steering += avoidance_steering(
        npc_ship.position,
        npc_ship.radius,
        NPC_STATION_CLEARANCE,
        context.stations,
    ) * NPC_ACCELERATION;
    steering += avoidance_steering(
        npc_ship.position,
        npc_ship.radius,
        NPC_PLANET_CLEARANCE,
        context.planets,
    ) * NPC_ACCELERATION;
    for (other_index, other) in context.npc_snapshots {
        if *other_index == context.npc_index {
            continue;
        }
        steering += avoidance_steering(
            npc_ship.position,
            npc_ship.radius,
            NPC_SEPARATION_PADDING,
            &[*other],
        ) * NPC_ACCELERATION;
    }

    npc_ship.velocity += clamp_vec2_length(steering, NPC_ACCELERATION) * context.dt;
    npc_ship.velocity = clamp_vec2_length(npc_ship.velocity, npc_ship.behavior.max_speed());
    npc_ship.position += npc_ship.velocity * context.dt;
    if npc_ship.velocity.length_squared() > 0.01 {
        npc_ship.angle = npc_ship.velocity.y.atan2(npc_ship.velocity.x);
    }
    if !npc_ship.position.is_finite() {
        npc_ship.position = npc_ship.anchor;
        npc_ship.velocity = Vec2::ZERO;
    }
}

fn npc_desired_velocity(npc_ship: &NpcShip, target: Vec2, player_position: Vec2) -> Vec2 {
    let to_target = target - npc_ship.position;
    let distance = to_target.length();
    if distance <= 1.0 {
        return Vec2::ZERO;
    }

    let mut speed = npc_ship.behavior.max_speed();
    if matches!(
        npc_ship.behavior,
        NpcBehaviorMode::Follow | NpcBehaviorMode::HostileIntercept
    ) && npc_ship.position.distance(player_position)
        <= npc_ship.radius + NPC_HOSTILE_STANDOFF_DISTANCE.min(NPC_FOLLOW_DISTANCE)
    {
        speed *= 0.35;
    }
    if !matches!(npc_ship.behavior, NpcBehaviorMode::Flee) {
        speed *= (distance / 180.0).clamp(0.25, 1.0);
    }

    to_target / distance * speed
}

fn avoidance_steering(
    position: Vec2,
    radius: f32,
    clearance: f32,
    bodies: &[NpcAvoidanceBody],
) -> Vec2 {
    bodies.iter().fold(Vec2::ZERO, |force, body| {
        let away = position - body.position;
        let distance = away.length();
        let desired_distance = radius + body.radius + clearance;
        if distance >= desired_distance {
            return force;
        }
        let direction = safe_direction(away, vec2(1.0, 0.0));
        let strength = if distance <= 1.0 {
            1.0
        } else {
            ((desired_distance - distance) / desired_distance).clamp(0.0, 1.0)
        };
        force + direction * strength
    })
}

fn npc_route_offset(route_index: usize) -> Vec2 {
    let offset = NPC_ROUTE_POINTS[route_index % NPC_ROUTE_POINTS.len()];
    vec2(offset[0], offset[1])
}

fn safe_direction(vector: Vec2, fallback: Vec2) -> Vec2 {
    if vector.length_squared() > 0.001 {
        vector.normalize()
    } else {
        fallback.normalize()
    }
}

fn clamp_vec2_length(vector: Vec2, max_length: f32) -> Vec2 {
    if vector.length_squared() > max_length * max_length {
        vector.normalize() * max_length
    } else {
        vector
    }
}

fn update_weapon_systems(game: &mut GameState, dt: f32) {
    game.weapon_fire_events.retain_mut(|event| {
        event.timer -= dt;
        event.timer > 0.0
    });

    for weapon_index in 0..game.equipped_weapons.len() {
        {
            let weapon = &mut game.equipped_weapons[weapon_index];
            weapon.cooldown_remaining = (weapon.cooldown_remaining - dt).max(0.0);
            if weapon.cooldown_remaining > 0.0 {
                weapon.status = WeaponStatus::Cooldown;
                continue;
            }
        }

        let target_index = defense_turret_target_index(
            &game.ship,
            &game.equipped_weapons[weapon_index],
            &game.defense_threats,
            &game.current_system_id,
        );
        let Some(target_index) = target_index else {
            game.equipped_weapons[weapon_index].status = WeaponStatus::NoThreat;
            continue;
        };

        let weapon = &mut game.equipped_weapons[weapon_index];
        if game.ship.systems.energy.current < weapon.energy_cost {
            weapon.status = WeaponStatus::InsufficientEnergy;
            continue;
        }

        let target = &mut game.defense_threats[target_index];
        game.ship.systems.energy.spend(weapon.energy_cost);
        target.hull.spend(weapon.damage);
        weapon.cooldown_remaining = weapon.cooldown_seconds;
        weapon.status = WeaponStatus::Fired;
        game.weapon_fire_events.push(WeaponFireEvent {
            from: game.ship.position,
            to: target.position,
            timer: WEAPON_FIRE_EVENT_SECONDS,
        });
        game.save_dirty = true;
    }
}

fn defense_turret_target_index(
    ship: &Ship,
    weapon: &WeaponSystem,
    threats: &[DefenseThreat],
    current_system_id: &str,
) -> Option<usize> {
    if weapon.kind != content::WeaponKind::TurretDefense {
        return None;
    }

    threats
        .iter()
        .enumerate()
        .filter(|(_, threat)| {
            defense_threat_is_valid_target(ship, weapon, threat, current_system_id)
        })
        .min_by(|(_, a), (_, b)| {
            a.position
                .distance_squared(ship.position)
                .total_cmp(&b.position.distance_squared(ship.position))
        })
        .map(|(index, _)| index)
}

fn defense_threat_is_valid_target(
    ship: &Ship,
    weapon: &WeaponSystem,
    threat: &DefenseThreat,
    current_system_id: &str,
) -> bool {
    threat.disposition == ThreatDisposition::Hostile
        && threat.system == current_system_id
        && threat.hull.current > 0.0
        && ship.position.distance(threat.position) <= weapon.range + threat.radius
        && target_within_tracking_arc(ship, weapon, threat.position)
}

fn target_within_tracking_arc(ship: &Ship, weapon: &WeaponSystem, target_position: Vec2) -> bool {
    if weapon.tracking_degrees >= 359.0 {
        return true;
    }
    let to_target = target_position - ship.position;
    if to_target.length_squared() <= f32::EPSILON {
        return true;
    }
    let forward = vec2(ship.angle.cos(), ship.angle.sin());
    let target_direction = to_target.normalize();
    let angle = forward.dot(target_direction).clamp(-1.0, 1.0).acos();
    angle <= weapon.tracking_degrees.to_radians() * 0.5
}

fn next_mining_bill_index(inventory: &Inventory, planet: &Planet) -> Option<usize> {
    planet
        .info
        .mineables
        .iter()
        .zip(planet.mining.iter())
        .enumerate()
        .find_map(|(index, (mineable, setting))| {
            if setting.queued > 0 {
                return Some(index);
            }
            if setting.keep == 0 {
                return None;
            }
            let current = inventory.count(&mineable.item);
            (current < setting.keep).then_some(index)
        })
}

fn work_setting_step() -> i32 {
    if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
        10
    } else if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
        5
    } else {
        1
    }
}

fn adjust_work_setting(keep: &mut u32, column: WorkColumn, direction: i32) {
    if column != WorkColumn::Keep {
        return;
    }
    let step = direction.unsigned_abs();

    if direction > 0 {
        *keep = keep.saturating_add(step).min(999);
    } else {
        *keep = keep.saturating_sub(step);
    }
}

fn adjust_camera_zoom(game: &mut GameState, direction: i32) {
    if direction > 0 {
        game.camera_zoom = (game.camera_zoom * CAMERA_ZOOM_STEP).min(CAMERA_ZOOM_MAX);
    } else if direction < 0 {
        game.camera_zoom = (game.camera_zoom / CAMERA_ZOOM_STEP).max(CAMERA_ZOOM_MIN);
    }
}

fn update_starmap_view_input(game: &mut GameState, _dt: f32, wheel: f32) {
    if wheel > 0.0 {
        game.starmap_zoom = (game.starmap_zoom * STARMAP_ZOOM_STEP).min(STARMAP_ZOOM_MAX);
    } else if wheel < 0.0 {
        game.starmap_zoom = (game.starmap_zoom / STARMAP_ZOOM_STEP).max(STARMAP_ZOOM_MIN);
    }

    if is_key_pressed(KeyCode::Home) {
        game.starmap_pan = Vec2::ZERO;
    }

    let mouse = mouse_vec2();
    if is_mouse_button_down(MouseButton::Right) {
        if let Some(previous_mouse) = game.starmap_drag_previous_mouse {
            let delta = mouse - previous_mouse;
            game.starmap_pan +=
                vec2(delta.x, delta.y) * STARMAP_PAN_PIXELS_TO_WORLD / game.starmap_zoom.max(0.01);
        }
        game.starmap_drag_previous_mouse = Some(mouse);
    } else {
        game.starmap_drag_previous_mouse = None;
    }
}

fn default_camera_zoom() -> f32 {
    1.0
}

fn default_credits() -> u32 {
    500
}

fn default_current_system_id() -> String {
    STARTER_SYSTEM_ID.to_string()
}

fn update_ship(ship: &mut Ship, dt: f32, energy_recharge: f32) {
    let forward = vec2(ship.angle.cos(), ship.angle.sin());
    let mut energy_spend = 0.0;

    let mut thrust = Vec2::ZERO;
    if is_key_down(KeyCode::W) {
        thrust -= forward * ship.forward_acceleration();
        energy_spend += 17.0;
    }
    if is_key_down(KeyCode::S) {
        thrust += forward * ship.reverse_acceleration();
        energy_spend += 11.0;
    }

    if thrust.length_squared() > 0.0 {
        ship.velocity += thrust * dt;
    }

    let mut turn = 0.0;
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        turn -= 1.0;
        energy_spend += 7.0;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        turn += 1.0;
        energy_spend += 7.0;
    }

    let target_turn_rate = turn * ship.max_turn_rate();
    let turn_response = if turn != 0.0 { 10.0 } else { 7.5 };
    let turn_blend = 1.0 - (-turn_response * dt).exp();
    ship.angular_velocity += (target_turn_rate - ship.angular_velocity) * turn_blend;

    if turn == 0.0 && ship.angular_velocity.abs() < 0.01 {
        ship.angular_velocity = 0.0;
    }
    ship.angle += ship.angular_velocity * dt;

    ship.velocity *= ship.attributes.linear_drag.powf(dt * 60.0);
    ship.position += ship.velocity * dt;

    if energy_spend > 0.0 {
        ship.systems.energy.spend(energy_spend * dt);
    } else {
        ship.systems.energy.restore(energy_recharge * dt);
    }
}

fn draw_scene(game: &GameState, background: &UniverseBackground, logo: Option<&Texture2D>) {
    clear_background(Color::from_rgba(5, 8, 18, 255));

    let center = vec2(screen_width() * 0.5, screen_height() * 0.5);
    let ship = &game.ship;
    let zoom = game.camera_zoom;
    let speed = ship.velocity.length();

    for layer in &background.star_layers {
        draw_star_layer(center, ship, layer, zoom);
    }

    let active_stars = game
        .system_stars
        .iter()
        .filter(|star| system_star_is_in_system(star, &game.current_system_id))
        .collect::<Vec<_>>();

    for star in &active_stars {
        draw_system_light_haze(
            center,
            ship,
            star,
            game.system_light_haze_texture.as_ref(),
            zoom,
        );
    }
    for star in active_stars {
        draw_system_star(center, ship, star, zoom);
    }

    draw_poi_route(
        center,
        ship,
        &game.planets,
        &game.current_system_id,
        game.destination_planet,
        zoom,
    );
    for planet in game
        .planets
        .iter()
        .filter(|planet| planet_is_in_system(planet, &game.current_system_id))
    {
        draw_planet(center, ship, planet, zoom);
    }
    for station in game
        .stations
        .iter()
        .filter(|station| station_is_in_system(station, &game.current_system_id))
    {
        draw_station(center, ship, station, zoom);
    }
    for npc_ship in game
        .npc_ships
        .iter()
        .filter(|npc_ship| npc_ship.system == game.current_system_id)
    {
        draw_npc_ship(&game.content_registry, center, ship, npc_ship, zoom);
    }
    for threat in game
        .defense_threats
        .iter()
        .filter(|threat| threat.system == game.current_system_id && threat.hull.current > 0.0)
    {
        draw_defense_threat(center, ship, threat, zoom);
    }
    for event in &game.weapon_fire_events {
        draw_weapon_fire_event(center, ship, event, zoom);
    }
    draw_poi_indicator(
        center,
        ship,
        &game.planets,
        &game.current_system_id,
        game.destination_planet,
        zoom,
    );
    draw_ship_status_arcs(center, ship, zoom);
    draw_ship(center, ship, game.ship_texture.as_ref(), zoom);
    let turn = ship.angular_velocity;
    draw_hud(HudView {
        ship,
        planets: &game.planets,
        stations: &game.stations,
        npc_ships: &game.npc_ships,
        selected_planet: game.selected_planet,
        selected_station: game.selected_station,
        selected_npc_ship: game.selected_npc_ship,
        destination_planet: game.destination_planet,
        orbiting_planet: game.orbiting_planet,
        current_system_id: &game.current_system_id,
        speed,
        turn,
    });
    draw_inventory_hint(
        game.inventory_open,
        game.map_open,
        game.skills_open,
        game.upgrades_open,
        game.content_open,
        game.save_status_timer > 0.0,
    );
    draw_interaction_prompt(game);

    if game.inventory_open {
        draw_inventory_overlay(game);
    }
    if game.map_open {
        draw_starmap_overlay(game);
    }
    if game.skills_open {
        draw_skills_overlay(game);
    }
    if game.upgrades_open {
        draw_ship_upgrades_overlay(game);
    }
    if game.content_open {
        draw_content_debug_overlay(game);
    }
    if game.escape_dialog_open {
        draw_escape_dialog(game, logo);
    }
    if game.save_status_timer > 0.0 {
        draw_save_confirmation(game.save_status_timer, game.save_status_manual);
    }
    if let Some(transition) = &game.scene_transition {
        draw_scene_transition_overlay(transition);
    }
}

fn draw_scene_transition_overlay(transition: &SceneTransition) {
    let opacity = transition.opacity();
    let screen_w = screen_width();
    let screen_h = screen_height();

    if let Some(texture) = &transition.texture {
        draw_fullscreen_texture_cover(texture, opacity);
    }

    draw_rectangle(
        0.0,
        0.0,
        screen_w,
        screen_h,
        Color::new(0.01, 0.02, 0.04, 0.28 + 0.38 * opacity),
    );
    draw_rectangle(
        0.0,
        screen_h * 0.72,
        screen_w,
        screen_h * 0.28,
        Color::new(0.01, 0.02, 0.04, 0.42 + 0.34 * opacity),
    );

    let title = "In Transit";
    let title_size = 34.0;
    let label_size = 20.0;
    let label = fit_debug_text(&transition.label, screen_w - 80.0, label_size as u16);
    let title_width = measure_text(title, None, title_size as u16, 1.0).width;
    let label_width = measure_text(&label, None, label_size as u16, 1.0).width;
    let text_x = (screen_w - title_width).max(0.0) * 0.5;
    let label_x = (screen_w - label_width).max(0.0) * 0.5;
    let base_y = screen_h * 0.8;

    draw_text(
        title,
        text_x,
        base_y,
        title_size,
        Color::new(0.92, 0.95, 0.89, opacity),
    );
    draw_text(
        &label,
        label_x,
        base_y + 34.0,
        label_size,
        Color::new(0.59, 0.87, 0.89, opacity),
    );

    let progress = (transition.timer / transition.total_seconds()).clamp(0.0, 1.0);
    let bar_width = 220.0;
    let bar_x = (screen_w - bar_width) * 0.5;
    let bar_y = base_y + 58.0;
    draw_rectangle_lines(
        bar_x,
        bar_y,
        bar_width,
        4.0,
        1.0,
        Color::new(0.33, 0.47, 0.51, 0.55 * opacity),
    );
    draw_rectangle(
        bar_x,
        bar_y,
        bar_width * progress,
        4.0,
        Color::new(0.59, 0.87, 0.89, 0.82 * opacity),
    );
}

fn draw_fullscreen_texture_cover(texture: &Texture2D, opacity: f32) {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let scale = (screen_w / texture.width()).max(screen_h / texture.height());
    let dest_size = vec2(texture.width() * scale, texture.height() * scale);
    let position = vec2(
        (screen_w - dest_size.x) * 0.5,
        (screen_h - dest_size.y) * 0.5,
    );

    draw_texture_ex(
        texture,
        position.x,
        position.y,
        Color::new(1.0, 1.0, 1.0, opacity),
        DrawTextureParams {
            dest_size: Some(dest_size),
            ..Default::default()
        },
    );
}

fn draw_texture_contain(texture: &Texture2D, rect: Rect, opacity: f32) {
    draw_texture_source_contain(
        texture,
        Rect::new(0.0, 0.0, texture.width(), texture.height()),
        rect,
        opacity,
    );
}

fn draw_texture_source_contain(texture: &Texture2D, source: Rect, rect: Rect, opacity: f32) {
    let opacity = opacity.clamp(0.0, 1.0);
    let scale = (rect.w / source.w).min(rect.h / source.h);
    let dest_size = vec2(source.w * scale, source.h * scale);
    let position = vec2(
        rect.x + (rect.w - dest_size.x) * 0.5,
        rect.y + (rect.h - dest_size.y) * 0.5,
    );

    draw_texture_ex(
        texture,
        position.x,
        position.y,
        Color::new(1.0, 1.0, 1.0, opacity),
        DrawTextureParams {
            source: Some(source),
            dest_size: Some(dest_size),
            ..Default::default()
        },
    );
}

fn draw_star_layer(center: Vec2, ship: &Ship, layer: &StarLayer, zoom: f32) {
    let layer_ship_position = ship.position * layer.depth;
    let screen_velocity = star_flow_velocity(ship, layer, zoom);

    for star in &layer.stars {
        let world_pos = wrap_star(star.position, layer_ship_position);
        let screen_pos = world_to_screen_layer(world_pos, layer_ship_position, center, zoom);
        draw_star_trail(screen_pos, star, layer, screen_velocity);
    }
}

fn wrap_star(base: Vec2, ship_position: Vec2) -> Vec2 {
    let diameter = STARFIELD_RADIUS * 2.0;
    let mut position = base;

    while position.x - ship_position.x > STARFIELD_RADIUS {
        position.x -= diameter;
    }
    while position.x - ship_position.x < -STARFIELD_RADIUS {
        position.x += diameter;
    }
    while position.y - ship_position.y > STARFIELD_RADIUS {
        position.y -= diameter;
    }
    while position.y - ship_position.y < -STARFIELD_RADIUS {
        position.y += diameter;
    }

    position
}

fn world_to_screen_layer(
    world_pos: Vec2,
    layer_ship_position: Vec2,
    center: Vec2,
    zoom: f32,
) -> Vec2 {
    let relative = world_pos - layer_ship_position;
    center + relative * zoom
}

fn star_flow_velocity(ship: &Ship, layer: &StarLayer, zoom: f32) -> Vec2 {
    -ship.velocity * layer.depth * zoom
}

fn rotate(point: Vec2, angle: f32) -> Vec2 {
    vec2(
        point.x * angle.cos() - point.y * angle.sin(),
        point.x * angle.sin() + point.y * angle.cos(),
    )
}

fn world_to_screen(world_position: Vec2, center: Vec2, ship: &Ship, zoom: f32) -> Vec2 {
    center + (world_position - ship.position) * zoom
}

fn system_star_is_in_system(star: &SystemStar, current_system_id: &str) -> bool {
    star.system == current_system_id
}

fn draw_system_light_haze(
    center: Vec2,
    ship: &Ship,
    star: &SystemStar,
    texture: Option<&Texture2D>,
    zoom: f32,
) {
    let Some(texture) = texture else {
        return;
    };
    let screen_pos = world_to_screen(star.position, center, ship, zoom);
    let star_radius = (star.radius * zoom).max(18.0);
    let haze_radius = star_radius * 6.8;
    let cull_padding = haze_radius + 120.0;

    if screen_pos.x < -cull_padding
        || screen_pos.x > screen_width() + cull_padding
        || screen_pos.y < -cull_padding
        || screen_pos.y > screen_height() + cull_padding
    {
        return;
    }

    let size = vec2(haze_radius * 2.0, haze_radius * 2.0);
    draw_texture_ex(
        texture,
        screen_pos.x - haze_radius,
        screen_pos.y - haze_radius,
        Color::new(star.color.r, star.color.g, star.color.b, 0.36),
        DrawTextureParams {
            dest_size: Some(size),
            ..Default::default()
        },
    );
}

fn draw_system_star(center: Vec2, ship: &Ship, star: &SystemStar, zoom: f32) {
    let screen_pos = world_to_screen(star.position, center, ship, zoom);
    let star_radius = (star.radius * zoom).max(18.0);
    let cull_padding = (star_radius * 1.4).max(120.0);

    if screen_pos.x < -cull_padding
        || screen_pos.x > screen_width() + cull_padding
        || screen_pos.y < -cull_padding
        || screen_pos.y > screen_height() + cull_padding
    {
        return;
    }

    draw_circle(
        screen_pos.x,
        screen_pos.y,
        star_radius * 1.08,
        Color::new(star.color.r, star.color.g, star.color.b, 0.56),
    );
    draw_circle(
        screen_pos.x,
        screen_pos.y,
        star_radius * 0.92,
        Color::new(star.color.r, star.color.g, star.color.b, 0.94),
    );
    draw_circle(
        screen_pos.x - star_radius * 0.28,
        screen_pos.y - star_radius * 0.28,
        star_radius * 0.34,
        Color::from_rgba(255, 242, 198, 120),
    );

    let label = fit_debug_text(&star.name, 220.0, 17);
    let label_measure = measure_text(&label, None, 17, 1.0);
    let label_y = screen_pos.y + star_radius + 28.0;
    draw_text(
        &label,
        screen_pos.x - label_measure.width * 0.5,
        label_y,
        17.0,
        Color::from_rgba(244, 226, 188, 235),
    );

    let subtitle = if star.is_primary {
        "PRIMARY STAR"
    } else {
        star.classification.as_str()
    };
    let subtitle = fit_debug_text(subtitle, 220.0, 13);
    let subtitle_measure = measure_text(&subtitle, None, 13, 1.0);
    draw_text(
        &subtitle,
        screen_pos.x - subtitle_measure.width * 0.5,
        label_y + 17.0,
        13.0,
        Color::from_rgba(205, 176, 126, 190),
    );
}

fn active_orbit_guides(planets: &[Planet], current_system_id: &str) -> Vec<OrbitGuide> {
    let mut guides = Vec::new();
    for planet in planets
        .iter()
        .filter(|planet| planet_is_in_system(planet, current_system_id))
    {
        let PlanetMotion::Orbit(orbit) = planet.motion else {
            continue;
        };
        if guides.iter().any(|guide: &OrbitGuide| {
            guide.center.distance_squared(orbit.center) < 1.0
                && (guide.radius - orbit.radius).abs() < 1.0
        }) {
            continue;
        }
        guides.push(OrbitGuide {
            center: orbit.center,
            radius: orbit.radius,
            semi_minor: orbit.semi_minor,
            axis_rotation: orbit.axis_rotation,
        });
    }
    guides.sort_by(|a, b| a.radius.total_cmp(&b.radius));
    guides
}

fn clicked_planet_index(
    mouse: Vec2,
    ship: &Ship,
    planets: &[Planet],
    current_system_id: &str,
    zoom: f32,
) -> Option<usize> {
    let center = vec2(screen_width() * 0.5, screen_height() * 0.5);

    planets.iter().enumerate().find_map(|(index, planet)| {
        if !planet_is_in_system(planet, current_system_id) {
            return None;
        }
        let screen_pos = world_to_screen(planet.position, center, ship, zoom);
        (mouse.distance(screen_pos) <= (planet.radius * zoom).max(48.0)).then_some(index)
    })
}

fn clicked_station_index(
    mouse: Vec2,
    ship: &Ship,
    stations: &[StationDestination],
    current_system_id: &str,
    zoom: f32,
) -> Option<usize> {
    let center = vec2(screen_width() * 0.5, screen_height() * 0.5);

    stations
        .iter()
        .enumerate()
        .filter(|(_, station)| station_is_in_system(station, current_system_id))
        .filter_map(|(index, station)| {
            let screen_pos = world_to_screen(station.position, center, ship, zoom);
            (mouse.distance(screen_pos) <= (station.radius * zoom).max(44.0))
                .then_some((index, mouse.distance_squared(screen_pos)))
        })
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index)
}

fn clicked_npc_ship_index(
    mouse: Vec2,
    ship: &Ship,
    npc_ships: &[NpcShip],
    current_system_id: &str,
    zoom: f32,
) -> Option<usize> {
    let center = vec2(screen_width() * 0.5, screen_height() * 0.5);

    npc_ships
        .iter()
        .enumerate()
        .filter(|(_, npc_ship)| npc_ship_is_in_system(npc_ship, current_system_id))
        .filter_map(|(index, npc_ship)| {
            let screen_pos = world_to_screen(npc_ship.position, center, ship, zoom);
            let hit_radius = (npc_ship.radius * 2.0 * zoom).clamp(30.0, 74.0);
            (mouse.distance(screen_pos) <= hit_radius)
                .then_some((index, mouse.distance_squared(screen_pos)))
        })
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index)
}

fn clicked_starmap_planet_index(mouse: Vec2, game: &GameState) -> Option<usize> {
    let (x, y, width, height) = starmap_panel_rect();
    if mouse.x < x || mouse.x > x + width || mouse.y < y || mouse.y > y + height {
        return None;
    }
    if known_systems_panel_rect().contains(mouse) {
        return None;
    }

    let camera = starmap_camera(x, y, width, height);
    game.planets
        .iter()
        .enumerate()
        .filter(|(index, _)| planet_in_active_system(game, *index))
        .filter(|(index, planet)| planet_matches_starmap_filter(game, *index, planet))
        .filter_map(|(index, planet)| {
            let screen_pos = starmap_planet_screen_pos(
                &game.ship,
                planet,
                &camera,
                game.starmap_zoom,
                game.starmap_pan,
            );
            let hit_radius = starmap_planet_hit_radius(planet, game.starmap_zoom);
            (mouse.distance(screen_pos) <= hit_radius)
                .then_some((index, mouse.distance_squared(screen_pos)))
        })
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index)
}

fn planet_matches_starmap_filter(game: &GameState, planet_index: usize, planet: &Planet) -> bool {
    match game.starmap_filter {
        StarmapFilter::All => true,
        StarmapFilter::Scanned => planet_has_surface_scan(planet),
        StarmapFilter::Unscanned => !planet_has_surface_scan(planet),
        StarmapFilter::Destination => game.destination_planet == Some(planet_index),
        StarmapFilter::Resource => {
            let Some(item_id) = selected_starmap_resource_filter(game).map(|item| item.id) else {
                return true;
            };
            planet_has_composition_scan(planet)
                && planet
                    .info
                    .mineables
                    .iter()
                    .any(|mineable| mineable.item.id == item_id)
        }
    }
}

fn starmap_resource_filters(game: &GameState) -> Vec<ItemRef> {
    let mut resources = Vec::new();
    for planet in game
        .planets
        .iter()
        .filter(|planet| planet_is_in_system(planet, &game.current_system_id))
        .filter(|planet| planet_has_composition_scan(planet))
    {
        for mineable in &planet.info.mineables {
            if !resources
                .iter()
                .any(|resource: &ItemRef| resource.id == mineable.item.id)
            {
                resources.push(mineable.item.clone());
            }
        }
    }
    resources.sort_by(|a, b| a.name.cmp(&b.name).then(a.id.cmp(&b.id)));
    resources
}

fn selected_starmap_resource_filter(game: &GameState) -> Option<ItemRef> {
    let resources = starmap_resource_filters(game);
    resources
        .get(game.starmap_resource_filter_index % resources.len().max(1))
        .cloned()
}

fn clicked_known_system_id(mouse: Vec2, game: &GameState) -> Option<String> {
    if !known_systems_panel_rect().contains(mouse) {
        return None;
    }

    known_system_ids(&game.content_registry)
        .iter()
        .enumerate()
        .find_map(|(index, system_id)| {
            known_system_row_rect(index)
                .contains(mouse)
                .then_some(system_id.clone())
        })
}

fn ship_over_planet_index(game: &GameState) -> Option<usize> {
    game.planets
        .iter()
        .enumerate()
        .filter(|(index, _)| planet_in_active_system(game, *index))
        .find_map(|(index, planet)| {
            planet_in_interaction_range(&game.ship, planet).then_some(index)
        })
}

fn ship_over_station_index(game: &GameState) -> Option<usize> {
    game.stations
        .iter()
        .enumerate()
        .filter(|(_, station)| station_is_in_system(station, &game.current_system_id))
        .find_map(|(index, station)| {
            station_in_interaction_range(&game.ship, station).then_some(index)
        })
}

fn ship_over_npc_ship_index(game: &GameState) -> Option<usize> {
    game.npc_ships
        .iter()
        .enumerate()
        .filter(|(_, npc_ship)| npc_ship_is_in_system(npc_ship, &game.current_system_id))
        .filter_map(|(index, npc_ship)| {
            npc_ship_in_interaction_range(&game.ship, npc_ship)
                .then_some((index, npc_ship_surface_distance(&game.ship, npc_ship)))
        })
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index)
}

fn planet_in_active_system(game: &GameState, planet_index: usize) -> bool {
    game.planets
        .get(planet_index)
        .is_some_and(|planet| planet_is_in_system(planet, &game.current_system_id))
}

fn planet_is_in_system(planet: &Planet, system_id: &str) -> bool {
    planet.system == system_id
}

fn station_is_in_system(station: &StationDestination, system_id: &str) -> bool {
    station.system == system_id
}

fn npc_ship_is_in_system(npc_ship: &NpcShip, system_id: &str) -> bool {
    npc_ship.system == system_id && npc_ship.hull.current > 0.0
}

fn planet_interaction_radius(planet: &Planet) -> f32 {
    planet.radius + SHIP_RADIUS + PLANET_INTERACTION_PADDING
}

fn station_interaction_radius(station: &StationDestination) -> f32 {
    station.radius + SHIP_RADIUS + STATION_INTERACTION_PADDING
}

fn npc_ship_interaction_radius(npc_ship: &NpcShip) -> f32 {
    npc_ship.radius + SHIP_RADIUS + NPC_INTERACTION_PADDING
}

fn planet_safe_orbit_radius(planet: &Planet) -> f32 {
    planet.radius + SHIP_RADIUS + PLANET_ORBIT_CLEARANCE
}

fn planet_surface_distance(ship: &Ship, planet: &Planet) -> f32 {
    (ship.position.distance(planet.position) - planet.radius - SHIP_RADIUS).max(0.0)
}

fn planet_in_interaction_range(ship: &Ship, planet: &Planet) -> bool {
    ship.position.distance(planet.position) <= planet_interaction_radius(planet)
}

fn station_surface_distance(ship: &Ship, station: &StationDestination) -> f32 {
    (ship.position.distance(station.position) - station.radius - SHIP_RADIUS).max(0.0)
}

fn npc_ship_surface_distance(ship: &Ship, npc_ship: &NpcShip) -> f32 {
    (ship.position.distance(npc_ship.position) - npc_ship.radius - SHIP_RADIUS).max(0.0)
}

fn station_in_interaction_range(ship: &Ship, station: &StationDestination) -> bool {
    ship.position.distance(station.position) <= station_interaction_radius(station)
}

fn npc_ship_in_interaction_range(ship: &Ship, npc_ship: &NpcShip) -> bool {
    ship.position.distance(npc_ship.position) <= npc_ship_interaction_radius(npc_ship)
}

fn planet_orbit_position(planet: &Planet, ship_position: Vec2) -> Vec2 {
    let offset = ship_position - planet.position;
    let direction = if offset.length_squared() > 0.001 {
        offset.normalize()
    } else {
        vec2(1.0, 0.0)
    };

    planet.position + direction * planet_safe_orbit_radius(planet)
}

fn enter_planet_orbit(game: &mut GameState, planet_index: usize) -> bool {
    let Some(planet) = game.planets.get(planet_index) else {
        return false;
    };
    if !planet_is_in_system(planet, &game.current_system_id)
        || !planet_in_interaction_range(&game.ship, planet)
    {
        return false;
    }

    game.orbiting_planet = Some(planet_index);
    game.selected_planet = Some(planet_index);
    game.destination_planet = None;
    remember_current_system_destination(game);
    update_ship_orbit(game);
    game.save_dirty = true;
    true
}

fn break_planet_orbit(game: &mut GameState) {
    if game.orbiting_planet.take().is_some() {
        game.save_dirty = true;
    }
}

fn update_ship_orbit(game: &mut GameState) {
    let Some(planet_index) = game.orbiting_planet else {
        return;
    };
    let Some(planet) = game.planets.get(planet_index) else {
        break_planet_orbit(game);
        return;
    };
    if !planet_is_in_system(planet, &game.current_system_id) {
        break_planet_orbit(game);
        return;
    }

    game.ship.position = planet_orbit_position(planet, game.ship.position);
    game.ship.velocity = Vec2::ZERO;
    game.ship.angular_velocity = 0.0;
}

fn orbit_break_input_down() -> bool {
    is_key_down(KeyCode::W)
        || is_key_down(KeyCode::S)
        || is_key_down(KeyCode::A)
        || is_key_down(KeyCode::D)
        || is_key_down(KeyCode::Left)
        || is_key_down(KeyCode::Right)
}

fn target_planet<'a>(
    ship: &Ship,
    planets: &'a [Planet],
    current_system_id: &str,
    selected: Option<usize>,
    destination: Option<usize>,
) -> Option<&'a Planet> {
    selected
        .and_then(|index| planets.get(index))
        .filter(|planet| planet_is_in_system(planet, current_system_id))
        .or_else(|| destination.and_then(|index| planets.get(index)))
        .filter(|planet| planet_is_in_system(planet, current_system_id))
        .or_else(|| closest_poi_planet(ship, planets, current_system_id))
}

fn draw_poi_route(
    center: Vec2,
    ship: &Ship,
    planets: &[Planet],
    current_system_id: &str,
    destination: Option<usize>,
    zoom: f32,
) {
    let Some(planet) = target_planet(ship, planets, current_system_id, None, destination) else {
        return;
    };

    let planet_screen_pos = world_to_screen(planet.position, center, ship, zoom);
    let to_planet = planet_screen_pos - center;
    let direction = to_planet.normalize_or_zero();
    let distance = to_planet.length();

    if distance < SHIP_RADIUS + planet.radius + 20.0 {
        return;
    }

    let start = center + direction * (ship_screen_size(zoom) * 0.5 + 64.0);
    let end = planet_screen_pos - direction * (planet.radius * zoom + 10.0);
    draw_dotted_line(
        start,
        end,
        8.0,
        9.0,
        1.0,
        Color::from_rgba(160, 220, 226, 145),
    );
}

fn closest_poi_planet<'a>(
    ship: &Ship,
    planets: &'a [Planet],
    current_system_id: &str,
) -> Option<&'a Planet> {
    planets
        .iter()
        .filter(|planet| planet.is_poi && planet_is_in_system(planet, current_system_id))
        .min_by(|a, b| {
            let distance_a = a.position.distance_squared(ship.position);
            let distance_b = b.position.distance_squared(ship.position);
            distance_a.total_cmp(&distance_b)
        })
}

fn draw_dotted_line(
    start: Vec2,
    end: Vec2,
    dash_length: f32,
    gap_length: f32,
    thickness: f32,
    color: Color,
) {
    let segment = end - start;
    let distance = segment.length();

    if distance <= 0.0 {
        return;
    }

    let direction = segment / distance;
    let step = dash_length + gap_length;
    let mut traveled = 0.0;

    while traveled < distance {
        let dash_start = start + direction * traveled;
        let dash_end = start + direction * (traveled + dash_length).min(distance);
        draw_line(
            dash_start.x,
            dash_start.y,
            dash_end.x,
            dash_end.y,
            thickness,
            color,
        );
        traveled += step;
    }
}

fn draw_planet(center: Vec2, ship: &Ship, planet: &Planet, zoom: f32) {
    let screen_pos = world_to_screen(planet.position, center, ship, zoom);
    let interaction_radius = planet_interaction_radius(planet) * zoom;
    let planet_radius = planet.radius * zoom;
    let in_range = planet_in_interaction_range(ship, planet);

    if screen_pos.x < -interaction_radius
        || screen_pos.x > screen_width() + interaction_radius
        || screen_pos.y < -interaction_radius
        || screen_pos.y > screen_height() + interaction_radius
    {
        return;
    }

    draw_circle_lines(
        screen_pos.x,
        screen_pos.y,
        interaction_radius,
        if in_range { 2.0 } else { 1.0 },
        if in_range {
            Color::from_rgba(150, 221, 226, 175)
        } else {
            Color::from_rgba(95, 137, 155, 80)
        },
    );

    if let Some(texture) = &planet.texture {
        draw_texture_ex(
            texture,
            screen_pos.x - planet_radius,
            screen_pos.y - planet_radius,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(planet_radius * 2.0, planet_radius * 2.0)),
                ..Default::default()
            },
        );
    } else {
        draw_circle(
            screen_pos.x,
            screen_pos.y,
            planet_radius,
            Color::from_rgba(65, 136, 154, 255),
        );
        draw_circle(
            screen_pos.x - planet_radius * 0.28,
            screen_pos.y - planet_radius * 0.24,
            planet_radius * 0.38,
            Color::from_rgba(104, 176, 160, 185),
        );
    }

    draw_circle_lines(
        screen_pos.x,
        screen_pos.y,
        planet_radius,
        2.0,
        Color::from_rgba(196, 231, 226, 160),
    );

    if in_range {
        let text = "IN RANGE";
        let measure = measure_text(text, None, 16, 1.0);
        draw_text(
            text,
            screen_pos.x - measure.width * 0.5,
            screen_pos.y - interaction_radius - 12.0,
            16.0,
            Color::from_rgba(150, 221, 226, 230),
        );
    }
}

fn draw_station(center: Vec2, ship: &Ship, station: &StationDestination, zoom: f32) {
    let screen_pos = world_to_screen(station.position, center, ship, zoom);
    let interaction_radius = station_interaction_radius(station) * zoom;
    let station_radius = station.radius * zoom;
    let in_range = station_in_interaction_range(ship, station);

    if screen_pos.x < -interaction_radius
        || screen_pos.x > screen_width() + interaction_radius
        || screen_pos.y < -interaction_radius
        || screen_pos.y > screen_height() + interaction_radius
    {
        return;
    }

    draw_circle_lines(
        screen_pos.x,
        screen_pos.y,
        interaction_radius,
        if in_range { 2.0 } else { 1.0 },
        if in_range {
            Color::from_rgba(150, 221, 226, 175)
        } else {
            Color::from_rgba(95, 137, 155, 80)
        },
    );

    if let Some(texture) = &station.texture {
        draw_texture_ex(
            texture,
            screen_pos.x - station_radius,
            screen_pos.y - station_radius,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(station_radius * 2.0, station_radius * 2.0)),
                ..Default::default()
            },
        );
    } else {
        draw_station_icon(screen_pos, station_radius, &station.icon);
    }

    let label_y = screen_pos.y + station_radius + 22.0;
    let label = fit_debug_text(&station.name, 180.0, 16);
    let measure = measure_text(&label, None, 16, 1.0);
    draw_text(
        &label,
        screen_pos.x - measure.width * 0.5,
        label_y,
        16.0,
        if in_range {
            Color::from_rgba(150, 221, 226, 240)
        } else {
            Color::from_rgba(205, 226, 230, 210)
        },
    );
}

fn draw_npc_ship(
    content_registry: &content::ContentRegistry,
    center: Vec2,
    ship: &Ship,
    npc_ship: &NpcShip,
    zoom: f32,
) {
    let screen_pos = world_to_screen(npc_ship.position, center, ship, zoom);
    let size = (npc_ship.radius * 2.0 * zoom).clamp(22.0, 72.0);
    let cull_padding = size + 100.0;

    if screen_pos.x < -cull_padding
        || screen_pos.x > screen_width() + cull_padding
        || screen_pos.y < -cull_padding
        || screen_pos.y > screen_height() + cull_padding
    {
        return;
    }

    let color = npc_ship
        .faction
        .as_deref()
        .map(|faction| faction_color(content_registry, faction, 235))
        .unwrap_or_else(|| npc_ship_role_color(&npc_ship.role));
    draw_circle_lines(
        screen_pos.x,
        screen_pos.y,
        size * 0.62,
        1.0,
        Color { a: 0.42, ..color },
    );
    if let Some(texture) = &npc_ship.texture {
        draw_texture_ex(
            texture,
            screen_pos.x - size * 0.5,
            screen_pos.y - size * 0.5,
            Color { a: 0.92, ..WHITE },
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                rotation: npc_ship.angle + std::f32::consts::FRAC_PI_2,
                pivot: Some(screen_pos),
                ..Default::default()
            },
        );
    } else {
        draw_ship_model(
            screen_pos,
            size * 0.22,
            false,
            npc_ship.angle + std::f32::consts::FRAC_PI_2,
        );
    }

    let status = format!(
        "{}  {}  {}  {}",
        npc_ship.name,
        local_content_id(&npc_ship.id),
        npc_ship.archetype,
        npc_ship
            .faction
            .as_deref()
            .map(|faction| faction_name(content_registry, faction))
            .unwrap_or(npc_ship.role.as_str())
    );
    draw_text(
        &fit_debug_text(&status, 230.0, 14),
        screen_pos.x + size * 0.44,
        screen_pos.y + 4.0,
        14.0,
        color,
    );
    let cargo_units = npc_ship
        .cargo_defaults
        .iter()
        .map(|stack| stack.count)
        .sum::<u32>();
    let metadata = format!(
        "{}  cargo {}/{}  tags {}  H{:.0} S{:.0} E{:.0}  loadout {}/{}",
        npc_ship.behavior.label(),
        cargo_units,
        format_mass(npc_ship.cargo_capacity),
        npc_ship.behavior_tags.len(),
        npc_ship.hull.max,
        npc_ship.shields.max,
        npc_ship.energy.max,
        npc_ship.shield_slots.len(),
        npc_ship.weapon_slots.len()
    );
    draw_text(
        &fit_debug_text(&metadata, 230.0, 12),
        screen_pos.x + size * 0.44,
        screen_pos.y + 20.0,
        12.0,
        Color { a: 0.78, ..color },
    );
    if !npc_ship.summary.is_empty() {
        draw_text(
            &fit_debug_text(&npc_ship.summary, 230.0, 12),
            screen_pos.x + size * 0.44,
            screen_pos.y + 35.0,
            12.0,
            Color { a: 0.62, ..color },
        );
    }
}

fn npc_ship_role_color(role: &str) -> Color {
    match role {
        "hostile" => Color::from_rgba(226, 104, 96, 235),
        "patrol" | "security" => Color::from_rgba(150, 221, 226, 235),
        "hauler" | "trader" => Color::from_rgba(226, 190, 150, 235),
        _ => Color::from_rgba(205, 226, 230, 220),
    }
}

fn draw_defense_threat(center: Vec2, ship: &Ship, threat: &DefenseThreat, zoom: f32) {
    let screen_pos = world_to_screen(threat.position, center, ship, zoom);
    let size = (threat.radius * zoom).clamp(10.0, 24.0);
    let cull_padding = size + 80.0;

    if screen_pos.x < -cull_padding
        || screen_pos.x > screen_width() + cull_padding
        || screen_pos.y < -cull_padding
        || screen_pos.y > screen_height() + cull_padding
    {
        return;
    }

    let color = match threat.disposition {
        ThreatDisposition::Hostile => Color::from_rgba(226, 104, 96, 245),
        ThreatDisposition::Neutral => Color::from_rgba(150, 221, 226, 205),
        ThreatDisposition::Owned => Color::from_rgba(113, 235, 138, 205),
        ThreatDisposition::Environmental => Color::from_rgba(226, 190, 150, 205),
    };
    draw_poly(screen_pos.x, screen_pos.y, 4, size, 45.0, color);
    draw_circle_lines(
        screen_pos.x,
        screen_pos.y,
        size + 5.0,
        1.0,
        Color { a: 0.5, ..color },
    );
    let label = format!(
        "{} {} {}",
        threat.name,
        local_content_id(&threat.id),
        threat.disposition.label()
    );
    draw_text(
        &fit_debug_text(&label, 180.0, 14),
        screen_pos.x + size + 8.0,
        screen_pos.y + 5.0,
        14.0,
        color,
    );
}

fn draw_weapon_fire_event(center: Vec2, ship: &Ship, event: &WeaponFireEvent, zoom: f32) {
    let from = world_to_screen(event.from, center, ship, zoom);
    let to = world_to_screen(event.to, center, ship, zoom);
    let alpha = (event.timer / WEAPON_FIRE_EVENT_SECONDS).clamp(0.0, 1.0);
    draw_line(
        from.x,
        from.y,
        to.x,
        to.y,
        2.5,
        Color::new(0.95, 0.34, 0.28, alpha),
    );
    draw_circle(
        to.x,
        to.y,
        5.0 + alpha * 5.0,
        Color::new(1.0, 0.72, 0.35, alpha),
    );
}

fn draw_station_icon(center: Vec2, radius: f32, icon: &str) {
    let hull = Color::from_rgba(76, 126, 139, 245);
    let bright = Color::from_rgba(196, 231, 226, 220);
    let warm = Color::from_rgba(226, 190, 150, 210);
    draw_circle(
        center.x,
        center.y,
        radius * 0.56,
        Color::from_rgba(10, 24, 31, 245),
    );
    draw_circle_lines(center.x, center.y, radius * 0.62, 2.0, bright);
    if icon == "ring" {
        draw_circle_lines(center.x, center.y, radius * 0.9, 4.0, hull);
        draw_line(
            center.x - radius * 0.95,
            center.y,
            center.x + radius * 0.95,
            center.y,
            3.0,
            hull,
        );
        draw_rectangle(
            center.x - radius * 0.18,
            center.y - radius * 0.72,
            radius * 0.36,
            radius * 1.44,
            warm,
        );
    } else {
        draw_rectangle(
            center.x - radius * 0.7,
            center.y - radius * 0.18,
            radius * 1.4,
            radius * 0.36,
            hull,
        );
        draw_rectangle(
            center.x - radius * 0.18,
            center.y - radius * 0.7,
            radius * 0.36,
            radius * 1.4,
            warm,
        );
    }
}

fn draw_poi_indicator(
    center: Vec2,
    ship: &Ship,
    planets: &[Planet],
    current_system_id: &str,
    destination: Option<usize>,
    zoom: f32,
) {
    let Some(planet) = target_planet(ship, planets, current_system_id, None, destination) else {
        return;
    };

    let screen_pos = world_to_screen(planet.position, center, ship, zoom);
    let margin = 44.0;
    if screen_pos.x >= margin
        && screen_pos.x <= screen_width() - margin
        && screen_pos.y >= margin
        && screen_pos.y <= screen_height() - margin
    {
        return;
    }

    let direction = (screen_pos - center).normalize_or_zero();
    if direction.length_squared() == 0.0 {
        return;
    }

    let arrow = vec2(
        screen_pos.x.clamp(margin, screen_width() - margin),
        screen_pos.y.clamp(margin, screen_height() - margin),
    );
    let side = vec2(-direction.y, direction.x);
    let color = if planet_in_interaction_range(ship, planet) {
        Color::from_rgba(150, 221, 226, 235)
    } else {
        Color::from_rgba(226, 190, 150, 225)
    };

    draw_triangle(
        arrow + direction * 18.0,
        arrow - direction * 14.0 + side * 10.0,
        arrow - direction * 14.0 - side * 10.0,
        color,
    );

    let distance = planet_surface_distance(ship, planet);
    let label = format!("{distance:.0}u");
    let measure = measure_text(&label, None, 16, 1.0);
    draw_text(
        &label,
        arrow.x - measure.width * 0.5,
        arrow.y + 34.0,
        16.0,
        Color::from_rgba(205, 226, 230, 235),
    );
}

fn draw_wrapped_text(
    text: &str,
    x: f32,
    y: f32,
    max_width: f32,
    font_size: u16,
    color: Color,
) -> f32 {
    let mut line = String::new();
    let mut line_y = y;

    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_string()
        } else {
            format!("{line} {word}")
        };

        if measure_text(&candidate, None, font_size, 1.0).width <= max_width {
            line = candidate;
        } else {
            draw_text(&line, x, line_y, font_size as f32, color);
            line = word.to_string();
            line_y += 22.0;
        }
    }

    if !line.is_empty() {
        draw_text(&line, x, line_y, font_size as f32, color);
    }

    line_y + 22.0
}

fn draw_star_trail(screen_pos: Vec2, star: &Star, layer: &StarLayer, screen_velocity: Vec2) {
    if screen_pos.x < -80.0
        || screen_pos.x > screen_width() + 80.0
        || screen_pos.y < -80.0
        || screen_pos.y > screen_height() + 80.0
    {
        return;
    }

    let base = Color::new(
        layer.color.r * star.brightness,
        layer.color.g * star.brightness,
        layer.color.b * star.brightness,
        layer.color.a,
    );

    let movement = screen_velocity.normalize_or_zero();
    let speed_strength = (screen_velocity.length() * 0.045 * layer.trail_scale).min(34.0);
    let trail = movement * speed_strength;

    for i in 1..=5 {
        let t = i as f32 / 5.0;
        let p = screen_pos - trail * t;
        let alpha = (1.0 - t) * 0.22 * star.brightness;
        draw_circle(
            p.x,
            p.y,
            star.size * (1.0 - t * 0.3),
            Color { a: alpha, ..base },
        );
    }

    draw_circle(screen_pos.x, screen_pos.y, star.size, base);
}

fn draw_ship(center: Vec2, ship: &Ship, texture: Option<&Texture2D>, zoom: f32) {
    let thrusting = is_key_down(KeyCode::W) || is_key_down(KeyCode::S);
    draw_ship_sprite(
        center,
        texture,
        ship_screen_size(zoom),
        thrusting,
        ship_screen_rotation(ship),
    );
}

fn ship_screen_size(zoom: f32) -> f32 {
    SHIP_SPRITE_SIZE * zoom
}

fn ship_screen_rotation(ship: &Ship) -> f32 {
    ship.angle - std::f32::consts::FRAC_PI_2
}

fn draw_ship_sprite(
    center: Vec2,
    texture: Option<&Texture2D>,
    size: f32,
    thrusting: bool,
    rotation: f32,
) {
    if let Some(texture) = texture {
        if thrusting {
            let flame = center + rotate(vec2(0.0, size * 0.48), rotation);
            draw_triangle(
                center + rotate(vec2(-size * 0.09, size * 0.32), rotation),
                center + rotate(vec2(size * 0.09, size * 0.32), rotation),
                flame
                    + rotate(
                        vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(5.0, 13.0)),
                        rotation,
                    ),
                ORANGE,
            );
        }

        draw_texture_ex(
            texture,
            center.x - size * 0.5,
            center.y - size * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                rotation,
                pivot: Some(center),
                ..Default::default()
            },
        );
    } else {
        draw_ship_model(center, size / 4.0, thrusting, rotation);
    }
}

fn draw_ship_model(center: Vec2, radius: f32, thrusting: bool, rotation: f32) {
    let nose = center + rotate(vec2(0.0, -radius * 1.35), rotation);
    let left = center + rotate(vec2(-radius * 0.82, radius), rotation);
    let right = center + rotate(vec2(radius * 0.82, radius), rotation);
    let wing_left = center + rotate(vec2(-radius * 1.4, radius * 0.45), rotation);
    let wing_right = center + rotate(vec2(radius * 1.4, radius * 0.45), rotation);
    let body_center = center + rotate(vec2(0.0, radius * 0.27), rotation);

    if thrusting {
        let flame = center + rotate(vec2(0.0, radius * 1.42), rotation);
        draw_triangle(
            center + rotate(vec2(-radius * 0.32, radius * 0.78), rotation),
            center + rotate(vec2(radius * 0.32, radius * 0.78), rotation),
            flame
                + rotate(
                    vec2(rand::gen_range(-3.0, 3.0), rand::gen_range(6.0, 18.0)),
                    rotation,
                ),
            ORANGE,
        );
    }

    draw_triangle(left, nose, right, Color::from_rgba(192, 221, 232, 255));
    draw_triangle(
        wing_left,
        left,
        body_center,
        Color::from_rgba(66, 176, 205, 255),
    );
    draw_triangle(
        right,
        wing_right,
        body_center,
        Color::from_rgba(66, 176, 205, 255),
    );
    let cockpit = center + rotate(vec2(0.0, -radius * 0.32), rotation);
    draw_circle(
        cockpit.x,
        cockpit.y,
        radius * 0.23,
        Color::from_rgba(12, 36, 54, 255),
    );
}

fn draw_ship_status_arcs(center: Vec2, ship: &Ship, zoom: f32) {
    let shield = Color::from_rgba(72, 185, 255, 152);
    let energy = Color::from_rgba(255, 206, 88, 148);
    let hull = Color::from_rgba(113, 235, 138, 136);
    let scale = zoom.clamp(0.75, 1.25);

    draw_status_arc(
        center,
        64.8 * scale,
        4.0,
        -220.0,
        80.0,
        ship.systems.shields.fraction(),
        shield,
    );
    draw_status_arc(
        center,
        74.4 * scale,
        4.0,
        -40.0,
        220.0,
        ship.systems.energy.fraction(),
        energy,
    );
    draw_status_arc(
        center,
        84.0 * scale,
        3.0,
        125.0,
        235.0,
        ship.systems.hull.fraction(),
        hull,
    );
}

fn draw_status_arc(
    center: Vec2,
    radius: f32,
    thickness: f32,
    start_degrees: f32,
    end_degrees: f32,
    fraction: f32,
    color: Color,
) {
    let start = start_degrees.to_radians();
    let end = end_degrees.to_radians();
    let sweep = end - start;
    let filled_end = start + sweep * fraction.clamp(0.0, 1.0);

    draw_arc_line(
        center,
        radius,
        thickness + 4.0,
        start,
        end,
        Color { a: 0.056, ..color },
    );
    draw_arc_line(
        center,
        radius,
        thickness,
        start,
        end,
        Color { a: 0.144, ..color },
    );
    draw_arc_line(
        center,
        radius,
        thickness + 2.0,
        start,
        filled_end,
        Color { a: 0.192, ..color },
    );
    draw_arc_line(center, radius, thickness, start, filled_end, color);
}

fn draw_arc_line(center: Vec2, radius: f32, thickness: f32, start: f32, end: f32, color: Color) {
    if (end - start).abs() < 0.001 {
        return;
    }

    let segments = (ARC_SEGMENTS as f32 * (end - start).abs() / std::f32::consts::TAU)
        .ceil()
        .max(3.0) as usize;
    let mut previous = center + vec2(start.cos(), start.sin()) * radius;

    for step in 1..=segments {
        let t = step as f32 / segments as f32;
        let angle = start + (end - start) * t;
        let current = center + vec2(angle.cos(), angle.sin()) * radius;
        draw_line(
            previous.x, previous.y, current.x, current.y, thickness, color,
        );
        previous = current;
    }
}

fn draw_dashed_ring(
    center: Vec2,
    radius: f32,
    dash_count: usize,
    dash_fraction: f32,
    phase: f32,
    thickness: f32,
    color: Color,
) {
    if dash_count == 0 {
        return;
    }

    let step = std::f32::consts::TAU / dash_count as f32;
    let dash_length = step * dash_fraction.clamp(0.05, 0.95);

    for dash in 0..dash_count {
        let start = phase + dash as f32 * step;
        draw_arc_line(center, radius, thickness, start, start + dash_length, color);
    }
}

fn draw_inventory_hint(
    inventory_open: bool,
    map_open: bool,
    skills_open: bool,
    upgrades_open: bool,
    content_open: bool,
    save_visible: bool,
) {
    let mut text = if map_open {
        "M close map"
    } else if content_open {
        "C close content"
    } else if upgrades_open {
        "Esc close upgrades"
    } else if skills_open {
        "K close skills"
    } else if inventory_open {
        "E/Tab close inventory   M map   K skills   C content   Esc close"
    } else {
        "E/Tab inventory   M map   K skills   C content   Esc menu"
    }
    .to_string();
    if save_visible {
        text.push_str("   saved");
    }
    let measure = measure_text(&text, None, 18, 1.0);
    let x = screen_width() - measure.width - 30.0;
    let y = 22.0;
    draw_rectangle(
        x - 12.0,
        y - 14.0,
        measure.width + 24.0,
        28.0,
        Color::from_rgba(5, 10, 16, 165),
    );
    draw_rectangle_lines(
        x - 12.0,
        y - 14.0,
        measure.width + 24.0,
        28.0,
        1.0,
        Color::from_rgba(95, 137, 155, 105),
    );
    draw_text(
        &text,
        x,
        y + 6.0,
        18.0,
        Color::from_rgba(205, 226, 230, 255),
    );
}

fn draw_save_confirmation(timer: f32, manual: bool) {
    let max_timer = if manual { 2.6 } else { 1.6 };
    let opacity = (timer / max_timer).clamp(0.0, 1.0);
    let label = if manual {
        "Manual save complete"
    } else {
        "Autosaved"
    };
    let detail = if manual {
        "Game state written to disk"
    } else {
        "Game saved"
    };
    let font_size = if manual { 24_u16 } else { 18_u16 };
    let detail_size = if manual { 16_u16 } else { 14_u16 };
    let label_measure = measure_text(label, None, font_size, 1.0);
    let detail_measure = measure_text(detail, None, detail_size, 1.0);
    let width = label_measure.width.max(detail_measure.width) + 54.0;
    let height = if manual { 76.0 } else { 52.0 };
    let x = (screen_width() - width) * 0.5;
    let y = if manual { 38.0 } else { 28.0 };
    let alpha = (opacity * 255.0).round() as u8;
    let border_alpha = (opacity * 230.0).round() as u8;
    let text_alpha = (opacity * 255.0).round() as u8;

    draw_rectangle(x, y, width, height, Color::from_rgba(3, 10, 14, alpha));
    draw_rectangle_lines(
        x,
        y,
        width,
        height,
        1.0,
        Color::from_rgba(150, 221, 226, border_alpha),
    );
    draw_text(
        label,
        x + (width - label_measure.width) * 0.5,
        y + 31.0,
        font_size as f32,
        Color::from_rgba(235, 242, 226, text_alpha),
    );
    if manual {
        draw_text(
            detail,
            x + (width - detail_measure.width) * 0.5,
            y + 57.0,
            detail_size as f32,
            Color::from_rgba(150, 221, 226, text_alpha),
        );
    }
}

fn escape_dialog_rect() -> Rect {
    let width = 660.0;
    let height = 190.0;
    Rect::new(
        (screen_width() - width) * 0.5,
        (screen_height() - height) * 0.5,
        width,
        height,
    )
}

fn escape_dialog_resume_button_rect() -> Rect {
    let panel = escape_dialog_rect();
    Rect::new(panel.x + 22.0, panel.y + panel.h - 58.0, 112.0, 36.0)
}

fn escape_dialog_save_button_rect() -> Rect {
    let panel = escape_dialog_rect();
    Rect::new(panel.x + 150.0, panel.y + panel.h - 58.0, 120.0, 36.0)
}

fn escape_dialog_title_button_rect() -> Rect {
    let panel = escape_dialog_rect();
    Rect::new(panel.x + 286.0, panel.y + panel.h - 58.0, 132.0, 36.0)
}

fn escape_dialog_quit_button_rect() -> Rect {
    let panel = escape_dialog_rect();
    Rect::new(panel.x + 434.0, panel.y + panel.h - 58.0, 142.0, 36.0)
}

fn draw_escape_dialog(game: &GameState, logo: Option<&Texture2D>) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(0, 0, 0, 145),
    );

    let panel = escape_dialog_rect();
    let text = Color::from_rgba(235, 242, 226, 255);
    let detail = Color::from_rgba(178, 197, 203, 255);
    let accent = Color::from_rgba(150, 221, 226, 255);
    let warning = Color::from_rgba(226, 190, 150, 255);

    draw_rectangle(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        Color::from_rgba(3, 8, 13, 255),
    );
    draw_rectangle_lines(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        1.0,
        Color::from_rgba(112, 151, 163, 220),
    );
    draw_text("Game Paused", panel.x + 22.0, panel.y + 36.0, 25.0, text);
    if let Some(logo) = logo {
        draw_texture_contain(
            logo,
            Rect::new(panel.x + panel.w - 174.0, panel.y + 18.0, 138.0, 54.0),
            0.9,
        );
    }
    draw_text(
        if game.save_dirty {
            "Unsaved changes are queued for autosave."
        } else {
            "Current game state is saved."
        },
        panel.x + 22.0,
        panel.y + 72.0,
        18.0,
        if game.save_dirty { warning } else { detail },
    );

    draw_escape_dialog_button(escape_dialog_resume_button_rect(), "Resume", accent);
    draw_escape_dialog_button(escape_dialog_save_button_rect(), "Save Now", accent);
    draw_escape_dialog_button(escape_dialog_title_button_rect(), "Title Menu", accent);
    draw_escape_dialog_button(escape_dialog_quit_button_rect(), "Quit Desktop", warning);

    draw_escape_dialog_tooltip();
}

fn draw_escape_dialog_button(rect: Rect, label: &str, color: Color) {
    let mouse = vec2(mouse_position().0, mouse_position().1);
    let hovered = rect.contains(mouse);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if hovered {
            Color::from_rgba(28, 62, 68, 245)
        } else {
            Color::from_rgba(13, 32, 40, 235)
        },
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, color);
    let measure = measure_text(label, None, 17, 1.0);
    draw_text(
        label,
        rect.x + (rect.w - measure.width) * 0.5,
        rect.y + 23.0,
        17.0,
        color,
    );
}

fn draw_escape_dialog_tooltip() {
    let mouse = mouse_vec2();
    let tooltip = if escape_dialog_resume_button_rect().contains(mouse) {
        Some((
            "Resume",
            "Esc",
            "Close the pause menu and return to gameplay.",
        ))
    } else if escape_dialog_save_button_rect().contains(mouse) {
        Some((
            "Save Now",
            "S",
            "Write the current run to disk without leaving gameplay.",
        ))
    } else if escape_dialog_title_button_rect().contains(mouse) {
        Some((
            "Title Menu",
            "T",
            "Save the current run, leave gameplay, and return to the title menu.",
        ))
    } else if escape_dialog_quit_button_rect().contains(mouse) {
        Some((
            "Quit Desktop",
            "Q",
            "Save the current run, then close Some Frontier.",
        ))
    } else {
        None
    };

    let Some((title, shortcut, detail)) = tooltip else {
        return;
    };
    draw_ui_tooltip(title, shortcut, detail, mouse);
}

fn draw_ui_tooltip(title: &str, shortcut: &str, detail: &str, mouse: Vec2) {
    let width = 330.0;
    let height = 118.0;
    let x = (mouse.x + 18.0)
        .min(screen_width() - width - 18.0)
        .max(18.0);
    let y = (mouse.y + 18.0)
        .min(screen_height() - height - 18.0)
        .max(18.0);
    let panel = Color::from_rgba(2, 6, 10, 255);
    let border = Color::from_rgba(112, 151, 163, 170);
    let label = Color::from_rgba(126, 156, 164, 220);
    let text = Color::from_rgba(205, 226, 230, 255);
    let active = Color::from_rgba(150, 221, 226, 255);

    draw_rectangle(x, y, width, height, panel);
    draw_rectangle_lines(x, y, width, height, 1.0, border);
    draw_text(title, x + 14.0, y + 28.0, 21.0, text);
    draw_text("Shortcut", x + 14.0, y + 54.0, 15.0, label);
    draw_text(shortcut, x + 92.0, y + 54.0, 17.0, active);
    draw_wrapped_text(detail, x + 14.0, y + 80.0, width - 28.0, 16, text);
}

fn draw_starmap_overlay(game: &GameState) {
    let (x, y, width, height) = starmap_panel_rect();

    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(2, 6, 12, 230),
    );
    draw_rectangle_lines(
        x,
        y,
        width,
        height,
        1.0,
        Color::from_rgba(112, 151, 163, 150),
    );

    let camera = starmap_camera(x, y, width, height);
    set_camera(&camera);

    draw_grid(
        32,
        24.0,
        Color::from_rgba(105, 184, 198, 120),
        Color::from_rgba(50, 92, 104, 70),
    );
    draw_starmap_orbit_guides(game);

    draw_starmap_route_3d(game);
    for (index, planet) in game
        .planets
        .iter()
        .enumerate()
        .filter(|(index, _)| planet_in_active_system(game, *index))
        .filter(|(index, planet)| planet_matches_starmap_filter(game, *index, planet))
    {
        draw_starmap_planet_3d(
            &game.ship,
            planet,
            game.destination_planet == Some(index),
            game.starmap_zoom,
            game.starmap_pan,
        );
    }
    draw_starmap_ship_3d(&game.ship, game.starmap_zoom, game.starmap_pan);

    set_default_camera();

    draw_rectangle(x, y, width, height, Color::from_rgba(4, 12, 18, 24));
    draw_starmap_planet_markers(game, &camera);
    draw_text(
        "3D Starmap",
        x + 24.0,
        y + 40.0,
        28.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_text(
        "Wheel zoom   Right-drag pan   Home center   Click destination   F/R filters   Esc close",
        x + width - 720.0,
        y + 36.0,
        18.0,
        Color::from_rgba(126, 156, 164, 220),
    );
    draw_starmap_filter_readout(game, x + 24.0, y + 66.0);
    draw_starmap_readout(
        x + 24.0,
        y + height - 82.0,
        &game.ship,
        &game.planets,
        &game.current_system_id,
        game.destination_planet,
    );
    draw_known_systems_panel(game);
}

fn starmap_panel_rect() -> (f32, f32, f32, f32) {
    let width = screen_width() * STARMAP_PANEL_SCREEN_FRACTION;
    let height = screen_height() * STARMAP_PANEL_SCREEN_FRACTION;
    let x = (screen_width() - width) * 0.5;
    let y = (screen_height() - height) * 0.5;
    (x, y, width, height)
}

fn known_systems_panel_rect() -> Rect {
    let (x, y, width, height) = starmap_panel_rect();
    Rect::new(
        x + width - KNOWN_SYSTEMS_PANEL_WIDTH - 18.0,
        y + 66.0,
        KNOWN_SYSTEMS_PANEL_WIDTH,
        height - 150.0,
    )
}

fn known_system_row_rect(index: usize) -> Rect {
    let panel = known_systems_panel_rect();
    Rect::new(
        panel.x + 10.0,
        panel.y + 38.0 + index as f32 * (KNOWN_SYSTEM_ROW_HEIGHT + 8.0),
        panel.w - 20.0,
        KNOWN_SYSTEM_ROW_HEIGHT,
    )
}

fn starmap_camera(x: f32, y: f32, width: f32, height: f32) -> Camera3D {
    let orbit = get_time() as f32 * 0.08;
    Camera3D {
        position: vec3(orbit.sin() * 260.0, 430.0, 760.0 + orbit.cos() * 160.0),
        target: vec3(0.0, 18.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        fovy: 46.0_f32.to_radians(),
        viewport: Some((x as i32, y as i32, width as i32, height as i32)),
        z_far: 3000.0,
        ..Default::default()
    }
}

fn draw_starmap_planet_markers(game: &GameState, camera: &Camera3D) {
    for (index, planet) in game
        .planets
        .iter()
        .enumerate()
        .filter(|(index, _)| planet_in_active_system(game, *index))
        .filter(|(index, planet)| planet_matches_starmap_filter(game, *index, planet))
    {
        let screen_pos = starmap_planet_screen_pos(
            &game.ship,
            planet,
            camera,
            game.starmap_zoom,
            game.starmap_pan,
        );
        let radius = starmap_planet_hit_radius(planet, game.starmap_zoom);
        let is_destination = game.destination_planet == Some(index);
        let color = if is_destination {
            Color::from_rgba(113, 235, 138, 220)
        } else {
            Color::from_rgba(150, 221, 226, 130)
        };
        let phase = get_time() as f32 * if is_destination { 0.36 } else { 0.21 };

        draw_dashed_ring(screen_pos, radius, 18, 0.46, phase, 1.0, color);
        draw_circle(
            screen_pos.x,
            screen_pos.y,
            3.0,
            if is_destination {
                Color::from_rgba(113, 235, 138, 235)
            } else {
                Color::from_rgba(235, 242, 226, 150)
            },
        );
    }
}

fn draw_starmap_route_3d(game: &GameState) {
    let Some(planet) = target_planet(
        &game.ship,
        &game.planets,
        &game.current_system_id,
        None,
        game.destination_planet,
    ) else {
        return;
    };

    let ship_pos = starmap_ship_position(game.starmap_pan);
    let planet_pos = world_to_starmap_3d(
        planet.position - game.ship.position,
        game.starmap_zoom,
        game.starmap_pan,
    );
    let segments = 28;
    for i in 0..segments {
        if i % 2 == 1 {
            continue;
        }
        let start_t = i as f32 / segments as f32;
        let end_t = (i + 1) as f32 / segments as f32;
        draw_line_3d(
            ship_pos.lerp(planet_pos, start_t),
            ship_pos.lerp(planet_pos, end_t),
            Color::from_rgba(150, 221, 226, 190),
        );
    }
}

fn draw_starmap_planet_3d(
    ship: &Ship,
    planet: &Planet,
    is_destination: bool,
    zoom: f32,
    pan: Vec2,
) {
    let relative = planet.position - ship.position;
    let position = world_to_starmap_3d(relative, zoom, pan);
    let floor = vec3(position.x, 0.0, position.z);
    let radius = starmap_planet_radius(planet, zoom) * 0.64;
    let color = if is_destination {
        Color::from_rgba(82, 194, 214, 235)
    } else {
        Color::from_rgba(65, 136, 154, 220)
    };

    draw_line_3d(floor, position, Color::from_rgba(94, 137, 151, 110));
    draw_starmap_ring_at_3d(floor, radius * 2.2, Color::from_rgba(75, 126, 139, 110));
    draw_sphere(position, radius, planet.texture.as_ref(), color);
    draw_sphere_wires(
        position,
        radius + 1.6,
        None,
        Color::from_rgba(205, 226, 230, if is_destination { 205 } else { 95 }),
    );

    if is_destination {
        draw_starmap_ring_at_3d(position, radius + 8.0, Color::from_rgba(235, 242, 226, 170));
    }
}

fn draw_starmap_ship_3d(ship: &Ship, zoom: f32, pan: Vec2) {
    let origin = starmap_ship_position(pan);
    let ship_scale = zoom.clamp(0.75, 1.6);
    let nose = origin + vec3(0.0, 32.0, -18.0) * ship_scale;
    let tail = origin + vec3(0.0, 14.0, 18.0) * ship_scale;
    let left = origin + vec3(-14.0, 12.0, 10.0) * ship_scale;
    let right = origin + vec3(14.0, 12.0, 10.0) * ship_scale;

    draw_cube(
        origin + vec3(0.0, 15.0, 3.0) * ship_scale,
        vec3(13.0, 8.0, 32.0) * ship_scale,
        None,
        Color::from_rgba(186, 211, 216, 220),
    );
    draw_line_3d(nose, left, Color::from_rgba(235, 242, 226, 235));
    draw_line_3d(nose, right, Color::from_rgba(235, 242, 226, 235));
    draw_line_3d(left, tail, Color::from_rgba(150, 221, 226, 220));
    draw_line_3d(right, tail, Color::from_rgba(150, 221, 226, 220));
    draw_starmap_ring_at_3d(
        starmap_floor_position(pan),
        (28.0 + ship.velocity.length().min(900.0) * 0.01) * ship_scale,
        Color::from_rgba(150, 221, 226, 135),
    );
}

fn draw_starmap_orbit_guides(game: &GameState) {
    for guide in active_orbit_guides(&game.planets, &game.current_system_id) {
        let center = world_to_starmap_floor(
            guide.center - game.ship.position,
            game.starmap_zoom,
            game.starmap_pan,
        );
        let semi_major = guide.radius * STARMAP_SCALE * game.starmap_zoom;
        let semi_minor = guide.semi_minor * STARMAP_SCALE * game.starmap_zoom;
        draw_starmap_ellipse_at_3d(
            center,
            semi_major,
            semi_minor,
            guide.axis_rotation,
            Color::from_rgba(92, 158, 174, 56),
        );
    }
}

fn draw_starmap_ring_at_3d(center: Vec3, radius: f32, color: Color) {
    draw_starmap_ellipse_at_3d(center, radius, radius, 0.0, color);
}

fn draw_starmap_ellipse_at_3d(
    center: Vec3,
    semi_major: f32,
    semi_minor: f32,
    rotation: f32,
    color: Color,
) {
    let segments = 96;
    let mut previous = starmap_ellipse_point(center, semi_major, semi_minor, rotation, 0.0);

    for step in 1..=segments {
        let angle = step as f32 / segments as f32 * std::f32::consts::TAU;
        let current = starmap_ellipse_point(center, semi_major, semi_minor, rotation, angle);
        draw_line_3d(previous, current, color);
        previous = current;
    }
}

fn starmap_ellipse_point(
    center: Vec3,
    semi_major: f32,
    semi_minor: f32,
    rotation: f32,
    angle: f32,
) -> Vec3 {
    let local = vec2(angle.cos() * semi_major, angle.sin() * semi_minor);
    let rotated = rotate(local, rotation);
    center + vec3(rotated.x, 0.0, rotated.y)
}

fn world_to_starmap_3d(relative: Vec2, zoom: f32, pan: Vec2) -> Vec3 {
    let horizontal = relative * STARMAP_SCALE * zoom + pan;
    let height = ((relative.x * 0.0043).sin() + (relative.y * 0.0031).cos()) * 34.0 + 48.0;
    vec3(horizontal.x, height.max(14.0), horizontal.y)
}

fn world_to_starmap_floor(relative: Vec2, zoom: f32, pan: Vec2) -> Vec3 {
    let horizontal = relative * STARMAP_SCALE * zoom + pan;
    vec3(horizontal.x, 0.0, horizontal.y)
}

fn starmap_ship_position(pan: Vec2) -> Vec3 {
    vec3(pan.x, 18.0, pan.y)
}

fn starmap_floor_position(pan: Vec2) -> Vec3 {
    vec3(pan.x, 0.0, pan.y)
}

fn starmap_planet_radius(planet: &Planet, zoom: f32) -> f32 {
    (planet.radius * STARMAP_SCALE * 0.7 * zoom).clamp(18.0, 46.0)
}

fn starmap_planet_hit_radius(planet: &Planet, zoom: f32) -> f32 {
    starmap_planet_radius(planet, zoom).max(24.0)
}

fn starmap_planet_screen_pos(
    ship: &Ship,
    planet: &Planet,
    camera: &Camera3D,
    zoom: f32,
    pan: Vec2,
) -> Vec2 {
    starmap_world_to_screen(
        world_to_starmap_3d(planet.position - ship.position, zoom, pan),
        camera,
    )
}

fn starmap_world_to_screen(position: Vec3, camera: &Camera3D) -> Vec2 {
    let projected = camera.matrix().project_point3(position);
    let (x, y, width, height) = starmap_camera_screen_rect(camera);

    vec2(
        x + (projected.x * 0.5 + 0.5) * width,
        y + (0.5 - projected.y * 0.5) * height,
    )
}

fn starmap_camera_screen_rect(camera: &Camera3D) -> (f32, f32, f32, f32) {
    camera
        .viewport
        .map(|(x, y, width, height)| {
            (
                x as f32,
                screen_height() - (y + height) as f32,
                width as f32,
                height as f32,
            )
        })
        .unwrap_or((0.0, 0.0, screen_width(), screen_height()))
}

fn draw_starmap_readout(
    x: f32,
    y: f32,
    ship: &Ship,
    planets: &[Planet],
    current_system_id: &str,
    destination: Option<usize>,
) {
    let nearest = planets
        .iter()
        .filter(|planet| planet_is_in_system(planet, current_system_id))
        .map(|planet| planet.position.distance(ship.position))
        .min_by(|a, b| a.total_cmp(b))
        .unwrap_or(0.0);
    let text = Color::from_rgba(205, 226, 230, 235);
    let label = Color::from_rgba(126, 156, 164, 200);

    draw_text("Position", x, y, 15.0, label);
    draw_text(
        &format!("{:>7.0} / {:>7.0}", ship.position.x, ship.position.y),
        x,
        y + 26.0,
        20.0,
        text,
    );
    draw_text("Nearest body", x + 250.0, y, 15.0, label);
    draw_text(&format!("{nearest:.0}u"), x + 250.0, y + 26.0, 20.0, text);
    draw_text("Velocity", x + 450.0, y, 15.0, label);
    draw_text(
        &format!("{:.0}u/s", ship.velocity.length()),
        x + 450.0,
        y + 26.0,
        20.0,
        text,
    );
    if let Some(planet) = target_planet(ship, planets, current_system_id, None, destination) {
        draw_text("Destination", x + 630.0, y, 15.0, label);
        let destination_name = if planet_has_surface_scan(planet) {
            planet.info.classification.as_str()
        } else {
            "Unscanned body"
        };
        draw_text(
            destination_name,
            x + 630.0,
            y + 26.0,
            20.0,
            Color::from_rgba(150, 221, 226, 255),
        );
    }
}

fn draw_starmap_filter_readout(game: &GameState, x: f32, y: f32) {
    let label = Color::from_rgba(126, 156, 164, 220);
    let text = Color::from_rgba(205, 226, 230, 245);
    let filter_text = if game.starmap_filter == StarmapFilter::Resource {
        selected_starmap_resource_filter(game)
            .map(|item| format!("{}: {}", game.starmap_filter.label(), item.name))
            .unwrap_or_else(|| "Resource: none scanned".to_string())
    } else {
        game.starmap_filter.label().to_string()
    };
    let visible_count = game
        .planets
        .iter()
        .enumerate()
        .filter(|(index, planet)| {
            planet_in_active_system(game, *index)
                && planet_matches_starmap_filter(game, *index, planet)
        })
        .count();

    draw_text("Filter", x, y, 15.0, label);
    draw_text(
        &format!("{filter_text}  {visible_count} body(s)"),
        x,
        y + 24.0,
        18.0,
        text,
    );
}

fn draw_known_systems_panel(game: &GameState) {
    let panel = known_systems_panel_rect();
    draw_rectangle(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        Color::from_rgba(4, 12, 18, 218),
    );
    draw_rectangle_lines(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        1.0,
        Color::from_rgba(112, 151, 163, 120),
    );
    draw_text(
        "Known Systems",
        panel.x + 12.0,
        panel.y + 25.0,
        20.0,
        Color::from_rgba(235, 242, 226, 255),
    );

    let mouse = vec2(mouse_position().0, mouse_position().1);
    let max_rows = ((panel.h - 46.0) / (KNOWN_SYSTEM_ROW_HEIGHT + 8.0))
        .floor()
        .max(0.0) as usize;
    let known_systems = known_system_ids(&game.content_registry);
    for (index, system_id) in known_systems.iter().take(max_rows).enumerate() {
        let Some(system) = game.content_registry.systems.get(system_id) else {
            continue;
        };
        let row = known_system_row_rect(index);
        let is_current = system_id == &game.current_system_id;
        let is_charging = game
            .pending_warp
            .as_ref()
            .is_some_and(|warp| warp.target_system_id == *system_id);
        let is_hovered = row.contains(mouse);
        let cost = warp_cost(
            &game.content_registry,
            &game.current_system_id,
            system_id.as_str(),
        );
        let can_warp = is_current || can_afford_cost(&game.inventory, &cost);
        let fill = if is_current {
            Color::from_rgba(24, 68, 74, 230)
        } else if is_charging {
            Color::from_rgba(44, 54, 28, 230)
        } else if is_hovered {
            Color::from_rgba(18, 42, 52, 230)
        } else {
            Color::from_rgba(8, 20, 28, 210)
        };
        let line = if is_current {
            Color::from_rgba(113, 235, 138, 175)
        } else if is_charging {
            Color::from_rgba(255, 206, 88, 175)
        } else if can_warp {
            Color::from_rgba(112, 151, 163, 90)
        } else {
            Color::from_rgba(126, 76, 72, 130)
        };
        draw_rectangle(row.x, row.y, row.w, row.h, fill);
        draw_rectangle_lines(row.x, row.y, row.w, row.h, 1.0, line);

        draw_text(
            &fit_debug_text(&system.name, row.w - 20.0, 17),
            row.x + 10.0,
            row.y + 19.0,
            17.0,
            Color::from_rgba(235, 242, 226, 255),
        );
        let body_count = game
            .planets
            .iter()
            .filter(|planet| planet.system == *system_id)
            .count();
        let marker = if is_current {
            "ACTIVE".to_string()
        } else if let Some(warp) = game
            .pending_warp
            .as_ref()
            .filter(|warp| warp.target_system_id == *system_id)
        {
            format!("CHARGE {:.1}s", warp.timer)
        } else if can_warp {
            format!("{} WARP", format_warp_cost(&cost))
        } else {
            format!("NEED {}", format_warp_cost(&cost))
        };
        let detail = format!(
            "{} bodies  {}  arrival {:>4.0},{:>4.0}  {}",
            body_count,
            system
                .faction
                .as_deref()
                .map(|faction| faction_name(&game.content_registry, faction))
                .unwrap_or("unclaimed"),
            system.arrival[0],
            system.arrival[1],
            marker
        );
        draw_text(
            &fit_debug_text(&detail, row.w - 20.0, 13),
            row.x + 10.0,
            row.y + 39.0,
            13.0,
            Color::from_rgba(126, 156, 164, 220),
        );
    }
}

fn inventory_panel_rect(action_rail_width: Option<f32>) -> (f32, f32, f32, f32) {
    let sidecar_space = action_rail_width
        .map(|width| width + OBJECT_ACTION_RAIL_GAP)
        .unwrap_or(0.0);
    let available_width = (screen_width() - 32.0 - sidecar_space).max(640.0);
    let panel_width = available_width.min(1176.0);
    let panel_height = inventory_panel_height();
    let total_width = panel_width + sidecar_space;
    let panel_x = (screen_width() - total_width) * 0.5 + sidecar_space;
    let panel_y = (screen_height() - panel_height) * 0.5 + 18.0;

    (
        panel_x.max(16.0 + sidecar_space),
        panel_y,
        panel_width,
        panel_height,
    )
}

fn inventory_panel_height() -> f32 {
    (screen_height() * 0.8).clamp(420.0, screen_height() - 56.0)
}

struct InventoryOverlayLayout {
    panel_x: f32,
    panel_y: f32,
    panel_width: f32,
    panel_height: f32,
    detail_x: f32,
    detail_width: f32,
    production_x: f32,
    production_width: f32,
    inventory_x: f32,
    inventory_width: f32,
    action_rail: Option<Rect>,
}

struct StationActionLayout {
    services: Rect,
    detail: Rect,
}

fn inventory_overlay_layout(action_rail_width: Option<f32>) -> InventoryOverlayLayout {
    let (panel_x, panel_y, panel_width, panel_height) = inventory_panel_rect(action_rail_width);
    let gap = 24.0;
    let inner_width = panel_width - gap * 2.0;
    let pane_width = (inner_width - gap * 2.0).max(0.0);
    let (detail_share, production_share, inventory_share) = if action_rail_width.is_some() {
        (0.34, 0.40, 0.26)
    } else {
        (0.36, 0.38, 0.26)
    };
    let detail_width = pane_width * detail_share;
    let production_width = pane_width * production_share;
    let inventory_width = pane_width * inventory_share;
    let detail_x = panel_x + gap;
    let production_x = detail_x + detail_width + gap;
    let inventory_x = production_x + production_width + gap;
    let action_rail = action_rail_width.map(|action_rail_width| {
        let height = (panel_height * 0.62).clamp(260.0, panel_height - 120.0);
        Rect::new(
            panel_x - action_rail_width - OBJECT_ACTION_RAIL_GAP,
            panel_y + 66.0,
            action_rail_width,
            height,
        )
    });

    InventoryOverlayLayout {
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        detail_x,
        detail_width,
        production_x,
        production_width,
        inventory_x,
        inventory_width,
        action_rail,
    }
}

fn selected_world_object(game: &GameState) -> bool {
    game.selected_planet.is_some()
        || game.selected_station.is_some()
        || game.selected_npc_ship.is_some()
}

fn selected_action_rail_width(game: &GameState) -> Option<f32> {
    if let Some(planet_index) = game.selected_planet {
        return game
            .planets
            .get(planet_index)
            .map(|planet| action_rail_width_with_override(planet_action_rail_width(planet), game));
    }
    if let Some(station_index) = game.selected_station {
        return game.stations.get(station_index).map(|station| {
            action_rail_width_with_override(station_action_rail_width(station), game)
        });
    }
    if let Some(npc_ship_index) = game.selected_npc_ship {
        return game.npc_ships.get(npc_ship_index).map(|npc_ship| {
            action_rail_width_with_override(
                npc_ship_action_rail_width(&game.content_registry, &game.ship, npc_ship),
                game,
            )
        });
    }

    None
}

fn action_rail_width_with_override(auto_width: f32, game: &GameState) -> f32 {
    action_rail_width_from_override(auto_width, game.action_rail_width_override)
}

fn action_rail_width_from_override(auto_width: f32, override_width: Option<f32>) -> f32 {
    clamp_action_rail_width(action_rail_override_candidate(auto_width, override_width))
}

fn action_rail_override_candidate(auto_width: f32, override_width: Option<f32>) -> f32 {
    override_width.map_or(auto_width, |width| width.max(auto_width))
}

fn clamp_action_rail_width(width: f32) -> f32 {
    let max_width = (screen_width() * OBJECT_ACTION_RAIL_MAX_SCREEN_FRACTION)
        .clamp(OBJECT_ACTION_RAIL_MIN_WIDTH, 540.0);
    width
        .max(OBJECT_ACTION_RAIL_MIN_WIDTH)
        .min(max_width)
        .min((screen_width() - 672.0).max(OBJECT_ACTION_RAIL_MIN_WIDTH))
}

fn planet_action_rail_width(planet: &Planet) -> f32 {
    let mineable_width = planet
        .info
        .mineables
        .iter()
        .map(|mineable| measure_text(&mineable.item.name, None, 20, 1.0).width)
        .fold(measure_text("Item", None, 16, 1.0).width, f32::max);
    let active_width = measure_text("Active", None, 16, 1.0).width.max(50.0);
    let table_width = mineable_width.max(132.0) + 42.0 + 68.0 + 42.0 + active_width + 12.0 * 4.0;

    clamp_action_rail_width(table_width + 34.0)
}

fn station_action_rail_width(station: &StationDestination) -> f32 {
    let service_button_width = station_service_button_width(station);
    let trade_width = station
        .services
        .iter()
        .flat_map(|service| service.trade.iter())
        .map(|offer| measure_text(&offer.item.name, None, 15, 1.0).width)
        .fold(measure_text("Trade stock", None, 14, 1.0).width, f32::max);
    let unlock_width = station
        .services
        .iter()
        .flat_map(|service| service.recipe_unlocks.iter())
        .map(|unlock| measure_text(&unlock.recipe, None, 15, 1.0).width)
        .fold(
            measure_text("Recipe unlocks", None, 14, 1.0).width,
            f32::max,
        );
    let trade_table_width = trade_width.max(116.0) + 54.0 + 58.0 + 8.0 * 2.0;
    let unlock_table_width = unlock_width.max(160.0) + 72.0 + 12.0;
    let detail_width = trade_table_width.max(unlock_table_width).max(240.0);

    clamp_action_rail_width(service_button_width + 16.0 + detail_width + 34.0)
}

fn station_service_button_width(station: &StationDestination) -> f32 {
    let service_width = station
        .services
        .iter()
        .map(|service| measure_text(&service.name, None, 16, 1.0).width)
        .fold(measure_text("Service", None, 14, 1.0).width, f32::max);

    (service_width + 24.0).clamp(132.0, 210.0)
}

fn npc_ship_action_rail_width(
    content_registry: &content::ContentRegistry,
    ship: &Ship,
    npc_ship: &NpcShip,
) -> f32 {
    let rows = npc_interaction_rows(content_registry, ship, npc_ship);
    let action_width = rows
        .iter()
        .map(|row| measure_text(row.action.label(), None, 16, 1.0).width)
        .fold(measure_text("Action", None, 14, 1.0).width, f32::max);
    let status_width = rows
        .iter()
        .map(|row| measure_text(row.status, None, 15, 1.0).width)
        .fold(measure_text("Status", None, 14, 1.0).width, f32::max);

    clamp_action_rail_width(action_width.max(150.0) + status_width.max(78.0) + 12.0 + 34.0)
}

fn draw_inventory_overlay(game: &GameState) {
    let has_object_actions = selected_world_object(game);
    let action_rail_width = selected_action_rail_width(game);
    let layout = inventory_overlay_layout(action_rail_width);

    draw_rectangle(
        layout.panel_x,
        layout.panel_y,
        layout.panel_width,
        layout.panel_height,
        Color::from_rgba(6, 12, 18, 228),
    );
    draw_rectangle_lines(
        layout.panel_x,
        layout.panel_y,
        layout.panel_width,
        layout.panel_height,
        1.0,
        Color::from_rgba(112, 151, 163, 150),
    );
    draw_inventory_pane_separators(&layout);

    let detail_title = if game.selected_planet.is_some() {
        "Planet Pane"
    } else if game.selected_station.is_some() {
        "Station Pane"
    } else if game.selected_npc_ship.is_some() {
        "Contact Pane"
    } else {
        "Ship Pane"
    };
    draw_text(
        detail_title,
        layout.detail_x,
        layout.panel_y + 38.0,
        26.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_text(
        "Production",
        layout.production_x,
        layout.panel_y + 38.0,
        26.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_production_mode_tabs(
        game.production_mode,
        layout.production_x + layout.production_width - 204.0,
        layout.panel_y + 19.0,
    );
    draw_text(
        "Inventory",
        layout.inventory_x,
        layout.panel_y + 38.0,
        26.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_text(
        &format!("Credits {}", game.credits),
        layout.inventory_x + 128.0,
        layout.panel_y + 36.0,
        17.0,
        Color::from_rgba(126, 156, 164, 220),
    );

    let mouse = vec2(mouse_position().0, mouse_position().1);
    draw_production_text_table(
        game,
        layout.production_x,
        work_table_y(),
        layout.production_width,
        game.work_scroll,
        mouse,
    );

    draw_inventory_text_list(
        &game.inventory,
        layout.inventory_x,
        layout.panel_y + 66.0,
        layout.inventory_width,
        game.inventory_scroll,
    );
    draw_detail_panel(
        game,
        layout.detail_x,
        layout.panel_y + 66.0,
        layout.detail_width,
    );
    if has_object_actions {
        draw_object_action_rail(game, &layout, mouse);
    }
    if let Some(recipe) = hovered_production_recipe(game, mouse, game.work_scroll) {
        draw_recipe_tooltip(recipe, &game.inventory, mouse);
    }
}

fn draw_inventory_pane_separators(layout: &InventoryOverlayLayout) {
    let top = layout.panel_y + 52.0;
    let bottom = layout.panel_y + layout.panel_height - 34.0;
    let first_x = layout.detail_x + layout.detail_width + 12.0;
    let second_x = layout.production_x + layout.production_width + 12.0;
    let color = Color::from_rgba(96, 137, 150, 115);

    draw_vertical_dotted_line(first_x, top, bottom, 1.0, 7.0, 7.0, color);
    draw_vertical_dotted_line(second_x, top, bottom, 1.0, 7.0, 7.0, color);
}

fn draw_vertical_dotted_line(
    x: f32,
    y1: f32,
    y2: f32,
    thickness: f32,
    dash: f32,
    gap: f32,
    color: Color,
) {
    let mut y = y1;
    while y < y2 {
        let end_y = (y + dash).min(y2);
        draw_line(x, y, x, end_y, thickness, color);
        y += dash + gap;
    }
}

fn draw_object_action_rail(game: &GameState, layout: &InventoryOverlayLayout, mouse: Vec2) {
    let Some(rail) = layout.action_rail else {
        return;
    };

    if let Some(planet_index) = game.selected_planet {
        if let Some(planet) = game.planets.get(planet_index) {
            draw_planet_action_rail(PlanetActionRailRender {
                content_registry: &game.content_registry,
                planet,
                inventory: &game.inventory,
                ship_upgrades: &game.ship_upgrades,
                action_rail_width: rail.w,
                is_orbiting: game.orbiting_planet == Some(planet_index),
                in_range: planet_in_interaction_range(&game.ship, planet),
                scroll: game.work_scroll,
                mouse,
            });
        }
    } else if let Some(station_index) = game.selected_station {
        if let Some(station) = game.stations.get(station_index) {
            draw_action_rail_frame(rail, "Actions");
            draw_station_service_list(StationActionRailRender {
                station,
                selected_service: game.selected_station_service,
                in_range: station_in_interaction_range(&game.ship, station),
                credits: game.credits,
                inventory: &game.inventory,
                purchased_unlocks: &game.purchased_recipe_unlocks,
                action_rail_width: rail.w,
            });
        }
    } else if let Some(npc_ship_index) = game.selected_npc_ship {
        if let Some(npc_ship) = game.npc_ships.get(npc_ship_index) {
            draw_action_rail_frame(rail, "Actions");
            draw_npc_ship_interaction_list(
                &game.content_registry,
                &game.ship,
                npc_ship,
                Rect::new(rail.x + 12.0, rail.y + 48.0, rail.w - 24.0, rail.h - 60.0),
            );
        }
    }
}

fn draw_action_rail_frame(rail: Rect, title: &str) {
    draw_rectangle(
        rail.x,
        rail.y,
        rail.w,
        rail.h,
        Color::from_rgba(8, 18, 24, 204),
    );
    draw_rectangle_lines(
        rail.x,
        rail.y,
        rail.w,
        rail.h,
        1.0,
        Color::from_rgba(112, 151, 163, 125),
    );
    draw_text(
        title,
        rail.x + 10.0,
        rail.y + 24.0,
        16.0,
        Color::from_rgba(88, 116, 126, 180),
    );
    draw_action_rail_resize_handle(rail);
}

fn action_rail_resize_handle_rect(rail: Rect) -> Rect {
    Rect::new(
        rail.x - ACTION_RAIL_RESIZE_HITBOX_WIDTH * 0.5,
        rail.y + 6.0,
        ACTION_RAIL_RESIZE_HITBOX_WIDTH,
        rail.h - 12.0,
    )
}

fn draw_action_rail_resize_handle(rail: Rect) {
    let handle = action_rail_resize_handle_rect(rail);
    let hovered = handle.contains(mouse_vec2());
    let color = if hovered {
        Color::from_rgba(150, 221, 226, 210)
    } else {
        Color::from_rgba(96, 137, 150, 125)
    };
    let x = rail.x;

    draw_vertical_dotted_line(
        x,
        handle.y + 8.0,
        handle.y + handle.h - 8.0,
        1.0,
        4.0,
        5.0,
        color,
    );
    if hovered {
        draw_rectangle(
            x - 2.0,
            handle.y + 10.0,
            4.0,
            handle.h - 20.0,
            Color::from_rgba(150, 221, 226, 28),
        );
    }
}

fn ship_detail_preview_rect() -> Rect {
    let layout = inventory_overlay_layout(None);
    let detail_width = layout.detail_width;
    let detail_x = layout.detail_x;
    let detail_y = layout.panel_y + 66.0;
    let image_size = 190.0;
    let center = vec2(detail_x + detail_width * 0.5, detail_y + 74.0);

    Rect::new(
        center.x - image_size * 0.5,
        center.y - image_size * 0.5,
        image_size,
        image_size,
    )
}

fn ship_shield_slot_rect(slot_index: usize) -> Rect {
    let layout = inventory_overlay_layout(None);
    let detail_width = layout.detail_width;
    let detail_x = layout.detail_x;
    let detail_y = layout.panel_y + 66.0;
    let stats_y = detail_y + 190.0 + 28.0;
    let row_y = stats_y + 28.0 + slot_index as f32 * 66.0;

    Rect::new(detail_x, row_y - 18.0, detail_width * 0.48, 62.0)
}

fn ship_weapon_slot_rect(slot_index: usize) -> Rect {
    let layout = inventory_overlay_layout(None);
    let detail_width = layout.detail_width;
    let detail_x = layout.detail_x;
    let detail_y = layout.panel_y + 66.0;
    let stats_y = detail_y + 190.0 + 28.0;
    let power_y = stats_y + 132.0;
    let weapons_y = power_y + 132.0;
    let row_y = weapons_y + 28.0 + slot_index as f32 * 44.0;

    Rect::new(detail_x, row_y - 18.0, detail_width, 40.0)
}

fn planet_scan_button_rect(action_rail_width: f32) -> Rect {
    let rail = action_rail_rect(action_rail_width);
    Rect::new(rail.x + 12.0, rail.y + 118.0, rail.w - 24.0, 36.0)
}

fn planet_orbit_button_rect(action_rail_width: f32) -> Rect {
    let rail = action_rail_rect(action_rail_width);
    Rect::new(rail.x + 12.0, rail.y + 74.0, rail.w - 24.0, 36.0)
}

fn action_rail_rect(action_rail_width: f32) -> Rect {
    inventory_overlay_layout(Some(action_rail_width))
        .action_rail
        .unwrap_or(Rect::new(0.0, 0.0, 0.0, 0.0))
}

fn station_action_layout(
    station: &StationDestination,
    action_rail_width: f32,
) -> StationActionLayout {
    let rail = action_rail_rect(action_rail_width);
    let inner = Rect::new(rail.x + 12.0, rail.y + 48.0, rail.w - 24.0, rail.h - 60.0);
    let gap = 16.0;
    let services_width = station_service_button_width(station).min(inner.w * 0.42);
    let detail_x = inner.x + services_width + gap;

    StationActionLayout {
        services: Rect::new(inner.x, inner.y, services_width, inner.h),
        detail: Rect::new(
            detail_x,
            inner.y,
            (inner.x + inner.w - detail_x).max(0.0),
            inner.h,
        ),
    }
}

fn station_service_button_rect(
    station: &StationDestination,
    index: usize,
    action_rail_width: f32,
) -> Rect {
    let layout = station_action_layout(station, action_rail_width);
    Rect::new(
        layout.services.x,
        layout.services.y + 28.0 + index as f32 * 40.0,
        layout.services.w,
        34.0,
    )
}

fn draw_skills_overlay(game: &GameState) {
    let (panel_x, panel_y, panel_width, panel_height) = skills_panel_rect();
    let mouse = vec2(mouse_position().0, mouse_position().1);

    draw_rectangle(
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        Color::from_rgba(6, 12, 18, 232),
    );
    draw_rectangle_lines(
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        1.0,
        Color::from_rgba(112, 151, 163, 150),
    );

    draw_text(
        "Skills",
        panel_x + 24.0,
        panel_y + 40.0,
        28.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_text(
        "K toggle   Esc close",
        panel_x + panel_width - 170.0,
        panel_y + 36.0,
        18.0,
        Color::from_rgba(126, 156, 164, 220),
    );

    draw_skills_table(
        &game.skills,
        panel_x + 30.0,
        panel_y + 82.0,
        panel_width - 60.0,
        mouse,
    );
}

fn draw_ship_upgrades_overlay(game: &GameState) {
    let (panel_x, panel_y, panel_width, panel_height) = ship_upgrades_panel_rect();
    let mouse = vec2(mouse_position().0, mouse_position().1);

    draw_rectangle(
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        Color::from_rgba(6, 12, 18, 234),
    );
    draw_rectangle_lines(
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        1.0,
        Color::from_rgba(112, 151, 163, 150),
    );

    draw_text(
        "Ship Upgrades",
        panel_x + 24.0,
        panel_y + 40.0,
        28.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_text(
        "Esc close",
        panel_x + panel_width - 100.0,
        panel_y + 36.0,
        18.0,
        Color::from_rgba(126, 156, 164, 220),
    );

    draw_ship_upgrades_table(ShipUpgradeTableRender {
        content_registry: &game.content_registry,
        upgrades: &game.ship_upgrades,
        inventory: &game.inventory,
        x: panel_x + 28.0,
        y: panel_y + 84.0,
        width: panel_width - 56.0,
        scroll: game.upgrades_scroll,
        viewport_height: ship_upgrades_table_viewport_height(),
        mouse,
    });
}

fn draw_content_debug_overlay(game: &GameState) {
    let layout = content_browser_layout();
    let width = layout.width;
    let height = layout.height;
    let x = layout.x;
    let y = layout.y;
    let registry = &game.content_registry;

    draw_rectangle(x, y, width, height, Color::from_rgba(6, 12, 18, 236));
    draw_rectangle_lines(
        x,
        y,
        width,
        height,
        1.0,
        Color::from_rgba(112, 151, 163, 150),
    );
    draw_text(
        "Content Browser",
        x + 24.0,
        y + 40.0,
        28.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_text(
        "C close",
        x + width - 86.0,
        y + 36.0,
        18.0,
        Color::from_rgba(126, 156, 164, 220),
    );

    let transition_pixels = game
        .transition_assets
        .iter()
        .map(|asset| asset.texture.width() * asset.texture.height())
        .sum::<f32>();
    let summary = format!(
        "{} pack(s) / {} item(s) / {} recipe(s) / {} faction(s) / {} NPC ship(s) / {} shield(s) / {} weapon(s) / {} system(s) / {} star(s) / {} planet(s) / {} station(s) / {} upgrade(s) / {} transition image(s)",
        registry.packs.len(),
        registry.items.len(),
        registry.recipes.len(),
        registry.factions.len(),
        registry.npc_ships.len(),
        registry.shields.len(),
        registry.weapons.len(),
        registry.systems.len(),
        registry.stars.len(),
        registry.planets.len(),
        registry.stations.len(),
        registry.upgrades.len(),
        game.transition_assets.len()
    );
    draw_text(
        &fit_debug_text(&summary, width - 48.0, 17),
        x + 24.0,
        y + 66.0,
        17.0,
        Color::from_rgba(150, 221, 226, 235),
    );

    if let Some(asset) = game.transition_assets.first() {
        let transition_summary = if game.transition_assets.len() == 1 {
            format!(
                "Transition asset: {} ({:.0} kp)",
                asset.path,
                transition_pixels / 1000.0
            )
        } else {
            format!(
                "Transition assets: {} (+{} more, {:.0} kp)",
                asset.path,
                game.transition_assets.len() - 1,
                transition_pixels / 1000.0
            )
        };
        draw_text(
            &fit_debug_text(&transition_summary, width - 48.0, 15),
            x + 24.0,
            y + 88.0,
            15.0,
            Color::from_rgba(126, 156, 164, 220),
        );
    }

    let selected_pack_id = selected_content_pack_id(game);
    let filter_label = selected_pack_id
        .map(|pack_id| format!("Showing pack: {pack_id}"))
        .unwrap_or_else(|| "Showing all packs".to_string());
    draw_text(
        &fit_debug_text(&filter_label, width - 48.0, 15),
        x + 24.0,
        y + 108.0,
        15.0,
        Color::from_rgba(226, 190, 150, 235),
    );

    draw_content_pack_column(
        game,
        &layout,
        x + 24.0,
        layout.column_y,
        layout.column_width,
    );

    let item_rows = filtered_content_item_rows(game, selected_pack_id);
    draw_content_debug_column(ContentColumnRender {
        title: "Items",
        x: x + 24.0 + (layout.column_width + layout.column_gap),
        y: layout.column_y,
        width: layout.column_width,
        row_height: layout.row_height,
        viewport_height: layout.viewport_height,
        scroll: game.content_browser.items_scroll,
        rows: &item_rows,
        selected_row: None,
    });

    let recipe_rows = filtered_content_recipe_rows(game, selected_pack_id);
    draw_content_debug_column(ContentColumnRender {
        title: "Recipes",
        x: x + 24.0 + (layout.column_width + layout.column_gap) * 2.0,
        y: layout.column_y,
        width: layout.column_width,
        row_height: layout.row_height,
        viewport_height: layout.viewport_height,
        scroll: game.content_browser.recipes_scroll,
        rows: &recipe_rows,
        selected_row: None,
    });

    let npc_ship_rows = filtered_content_npc_ship_rows(game, selected_pack_id);
    let planet_rows = filtered_content_planet_rows(game, selected_pack_id);
    draw_content_debug_column(ContentColumnRender {
        title: "NPC Ships",
        x: x + 24.0 + (layout.column_width + layout.column_gap) * 3.0,
        y: layout.column_y,
        width: layout.column_width,
        row_height: layout.row_height,
        viewport_height: layout.viewport_height,
        scroll: game.content_browser.npc_ships_scroll,
        rows: &npc_ship_rows,
        selected_row: None,
    });

    draw_content_debug_column(ContentColumnRender {
        title: "Planets",
        x: x + 24.0 + (layout.column_width + layout.column_gap) * 4.0,
        y: layout.column_y,
        width: layout.column_width,
        row_height: layout.row_height,
        viewport_height: layout.viewport_height,
        scroll: game.content_browser.planets_scroll,
        rows: &planet_rows,
        selected_row: None,
    });
}

fn filtered_content_item_rows(game: &GameState, selected_pack_id: Option<&str>) -> Vec<String> {
    let registry = &game.content_registry;
    registry
        .item_order
        .iter()
        .filter(|item_id| {
            selected_pack_id
                .map(|pack_id| content_id_belongs_to_pack(item_id, pack_id))
                .unwrap_or(true)
        })
        .filter_map(|item_id| registry.items.get(item_id))
        .map(|item| format!("{}  {}", item.name, item.tier))
        .collect()
}

fn filtered_content_recipe_rows(game: &GameState, selected_pack_id: Option<&str>) -> Vec<String> {
    let registry = &game.content_registry;
    registry
        .recipe_order
        .iter()
        .filter(|recipe_id| {
            selected_pack_id
                .map(|pack_id| content_id_belongs_to_pack(recipe_id, pack_id))
                .unwrap_or(true)
        })
        .filter_map(|recipe_id| registry.recipes.get(recipe_id))
        .map(|recipe| {
            let output_name = registry
                .items
                .get(&recipe.output.item)
                .map(|item| item.name.as_str())
                .unwrap_or(recipe.output.item.as_str());
            format!(
                "{} x{}  {}",
                output_name, recipe.output.count, recipe.station
            )
        })
        .collect()
}

fn filtered_content_npc_ship_rows(game: &GameState, selected_pack_id: Option<&str>) -> Vec<String> {
    let registry = &game.content_registry;
    registry
        .npc_ship_order
        .iter()
        .filter(|npc_ship_id| {
            selected_pack_id
                .map(|pack_id| content_id_belongs_to_pack(npc_ship_id, pack_id))
                .unwrap_or(true)
        })
        .filter_map(|npc_ship_id| registry.npc_ships.get(npc_ship_id))
        .map(|npc_ship| {
            let faction = npc_ship
                .faction
                .as_deref()
                .map(|faction| faction_name(registry, faction))
                .unwrap_or("unclaimed");
            format!(
                "{}  {}  {}  {}",
                npc_ship.name, faction, npc_ship.archetype, npc_ship.role
            )
        })
        .collect()
}

fn filtered_content_planet_rows(game: &GameState, selected_pack_id: Option<&str>) -> Vec<String> {
    let registry = &game.content_registry;
    registry
        .planet_order
        .iter()
        .filter(|planet_id| {
            selected_pack_id
                .map(|pack_id| content_id_belongs_to_pack(planet_id, pack_id))
                .unwrap_or(true)
        })
        .filter_map(|planet_id| registry.planets.get(planet_id))
        .map(|planet| {
            let faction = planet
                .faction
                .as_deref()
                .map(|faction| faction_name(registry, faction))
                .unwrap_or("unclaimed");
            format!(
                "{}  {}  {} resource(s)",
                faction,
                planet.classification,
                planet.mineables.len()
            )
        })
        .collect()
}

fn draw_content_pack_column(
    game: &GameState,
    layout: &ContentBrowserLayout,
    x: f32,
    y: f32,
    width: f32,
) {
    let rows = std::iter::once("All packs".to_string())
        .chain(
            game.content_registry
                .packs
                .iter()
                .map(|pack| format!("{} {}", pack.id, pack.version)),
        )
        .collect::<Vec<_>>();
    draw_content_debug_column(ContentColumnRender {
        title: "Packs",
        x,
        y,
        width,
        row_height: layout.row_height,
        viewport_height: layout.viewport_height,
        scroll: game.content_browser.packs_scroll,
        rows: &rows,
        selected_row: game
            .content_browser
            .selected_pack_index
            .map(|index| index + 1)
            .or(Some(0)),
    });
}

fn draw_content_debug_column(column: ContentColumnRender<'_>) {
    let title_color = Color::from_rgba(235, 242, 226, 255);
    let text = Color::from_rgba(205, 226, 230, 245);
    let selected_text = Color::from_rgba(235, 242, 226, 255);
    let scroll = column.scroll.clamp(
        0.0,
        max_scroll_offset(column.rows.len(), column.row_height, column.viewport_height),
    );
    let viewport_top = column.y + 18.0;
    let viewport_bottom = viewport_top + column.viewport_height;
    draw_text(
        &format!("{} ({})", column.title, column.rows.len()),
        column.x,
        column.y,
        18.0,
        title_color,
    );
    draw_line(
        column.x,
        column.y + 12.0,
        column.x + column.width,
        column.y + 12.0,
        1.0,
        Color::from_rgba(96, 137, 150, 205),
    );

    for (index, row) in column.rows.iter().enumerate() {
        let row_y =
            viewport_top + column.row_height - 5.0 + index as f32 * column.row_height - scroll;
        if row_y - 17.0 < viewport_top || row_y + 6.0 > viewport_bottom {
            continue;
        }
        let selected = column.selected_row == Some(index);
        if index % 2 == 0 || selected {
            draw_rectangle(
                column.x,
                row_y - 17.0,
                column.width,
                column.row_height,
                if selected {
                    Color::from_rgba(24, 58, 66, 215)
                } else {
                    Color::from_rgba(10, 18, 24, 94)
                },
            );
        }
        draw_text(
            &fit_debug_text(row, column.width - 10.0, 15),
            column.x + 6.0,
            row_y,
            15.0,
            if selected { selected_text } else { text },
        );
    }
    draw_scrollbar(
        column.x + column.width - 4.0,
        viewport_top,
        column.viewport_height,
        column.rows.len(),
        column.row_height,
        scroll,
    );
}

fn fit_debug_text(text: &str, width: f32, font_size: u16) -> String {
    let mut fitted = text.to_string();
    while fitted.len() > 3 && measure_text(&fitted, None, font_size, 1.0).width > width - 12.0 {
        fitted.pop();
    }
    if fitted.len() < text.len() && fitted.len() > 3 {
        fitted.truncate(fitted.len() - 3);
        fitted.push_str("...");
    }
    fitted
}

fn ship_upgrades_panel_rect() -> (f32, f32, f32, f32) {
    let width = (screen_width() - 48.0).min(900.0);
    let max_height = (screen_height() - 72.0).max(320.0);
    let height = (screen_height() * 0.8).min(max_height).max(320.0);
    let x = (screen_width() - width) * 0.5;
    let y = (screen_height() - height) * 0.5;
    (x, y, width, height)
}

fn ship_upgrade_table_origin() -> Vec2 {
    let (panel_x, panel_y, _, _) = ship_upgrades_panel_rect();
    vec2(panel_x + 28.0, panel_y + 84.0)
}

fn ship_upgrades_table_viewport_top() -> f32 {
    ship_upgrade_table_origin().y + 18.0
}

fn ship_upgrades_table_viewport_height() -> f32 {
    let (_, _, _, panel_height) = ship_upgrades_panel_rect();
    (panel_height - 126.0).max(0.0)
}

fn hovered_ship_upgrade_plus(mouse: Vec2, row_count: usize, scroll: f32) -> Option<usize> {
    let origin = ship_upgrade_table_origin();
    let (_, _, panel_width, _) = ship_upgrades_panel_rect();
    let layout = ship_upgrade_table_layout(
        origin.x,
        origin.y,
        panel_width - 56.0,
        ship_upgrades_table_viewport_height(),
    );
    ui_hovered_table_cell(mouse, &layout, row_count, scroll)
        .filter(|cell| cell.column == 3)
        .map(|cell| cell.row)
}

fn draw_ship_upgrades_table(table: ShipUpgradeTableRender<'_>) {
    let ShipUpgradeTableRender {
        content_registry,
        upgrades,
        inventory,
        x,
        y,
        width,
        scroll,
        viewport_height,
        mouse,
    } = table;
    let layout = ship_upgrade_table_layout(x, y, width, viewport_height);
    let name_column = layout.columns[0];
    let level_column = layout.columns[1];
    let cost_column = layout.columns[2];
    let plus_column = layout.columns[3];
    let header = Color::from_rgba(168, 204, 210, 255);
    let text = Color::from_rgba(205, 226, 230, 255);
    let detail = Color::from_rgba(178, 197, 203, 255);
    let active = Color::from_rgba(150, 221, 226, 255);
    let unavailable = Color::from_rgba(126, 143, 148, 255);
    let scroll = scroll.clamp(
        0.0,
        max_scroll_offset(upgrades.len(), layout.row_height, viewport_height),
    );
    let hovered = hovered_ship_upgrade_plus(mouse, upgrades.len(), scroll);

    draw_text("Upgrade", name_column.x, y, 16.0, header);
    draw_text("Level", level_column.x, y, 16.0, header);
    draw_text("Next cost", cost_column.x, y, 16.0, header);
    draw_text("+", plus_column.x + 10.0, y, 16.0, header);

    for (row, upgrade) in upgrades.iter().enumerate() {
        let row_rect = ui_table_row_rect(&layout, row, scroll);
        if !ui_table_row_visible(&layout, row_rect) {
            continue;
        }
        let row_y = row_rect.y + 24.0;
        let cost = upgrade.next_cost(content_registry);
        let affordable = can_afford_cost(inventory, &cost);
        let is_hovered = hovered == Some(row);
        if row % 2 == 0 || is_hovered {
            draw_rectangle(
                row_rect.x,
                row_rect.y,
                row_rect.w,
                row_rect.h,
                Color::from_rgba(10, 18, 24, if is_hovered { 170 } else { 100 }),
            );
        }

        draw_text(
            &fit_debug_text(upgrade.kind.name(), name_column.w, 21),
            name_column.x,
            row_y - 4.0,
            21.0,
            text,
        );
        draw_text(
            &fit_debug_text(upgrade.kind.effect_text(), name_column.w, 15),
            name_column.x,
            row_y + 18.0,
            15.0,
            detail,
        );
        draw_text(
            &upgrade.level.to_string(),
            level_column.x,
            row_y,
            22.0,
            active,
        );
        draw_text(
            &fit_debug_text(&format_cost(&cost), cost_column.w, 18),
            cost_column.x,
            row_y,
            18.0,
            if affordable { active } else { unavailable },
        );

        draw_plus_button(
            plus_column.x,
            row_rect.y + 1.0,
            affordable,
            active,
            unavailable,
        );
    }
    draw_scrollbar(
        x + width - 4.0,
        layout.viewport.y,
        layout.viewport.h,
        upgrades.len(),
        layout.row_height,
        scroll,
    );
}

struct ShipUpgradeTableRender<'a> {
    content_registry: &'a content::ContentRegistry,
    upgrades: &'a [ShipUpgrade; SHIP_UPGRADE_COUNT],
    inventory: &'a Inventory,
    x: f32,
    y: f32,
    width: f32,
    scroll: f32,
    viewport_height: f32,
    mouse: Vec2,
}

fn format_cost(cost: &[ItemStack]) -> String {
    cost.iter()
        .map(|stack| format!("{} {}", stack.count, stack.item.name))
        .collect::<Vec<_>>()
        .join(" / ")
}

fn draw_plus_button(x: f32, y: f32, enabled: bool, enabled_color: Color, disabled_color: Color) {
    let button_size = 28.0;
    draw_rectangle_lines(
        x,
        y,
        button_size,
        button_size,
        1.0,
        if enabled {
            Color::from_rgba(150, 221, 226, 180)
        } else {
            Color::from_rgba(82, 114, 124, 95)
        },
    );
    let plus_measure = measure_text("+", None, 24, 1.0);
    draw_text(
        "+",
        x + (button_size - plus_measure.width) * 0.5,
        y + (button_size + plus_measure.height) * 0.5 - 2.0,
        24.0,
        if enabled {
            enabled_color
        } else {
            disabled_color
        },
    );
}

fn skills_panel_rect() -> (f32, f32, f32, f32) {
    let width = (screen_width() - 48.0).min(760.0);
    let height = 320.0_f32.min(screen_height() - 72.0);
    let x = (screen_width() - width) * 0.5;
    let y = (screen_height() - height) * 0.5 + 10.0;
    (x, y, width, height)
}

fn skill_table_origin() -> Vec2 {
    let (panel_x, panel_y, _, _) = skills_panel_rect();
    vec2(panel_x + 30.0, panel_y + 82.0)
}

fn hovered_skill_plus(mouse: Vec2, row_count: usize) -> Option<usize> {
    let origin = skill_table_origin();
    let (_, _, panel_width, _) = skills_panel_rect();
    let layout = skills_table_layout(origin.x, origin.y, panel_width - 60.0);
    ui_hovered_table_cell(mouse, &layout, row_count, 0.0)
        .filter(|cell| cell.column == 3)
        .map(|cell| cell.row)
}

fn hovered_skill_row(mouse: Vec2, row_count: usize) -> Option<usize> {
    let origin = skill_table_origin();
    let (_, _, panel_width, _) = skills_panel_rect();
    let layout = skills_table_layout(origin.x, origin.y, panel_width - 60.0);
    ui_hovered_table_cell(mouse, &layout, row_count, 0.0).map(|cell| cell.row)
}

fn draw_skills_table(skills: &[Skill; SKILL_COUNT], x: f32, y: f32, width: f32, mouse: Vec2) {
    let layout = skills_table_layout(x, y, width);
    let skill_column = layout.columns[0];
    let level_column = layout.columns[1];
    let xp_column = layout.columns[2];
    let plus_column = layout.columns[3];
    let header = Color::from_rgba(168, 204, 210, 255);
    let text = Color::from_rgba(205, 226, 230, 255);
    let active = Color::from_rgba(150, 221, 226, 255);
    let unavailable = Color::from_rgba(126, 143, 148, 255);

    draw_text("Skill", skill_column.x, y, 16.0, header);
    draw_text("Level", level_column.x, y, 16.0, header);
    draw_text("XP / Cost", xp_column.x, y, 16.0, header);
    draw_text("+", plus_column.x + 10.0, y, 16.0, header);

    let hovered = hovered_skill_plus(mouse, skills.len());
    let hovered_row = hovered_skill_row(mouse, skills.len());
    for (row, skill) in skills.iter().enumerate() {
        let row_rect = ui_table_row_rect(&layout, row, 0.0);
        if !ui_table_row_visible(&layout, row_rect) {
            continue;
        }
        let row_y = row_rect.y + 25.0;
        let affordable = skill.can_level_up();
        let is_hovered = hovered == Some(row) || hovered_row == Some(row);

        if row % 2 == 0 || is_hovered {
            draw_rectangle(
                row_rect.x,
                row_rect.y,
                row_rect.w,
                row_rect.h,
                Color::from_rgba(10, 18, 24, if is_hovered { 170 } else { 100 }),
            );
        }

        draw_text(
            &fit_debug_text(skill.kind.name(), skill_column.w, 22),
            skill_column.x,
            row_y,
            22.0,
            text,
        );
        draw_text(
            &skill.level.to_string(),
            level_column.x,
            row_y,
            22.0,
            active,
        );
        draw_text(
            &fit_debug_text(
                &format!("{:.0} / {:.0}", skill.xp, skill.cost_to_next_level()),
                xp_column.w,
                22,
            ),
            xp_column.x,
            row_y,
            22.0,
            if affordable { active } else { unavailable },
        );

        draw_plus_button(
            plus_column.x,
            row_rect.y + 1.0,
            affordable,
            active,
            unavailable,
        );
    }

    if let Some(row) = hovered_row {
        draw_skill_tooltip(skills[row], mouse);
    }
}

fn draw_skill_tooltip(skill: Skill, mouse: Vec2) {
    let width = 350.0;
    let height = 130.0;
    let x = (mouse.x + 18.0)
        .min(screen_width() - width - 18.0)
        .max(18.0);
    let y = (mouse.y + 18.0)
        .min(screen_height() - height - 18.0)
        .max(18.0);
    let panel = Color::from_rgba(2, 6, 10, 255);
    let border = Color::from_rgba(112, 151, 163, 170);
    let label = Color::from_rgba(126, 156, 164, 220);
    let text = Color::from_rgba(205, 226, 230, 255);
    let active = Color::from_rgba(150, 221, 226, 255);

    draw_rectangle(x, y, width, height, panel);
    draw_rectangle_lines(x, y, width, height, 1.0, border);
    draw_text(skill.kind.name(), x + 14.0, y + 28.0, 22.0, text);
    draw_text("Levels 1-10", x + 14.0, y + 56.0, 15.0, label);
    draw_text(
        &format!(
            "+{:.0}% {} speed",
            skill.speed_bonus_percent(),
            skill.kind.action_name()
        ),
        x + 118.0,
        y + 56.0,
        17.0,
        active,
    );
    draw_text("Levels 11-20", x + 14.0, y + 86.0, 15.0, label);
    draw_text(
        &format!(
            "{:.2}% free extra output",
            skill.bonus_output_chance_percent()
        ),
        x + 118.0,
        y + 86.0,
        17.0,
        active,
    );
    draw_text(
        &format!("Next level cost {:.0} XP", skill.cost_to_next_level()),
        x + 14.0,
        y + 116.0,
        16.0,
        Color::from_rgba(226, 190, 150, 245),
    );
}

fn ui_column_spec_fixed(width: f32) -> UiColumnSpec {
    UiColumnSpec {
        sizing: UiColumnSizing::Fixed(width),
    }
}

fn ui_column_spec_content(measured: f32, min: f32, max: f32) -> UiColumnSpec {
    UiColumnSpec {
        sizing: UiColumnSizing::Content { measured, min, max },
    }
}

fn ui_column_spec_flex(min: f32, weight: f32) -> UiColumnSpec {
    UiColumnSpec {
        sizing: UiColumnSizing::Flex { min, weight },
    }
}

fn ui_resolve_columns(rect: Rect, gap: f32, specs: &[UiColumnSpec]) -> Vec<Rect> {
    if specs.is_empty() {
        return Vec::new();
    }

    let gap_total = gap * specs.len().saturating_sub(1) as f32;
    let mut fixed_total = 0.0;
    let mut flex_min_total = 0.0;
    let mut flex_weight_total = 0.0;

    for spec in specs {
        match spec.sizing {
            UiColumnSizing::Fixed(width) => fixed_total += width.max(0.0),
            UiColumnSizing::Content { measured, min, max } => {
                fixed_total += measured.clamp(min, max).max(0.0);
            }
            UiColumnSizing::Flex { min, weight } => {
                flex_min_total += min.max(0.0);
                flex_weight_total += weight.max(0.0);
            }
        }
    }

    let available_for_flex = (rect.w - gap_total - fixed_total).max(0.0);
    let flex_extra = (available_for_flex - flex_min_total).max(0.0);
    let mut x = rect.x;
    specs
        .iter()
        .map(|spec| {
            let width = match spec.sizing {
                UiColumnSizing::Fixed(width) => width.max(0.0),
                UiColumnSizing::Content { measured, min, max } => measured.clamp(min, max).max(0.0),
                UiColumnSizing::Flex { min, weight } => {
                    let share = if flex_weight_total > 0.0 {
                        flex_extra * weight.max(0.0) / flex_weight_total
                    } else {
                        0.0
                    };
                    min.max(0.0) + share
                }
            };
            let column = Rect::new(x, rect.y, width, rect.h);
            x += width + gap;
            column
        })
        .collect()
}

fn ui_table_layout(
    bounds: Rect,
    viewport_top: f32,
    viewport_height: f32,
    row_height: f32,
    column_gap: f32,
    columns: &[UiColumnSpec],
) -> UiTableLayout {
    let viewport = Rect::new(bounds.x, viewport_top, bounds.w, viewport_height);
    let column_rect = Rect::new(bounds.x, viewport.y, bounds.w, viewport.h);
    UiTableLayout {
        bounds,
        viewport,
        columns: ui_resolve_columns(column_rect, column_gap, columns),
        row_height,
    }
}

fn ui_table_layout_until_bottom(config: UiTableBottomLayout<'_>) -> UiTableLayout {
    let viewport_top = config.y + config.row_start_offset;
    let viewport_height = (config.viewport_bottom - viewport_top).max(0.0);
    ui_table_layout(
        Rect::new(config.x, config.y, config.width, viewport_height),
        viewport_top,
        viewport_height,
        config.row_height,
        config.column_gap,
        config.columns,
    )
}

fn ui_table_row_rect(layout: &UiTableLayout, row: usize, scroll: f32) -> Rect {
    Rect::new(
        layout.viewport.x,
        layout.viewport.y + row as f32 * layout.row_height - scroll,
        layout.viewport.w,
        layout.row_height,
    )
}

fn ui_table_row_visible(layout: &UiTableLayout, row_rect: Rect) -> bool {
    row_rect.y >= layout.viewport.y
        && row_rect.y + row_rect.h <= layout.viewport.y + layout.viewport.h
}

fn ui_hovered_table_cell(
    mouse: Vec2,
    layout: &UiTableLayout,
    row_count: usize,
    scroll: f32,
) -> Option<UiTableCell> {
    if !layout.viewport.contains(mouse) {
        return None;
    }

    let row = ((mouse.y - layout.viewport.y + scroll) / layout.row_height).floor() as isize;
    if row < 0 || row as usize >= row_count {
        return None;
    }

    let row_rect = ui_table_row_rect(layout, row as usize, scroll);
    if !row_rect.contains(mouse) {
        return None;
    }

    layout
        .columns
        .iter()
        .position(|column| mouse.x >= column.x && mouse.x <= column.x + column.w)
        .map(|column| UiTableCell {
            row: row as usize,
            column,
        })
}

fn work_table_layout(x: f32, y: f32, width: f32) -> UiTableLayout {
    work_table_layout_with_height(x, y, width, work_table_height())
}

fn work_table_layout_with_height(x: f32, y: f32, width: f32, height: f32) -> UiTableLayout {
    ui_table_layout(
        Rect::new(x, y, width, height),
        y + 13.0,
        height,
        WORK_ROW_HEIGHT,
        12.0,
        &[
            ui_column_spec_flex(132.0, 1.0),
            ui_column_spec_content(48.0, 42.0, 58.0),
            ui_column_spec_content(76.0, 68.0, 92.0),
            ui_column_spec_content(46.0, 42.0, 58.0),
            ui_column_spec_content(measure_text("Active", None, 16, 1.0).width, 50.0, 64.0),
        ],
    )
}

fn inventory_table_layout(x: f32, y: f32, width: f32) -> UiTableLayout {
    ui_table_layout(
        Rect::new(x, y, width, work_table_height()),
        y + 13.0,
        work_table_height(),
        INVENTORY_ROW_HEIGHT,
        12.0,
        &[
            ui_column_spec_flex(130.0, 1.0),
            ui_column_spec_content(42.0, 38.0, 58.0),
            ui_column_spec_content(64.0, 58.0, 84.0),
        ],
    )
}

fn npc_interaction_table_layout(x: f32, y: f32, width: f32) -> UiTableLayout {
    ui_table_layout_until_bottom(UiTableBottomLayout {
        x,
        y,
        width,
        row_start_offset: 28.0,
        viewport_bottom: action_table_bottom(),
        row_height: WORK_ROW_HEIGHT,
        column_gap: 12.0,
        columns: &[
            ui_column_spec_flex(150.0, 1.0),
            ui_column_spec_content(82.0, 78.0, 104.0),
        ],
    })
}

fn station_trade_table_layout(x: f32, y: f32, width: f32) -> UiTableLayout {
    ui_table_layout_until_bottom(UiTableBottomLayout {
        x,
        y,
        width,
        row_start_offset: 28.0,
        viewport_bottom: action_table_bottom(),
        row_height: 40.0,
        column_gap: 8.0,
        columns: &[
            ui_column_spec_flex(88.0, 1.0),
            ui_column_spec_content(44.0, 42.0, 54.0),
            ui_column_spec_content(46.0, 44.0, 58.0),
        ],
    })
}

fn recipe_unlock_table_layout(x: f32, y: f32, width: f32) -> UiTableLayout {
    ui_table_layout_until_bottom(UiTableBottomLayout {
        x,
        y,
        width,
        row_start_offset: 28.0,
        viewport_bottom: action_table_bottom(),
        row_height: WORK_ROW_HEIGHT,
        column_gap: 12.0,
        columns: &[
            ui_column_spec_flex(160.0, 1.0),
            ui_column_spec_content(82.0, 72.0, 96.0),
        ],
    })
}

fn action_table_bottom() -> f32 {
    let rail = action_rail_rect(clamp_action_rail_width(OBJECT_ACTION_RAIL_MIN_WIDTH));
    rail.y + rail.h - 12.0
}

fn ship_upgrade_table_layout(x: f32, y: f32, width: f32, viewport_height: f32) -> UiTableLayout {
    ui_table_layout(
        Rect::new(x, y, width, viewport_height),
        y + 18.0,
        viewport_height,
        SHIP_UPGRADE_ROW_HEIGHT,
        12.0,
        &[
            ui_column_spec_flex(220.0, 1.0),
            ui_column_spec_content(48.0, 44.0, 60.0),
            ui_column_spec_flex(220.0, 1.35),
            ui_column_spec_fixed(34.0),
        ],
    )
}

fn skills_table_layout(x: f32, y: f32, width: f32) -> UiTableLayout {
    let row_height = 38.0;
    ui_table_layout(
        Rect::new(x, y, width, SKILL_COUNT as f32 * row_height),
        y + 15.0,
        SKILL_COUNT as f32 * row_height,
        row_height,
        12.0,
        &[
            ui_column_spec_flex(190.0, 1.0),
            ui_column_spec_content(48.0, 44.0, 60.0),
            ui_column_spec_content(100.0, 92.0, 128.0),
            ui_column_spec_fixed(34.0),
        ],
    )
}

fn hovered_work_cell(
    mouse: Vec2,
    row_count: usize,
    scroll: f32,
    action_rail_width: Option<f32>,
) -> Option<(usize, WorkColumn)> {
    let overlay = inventory_overlay_layout(action_rail_width);
    let layout = work_table_layout(
        overlay.production_x,
        work_table_y(),
        overlay.production_width,
    );
    hovered_work_cell_in_layout(mouse, &layout, row_count, scroll)
}

fn hovered_work_cell_with_action_rail(
    mouse: Vec2,
    row_count: usize,
    scroll: f32,
    action_rail_width: f32,
) -> Option<(usize, WorkColumn)> {
    let overlay = inventory_overlay_layout(Some(action_rail_width));
    let rail = overlay.action_rail?;
    let layout = work_table_layout_with_height(
        rail.x + 12.0,
        rail.y + 198.0,
        rail.w - 24.0,
        (rail.h - 252.0).max(WORK_ROW_HEIGHT),
    );
    hovered_work_cell_in_layout(mouse, &layout, row_count, scroll)
}

fn hovered_work_cell_in_layout(
    mouse: Vec2,
    layout: &UiTableLayout,
    row_count: usize,
    scroll: f32,
) -> Option<(usize, WorkColumn)> {
    ui_hovered_table_cell(mouse, layout, row_count, scroll).and_then(|cell| {
        let column = match cell.column {
            0 => WorkColumn::Item,
            1 => WorkColumn::Keep,
            _ => return None,
        };
        Some((cell.row, column))
    })
}

fn work_table_y() -> f32 {
    let panel_height = inventory_panel_height();
    let panel_y = (screen_height() - panel_height) * 0.5 + 18.0;
    panel_y + 66.0
}

fn work_table_height() -> f32 {
    let panel_height = inventory_panel_height();
    panel_height - 104.0
}

fn clicked_production_mode(mouse: Vec2, action_rail_width: Option<f32>) -> Option<ProductionMode> {
    let layout = inventory_overlay_layout(action_rail_width);
    let x = layout.production_x + layout.production_width - 204.0;
    let y = layout.panel_y + 19.0;

    if mouse.y < y || mouse.y > y + 26.0 {
        return None;
    }
    if mouse.x >= x && mouse.x <= x + 58.0 {
        Some(ProductionMode::Smelting)
    } else if mouse.x >= x + 64.0 && mouse.x <= x + 122.0 {
        Some(ProductionMode::Crafting)
    } else if mouse.x >= x + 128.0 && mouse.x <= x + 200.0 {
        Some(ProductionMode::Processing)
    } else {
        None
    }
}

fn draw_production_mode_tabs(mode: ProductionMode, x: f32, y: f32) {
    draw_production_tab("Smelt", ProductionMode::Smelting, mode, x, y, 58.0);
    draw_production_tab("Craft", ProductionMode::Crafting, mode, x + 64.0, y, 58.0);
    draw_production_tab(
        "Process",
        ProductionMode::Processing,
        mode,
        x + 128.0,
        y,
        72.0,
    );
}

fn draw_production_tab(
    label: &str,
    tab_mode: ProductionMode,
    active_mode: ProductionMode,
    x: f32,
    y: f32,
    width: f32,
) {
    let active = tab_mode == active_mode;
    draw_rectangle(
        x,
        y,
        width,
        26.0,
        if active {
            Color::from_rgba(20, 46, 48, 205)
        } else {
            Color::from_rgba(8, 16, 22, 130)
        },
    );
    draw_rectangle_lines(
        x,
        y,
        width,
        26.0,
        1.0,
        if active {
            Color::from_rgba(150, 221, 226, 180)
        } else {
            Color::from_rgba(82, 114, 124, 120)
        },
    );
    draw_text(
        label,
        x + 9.0,
        y + 18.0,
        16.0,
        if active {
            Color::from_rgba(205, 226, 230, 255)
        } else {
            Color::from_rgba(126, 156, 164, 220)
        },
    );
}

fn draw_production_text_table(
    game: &GameState,
    x: f32,
    y: f32,
    width: f32,
    scroll: f32,
    mouse: Vec2,
) {
    let (recipes, settings) = match game.production_mode {
        ProductionMode::Smelting => (&game.smelt_recipes, &game.smelt_settings),
        ProductionMode::Crafting => (&game.craft_recipes, &game.craft_settings),
        ProductionMode::Processing => (&game.processing_recipes, &game.processing_settings),
    };
    let active_row = next_recipe_bill_index(game, recipes, settings);
    let rows = recipes
        .iter()
        .zip(settings.iter())
        .enumerate()
        .map(|(index, (recipe, setting))| {
            let can_craft = game.inventory.can_craft(recipe);
            let unlocked = recipe_is_unlocked(game, &recipe.id);
            let current = game.inventory.count(&recipe.output.item);
            WorkRow {
                item: recipe_row_label(recipes, index),
                keep: setting.keep,
                enabled: unlocked && can_craft,
                status: if unlocked {
                    work_row_status(current, setting.keep, setting.queued)
                } else {
                    "Locked".to_string()
                },
                percent: if active_row == Some(index) {
                    format!("{:.0}%", setting.progress() * 100.0)
                } else {
                    String::new()
                },
                active: active_row == Some(index),
            }
        })
        .collect::<Vec<_>>();
    draw_work_text_table(&rows, x, y, width, scroll, mouse);
}

fn hovered_production_recipe(game: &GameState, mouse: Vec2, scroll: f32) -> Option<&Recipe> {
    let recipes = match game.production_mode {
        ProductionMode::Smelting => &game.smelt_recipes,
        ProductionMode::Crafting => &game.craft_recipes,
        ProductionMode::Processing => &game.processing_recipes,
    };
    if let Some((recipe_index, _)) = hovered_work_cell(
        mouse,
        recipes.len(),
        scroll,
        selected_action_rail_width(game),
    ) {
        return recipes.get(recipe_index);
    }

    None
}

fn recipe_row_label(recipes: &[Recipe], recipe_index: usize) -> String {
    let Some(recipe) = recipes.get(recipe_index) else {
        return String::new();
    };
    if recipes
        .iter()
        .filter(|other| other.output.item.id == recipe.output.item.id)
        .count()
        <= 1
    {
        return recipe.output.item.name.clone();
    }

    recipe_variant_label(recipe)
}

fn recipe_variant_label(recipe: &Recipe) -> String {
    let recipe_local_id = local_content_id(&recipe.id);
    let output_local_id = local_content_id(&recipe.output.item.id);
    let suffix = format!("_{output_local_id}");
    let variant = recipe_local_id
        .strip_suffix(&suffix)
        .filter(|label| !label.is_empty())
        .unwrap_or(recipe_local_id)
        .replace('_', " ");

    if variant == output_local_id.replace('_', " ") {
        recipe.output.item.name.clone()
    } else {
        format!("{} ({variant})", recipe.output.item.name)
    }
}

fn local_content_id(id: &str) -> &str {
    id.rsplit_once(':')
        .map(|(_, local_id)| local_id)
        .unwrap_or(id)
}

fn faction_name<'a>(
    content_registry: &'a content::ContentRegistry,
    faction_id: &'a str,
) -> &'a str {
    content_registry
        .factions
        .get(faction_id)
        .map(|faction| faction.name.as_str())
        .unwrap_or_else(|| local_content_id(faction_id))
}

fn faction_disposition_label(
    content_registry: &content::ContentRegistry,
    faction_id: &str,
) -> &'static str {
    content_registry
        .factions
        .get(faction_id)
        .map(|faction| match faction.default_disposition {
            content::FactionDisposition::Friendly => "friendly",
            content::FactionDisposition::Neutral => "neutral",
            content::FactionDisposition::Hostile => "hostile",
            content::FactionDisposition::Unknown => "unknown",
        })
        .unwrap_or("unknown")
}

fn faction_color(
    content_registry: &content::ContentRegistry,
    faction_id: &str,
    alpha: u8,
) -> Color {
    content_registry
        .factions
        .get(faction_id)
        .map(|faction| {
            Color::from_rgba(faction.color[0], faction.color[1], faction.color[2], alpha)
        })
        .unwrap_or_else(|| Color::from_rgba(205, 226, 230, alpha))
}

fn draw_recipe_tooltip(recipe: &Recipe, inventory: &Inventory, mouse: Vec2) {
    let row_height = 24.0;
    let width = 286.0;
    let height = 74.0 + recipe.ingredients.len() as f32 * row_height;
    let x = (mouse.x + 18.0)
        .min(screen_width() - width - 18.0)
        .max(18.0);
    let y = (mouse.y + 18.0)
        .min(screen_height() - height - 18.0)
        .max(18.0);
    let panel = Color::from_rgba(2, 6, 10, 255);
    let border = Color::from_rgba(112, 151, 163, 170);
    let label = Color::from_rgba(126, 156, 164, 220);
    let text = Color::from_rgba(205, 226, 230, 255);
    let active = Color::from_rgba(150, 221, 226, 255);
    let unavailable = Color::from_rgba(226, 190, 150, 245);

    draw_rectangle(x, y, width, height, panel);
    draw_rectangle_lines(x, y, width, height, 1.0, border);
    draw_text(
        &recipe_variant_label(recipe),
        x + 14.0,
        y + 28.0,
        21.0,
        text,
    );
    draw_text("Requires", x + 14.0, y + 54.0, 15.0, label);

    for (index, ingredient) in recipe.ingredients.iter().enumerate() {
        let row_y = y + 82.0 + index as f32 * row_height;
        let have = inventory.count(&ingredient.item);
        let enough = have >= ingredient.count;
        draw_text(
            &ingredient.item.name,
            x + 14.0,
            row_y,
            18.0,
            if enough { text } else { unavailable },
        );
        let count_label = format!("{have} / {}", ingredient.count);
        let measure = measure_text(&count_label, None, 18, 1.0);
        draw_text(
            &count_label,
            x + width - 14.0 - measure.width,
            row_y,
            18.0,
            if enough { active } else { unavailable },
        );
    }
}

fn draw_mining_text_table_with_alignment(
    inventory: &Inventory,
    planet: &Planet,
    in_range: bool,
    rect: Rect,
    scroll: f32,
    mouse: Vec2,
    item_alignment: WorkTableItemAlignment,
) {
    let Rect {
        x,
        y,
        w: width,
        h: height,
    } = rect;

    if !planet_has_composition_scan(planet) {
        draw_work_text_table_in_rect_with_alignment(&[], rect, scroll, mouse, item_alignment);
        draw_wrapped_text(
            "Planet composition unknown. Launch survey drones from the planet panel to reveal mineable resources and unlock mining.",
            x,
            y + 92.0,
            width,
            18,
            Color::from_rgba(205, 226, 230, 210),
        );
        return;
    }

    let active_row = in_range
        .then(|| next_mining_bill_index(inventory, planet))
        .flatten();
    let rows = planet
        .info
        .mineables
        .iter()
        .zip(planet.mining.iter())
        .enumerate()
        .map(|(index, (mineable, setting))| {
            let current = inventory.count(&mineable.item);
            let percent = if planet_has_richness_scan(planet) {
                format!(
                    "{:.0}%",
                    mineable_richness_multiplier(planet, index) * 100.0
                )
            } else {
                String::new()
            };
            WorkRow {
                item: mineable.item.name.clone(),
                keep: setting.keep,
                enabled: in_range,
                status: work_row_status(current, setting.keep, setting.queued),
                percent,
                active: active_row == Some(index),
            }
        })
        .collect::<Vec<_>>();
    draw_work_text_table_in_rect_with_alignment(
        &rows,
        Rect::new(x, y, width, height),
        scroll,
        mouse,
        item_alignment,
    );
}

fn draw_compact_list(lines: &[&str], x: f32, y: f32, max_width: f32, font_size: u16, color: Color) {
    let mut cursor_x = x;
    let mut cursor_y = y;

    for line in lines.iter().take(3) {
        let label = if cursor_x == x {
            (*line).to_string()
        } else {
            format!("/ {line}")
        };
        let width = measure_text(&label, None, font_size, 1.0).width;
        if cursor_x + width > x + max_width {
            cursor_x = x;
            cursor_y += font_size as f32 + 5.0;
        }
        draw_text(&label, cursor_x, cursor_y, font_size as f32, color);
        cursor_x += width + 10.0;
    }
}

fn draw_detail_panel(game: &GameState, x: f32, y: f32, width: f32) {
    if let Some(planet_index) = game.selected_planet {
        if let Some(planet) = game.planets.get(planet_index) {
            draw_planet_detail(
                &game.content_registry,
                planet,
                planet_in_interaction_range(&game.ship, planet),
                game.orbiting_planet == Some(planet_index),
                x,
                y,
                width,
            );
            return;
        }
    }

    if let Some(station_index) = game.selected_station {
        if let Some(station) = game.stations.get(station_index) {
            draw_station_detail(StationDetailRender {
                content_registry: &game.content_registry,
                station,
                selected_service: game.selected_station_service,
                in_range: station_in_interaction_range(&game.ship, station),
                distance: station_surface_distance(&game.ship, station),
                x,
                y,
                width,
            });
            return;
        }
    }

    if let Some(npc_ship_index) = game.selected_npc_ship {
        if let Some(npc_ship) = game.npc_ships.get(npc_ship_index) {
            draw_npc_ship_detail(&game.content_registry, &game.ship, npc_ship, x, y, width);
            return;
        }
    }

    draw_ship_detail(ShipDetailView {
        ship: &game.ship,
        power_modules: &game.installed_power_modules,
        shields: &game.equipped_shields,
        weapons: &game.equipped_weapons,
        threats: &game.defense_threats,
        cargo_mass: game.inventory.total_mass(),
        cargo_capacity: cargo_rating_kg(&game.ship_upgrades),
        current_system_id: &game.current_system_id,
        shield_recharge_delay_remaining: game.shield_recharge_delay_remaining,
        texture: game.ship_texture.as_ref(),
        x,
        y,
        width,
    });
}

fn draw_npc_ship_interaction_list(
    content_registry: &content::ContentRegistry,
    ship: &Ship,
    npc_ship: &NpcShip,
    rect: Rect,
) {
    let Rect { x, y, w: width, .. } = rect;
    let layout = npc_interaction_table_layout(x, y, width);
    let action_column = layout.columns[0];
    let status_column = layout.columns[1];
    draw_text(
        "Action",
        action_column.x,
        y + 16.0,
        14.0,
        Color::from_rgba(168, 204, 210, 255),
    );
    draw_text(
        "Status",
        status_column.x,
        y + 16.0,
        14.0,
        Color::from_rgba(168, 204, 210, 255),
    );
    draw_line(
        x,
        y + 24.0,
        x + width,
        y + 24.0,
        1.0,
        Color::from_rgba(96, 137, 150, 220),
    );

    let rows = npc_interaction_rows(content_registry, ship, npc_ship);
    for (index, row) in rows.iter().enumerate() {
        let row_rect = ui_table_row_rect(&layout, index, 0.0);
        if !ui_table_row_visible(&layout, row_rect) {
            continue;
        }
        let hovered = row_rect.contains(mouse_vec2());
        draw_rectangle(
            row_rect.x,
            row_rect.y,
            row_rect.w,
            row_rect.h,
            if hovered {
                Color::from_rgba(13, 32, 40, 210)
            } else if index % 2 == 0 {
                Color::from_rgba(8, 18, 24, 118)
            } else {
                Color::from_rgba(6, 12, 18, 82)
            },
        );
        let color = match row.state {
            NpcInteractionState::Available => Color::from_rgba(150, 221, 226, 255),
            NpcInteractionState::Complete => Color::from_rgba(235, 242, 226, 255),
            NpcInteractionState::Unavailable => Color::from_rgba(126, 143, 148, 220),
        };
        draw_text(
            &fit_debug_text(row.action.label(), action_column.w, 16),
            action_column.x,
            row_rect.y + 20.0,
            16.0,
            color,
        );
        draw_text(
            &fit_debug_text(row.status, status_column.w, 15),
            status_column.x,
            row_rect.y + 20.0,
            15.0,
            color,
        );
    }

    let hint = if npc_ship.identified {
        "Known contacts expose available hooks. Disabled rows indicate systems that are not implemented yet."
    } else if npc_ship_in_interaction_range(ship, npc_ship) {
        "Identify this contact to reveal faction, loadout, and supported interaction hooks."
    } else {
        "Approach this contact to identify it."
    };
    draw_wrapped_text(
        hint,
        x + 6.0,
        y + 28.0 + rows.len() as f32 * WORK_ROW_HEIGHT + 22.0,
        width - 12.0,
        15,
        Color::from_rgba(178, 197, 203, 235),
    );
}

fn draw_station_service_list(render: StationActionRailRender<'_>) {
    let StationActionRailRender {
        station,
        selected_service,
        in_range,
        credits,
        inventory,
        purchased_unlocks,
        action_rail_width,
    } = render;
    let layout = station_action_layout(station, action_rail_width);
    let services = layout.services;
    let detail = layout.detail;
    let header = Color::from_rgba(168, 204, 210, 255);
    let accent = Color::from_rgba(150, 221, 226, 255);
    let warning = Color::from_rgba(226, 190, 150, 255);

    draw_text("Shops", services.x, services.y + 16.0, 14.0, header);
    draw_text("Items", detail.x, detail.y + 16.0, 14.0, header);
    draw_line(
        services.x,
        services.y + 24.0,
        services.x + services.w,
        services.y + 24.0,
        1.0,
        Color::from_rgba(96, 137, 150, 220),
    );
    draw_line(
        detail.x,
        detail.y + 24.0,
        detail.x + detail.w,
        detail.y + 24.0,
        1.0,
        Color::from_rgba(96, 137, 150, 220),
    );
    draw_vertical_dotted_line(
        services.x + services.w + 8.0,
        services.y,
        services.y + services.h,
        0.5,
        5.0,
        6.0,
        Color::from_rgba(96, 137, 150, 100),
    );

    if station.services.is_empty() {
        draw_text(
            "No services declared",
            services.x,
            services.y + 48.0,
            16.0,
            warning,
        );
        return;
    }

    for (index, service) in station.services.iter().enumerate() {
        let row = station_service_button_rect(station, index, action_rail_width);
        let selected = selected_service == Some(index);
        let hovered = row.contains(mouse_vec2());
        draw_rectangle(
            row.x,
            row.y,
            row.w,
            row.h,
            if selected {
                Color::from_rgba(24, 58, 66, 230)
            } else if hovered {
                Color::from_rgba(13, 32, 40, 210)
            } else if index % 2 == 0 {
                Color::from_rgba(8, 18, 24, 118)
            } else {
                Color::from_rgba(6, 12, 18, 82)
            },
        );
        draw_rectangle_lines(
            row.x,
            row.y,
            row.w,
            row.h,
            1.0,
            if selected {
                Color::from_rgba(150, 221, 226, 170)
            } else {
                Color::from_rgba(82, 114, 124, 105)
            },
        );
        draw_text(
            &fit_debug_text(&service.name, row.w - 14.0, 16),
            row.x + 7.0,
            row.y + 20.0,
            16.0,
            if selected {
                Color::from_rgba(235, 242, 226, 255)
            } else {
                Color::from_rgba(205, 226, 230, 255)
            },
        );
        draw_text(
            &fit_debug_text(&service.kind, row.w - 14.0, 13),
            row.x + 7.0,
            row.y + 31.0,
            13.0,
            if selected {
                accent
            } else {
                Color::from_rgba(126, 156, 164, 220)
            },
        );
    }

    if let Some(service) = selected_service.and_then(|index| station.services.get(index)) {
        let status = if in_range {
            "Ready"
        } else {
            "Approach to trade"
        };
        draw_text(
            &fit_debug_text(&format!("{} / {}", service.name, status), detail.w, 16),
            detail.x,
            detail.y + 48.0,
            16.0,
            if in_range { accent } else { warning },
        );
        draw_station_trade_table(StationTradeTableRender {
            station,
            service,
            in_range,
            credits,
            inventory,
            action_rail_width,
            x: detail.x,
            width: detail.w,
        });
        draw_recipe_unlock_table(RecipeUnlockTableRender {
            station,
            service,
            in_range,
            credits,
            purchased_unlocks,
            action_rail_width,
            x: detail.x,
            width: detail.w,
        });
    } else {
        draw_text("Select a shop", detail.x, detail.y + 48.0, 16.0, warning);
    }
}

fn draw_station_trade_table(render: StationTradeTableRender<'_>) {
    let StationTradeTableRender {
        station,
        service,
        in_range,
        credits,
        inventory,
        action_rail_width,
        x,
        width,
    } = render;
    let y = station_trade_table_y(station, action_rail_width);
    let layout = station_trade_table_layout(x, y, width);
    let item_column = layout.columns[0];
    let buy_column = layout.columns[1];
    let sell_column = layout.columns[2];
    let mouse = mouse_vec2();
    draw_text(
        "Item",
        item_column.x,
        y + 16.0,
        14.0,
        Color::from_rgba(168, 204, 210, 255),
    );
    draw_text(
        "Buy",
        buy_column.x,
        y + 16.0,
        14.0,
        Color::from_rgba(168, 204, 210, 255),
    );
    draw_text(
        "Sell",
        sell_column.x,
        y + 16.0,
        14.0,
        Color::from_rgba(168, 204, 210, 255),
    );
    draw_line(
        x,
        y + 24.0,
        x + width,
        y + 24.0,
        1.0,
        Color::from_rgba(96, 137, 150, 220),
    );

    if service.trade.is_empty() {
        draw_text(
            "No trade stock declared",
            x + 6.0,
            y + 48.0,
            16.0,
            Color::from_rgba(226, 190, 150, 255),
        );
        return;
    }

    for (index, offer) in service.trade.iter().enumerate() {
        let row = ui_table_row_rect(&layout, index, 0.0);
        if !ui_table_row_visible(&layout, row) {
            continue;
        }
        let hovered = row.contains(mouse);
        let buy_rect = Rect::new(buy_column.x, row.y + 8.0, buy_column.w, 22.0);
        let sell_rect = Rect::new(sell_column.x, row.y + 8.0, sell_column.w, 22.0);
        let can_buy =
            in_range && !offer.unavailable && offer.stock != Some(0) && credits >= offer.buy_price;
        let cargo_count = inventory.count(&offer.item);
        let can_sell = in_range && cargo_count > 0;
        draw_rectangle(
            row.x,
            row.y,
            row.w,
            row.h,
            if hovered {
                Color::from_rgba(13, 32, 40, 210)
            } else if index % 2 == 0 {
                Color::from_rgba(8, 18, 24, 118)
            } else {
                Color::from_rgba(6, 12, 18, 82)
            },
        );
        draw_text(
            &fit_debug_text(&offer.item.name, item_column.w, 15),
            item_column.x,
            row.y + 17.0,
            15.0,
            Color::from_rgba(205, 226, 230, 255),
        );
        let detail = format!(
            "Station {}  Cargo {}  Buy {} / Sell {}",
            format_trade_stock(offer),
            cargo_count,
            offer.buy_price,
            offer.sell_price
        );
        let detail_color = if offer.unavailable || offer.stock == Some(0) {
            Color::from_rgba(226, 190, 150, 230)
        } else {
            Color::from_rgba(126, 156, 164, 220)
        };
        draw_text(
            &fit_debug_text(&detail, item_column.w, 12),
            item_column.x,
            row.y + 32.0,
            12.0,
            detail_color,
        );
        draw_trade_action_button(buy_rect, "Buy", can_buy, buy_rect.contains(mouse));
        draw_trade_action_button(sell_rect, "Sell", can_sell, sell_rect.contains(mouse));
    }
}

fn draw_trade_action_button(rect: Rect, label: &str, enabled: bool, hovered: bool) {
    let fill = if enabled && hovered {
        Color::from_rgba(30, 75, 83, 235)
    } else if enabled {
        Color::from_rgba(16, 42, 50, 220)
    } else {
        Color::from_rgba(8, 18, 24, 120)
    };
    let stroke = if enabled {
        Color::from_rgba(150, 221, 226, 155)
    } else {
        Color::from_rgba(82, 114, 124, 90)
    };
    let text = if enabled {
        Color::from_rgba(205, 226, 230, 255)
    } else {
        Color::from_rgba(126, 143, 148, 210)
    };

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, stroke);
    draw_text(
        &fit_debug_text(label, rect.w - 8.0, 14),
        rect.x + 4.0,
        rect.y + 14.0,
        14.0,
        text,
    );
}

fn station_trade_table_y(station: &StationDestination, action_rail_width: f32) -> f32 {
    let layout = station_action_layout(station, action_rail_width);
    layout.detail.y + 70.0
}

fn format_trade_stock(offer: &TradeOffer) -> String {
    if offer.unavailable {
        return "unavail".to_string();
    }
    let stock = offer
        .stock
        .map(|stock| stock.to_string())
        .unwrap_or_else(|| "open".to_string());
    if let Some(restock_days) = offer.restock_days {
        format!("{stock}/{restock_days:.0}d")
    } else {
        stock
    }
}

fn draw_recipe_unlock_table(render: RecipeUnlockTableRender<'_>) {
    let RecipeUnlockTableRender {
        station,
        service,
        in_range,
        credits,
        purchased_unlocks,
        action_rail_width,
        x,
        width,
    } = render;
    if service.recipe_unlocks.is_empty() {
        return;
    }
    let y = recipe_unlock_table_y(station, service, action_rail_width);
    let layout = recipe_unlock_table_layout(x, y, width);
    let recipe_column = layout.columns[0];
    let price_column = layout.columns[1];
    draw_text(
        "Recipe unlocks",
        recipe_column.x,
        y + 16.0,
        14.0,
        Color::from_rgba(168, 204, 210, 255),
    );
    draw_text(
        "Left purchase",
        x + width - 118.0,
        y + 16.0,
        14.0,
        Color::from_rgba(126, 156, 164, 220),
    );
    draw_line(
        x,
        y + 24.0,
        x + width,
        y + 24.0,
        1.0,
        Color::from_rgba(96, 137, 150, 220),
    );
    for (index, unlock) in service.recipe_unlocks.iter().enumerate() {
        let row = ui_table_row_rect(&layout, index, 0.0);
        if !ui_table_row_visible(&layout, row) {
            continue;
        }
        let hovered = row.contains(mouse_vec2());
        let purchased = purchased_unlocks.contains(&unlock.recipe);
        let affordable = in_range && !unlock.unavailable && credits >= unlock.price && !purchased;
        draw_rectangle(
            row.x,
            row.y,
            row.w,
            row.h,
            if hovered {
                Color::from_rgba(13, 32, 40, 210)
            } else if index % 2 == 0 {
                Color::from_rgba(8, 18, 24, 118)
            } else {
                Color::from_rgba(6, 12, 18, 82)
            },
        );
        draw_text(
            &fit_debug_text(&unlock.recipe, recipe_column.w, 15),
            recipe_column.x,
            row.y + 20.0,
            15.0,
            if unlock.unavailable {
                Color::from_rgba(126, 143, 148, 220)
            } else {
                Color::from_rgba(205, 226, 230, 255)
            },
        );
        let price_label = if purchased {
            "Owned".to_string()
        } else {
            unlock.price.to_string()
        };
        draw_text(
            &fit_debug_text(&price_label, price_column.w, 15),
            price_column.x,
            row.y + 20.0,
            15.0,
            if affordable {
                Color::from_rgba(150, 221, 226, 255)
            } else {
                Color::from_rgba(226, 190, 150, 255)
            },
        );
    }
}

fn recipe_unlock_table_y(
    station: &StationDestination,
    service: &StationService,
    action_rail_width: f32,
) -> f32 {
    station_trade_table_y(station, action_rail_width)
        + 28.0
        + service.trade.len() as f32 * 40.0
        + if service.trade.is_empty() { 42.0 } else { 20.0 }
}

fn draw_station_detail(render: StationDetailRender<'_>) {
    let StationDetailRender {
        content_registry,
        station,
        selected_service,
        in_range,
        distance,
        x,
        y,
        width,
    } = render;
    let label = Color::from_rgba(88, 116, 126, 180);
    let text = Color::from_rgba(205, 226, 230, 255);
    let warning = Color::from_rgba(226, 190, 150, 255);
    let preview_size = 118.0;

    draw_text(
        "Station",
        x,
        y - 28.0,
        26.0,
        Color::from_rgba(235, 242, 226, 255),
    );

    if let Some(texture) = &station.texture {
        draw_texture_ex(
            texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(preview_size, preview_size)),
                ..Default::default()
            },
        );
    } else {
        draw_station_icon(
            vec2(x + preview_size * 0.5, y + preview_size * 0.5),
            preview_size * 0.46,
            &station.icon,
        );
    }

    let data_y = y + preview_size + 34.0;
    draw_text(&station.name, x, data_y, 18.0, text);
    draw_text(
        &fit_debug_text(
            &format!("{} / dock range {:.0}u", station.id, distance),
            width,
            16,
        ),
        x,
        data_y + 30.0,
        16.0,
        if in_range {
            Color::from_rgba(150, 221, 226, 255)
        } else {
            warning
        },
    );
    let mut detail_y = draw_wrapped_text(&station.summary, x, data_y + 58.0, width, 16, text);
    detail_y += 24.0;

    draw_text("Ownership", x, detail_y, 16.0, label);
    let ownership = match (&station.faction, &station.culture) {
        (Some(faction), Some(culture)) => format!(
            "{} / {} / {}",
            faction_name(content_registry, faction),
            faction_name(content_registry, culture),
            faction_disposition_label(content_registry, faction)
        ),
        (Some(faction), None) => format!(
            "{} / {}",
            faction_name(content_registry, faction),
            faction_disposition_label(content_registry, faction)
        ),
        (None, Some(culture)) => faction_name(content_registry, culture).to_string(),
        (None, None) => "Independent".to_string(),
    };
    draw_text(
        &fit_debug_text(&ownership, width, 16),
        x,
        detail_y + 26.0,
        16.0,
        text,
    );

    let services_y = detail_y + 64.0;
    draw_text("Services", x, services_y, 16.0, label);
    if station.services.is_empty() {
        draw_text(
            "No service groups declared",
            x,
            services_y + 26.0,
            16.0,
            warning,
        );
    } else {
        let service_names = station
            .services
            .iter()
            .map(|service| service.name.as_str())
            .collect::<Vec<_>>();
        draw_compact_list(&service_names, x, services_y + 27.0, width, 16, text);
    }

    let selected_y = services_y + 90.0;
    draw_text("Selected service", x, selected_y, 16.0, label);
    if let Some(service) = selected_service.and_then(|index| station.services.get(index)) {
        draw_text(
            &fit_debug_text(
                &format!("{} / {} / {}", service.name, service.kind, service.id),
                width,
                17,
            ),
            x,
            selected_y + 27.0,
            17.0,
            Color::from_rgba(235, 242, 226, 255),
        );
        let status = if !in_range {
            "Approach within dock range to use this service group."
        } else if service.trade.is_empty() {
            "Service group available. Detailed actions are staged for future garage, vendor, social, and contract systems."
        } else {
            "Trade stock available. Left-click an offer to buy one unit, or right-click to sell one unit from cargo."
        };
        let after_status = draw_wrapped_text(
            status,
            x,
            selected_y + 54.0,
            width,
            15,
            if in_range {
                Color::from_rgba(150, 221, 226, 255)
            } else {
                warning
            },
        );
        if let Some(description) = service.description.as_deref() {
            draw_wrapped_text(description, x, after_status + 16.0, width, 15, text);
        }
    } else {
        draw_text(
            "Select a service group from the station list.",
            x,
            selected_y + 27.0,
            16.0,
            warning,
        );
    }
}

fn draw_planet_detail(
    content_registry: &content::ContentRegistry,
    planet: &Planet,
    in_range: bool,
    is_orbiting: bool,
    x: f32,
    y: f32,
    width: f32,
) {
    let label = Color::from_rgba(88, 116, 126, 180);
    let text = Color::from_rgba(205, 226, 230, 255);
    let warning = Color::from_rgba(226, 190, 150, 255);
    let unavailable = Color::from_rgba(108, 127, 132, 190);
    let preview_size = 118.0;

    draw_text(
        "Planet",
        x,
        y - 28.0,
        26.0,
        Color::from_rgba(235, 242, 226, 255),
    );

    if let Some(texture) = &planet.texture {
        draw_texture_ex(
            texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(preview_size, preview_size)),
                ..Default::default()
            },
        );
    } else {
        draw_circle(
            x + preview_size * 0.5,
            y + preview_size * 0.5,
            preview_size * 0.5,
            Color::from_rgba(65, 136, 154, 255),
        );
    }

    let data_y = y + preview_size + 34.0;
    let title = if planet_has_surface_scan(planet) {
        planet.info.classification.as_str()
    } else {
        "Unscanned body"
    };
    draw_text(title, x, data_y, 18.0, text);
    draw_text("Survey status", x, data_y + 32.0, 16.0, label);
    let summary = if planet_has_surface_scan(planet) {
        planet.info.summary.as_str()
    } else {
        "Classification, hazards, composition, and landing profiles are unknown. Launch survey drones while in range to build the planet record."
    };
    let after_scan = draw_wrapped_text(summary, x, data_y + 56.0, width, 16, text);

    let mut next_y = after_scan.max(data_y + 124.0);
    if planet_has_surface_scan(planet) {
        draw_text("Ownership", x, next_y, 16.0, label);
        let ownership = planet
            .faction
            .as_deref()
            .map(|faction| {
                format!(
                    "{} / {}",
                    faction_name(content_registry, faction),
                    faction_disposition_label(content_registry, faction)
                )
            })
            .unwrap_or_else(|| "Unclaimed".to_string());
        draw_text(
            &fit_debug_text(&ownership, width, 16),
            x,
            next_y + 27.0,
            16.0,
            text,
        );
        next_y += 64.0;
    }

    let mine_y = next_y;
    draw_text("Mineable", x, mine_y, 16.0, label);
    if planet_has_composition_scan(planet) {
        let mineable_names = planet
            .info
            .mineables
            .iter()
            .map(|mineable| mineable.item.name.as_str())
            .collect::<Vec<_>>();
        draw_compact_list(&mineable_names, x, mine_y + 27.0, width, 16, text);
    } else {
        draw_text(
            "Unknown until surveyed",
            x,
            mine_y + 27.0,
            16.0,
            unavailable,
        );
    }

    let hazards_y = mine_y + 86.0;
    draw_text("Surface readings", x, hazards_y, 16.0, label);
    if planet_has_surface_scan(planet) {
        let hazards = planet
            .info
            .hazards
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        draw_compact_list(&hazards, x, hazards_y + 27.0, width, 16, warning);
        let shield_drain = planet_hazard_shield_drain_per_second(planet);
        let slowdown = planet_hazard_mining_slowdown(planet);
        if shield_drain > 0.0 || slowdown > 1.0 {
            draw_text(
                &format!(
                    "Mining hazard: {:.0}% speed / {:.1} shield/s",
                    100.0 / slowdown,
                    shield_drain
                ),
                x,
                hazards_y + 68.0,
                15.0,
                warning,
            );
        }
    } else {
        draw_text(
            "No survey telemetry",
            x,
            hazards_y + 27.0,
            16.0,
            unavailable,
        );
    }

    let richness_y = hazards_y + 96.0;
    draw_text("Resource richness", x, richness_y, 16.0, label);
    let richness_text = if planet_has_richness_scan(planet) {
        let min = planet
            .info
            .mineables
            .iter()
            .enumerate()
            .map(|(index, _)| mineable_richness_multiplier(planet, index))
            .fold(f32::INFINITY, f32::min);
        let max = planet_richness_multiplier(planet);
        format!(
            "{:.0}-{:.0}% extraction by resource",
            min * 100.0,
            max * 100.0
        )
    } else {
        "Unknown until deep scan".to_string()
    };
    draw_text(
        &richness_text,
        x,
        richness_y + 26.0,
        18.0,
        if planet_has_richness_scan(planet) {
            Color::from_rgba(150, 221, 226, 255)
        } else {
            unavailable
        },
    );

    let status_y = richness_y + 66.0;
    draw_text("Proximity", x, status_y, 16.0, label);
    draw_text(
        if is_orbiting {
            "Stable orbit locked"
        } else if in_range {
            "Docking envelope reached"
        } else {
            "Approach required for mining"
        },
        x,
        status_y + 26.0,
        18.0,
        if is_orbiting || in_range {
            Color::from_rgba(150, 221, 226, 255)
        } else {
            warning
        },
    );
}

fn draw_planet_action_rail(render: PlanetActionRailRender<'_>) {
    let PlanetActionRailRender {
        content_registry,
        planet,
        inventory,
        ship_upgrades,
        action_rail_width,
        is_orbiting,
        in_range,
        scroll,
        mouse,
    } = render;
    let rail_width = action_rail_width;
    let rail = action_rail_rect(rail_width);
    let text = Color::from_rgba(205, 226, 230, 255);
    let unavailable = Color::from_rgba(108, 127, 132, 190);

    draw_action_rail_frame(rail, "Actions");
    draw_text(
        &format!("Scan level {}/{}", planet.scan_level, MAX_SCAN_LEVEL),
        rail.x + 10.0,
        rail.y + 50.0,
        16.0,
        text,
    );

    let orbit_button = planet_orbit_button_rect(rail_width);
    let orbit_enabled = in_range && !is_orbiting;
    let orbit_hovered = orbit_button.contains(mouse);
    let orbit_button_color = if orbit_enabled {
        if orbit_hovered {
            Color::from_rgba(42, 86, 88, 230)
        } else {
            Color::from_rgba(24, 58, 66, 210)
        }
    } else {
        Color::from_rgba(22, 32, 38, 190)
    };
    draw_rectangle(
        orbit_button.x,
        orbit_button.y,
        orbit_button.w,
        orbit_button.h,
        orbit_button_color,
    );
    draw_rectangle_lines(
        orbit_button.x,
        orbit_button.y,
        orbit_button.w,
        orbit_button.h,
        1.0,
        if orbit_enabled || is_orbiting {
            Color::from_rgba(150, 221, 226, 190)
        } else {
            Color::from_rgba(82, 114, 124, 110)
        },
    );
    let orbit_label = if is_orbiting {
        "In orbit"
    } else if in_range {
        "Orbit"
    } else {
        "Approach"
    };
    let orbit_measure = measure_text(orbit_label, None, 17, 1.0);
    draw_text(
        orbit_label,
        orbit_button.x + (orbit_button.w - orbit_measure.width) * 0.5,
        orbit_button.y + 23.0,
        17.0,
        if orbit_enabled || is_orbiting {
            text
        } else {
            unavailable
        },
    );

    let drone_count = core_item(content_registry, "survey_drone")
        .map(|survey_drone| inventory.count(&survey_drone))
        .unwrap_or(0);
    let improved_drone_count = core_item(content_registry, "improved_survey_drone")
        .map(|survey_drone| inventory.count(&survey_drone))
        .unwrap_or(0);
    if planet.scan_level >= MAX_SCAN_LEVEL {
        draw_text("Survey complete", rail.x + 10.0, rail.y + 142.0, 17.0, text);
    } else {
        let button = planet_scan_button_rect(rail_width);
        let has_drone = drone_count > 0 || improved_drone_count > 0;
        let enabled = in_range && has_drone;
        let hovered = button.contains(mouse);
        let button_color = if enabled {
            if hovered {
                Color::from_rgba(42, 86, 88, 230)
            } else {
                Color::from_rgba(24, 58, 66, 210)
            }
        } else {
            Color::from_rgba(22, 32, 38, 190)
        };

        draw_rectangle(button.x, button.y, button.w, button.h, button_color);
        draw_rectangle_lines(
            button.x,
            button.y,
            button.w,
            button.h,
            1.0,
            if enabled {
                Color::from_rgba(150, 221, 226, 190)
            } else {
                Color::from_rgba(82, 114, 124, 110)
            },
        );
        let button_label = if !in_range {
            "Approach"
        } else if !has_drone {
            "No drones"
        } else {
            "Send drone"
        };
        let measure = measure_text(button_label, None, 17, 1.0);
        draw_text(
            button_label,
            button.x + (button.w - measure.width) * 0.5,
            button.y + 23.0,
            17.0,
            if enabled { text } else { unavailable },
        );
    }
    draw_text(
        "Mining",
        rail.x + 10.0,
        rail.y + 178.0,
        16.0,
        Color::from_rgba(88, 116, 126, 180),
    );
    if planet_has_composition_scan(planet) {
        draw_mining_text_table_with_alignment(
            inventory,
            planet,
            in_range,
            Rect::new(
                rail.x + 12.0,
                rail.y + 198.0,
                rail.w - 24.0,
                (rail.h - 252.0).max(WORK_ROW_HEIGHT),
            ),
            scroll,
            mouse,
            WorkTableItemAlignment::Left,
        );
    } else {
        draw_wrapped_text(
            "Survey composition before mining actions become available.",
            rail.x + 12.0,
            rail.y + 202.0,
            rail.w - 24.0,
            15,
            unavailable,
        );
    }
    draw_text(
        &format!("Basic: {drone_count}   Improved: {improved_drone_count}"),
        rail.x + 10.0,
        rail.y + rail.h - 12.0,
        15.0,
        Color::from_rgba(88, 116, 126, 180),
    );
    draw_text(
        &format!(
            "Return: {:.0}%",
            survey_drone_return_chance(ship_upgrades) * 100.0
        ),
        rail.x + 10.0,
        rail.y + rail.h - 32.0,
        15.0,
        Color::from_rgba(88, 116, 126, 180),
    );
}

fn draw_npc_ship_detail(
    content_registry: &content::ContentRegistry,
    ship: &Ship,
    npc_ship: &NpcShip,
    x: f32,
    y: f32,
    width: f32,
) {
    let label = Color::from_rgba(88, 116, 126, 180);
    let text = Color::from_rgba(205, 226, 230, 255);
    let active = Color::from_rgba(150, 221, 226, 255);
    let warning = Color::from_rgba(226, 190, 150, 255);
    let preview_size = 118.0;
    let in_range = npc_ship_in_interaction_range(ship, npc_ship);
    let distance = npc_ship_surface_distance(ship, npc_ship);
    let identified = npc_ship.identified;

    draw_text(
        if identified {
            "NPC Ship"
        } else {
            "Unknown Contact"
        },
        x,
        y - 28.0,
        26.0,
        Color::from_rgba(235, 242, 226, 255),
    );

    if let Some(texture) = &npc_ship.texture {
        draw_texture_ex(
            texture,
            x,
            y,
            if identified {
                WHITE
            } else {
                Color::from_rgba(150, 170, 176, 190)
            },
            DrawTextureParams {
                dest_size: Some(vec2(preview_size, preview_size)),
                rotation: npc_ship.angle + std::f32::consts::FRAC_PI_2,
                ..Default::default()
            },
        );
    } else {
        draw_ship_model(
            vec2(x + preview_size * 0.5, y + preview_size * 0.5),
            preview_size * 0.18,
            false,
            npc_ship.angle + std::f32::consts::FRAC_PI_2,
        );
    }

    let data_y = y + preview_size + 34.0;
    draw_text(
        if identified {
            npc_ship.name.as_str()
        } else {
            "Unidentified transponder"
        },
        x,
        data_y,
        18.0,
        text,
    );
    draw_text(
        &fit_debug_text(
            &format!(
                "{} / {:.0}u / {}",
                if in_range { "scan range" } else { "approach" },
                distance,
                npc_ship.behavior.label()
            ),
            width,
            16,
        ),
        x,
        data_y + 30.0,
        16.0,
        if in_range { active } else { warning },
    );

    let mut detail_y = data_y + 62.0;
    draw_text("Identity", x, detail_y, 16.0, label);
    let identity = if identified {
        format!(
            "{} / {} / {}",
            local_content_id(&npc_ship.id),
            npc_ship.archetype,
            npc_ship.role
        )
    } else {
        "Run identification scan from interaction range.".to_string()
    };
    draw_text(
        &fit_debug_text(&identity, width, 16),
        x,
        detail_y + 26.0,
        16.0,
        text,
    );
    detail_y += 64.0;

    draw_text("Disposition", x, detail_y, 16.0, label);
    let disposition = if identified {
        npc_ship
            .faction
            .as_deref()
            .map(|faction| {
                format!(
                    "{} / {}",
                    faction_name(content_registry, faction),
                    faction_disposition_label(content_registry, faction)
                )
            })
            .unwrap_or_else(|| "Independent / unknown".to_string())
    } else {
        "Unknown".to_string()
    };
    draw_text(
        &fit_debug_text(&disposition, width, 16),
        x,
        detail_y + 26.0,
        16.0,
        if identified { text } else { warning },
    );
    detail_y += 64.0;

    draw_text("Systems", x, detail_y, 16.0, label);
    let systems = if identified {
        format!(
            "H{:.0}/{:.0}  S{:.0}/{:.0}  E{:.0}/{:.0}",
            npc_ship.hull.current,
            npc_ship.hull.max,
            npc_ship.shields.current,
            npc_ship.shields.max,
            npc_ship.energy.current,
            npc_ship.energy.max
        )
    } else {
        "Unscanned".to_string()
    };
    draw_text(
        &fit_debug_text(&systems, width, 16),
        x,
        detail_y + 26.0,
        16.0,
        text,
    );
    detail_y += 64.0;

    draw_text("Loadout", x, detail_y, 16.0, label);
    let loadout = if identified {
        format!(
            "{} shield / {} weapon / cargo {}",
            npc_ship.shield_slots.len(),
            npc_ship.weapon_slots.len(),
            format_mass(npc_ship.cargo_capacity)
        )
    } else {
        "Unknown".to_string()
    };
    draw_text(
        &fit_debug_text(&loadout, width, 16),
        x,
        detail_y + 26.0,
        16.0,
        text,
    );
    detail_y += 64.0;

    if identified {
        let cargo_units = npc_ship
            .cargo_defaults
            .iter()
            .map(|stack| stack.count)
            .sum::<u32>();
        let cargo = format!(
            "{} manifest item(s), {} unit(s)",
            npc_ship.cargo_defaults.len(),
            cargo_units
        );
        draw_text("Cargo", x, detail_y, 16.0, label);
        draw_text(
            &fit_debug_text(&cargo, width, 16),
            x,
            detail_y + 26.0,
            16.0,
            text,
        );
        detail_y += 64.0;
    }

    draw_text("Summary", x, detail_y, 16.0, label);
    let summary = if identified {
        npc_ship.summary.as_str()
    } else {
        "Contact details are not available until identification completes."
    };
    draw_wrapped_text(summary, x, detail_y + 26.0, width, 15, text);
}

fn draw_ship_detail(view: ShipDetailView<'_>) {
    let ShipDetailView {
        ship,
        power_modules,
        shields,
        weapons,
        threats,
        cargo_mass,
        cargo_capacity,
        current_system_id,
        shield_recharge_delay_remaining,
        texture,
        x,
        y,
        width,
    } = view;
    let label = Color::from_rgba(88, 116, 126, 180);
    let text = Color::from_rgba(205, 226, 230, 255);
    let accent = Color::from_rgba(150, 221, 226, 255);
    let image_size = 190.0;
    let center = vec2(x + width * 0.5, y + 74.0);

    draw_text(
        "Ship",
        x,
        y - 28.0,
        26.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    draw_ship_sprite(center, texture, image_size, false, 0.0);
    let preview_rect = ship_detail_preview_rect();
    if preview_rect.contains(vec2(mouse_position().0, mouse_position().1)) {
        draw_rectangle_lines(
            preview_rect.x,
            preview_rect.y,
            preview_rect.w,
            preview_rect.h,
            1.0,
            Color::from_rgba(150, 221, 226, 170),
        );
    }

    let cargo_y = y + image_size + 8.0;
    let cargo_label = format!(
        "Cargo {} / {}",
        format_mass(cargo_mass),
        format_mass(cargo_capacity)
    );
    let cargo_width = measure_text(&cargo_label, None, 17, 1.0).width;
    draw_text(
        &cargo_label,
        x + (width - cargo_width) * 0.5,
        cargo_y,
        17.0,
        accent,
    );

    let stats_y = y + image_size + 48.0;
    draw_line(
        x,
        stats_y - 18.0,
        x + width,
        stats_y - 18.0,
        1.0,
        Color::from_rgba(82, 114, 124, 95),
    );

    let left_width = width * 0.47;
    let right_x = x + width * 0.5;
    let right_width = width - (right_x - x);
    draw_text("Hull systems", x, stats_y, 15.0, label);
    let shield_name = shields
        .first()
        .map(|shield| shield.name.as_str())
        .unwrap_or("Standard shield");
    draw_text(
        &fit_debug_text(shield_name, left_width, 16),
        x,
        stats_y + 24.0,
        16.0,
        accent,
    );
    let shield_status = if ship.systems.shields.current >= ship.systems.shields.max {
        "full".to_string()
    } else if shield_recharge_delay_remaining > 0.0 {
        format!("delay {:.1}s", shield_recharge_delay_remaining)
    } else {
        shields
            .first()
            .map(|shield| format!("+{:.1}/s", shield.recharge_rate))
            .unwrap_or_else(|| "offline".to_string())
    };
    draw_text(
        &fit_debug_text(
            &format!(
                "Shield {:.0}% {shield_status}",
                ship.systems.shields.fraction() * 100.0
            ),
            left_width,
            15,
        ),
        x,
        stats_y + 45.0,
        15.0,
        text,
    );
    let resistance_label = shields
        .first()
        .map(|shield| {
            format!(
                "Resist {:.0}% / Hazard {:.0}%",
                shield.damage_resistance * 100.0,
                shield.hazard_resistance * 100.0
            )
        })
        .unwrap_or_else(|| "Resist 0% / Hazard 0%".to_string());
    draw_text(
        &fit_debug_text(&resistance_label, left_width, 14),
        x,
        stats_y + 64.0,
        14.0,
        text,
    );
    draw_text(
        &fit_debug_text(
            &format!(
                "Energy {:.0}%  Hull {:.0}%",
                ship.systems.energy.fraction() * 100.0,
                ship.systems.hull.fraction() * 100.0
            ),
            left_width,
            16,
        ),
        x,
        stats_y + 88.0,
        16.0,
        accent,
    );

    draw_text("Flight profile", right_x, stats_y, 15.0, label);
    draw_text(
        &fit_debug_text(
            &format!("Mass {}", format_mass(ship.attributes.mass)),
            right_width,
            18,
        ),
        right_x,
        stats_y + 28.0,
        18.0,
        text,
    );
    draw_text(
        &fit_debug_text(
            &format!("Engine {:.0}", ship.forward_acceleration()),
            right_width,
            18,
        ),
        right_x,
        stats_y + 56.0,
        18.0,
        text,
    );
    draw_text(
        &fit_debug_text(
            &format!("Turn {:.1}", ship.turn_acceleration()),
            right_width,
            18,
        ),
        right_x,
        stats_y + 84.0,
        18.0,
        text,
    );

    let power_y = stats_y + 132.0;
    draw_line(
        x,
        power_y - 18.0,
        x + width,
        power_y - 18.0,
        1.0,
        Color::from_rgba(82, 114, 124, 95),
    );
    draw_text("Power", x, power_y, 15.0, label);
    draw_text(
        &fit_debug_text(
            &format!(
                "Recharge {:.1}/s",
                ship_energy_recharge(ship, power_modules)
            ),
            left_width,
            18,
        ),
        x,
        power_y + 28.0,
        18.0,
        accent,
    );
    draw_text(
        &fit_debug_text(
            &format!("Base {:.1}/s", ship.attributes.energy_recharge),
            left_width,
            18,
        ),
        x,
        power_y + 56.0,
        18.0,
        text,
    );
    let module_mass = power_modules.iter().map(|module| module.mass).sum::<f32>();
    draw_text(
        &fit_debug_text(
            &format!("Module mass {}", format_mass(module_mass)),
            left_width,
            18,
        ),
        x,
        power_y + 84.0,
        18.0,
        text,
    );

    let module = power_modules.first();
    draw_text("Installed", right_x, power_y, 15.0, label);
    if let Some(module) = module {
        draw_text(
            &fit_debug_text(
                &format!("{} {}", module.family, module.name),
                right_width,
                18,
            ),
            right_x,
            power_y + 28.0,
            18.0,
            accent,
        );
        let fuel = module
            .fuel_item
            .as_deref()
            .map(|item| {
                format!(
                    "Fuel {} {:.2}/min",
                    local_content_id(item).replace('_', " "),
                    module.fuel_per_minute
                )
            })
            .unwrap_or_else(|| "Fuel none".to_string());
        draw_text(
            &fit_debug_text(&fuel, right_width, 18),
            right_x,
            power_y + 56.0,
            18.0,
            text,
        );
        draw_text(
            &fit_debug_text(
                &format!(
                    "Heat {:.0}%  Risk {:.0}%",
                    module.heat * 100.0,
                    module.risk * 100.0
                ),
                right_width,
                18,
            ),
            right_x,
            power_y + 84.0,
            18.0,
            text,
        );
    } else {
        draw_text(
            &fit_debug_text("No module installed", right_width, 18),
            right_x,
            power_y + 28.0,
            18.0,
            text,
        );
    }

    let weapons_y = power_y + 132.0;
    draw_line(
        x,
        weapons_y - 18.0,
        x + width,
        weapons_y - 18.0,
        1.0,
        Color::from_rgba(82, 114, 124, 95),
    );
    draw_text("Turret defense", x, weapons_y, 15.0, label);
    let hostile_count = threats
        .iter()
        .filter(|threat| {
            threat.system == current_system_id
                && threat.disposition == ThreatDisposition::Hostile
                && threat.hull.current > 0.0
        })
        .count();
    let threat_label = format!("Threats {hostile_count}");
    draw_text(
        &fit_debug_text(&threat_label, right_width, 15),
        right_x,
        weapons_y,
        15.0,
        label,
    );
    if weapons.is_empty() {
        draw_text("No turret slots equipped", x, weapons_y + 28.0, 18.0, text);
        return;
    }

    for (index, weapon) in weapons.iter().take(3).enumerate() {
        let row_y = weapons_y + 28.0 + index as f32 * 44.0;
        draw_text(
            &fit_debug_text(&weapon.name, width, 17),
            x,
            row_y,
            17.0,
            accent,
        );
        let cooldown = if weapon.cooldown_remaining > 0.0 {
            format!(" {:.1}s", weapon.cooldown_remaining)
        } else {
            String::new()
        };
        let status = format!(
            "{}{}  {:.0}u  {:.0}e",
            weapon.readiness_label(),
            cooldown,
            weapon.range,
            weapon.energy_cost
        );
        draw_text(
            &fit_debug_text(&status, width, 15),
            x,
            row_y + 19.0,
            15.0,
            if weapon.status == WeaponStatus::InsufficientEnergy {
                Color::from_rgba(226, 190, 150, 245)
            } else {
                text
            },
        );
    }
}

struct WorkRow {
    item: String,
    keep: u32,
    status: String,
    percent: String,
    enabled: bool,
    active: bool,
}

#[derive(Clone, Copy)]
enum WorkTableItemAlignment {
    Left,
    Right,
}

struct ShipDetailView<'a> {
    ship: &'a Ship,
    power_modules: &'a [PowerModule],
    shields: &'a [ShieldSystem],
    weapons: &'a [WeaponSystem],
    threats: &'a [DefenseThreat],
    cargo_mass: f32,
    cargo_capacity: f32,
    current_system_id: &'a str,
    shield_recharge_delay_remaining: f32,
    texture: Option<&'a Texture2D>,
    x: f32,
    y: f32,
    width: f32,
}

fn work_row_status(current: u32, keep: u32, queued: u32) -> String {
    if keep > 0 {
        format!("{current}/{keep}")
    } else if queued > 0 {
        format!("x{queued}")
    } else {
        String::new()
    }
}

fn draw_work_text_table(rows: &[WorkRow], x: f32, y: f32, width: f32, scroll: f32, mouse: Vec2) {
    draw_work_text_table_in_rect(
        rows,
        Rect::new(x, y, width, work_table_height()),
        scroll,
        mouse,
    );
}

fn draw_work_text_table_in_rect(rows: &[WorkRow], rect: Rect, scroll: f32, mouse: Vec2) {
    draw_work_text_table_in_rect_with_alignment(
        rows,
        rect,
        scroll,
        mouse,
        WorkTableItemAlignment::Right,
    );
}

fn draw_work_text_table_in_rect_with_alignment(
    rows: &[WorkRow],
    rect: Rect,
    scroll: f32,
    mouse: Vec2,
    item_alignment: WorkTableItemAlignment,
) {
    let Rect {
        x,
        y,
        w: width,
        h: height,
    } = rect;
    let layout = work_table_layout_with_height(x, y, width, height);
    let item_column = layout.columns[0];
    let keep_column = layout.columns[1];
    let status_column = layout.columns[2];
    let percent_column = layout.columns[3];
    let active_column = layout.columns[4];
    let header = Color::from_rgba(168, 204, 210, 255);
    let active = Color::from_rgba(205, 226, 230, 255);
    let available = Color::from_rgba(150, 221, 226, 255);
    let unavailable = Color::from_rgba(126, 143, 148, 255);

    draw_table_column_separators(&layout, y - 10.0, layout.viewport.y + layout.viewport.h);

    let item_header_width = measure_text("Item", None, 16, 1.0).width;
    let item_header_x = match item_alignment {
        WorkTableItemAlignment::Left => item_column.x,
        WorkTableItemAlignment::Right => item_column.x + item_column.w - item_header_width,
    };
    draw_text("Item", item_header_x, y, 16.0, header);
    draw_text("Keep", keep_column.x, y, 16.0, header);
    draw_text("Status", status_column.x, y, 16.0, header);
    draw_text("%", percent_column.x, y, 16.0, header);
    draw_text("Active", active_column.x, y, 16.0, header);

    let hovered = hovered_work_cell_in_layout(mouse, &layout, rows.len(), scroll);
    for (row, work_row) in rows.iter().enumerate() {
        let row_rect = ui_table_row_rect(&layout, row, scroll);
        if !ui_table_row_visible(&layout, row_rect) {
            continue;
        }
        let row_y = row_rect.y + 21.0;
        let row_color = if work_row.enabled {
            active
        } else {
            unavailable
        };
        let value_color = if work_row.enabled {
            available
        } else {
            unavailable
        };
        let is_hovered = hovered.is_some_and(|(hovered_row, _)| hovered_row == row);

        if row % 2 == 0 || is_hovered {
            draw_rectangle(
                row_rect.x,
                row_rect.y,
                row_rect.w,
                row_rect.h,
                Color::from_rgba(10, 18, 24, if is_hovered { 170 } else { 100 }),
            );
        }
        let item_label = fit_debug_text(&work_row.item, item_column.w, 20);
        let item_width = measure_text(&item_label, None, 20, 1.0).width;
        let item_x = match item_alignment {
            WorkTableItemAlignment::Left => item_column.x,
            WorkTableItemAlignment::Right => item_column.x + item_column.w - item_width,
        };
        draw_text(&item_label, item_x, row_y, 20.0, row_color);
        draw_text(
            &work_row.keep.to_string(),
            keep_column.x,
            row_y,
            20.0,
            value_color,
        );
        let status_label = fit_debug_text(&work_row.status, status_column.w, 18);
        draw_text(&status_label, status_column.x, row_y, 18.0, value_color);
        let percent_label = fit_debug_text(&work_row.percent, percent_column.w, 18);
        draw_text(&percent_label, percent_column.x, row_y, 18.0, value_color);
        if work_row.active {
            let cog_x = active_column.x + active_column.w * 0.5;
            draw_work_cog(vec2(cog_x, row_y - 6.0), get_time() as f32 * 4.0, available);
        }
    }
    draw_scrollbar(
        x + width - 4.0,
        layout.viewport.y,
        layout.viewport.h,
        rows.len(),
        layout.row_height,
        scroll,
    );
}

fn draw_table_column_separators(layout: &UiTableLayout, y1: f32, y2: f32) {
    let separator_color = Color::from_rgba(96, 137, 150, 105);
    for pair in layout.columns.windows(2) {
        let left = pair[0];
        let right = pair[1];
        let separator_x = left.x + left.w + (right.x - left.x - left.w) * 0.5;
        draw_line(separator_x, y1, separator_x, y2, 0.5, separator_color);
    }
}

fn draw_work_cog(center: Vec2, rotation: f32, color: Color) {
    let teeth = 8;
    let inner_radius = 5.0;
    let outer_radius = 9.0;
    draw_circle_lines(center.x, center.y, inner_radius, 1.5, color);
    for tooth in 0..teeth {
        let angle = rotation + tooth as f32 * std::f32::consts::TAU / teeth as f32;
        let direction = vec2(angle.cos(), angle.sin());
        let start = center + direction * (inner_radius + 1.5);
        let end = center + direction * outer_radius;
        draw_line(start.x, start.y, end.x, end.y, 1.7, color);
    }
    draw_circle(center.x, center.y, 1.7, color);
}

fn draw_scrollbar(x: f32, y: f32, height: f32, row_count: usize, row_height: f32, scroll: f32) {
    let content_height = row_count as f32 * row_height;
    if content_height <= height {
        return;
    }

    let track = Color::from_rgba(82, 114, 124, 70);
    let thumb = Color::from_rgba(150, 221, 226, 165);
    let thumb_height = (height * height / content_height).clamp(28.0, height);
    let max_scroll = max_scroll_offset(row_count, row_height, height);
    let thumb_y = y + (height - thumb_height) * (scroll / max_scroll.max(1.0));

    draw_rectangle(x, y, 2.0, height, track);
    draw_rectangle(x - 1.0, thumb_y, 4.0, thumb_height, thumb);
}

fn draw_inventory_text_list(inventory: &Inventory, x: f32, y: f32, width: f32, scroll: f32) {
    let layout = inventory_table_layout(x, y, width);
    let item_column = layout.columns[0];
    let quantity_column = layout.columns[1];
    let mass_column = layout.columns[2];
    let name_color = Color::from_rgba(205, 226, 230, 255);
    let amount_color = Color::from_rgba(150, 221, 226, 255);
    let header = Color::from_rgba(168, 204, 210, 255);
    let empty_text = Color::from_rgba(168, 184, 188, 255);

    draw_table_column_separators(&layout, y - 10.0, layout.viewport.y + layout.viewport.h);
    let item_header_width = measure_text("Item", None, 16, 1.0).width;
    draw_text(
        "Item",
        item_column.x + item_column.w - item_header_width,
        y,
        16.0,
        header,
    );
    draw_text("Qty", quantity_column.x, y, 16.0, header);
    draw_text("Mass", mass_column.x, y, 16.0, header);

    let stacks = inventory
        .slots
        .iter()
        .filter_map(|slot| slot.as_ref())
        .collect::<Vec<_>>();
    for (row, stack) in stacks.iter().enumerate() {
        let row_rect = ui_table_row_rect(&layout, row, scroll);
        if !ui_table_row_visible(&layout, row_rect) {
            continue;
        }
        let row_y = row_rect.y + 21.0;
        let name = &stack.item.name;
        let amount = stack.count.to_string();
        let mass = format_mass(stack.item.unit_mass * stack.count as f32);
        let item_label = fit_debug_text(name, item_column.w, 20);
        let name_width = measure_text(&item_label, None, 20, 1.0).width;

        if row % 2 == 0 {
            draw_rectangle(
                row_rect.x,
                row_rect.y,
                row_rect.w,
                row_rect.h,
                Color::from_rgba(10, 18, 24, 100),
            );
        }

        draw_text(
            &item_label,
            item_column.x + item_column.w - name_width,
            row_y,
            20.0,
            name_color,
        );
        draw_text(
            &fit_debug_text(&amount, quantity_column.w, 20),
            quantity_column.x,
            row_y,
            20.0,
            amount_color,
        );
        draw_text(
            &fit_debug_text(&mass, mass_column.w, 18),
            mass_column.x,
            row_y,
            18.0,
            amount_color,
        );
    }

    if stacks.is_empty() {
        let text = "empty";
        let text_width = measure_text(text, None, 20, 1.0).width;
        draw_text(
            text,
            item_column.x + item_column.w - text_width,
            y + 34.0,
            20.0,
            empty_text,
        );
        draw_text("0", quantity_column.x, y + 34.0, 20.0, empty_text);
        draw_text("0 kg", mass_column.x, y + 34.0, 18.0, empty_text);
    }
    draw_scrollbar(
        x + width - 4.0,
        layout.viewport.y,
        layout.viewport.h,
        stacks.len(),
        layout.row_height,
        scroll,
    );
}

fn format_mass(mass_kg: f32) -> String {
    if mass_kg >= 10_000.0 {
        format!("{:.1} t", mass_kg / 1_000.0)
    } else if mass_kg >= 1_000.0 {
        format!("{:.2} t", mass_kg / 1_000.0)
    } else {
        format!("{mass_kg:.0} kg")
    }
}

struct HudView<'a> {
    ship: &'a Ship,
    planets: &'a [Planet],
    stations: &'a [StationDestination],
    npc_ships: &'a [NpcShip],
    selected_planet: Option<usize>,
    selected_station: Option<usize>,
    selected_npc_ship: Option<usize>,
    destination_planet: Option<usize>,
    orbiting_planet: Option<usize>,
    current_system_id: &'a str,
    speed: f32,
    turn: f32,
}

fn draw_hud(view: HudView<'_>) {
    let HudView {
        ship,
        planets,
        stations,
        npc_ships,
        selected_planet,
        selected_station,
        selected_npc_ship,
        destination_planet,
        orbiting_planet,
        current_system_id,
        speed,
        turn,
    } = view;

    let panel = Color::from_rgba(5, 10, 16, 185);
    draw_rectangle(18.0, 18.0, 430.0, 188.0, panel);
    draw_rectangle_lines(
        18.0,
        18.0,
        430.0,
        188.0,
        1.0,
        Color::from_rgba(95, 137, 155, 120),
    );
    draw_text("W/S thrust   A/D or arrows turn", 34.0, 47.0, 20.0, WHITE);
    draw_text(
        &format!("speed {:>4.0}   turn {:>5.2}", speed, turn),
        34.0,
        76.0,
        20.0,
        Color::from_rgba(150, 221, 226, 255),
    );
    draw_text(
        &format!(
            "mass {}   engine {:.0}   turn {:.1}",
            format_mass(ship.attributes.mass),
            ship.forward_acceleration(),
            ship.turn_acceleration()
        ),
        34.0,
        104.0,
        20.0,
        Color::from_rgba(178, 197, 203, 255),
    );
    draw_text(
        &format!(
            "shield {:>3.0}%   energy {:>3.0}%   hull {:>3.0}%",
            ship.systems.shields.fraction() * 100.0,
            ship.systems.energy.fraction() * 100.0,
            ship.systems.hull.fraction() * 100.0
        ),
        34.0,
        130.0,
        20.0,
        Color::from_rgba(205, 226, 230, 255),
    );
    draw_text(
        &fit_debug_text(&format!("system {current_system_id}"), 392.0, 18),
        34.0,
        158.0,
        18.0,
        Color::from_rgba(178, 197, 203, 255),
    );

    if let Some(planet) = orbiting_planet
        .and_then(|index| planets.get(index))
        .filter(|planet| planet_is_in_system(planet, current_system_id))
    {
        draw_text(
            &fit_debug_text(&format!("orbit {}", planet.info.classification), 392.0, 18),
            34.0,
            184.0,
            18.0,
            Color::from_rgba(150, 221, 226, 255),
        );
        return;
    }

    if let Some(station) = selected_station
        .and_then(|index| stations.get(index))
        .filter(|station| station_is_in_system(station, current_system_id))
    {
        let in_range = station_in_interaction_range(ship, station);
        let distance = station_surface_distance(ship, station);
        draw_text(
            &format!(
                "station {:>4.0}u   {}",
                distance,
                if in_range { "dock range" } else { "approach" }
            ),
            34.0,
            184.0,
            20.0,
            if in_range {
                Color::from_rgba(150, 221, 226, 255)
            } else {
                Color::from_rgba(226, 190, 150, 255)
            },
        );
        return;
    }

    if let Some(npc_ship) = selected_npc_ship
        .and_then(|index| npc_ships.get(index))
        .filter(|npc_ship| npc_ship_is_in_system(npc_ship, current_system_id))
    {
        let in_range = npc_ship_in_interaction_range(ship, npc_ship);
        let distance = npc_ship_surface_distance(ship, npc_ship);
        draw_text(
            &format!(
                "contact {:>4.0}u   {}",
                distance,
                if npc_ship.identified {
                    "identified"
                } else if in_range {
                    "scan"
                } else {
                    "approach"
                }
            ),
            34.0,
            184.0,
            20.0,
            if in_range {
                Color::from_rgba(150, 221, 226, 255)
            } else {
                Color::from_rgba(226, 190, 150, 255)
            },
        );
        return;
    }

    if let Some(planet) = target_planet(
        ship,
        planets,
        current_system_id,
        selected_planet,
        destination_planet,
    ) {
        let in_range = planet_in_interaction_range(ship, planet);
        let distance = planet_surface_distance(ship, planet);
        draw_text(
            &format!(
                "target {:>4.0}u   {}",
                distance,
                if in_range { "in range" } else { "approach" }
            ),
            34.0,
            184.0,
            20.0,
            if in_range {
                Color::from_rgba(150, 221, 226, 255)
            } else {
                Color::from_rgba(226, 190, 150, 255)
            },
        );
    }
}

fn draw_interaction_prompt(game: &GameState) {
    if game.map_open || game.skills_open || game.upgrades_open || game.content_open {
        return;
    }

    let text = if let Some(planet_index) = ship_over_planet_index(game) {
        if game.planets.get(planet_index).is_none() {
            return;
        }
        let planet_scanned = game
            .planets
            .get(planet_index)
            .is_some_and(planet_has_composition_scan);
        if game.selected_planet == Some(planet_index) && planet_scanned {
            "Space open mining"
        } else if game.selected_planet == Some(planet_index) {
            "Space open survey"
        } else {
            "Space inspect planet"
        }
    } else if let Some(station_index) = ship_over_station_index(game) {
        if game.stations.get(station_index).is_none() {
            return;
        }
        if game.selected_station == Some(station_index) {
            "Space dock station"
        } else {
            "Space inspect station"
        }
    } else if let Some(npc_ship_index) = ship_over_npc_ship_index(game) {
        let Some(npc_ship) = game.npc_ships.get(npc_ship_index) else {
            return;
        };
        if game.selected_npc_ship == Some(npc_ship_index) && npc_ship.identified {
            "Space review contact"
        } else if game.selected_npc_ship == Some(npc_ship_index) {
            "Space identify contact"
        } else {
            "Space inspect ship"
        }
    } else {
        return;
    };
    let measure = measure_text(text, None, 20, 1.0);
    let x = (screen_width() - measure.width) * 0.5;
    let y = screen_height() - 72.0;
    draw_rectangle(
        x - 16.0,
        y - 25.0,
        measure.width + 32.0,
        38.0,
        Color::from_rgba(5, 10, 16, 185),
    );
    draw_rectangle_lines(
        x - 16.0,
        y - 25.0,
        measure.width + 32.0,
        38.0,
        1.0,
        Color::from_rgba(150, 221, 226, 145),
    );
    draw_text(text, x, y, 20.0, Color::from_rgba(205, 226, 230, 255));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::{Path, PathBuf},
        process,
    };

    fn test_save_path(name: &str) -> PathBuf {
        env::temp_dir().join(format!(
            "some-frontier-{name}-{}-{}.toml",
            process::id(),
            current_unix_seconds()
        ))
    }

    #[test]
    fn title_seed_parser_accepts_decimal_u64_values() {
        assert_eq!(parse_title_seed("12345"), Some(12_345));
        assert_eq!(parse_title_seed(" 42 "), Some(42));
        assert_eq!(parse_title_seed(""), None);
        assert_eq!(parse_title_seed("abc"), None);
        assert_eq!(parse_title_seed("18446744073709551616"), None);
    }

    #[test]
    fn title_save_row_double_click_requires_same_row_within_threshold() {
        assert!(title_save_row_double_clicked(Some(2), 10.0, 2, 10.2));
        assert!(!title_save_row_double_clicked(Some(1), 10.0, 2, 10.2));
        assert!(!title_save_row_double_clicked(Some(2), 10.0, 2, 10.8));
        assert!(!title_save_row_double_clicked(None, 10.0, 2, 10.2));
    }

    #[test]
    fn title_load_layout_has_wide_save_list_and_detail_pane() {
        let panel = title_load_panel_rect_for_screen(1024.0, 768.0);
        let list = title_save_list_rect_for_panel(panel);
        let detail_width = panel.x + panel.w - (list.x + list.w + 28.0) - 28.0;

        assert!(panel.w >= 720.0);
        assert!(list.w >= 300.0);
        assert!(detail_width >= 300.0);
    }

    #[test]
    fn app_settings_clamp_to_supported_ranges() {
        let settings = AppSettings {
            ui_scale: 5.0,
            master_volume: -1.0,
            controls_profile: "unknown".to_string(),
            gameplay_autosave_minutes: 99,
        }
        .clamped();

        assert_eq!(settings.ui_scale, 1.25);
        assert_eq!(settings.master_volume, 0.0);
        assert_eq!(settings.controls_profile, "standard");
        assert_eq!(settings.gameplay_autosave_minutes, 10);
    }

    #[test]
    fn save_data_round_trips_content_pack_options() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.content_pack_options = vec![
            PackOptionSelection {
                pack_id: "core".to_string(),
                option_id: "core:resource_density".to_string(),
                value: "rich".to_string(),
            },
            PackOptionSelection {
                pack_id: "remote-duskfall".to_string(),
                option_id: "remote-duskfall:redwake_hostility".to_string(),
                value: "watchful".to_string(),
            },
        ];
        game.purchased_recipe_unlocks = vec!["core:advanced_scanner_core".to_string()];
        game.installed_power_modules = installed_power_modules_from_ids(
            &game.content_registry,
            &["core:compact_fission_cell".to_string()],
        );
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        game.equipped_shields = equipped_shields_from_ids(
            &game.content_registry,
            &["core:hazard_shield_matrix".to_string()],
        );
        game.shield_recharge_delay_remaining = 2.5;

        let serialized = toml::to_string(&game.to_save()).expect("save should serialize");
        let restored = toml::from_str::<SaveData>(&serialized).expect("save should deserialize");

        assert_eq!(restored.content_pack_options, game.content_pack_options);
        assert_eq!(
            restored.purchased_recipe_unlocks,
            game.purchased_recipe_unlocks
        );
        assert_eq!(
            restored.installed_power_modules,
            vec!["core:compact_fission_cell".to_string()]
        );
        assert_eq!(
            restored.shield_slots,
            vec!["core:hazard_shield_matrix".to_string()]
        );
        assert_eq!(restored.shield_recharge_delay_remaining, 2.5);
        assert_eq!(
            restored.weapon_slots,
            vec!["core:point_defense_turret".to_string()]
        );
    }

    #[test]
    fn save_data_sanitizes_non_finite_runtime_floats() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.world_elapsed_days = f32::NAN;
        game.camera_zoom = f32::INFINITY;
        game.ship.position = vec2(f32::NAN, f32::NEG_INFINITY);
        game.ship.velocity = vec2(f32::INFINITY, f32::NAN);
        game.ship.angle = f32::NAN;
        game.ship.angular_velocity = f32::INFINITY;
        game.ship.systems.hull.current = f32::NAN;
        game.ship.systems.shields.max = f32::INFINITY;
        game.shield_recharge_delay_remaining = f32::INFINITY;
        game.skills[0].xp = f32::NAN;
        game.smelt_recipes = make_smelting_recipes(&game.content_registry);
        game.smelt_settings = vec![
            CraftSetting {
                keep: 1,
                queued: 0,
                progress: f32::NAN,
            };
            game.smelt_recipes.len()
        ];

        let serialized =
            toml::to_string_pretty(&game.to_save()).expect("save should serialize pretty TOML");
        let restored = toml::from_str::<SaveData>(&serialized).expect("save should deserialize");

        assert_eq!(restored.world_elapsed_days, 0.0);
        assert_eq!(restored.camera_zoom, default_camera_zoom());
        assert_eq!(restored.ship.position, [0.0, 0.0]);
        assert_eq!(restored.ship.velocity, [0.0, 0.0]);
        assert_eq!(restored.ship.angle, 0.0);
        assert_eq!(restored.ship.angular_velocity, 0.0);
        assert_eq!(restored.ship.hull.current, restored.ship.hull.max);
        assert_eq!(restored.ship.shields.max, 1.0);
        assert_eq!(restored.shield_recharge_delay_remaining, 0.0);
        assert_eq!(restored.skills[0].xp, 0.0);
        assert!(restored
            .smelt_settings
            .iter()
            .all(|setting| setting.progress == 0.0));
    }

    #[test]
    fn starter_ship_uses_core_content_metadata() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let ship_def = registry
            .ships
            .get(STARTER_SHIP_ID)
            .expect("starter ship should be loaded from core content");
        let ship = Ship::from_content(ship_def);

        assert_eq!(ship.forward_acceleration(), STARTER_FORWARD_ACCELERATION);
        assert_eq!(ship.reverse_acceleration(), STARTER_REVERSE_ACCELERATION);
        assert_eq!(ship.attributes.turn_thruster_strength, 85_000.0 * 4.8);
        assert_eq!(ship.systems.hull.max, 100.0);
        assert_eq!(ship.systems.shields.max, 100.0);
        assert_eq!(ship.systems.energy.max, 100.0);

        let shields = equipped_shields_from_ids(&registry, &ship_def.shield_slots);
        assert_eq!(shields.len(), 1);
        assert_eq!(shields[0].name, "Balanced Shield Matrix");
        assert_eq!(shields[0].install_item, "core:balanced_shield_matrix");
        assert_eq!(shields[0].capacity, 100.0);
        assert_eq!(shields[0].recharge_delay, 4.0);
        assert_eq!(shields[0].recharge_rate, 7.5);
        assert_eq!(shields[0].damage_resistance, 0.10);
        assert_eq!(shields[0].hazard_resistance, 0.15);

        let power_modules = installed_power_modules_from_ids(&registry, &ship_def.power_modules);
        assert_eq!(power_modules.len(), 1);
        assert_eq!(power_modules[0].family, "Nuclear");
        assert_eq!(ship.attributes.energy_recharge, 8.0);
        assert_eq!(ship_energy_recharge(&ship, &power_modules), 22.0);

        let weapons = equipped_weapons_from_ids(&registry, &ship_def.weapon_slots);
        assert_eq!(weapons.len(), 1);
        assert_eq!(weapons[0].name, "Point Defense Turret");
        assert_eq!(weapons[0].install_item, "core:point_defense_turret");
        assert_eq!(weapons[0].range, 460.0);
        assert_eq!(weapons[0].energy_cost, 7.0);
    }

    #[test]
    fn weapon_slots_swap_with_inventory_install_items() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        game.content_registry.items.insert(
            "core:test_turret_item".to_string(),
            content::ItemDef {
                id: "core:test_turret_item".to_string(),
                name: "Test turret item".to_string(),
                tier: "weapon".to_string(),
                xp_value: 1.0,
                unit_mass: 100.0,
            },
        );
        game.content_registry.weapons.insert(
            "core:test_turret".to_string(),
            content::WeaponDef {
                id: "core:test_turret".to_string(),
                name: "Test Turret".to_string(),
                kind: content::WeaponKind::TurretDefense,
                install_item: "core:test_turret_item".to_string(),
                range: 240.0,
                cooldown_seconds: 2.0,
                damage: 6.0,
                energy_cost: 3.0,
                tracking_degrees: 360.0,
                summary: None,
            },
        );

        assert_eq!(
            install_weapon_in_slot(&mut game, 1, "core:test_turret"),
            Err(WeaponInstallError::InvalidSlot)
        );
        assert_eq!(
            install_weapon_in_slot(&mut game, 0, "core:missing_turret"),
            Err(WeaponInstallError::UnknownWeapon)
        );
        assert_eq!(
            install_weapon_in_slot(&mut game, 0, "core:test_turret"),
            Err(WeaponInstallError::MissingInstallItem)
        );

        let install_item = required_item(&game.content_registry, "core:test_turret_item");
        let previous_item = required_item(&game.content_registry, "core:point_defense_turret");
        game.inventory.add_item(install_item.clone(), 1);

        install_weapon_in_slot(&mut game, 0, "core:test_turret")
            .expect("crafted turret should install into the weapon slot");

        assert_eq!(game.equipped_weapons[0].id, "core:test_turret");
        assert_eq!(game.inventory.count(&install_item), 0);
        assert_eq!(game.inventory.count(&previous_item), 1);
        assert!(game.save_dirty);
        assert_eq!(game.to_save().weapon_slots, vec!["core:test_turret"]);
    }

    #[test]
    fn shield_slots_swap_with_inventory_install_items() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.equipped_shields = equipped_shields_from_ids(
            &game.content_registry,
            &["core:balanced_shield_matrix".to_string()],
        );
        game.rebuild_ship_from_upgrades();

        assert_eq!(
            install_shield_in_slot(&mut game, 1, "core:hazard_shield_matrix"),
            Err(ShieldInstallError::InvalidSlot)
        );
        assert_eq!(
            install_shield_in_slot(&mut game, 0, "core:missing_shield"),
            Err(ShieldInstallError::UnknownShield)
        );
        assert_eq!(
            install_shield_in_slot(&mut game, 0, "core:hazard_shield_matrix"),
            Err(ShieldInstallError::MissingInstallItem)
        );

        let install_item = required_item(&game.content_registry, "core:hazard_shield_matrix");
        let previous_item = required_item(&game.content_registry, "core:balanced_shield_matrix");
        game.inventory.add_item(install_item.clone(), 1);

        install_shield_in_slot(&mut game, 0, "core:hazard_shield_matrix")
            .expect("crafted shield should install into the shield slot");

        assert_eq!(game.equipped_shields[0].id, "core:hazard_shield_matrix");
        assert_eq!(game.ship.systems.shields.max, 85.0);
        assert_eq!(game.inventory.count(&install_item), 0);
        assert_eq!(game.inventory.count(&previous_item), 1);
        assert!(game.save_dirty);
        assert_eq!(
            game.to_save().shield_slots,
            vec!["core:hazard_shield_matrix"]
        );
    }

    #[test]
    fn shield_variants_control_hazard_drain_and_recharge() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut planet = test_planet("core:hazard", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        planet.info.hazard_effects = HazardEffects {
            shield_drain_per_second: 10.0,
            mining_speed_multiplier: 1.0,
        };
        let mut game = test_game_with_systems(registry, vec![planet]);
        game.equipped_shields = equipped_shields_from_ids(
            &game.content_registry,
            &["core:hazard_shield_matrix".to_string()],
        );
        game.rebuild_ship_from_upgrades();
        game.ship.systems.shields.current = game.ship.systems.shields.max;

        update_orbital_hazards(&mut game, 1.0);

        assert_eq!(game.equipped_shields[0].damage_resistance, 0.05);
        assert_eq!(game.ship.systems.shields.current, 80.5);
        assert_eq!(game.shield_recharge_delay_remaining, 3.0);

        update_shield_recharge(&mut game, 1.0);
        assert_eq!(game.ship.systems.shields.current, 80.5);
        assert_eq!(game.shield_recharge_delay_remaining, 2.0);

        update_shield_recharge(&mut game, 2.0);
        assert_eq!(game.ship.systems.shields.current, 80.5);
        assert_eq!(game.shield_recharge_delay_remaining, 0.0);

        update_shield_recharge(&mut game, 1.0);
        assert_eq!(game.ship.systems.shields.current, 85.0);
    }

    #[test]
    fn defensive_turrets_fire_at_hostile_threats_only() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        game.ship.systems.energy.current = 100.0;
        game.defense_threats = vec![
            test_defense_threat(
                "core:neutral",
                ThreatDisposition::Neutral,
                vec2(90.0, 0.0),
                24.0,
            ),
            test_defense_threat(
                "core:hostile",
                ThreatDisposition::Hostile,
                vec2(120.0, 0.0),
                36.0,
            ),
        ];

        update_weapon_systems(&mut game, 0.1);

        assert_eq!(game.equipped_weapons[0].status, WeaponStatus::Fired);
        assert_eq!(game.ship.systems.energy.current, 93.0);
        assert_eq!(game.defense_threats[0].hull.current, 24.0);
        assert_eq!(game.defense_threats[1].hull.current, 18.0);
        assert_eq!(game.weapon_fire_events.len(), 1);
        assert!(game.save_dirty);
    }

    #[test]
    fn defensive_turret_cooldown_blocks_repeated_fire() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        game.ship.systems.energy.current = 100.0;
        game.defense_threats = vec![test_defense_threat(
            "core:hostile",
            ThreatDisposition::Hostile,
            vec2(120.0, 0.0),
            60.0,
        )];

        update_weapon_systems(&mut game, 0.1);
        let hull_after_first_shot = game.defense_threats[0].hull.current;
        update_weapon_systems(&mut game, 0.1);

        assert_eq!(game.equipped_weapons[0].status, WeaponStatus::Cooldown);
        assert_eq!(game.defense_threats[0].hull.current, hull_after_first_shot);
        assert_eq!(game.weapon_fire_events.len(), 1);
    }

    #[test]
    fn defensive_turret_requires_energy_to_fire() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        game.ship.systems.energy.current = 1.0;
        game.defense_threats = vec![test_defense_threat(
            "core:hostile",
            ThreatDisposition::Hostile,
            vec2(120.0, 0.0),
            36.0,
        )];

        update_weapon_systems(&mut game, 0.1);

        assert_eq!(
            game.equipped_weapons[0].status,
            WeaponStatus::InsufficientEnergy
        );
        assert_eq!(game.ship.systems.energy.current, 1.0);
        assert_eq!(game.defense_threats[0].hull.current, 36.0);
        assert!(game.weapon_fire_events.is_empty());
    }

    #[test]
    fn defensive_turret_ignores_owned_and_environmental_threats() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        game.ship.systems.energy.current = 100.0;
        game.defense_threats = vec![
            test_defense_threat(
                "core:owned",
                ThreatDisposition::Owned,
                vec2(90.0, 0.0),
                24.0,
            ),
            test_defense_threat(
                "core:hazard",
                ThreatDisposition::Environmental,
                vec2(120.0, 0.0),
                24.0,
            ),
        ];

        update_weapon_systems(&mut game, 0.1);

        assert_eq!(game.equipped_weapons[0].status, WeaponStatus::NoThreat);
        assert_eq!(game.ship.systems.energy.current, 100.0);
        assert!(game
            .defense_threats
            .iter()
            .all(|threat| threat.hull.current == 24.0));
        assert!(game.weapon_fire_events.is_empty());
    }

    #[test]
    fn ui_columns_allocate_fixed_content_and_flexible_widths() {
        let columns = ui_resolve_columns(
            Rect::new(10.0, 20.0, 300.0, 40.0),
            10.0,
            &[
                ui_column_spec_fixed(50.0),
                ui_column_spec_content(80.0, 40.0, 100.0),
                ui_column_spec_flex(60.0, 1.0),
            ],
        );

        assert_eq!(columns.len(), 3);
        assert!((columns[0].w - 50.0).abs() < 0.01);
        assert!((columns[1].w - 80.0).abs() < 0.01);
        assert!((columns[2].w - 150.0).abs() < 0.01);
        assert!((columns[1].x - 70.0).abs() < 0.01);
        assert!((columns[2].x - 160.0).abs() < 0.01);
    }

    #[test]
    fn action_rail_override_can_expand_but_not_shrink_auto_width() {
        assert_eq!(action_rail_override_candidate(320.0, None), 320.0);
        assert_eq!(action_rail_override_candidate(320.0, Some(420.0)), 420.0);
        assert_eq!(action_rail_override_candidate(320.0, Some(280.0)), 320.0);
    }

    #[test]
    fn action_rail_resize_handle_has_wide_grab_target() {
        let rail = Rect::new(100.0, 50.0, 320.0, 260.0);
        let handle = action_rail_resize_handle_rect(rail);
        let visual_grip_x = rail.x;

        assert!((handle.w - ACTION_RAIL_RESIZE_HITBOX_WIDTH).abs() < 0.01);
        assert!(handle.contains(vec2(visual_grip_x - 10.0, rail.y + 80.0)));
        assert!(handle.contains(vec2(visual_grip_x + 10.0, rail.y + 80.0)));
    }

    #[test]
    fn action_rail_blocks_pointer_inside_rail_and_resize_handle() {
        let rail = Rect::new(100.0, 50.0, 320.0, 260.0);

        assert!(action_rail_blocks_pointer(rail, vec2(180.0, 90.0)));
        assert!(action_rail_blocks_pointer(rail, vec2(rail.x - 10.0, 90.0)));
        assert!(!action_rail_blocks_pointer(rail, vec2(rail.x - 30.0, 90.0)));
    }

    #[test]
    fn ui_table_rows_and_hover_cells_share_geometry() {
        let layout = ui_table_layout(
            Rect::new(20.0, 40.0, 240.0, 100.0),
            52.0,
            90.0,
            30.0,
            8.0,
            &[ui_column_spec_flex(80.0, 1.0), ui_column_spec_fixed(44.0)],
        );

        let second_row = ui_table_row_rect(&layout, 1, 6.0);
        assert!((second_row.y - 76.0).abs() < 0.01);
        assert_eq!(
            ui_hovered_table_cell(
                vec2(layout.columns[1].x + 2.0, second_row.y + 8.0),
                &layout,
                4,
                6.0
            ),
            Some(UiTableCell { row: 1, column: 1 })
        );
        assert_eq!(
            ui_hovered_table_cell(
                vec2(layout.viewport.x - 2.0, second_row.y + 8.0),
                &layout,
                4,
                6.0
            ),
            None
        );
    }

    #[test]
    fn ui_table_layout_until_bottom_clamps_viewport_height() {
        let layout = ui_table_layout_until_bottom(UiTableBottomLayout {
            x: 12.0,
            y: 30.0,
            width: 180.0,
            row_start_offset: 24.0,
            viewport_bottom: 130.0,
            row_height: 28.0,
            column_gap: 8.0,
            columns: &[ui_column_spec_flex(70.0, 1.0), ui_column_spec_fixed(40.0)],
        });

        assert!((layout.viewport.y - 54.0).abs() < 0.01);
        assert!((layout.viewport.h - 76.0).abs() < 0.01);
        assert!(ui_table_row_visible(
            &layout,
            ui_table_row_rect(&layout, 1, 0.0)
        ));
        assert!(!ui_table_row_visible(
            &layout,
            ui_table_row_rect(&layout, 3, 0.0)
        ));
    }

    #[test]
    fn work_table_hover_uses_adaptive_columns() {
        let layout = ui_table_layout(
            Rect::new(100.0, 80.0, 342.0, 180.0),
            93.0,
            180.0,
            WORK_ROW_HEIGHT,
            12.0,
            &[
                ui_column_spec_flex(132.0, 1.0),
                ui_column_spec_fixed(42.0),
                ui_column_spec_content(56.0, 50.0, 82.0),
            ],
        );
        let row_rect = ui_table_row_rect(&layout, 2, 0.0);

        assert_eq!(
            ui_hovered_table_cell(
                vec2(layout.columns[0].x + 12.0, row_rect.y + 8.0),
                &layout,
                5,
                0.0
            ),
            Some(UiTableCell { row: 2, column: 0 })
        );
        assert_eq!(
            ui_hovered_table_cell(
                vec2(layout.columns[2].x + 12.0, row_rect.y + 8.0),
                &layout,
                5,
                0.0
            ),
            Some(UiTableCell { row: 2, column: 2 })
        );
    }

    #[test]
    fn npc_behavior_modes_derive_from_existing_content_hooks() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");

        assert_eq!(
            npc_behavior_mode(
                &registry,
                registry
                    .npc_ships
                    .get("core:frontier_freehauler")
                    .expect("freehauler should load")
            ),
            NpcBehaviorMode::TradeRoute
        );
        assert_eq!(
            npc_behavior_mode(
                &registry,
                registry
                    .npc_ships
                    .get("core:frontier_patrol_cutter")
                    .expect("patrol should load")
            ),
            NpcBehaviorMode::Patrol
        );
        assert_eq!(
            npc_behavior_mode(
                &registry,
                registry
                    .npc_ships
                    .get("core:redwake_probe")
                    .expect("probe should load")
            ),
            NpcBehaviorMode::HostileIntercept
        );
    }

    #[test]
    fn npc_patrol_motion_advances_toward_route_target() {
        let mut npc_ship = test_npc_ship(NpcBehaviorMode::Patrol, Vec2::ZERO);
        let target = npc_behavior_target(&npc_ship, vec2(10_000.0, 0.0), &[]);

        update_npc_ship_motion(
            &mut npc_ship,
            NpcMotionContext {
                target,
                player_position: vec2(10_000.0, 0.0),
                stations: &[],
                planets: &[],
                npc_snapshots: &[],
                npc_index: 0,
                dt: 1.0,
            },
        );

        assert!(npc_ship.position.x > 0.0);
        assert!(npc_ship.position.is_finite());
        assert!(npc_ship.velocity.length() <= NPC_PATROL_SPEED);
    }

    #[test]
    fn npc_follow_flee_and_intercept_targets_respect_standoff_rules() {
        let player_position = Vec2::ZERO;
        let follow = test_npc_ship(NpcBehaviorMode::Follow, vec2(900.0, 0.0));
        let flee = test_npc_ship(NpcBehaviorMode::Flee, vec2(100.0, 0.0));
        let hostile = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(900.0, 0.0));

        assert_vec2_near(
            npc_behavior_target(&follow, player_position, &[]),
            vec2(NPC_FOLLOW_DISTANCE, 0.0),
        );
        assert!(npc_behavior_target(&flee, player_position, &[]).x > flee.position.x);
        assert_vec2_near(
            npc_behavior_target(&hostile, player_position, &[]),
            vec2(NPC_HOSTILE_STANDOFF_DISTANCE, 0.0),
        );
    }

    #[test]
    fn npc_route_progress_advances_when_target_is_reached() {
        let mut npc_ship = test_npc_ship(NpcBehaviorMode::Patrol, Vec2::ZERO);

        update_npc_route_progress(&mut npc_ship, vec2(8.0, 0.0));

        assert_eq!(npc_ship.route_index, 1);
    }

    #[test]
    fn npc_avoidance_pushes_away_from_overlapping_bodies() {
        let steering = avoidance_steering(
            vec2(30.0, 0.0),
            24.0,
            NPC_SEPARATION_PADDING,
            &[NpcAvoidanceBody {
                position: Vec2::ZERO,
                radius: SHIP_RADIUS,
            }],
        );

        assert!(steering.x > 0.0);
        assert!(steering.y.abs() < 0.01);
    }

    #[test]
    fn npc_ship_range_selection_and_identification_hooks_work() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.npc_ships = vec![test_npc_ship(NpcBehaviorMode::Patrol, vec2(120.0, 0.0))];
        game.ship.position = Vec2::ZERO;

        assert!(npc_ship_in_interaction_range(
            &game.ship,
            &game.npc_ships[0]
        ));
        assert_eq!(ship_over_npc_ship_index(&game), Some(0));

        select_nearby_destination(&mut game);

        assert_eq!(game.selected_planet, None);
        assert_eq!(game.selected_station, None);
        assert_eq!(game.selected_npc_ship, Some(0));
        assert!(!game.npc_ships[0].identified);

        assert!(identify_selected_npc_ship(&mut game));
        assert!(game.npc_ships[0].identified);
    }

    #[test]
    fn npc_identification_requires_interaction_range() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.npc_ships = vec![test_npc_ship(NpcBehaviorMode::Patrol, vec2(2_000.0, 0.0))];
        game.selected_npc_ship = Some(0);

        assert!(!npc_ship_in_interaction_range(
            &game.ship,
            &game.npc_ships[0]
        ));
        assert!(!identify_selected_npc_ship(&mut game));
        assert!(!game.npc_ships[0].identified);
    }

    #[test]
    fn npc_interaction_rows_reflect_friendly_and_hostile_states() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let ship = Ship::starter();
        let mut friendly = test_npc_ship(NpcBehaviorMode::TradeRoute, vec2(120.0, 0.0));
        friendly.role = "hauler".to_string();
        friendly.behavior_tags = vec!["trade-route".to_string(), "non-hostile".to_string()];
        friendly.identified = true;
        let mut hostile = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(120.0, 0.0));
        hostile.role = "hostile".to_string();
        hostile.behavior_tags = vec!["hostile".to_string()];
        hostile.identified = true;

        let friendly_rows = npc_interaction_rows(&registry, &ship, &friendly);
        assert_eq!(
            friendly_rows
                .iter()
                .find(|row| row.action == NpcInteractionAction::Hail)
                .map(|row| row.state),
            Some(NpcInteractionState::Available)
        );
        assert_eq!(
            friendly_rows
                .iter()
                .find(|row| row.action == NpcInteractionAction::Trade)
                .map(|row| row.status),
            Some("No exchange")
        );

        let hostile_rows = npc_interaction_rows(&registry, &ship, &hostile);
        assert_eq!(
            hostile_rows
                .iter()
                .find(|row| row.action == NpcInteractionAction::Hail)
                .map(|row| row.status),
            Some("Hostile")
        );
        assert_eq!(
            hostile_rows
                .iter()
                .find(|row| row.action == NpcInteractionAction::Conflict)
                .map(|row| row.status),
            Some("Auto defense")
        );
    }

    #[test]
    fn system_stars_load_as_runtime_bodies() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let stars = make_system_stars(&registry);
        let frontier_primary = stars
            .iter()
            .find(|star| {
                star.system == STARTER_SYSTEM_ID
                    && star.name == "Frontier Primary"
                    && star.is_primary
            })
            .expect("frontier primary should become a runtime star body");

        assert_eq!(frontier_primary.system, STARTER_SYSTEM_ID);
        assert_eq!(frontier_primary.name, "Frontier Primary");
        assert_eq!(frontier_primary.classification, "K-type main sequence");
        assert_eq!(frontier_primary.position, vec2(-900.0, -700.0));
        assert_eq!(frontier_primary.radius, 180.0);
        assert!(frontier_primary.is_primary);
        assert!(system_star_is_in_system(
            frontier_primary,
            STARTER_SYSTEM_ID
        ));
        assert!(!system_star_is_in_system(
            frontier_primary,
            "remote-duskfall:duskfall_reach"
        ));
    }

    #[test]
    fn radial_haze_alpha_smoothly_fades_outward() {
        let samples = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
        let alphas = samples.map(radial_haze_alpha);

        assert_eq!(alphas[5], 0);
        assert!(alphas.windows(2).all(|window| window[0] >= window[1]));
        assert!(alphas
            .windows(2)
            .all(|window| window[0] > window[1] || window[1] == 0));
    }

    #[test]
    fn planet_orbit_can_resolve_primary_star_anchor() {
        let mut registry = content::ContentRegistry::default();
        registry.systems.insert(
            "test:system".to_string(),
            content::SystemDef {
                id: "test:system".to_string(),
                name: "Test System".to_string(),
                region: None,
                galaxy: None,
                universe: None,
                primary_star: Some("test:star".to_string()),
                faction: None,
                arrival: [0.0, 0.0],
                description: None,
                tags: Vec::new(),
            },
        );
        registry.stars.insert(
            "test:star".to_string(),
            content::StarDef {
                id: "test:star".to_string(),
                name: "Test Star".to_string(),
                system: "test:system".to_string(),
                classification: "G-type main sequence".to_string(),
                color: [255, 230, 190],
                radius: 180.0,
                position: [50.0, -25.0],
            },
        );
        let planet = content::PlanetDef {
            id: "test:orbiter".to_string(),
            system: "test:system".to_string(),
            faction: None,
            classification: "Orbiter".to_string(),
            texture: None,
            position: [1000.0, 1000.0],
            orbit: Some(content::OrbitDef {
                center: None,
                around: Some("primary_star".to_string()),
                radius: 200.0,
                eccentricity: 0.0,
                axis_phase: 0.0,
                period_days: content::MIN_ORBIT_PERIOD_DAYS,
                phase: 0.0,
            }),
            radius: 64.0,
            is_poi: true,
            mineables: Vec::new(),
            hazards: Vec::new(),
            hazard_effects: content::HazardEffectsDef::default(),
            summary: "A test planet.".to_string(),
        };

        let motion = planet_motion_from_def(&registry, &planet, 0);
        let PlanetMotion::Orbit(orbit) = motion else {
            panic!("anchored orbit metadata should produce orbital motion");
        };

        assert_eq!(orbit.center, vec2(50.0, -25.0));
        assert_eq!(orbit.radius, 200.0);
    }

    #[test]
    fn active_orbit_guides_include_unique_active_system_orbits() {
        let mut first = test_planet("test:first", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        first.motion = PlanetMotion::Orbit(OrbitMotion {
            center: vec2(10.0, 20.0),
            anchor_planet: None,
            radius: 300.0,
            semi_minor: 300.0,
            axis_rotation: 0.0,
            period_days: 1800.0,
            phase: 0.0,
        });
        let mut duplicate = test_planet("test:duplicate", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        duplicate.motion = first.motion;
        let mut outer = test_planet("test:outer", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        outer.motion = PlanetMotion::Orbit(OrbitMotion {
            center: vec2(10.0, 20.0),
            anchor_planet: None,
            radius: 700.0,
            semi_minor: 700.0,
            axis_rotation: 0.0,
            period_days: 3600.0,
            phase: 0.5,
        });
        let static_body = test_planet("test:static", STARTER_SYSTEM_ID, vec2(50.0, 50.0), true);
        let mut remote = test_planet("test:remote", "test:remote_system", Vec2::ZERO, true);
        remote.motion = PlanetMotion::Orbit(OrbitMotion {
            center: Vec2::ZERO,
            anchor_planet: None,
            radius: 900.0,
            semi_minor: 900.0,
            axis_rotation: 0.0,
            period_days: 4000.0,
            phase: 0.2,
        });

        let guides = active_orbit_guides(
            &[outer, static_body, duplicate, remote, first],
            STARTER_SYSTEM_ID,
        );

        assert_eq!(guides.len(), 2);
        assert_eq!(guides[0].radius, 300.0);
        assert_eq!(guides[1].radius, 700.0);
    }

    #[test]
    fn planet_anchored_orbits_follow_anchor_runtime_position() {
        let mut anchor = test_planet("test:anchor", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        anchor.motion = PlanetMotion::Orbit(OrbitMotion {
            center: Vec2::ZERO,
            anchor_planet: None,
            radius: 100.0,
            semi_minor: 100.0,
            axis_rotation: 0.0,
            period_days: 40.0,
            phase: 0.0,
        });
        let mut moon = test_planet("test:moon", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        moon.motion = PlanetMotion::Orbit(OrbitMotion {
            center: vec2(999.0, 999.0),
            anchor_planet: Some(0),
            radius: 20.0,
            semi_minor: 20.0,
            axis_rotation: 0.0,
            period_days: 40_000.0,
            phase: 0.0,
        });
        let mut planets = vec![anchor, moon];

        update_planet_runtime_positions(&mut planets, 0.0);
        assert_vec2_near(planets[0].position, vec2(100.0, 0.0));
        assert_vec2_near(planets[1].position, vec2(120.0, 0.0));

        update_planet_runtime_positions(&mut planets, 10.0);
        assert_vec2_near(planets[0].position, vec2(0.0, 100.0));
        assert!(
            (planets[1].position.distance(planets[0].position) - 20.0).abs() < 0.01,
            "anchored orbit should stay around the anchor runtime position"
        );
        assert!(
            planets[1].position.distance(vec2(999.0, 999.0)) > 1000.0,
            "anchored orbit should not keep using the fallback authored center"
        );
    }

    #[test]
    fn station_destinations_have_docking_range_and_selection() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.ship.position = vec2(120.0, 0.0);

        assert!(station_in_interaction_range(&game.ship, &game.stations[0]));
        assert_eq!(ship_over_station_index(&game), Some(0));

        select_nearby_destination(&mut game);
        assert_eq!(game.selected_planet, None);
        assert_eq!(game.selected_station, Some(0));

        assert!(select_station_service(&mut game, 0, 1));
        assert_eq!(game.selected_station, Some(0));
        assert_eq!(game.selected_station_service, Some(1));
        assert_eq!(game.stations[0].services[1].kind, "garage");
    }

    #[test]
    fn station_trade_buy_and_sell_moves_credits_inventory_and_stock() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let iron = required_item(&registry, "core:iron_ore");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.credits = 100;
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.stations[0].services[0].trade = vec![TradeOffer {
            item: iron.clone(),
            buy_price: 25,
            sell_price: 10,
            stock: Some(2),
            restock_days: Some(3.0),
            unavailable: false,
        }];
        game.ship.position = vec2(120.0, 0.0);

        assert!(buy_station_trade_offer(&mut game, 0, 0, 0));
        assert_eq!(game.credits, 75);
        assert_eq!(game.inventory.count(&iron), 1);
        assert_eq!(game.stations[0].services[0].trade[0].stock, Some(1));

        assert!(sell_station_trade_offer(&mut game, 0, 0, 0));
        assert_eq!(game.credits, 85);
        assert_eq!(game.inventory.count(&iron), 0);
        assert_eq!(game.stations[0].services[0].trade[0].stock, Some(2));
    }

    #[test]
    fn recipe_vendor_unlock_purchase_gates_production_recipe() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.credits = 1_000;
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.stations[0].services[0].recipe_unlocks = vec![RecipeUnlockOffer {
            recipe: "core:advanced_scanner_core".to_string(),
            price: 250,
            unavailable: false,
        }];
        game.recipe_vendor_locked_recipes = recipe_vendor_locked_recipes(&game.stations);
        game.ship.position = vec2(120.0, 0.0);

        assert!(!recipe_is_unlocked(&game, "core:advanced_scanner_core"));
        assert!(purchase_recipe_unlock(&mut game, 0, 0, 0));
        assert_eq!(game.credits, 750);
        assert!(recipe_is_unlocked(&game, "core:advanced_scanner_core"));
        assert_eq!(
            game.purchased_recipe_unlocks,
            vec!["core:advanced_scanner_core".to_string()]
        );
    }

    #[test]
    fn remote_destination_pack_drives_route_discovery_and_system_switching() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");

        let remote_system = transition_target_system_id(&registry, STARTER_SYSTEM_ID)
            .expect("loaded content should provide a remote transition target");
        assert_eq!(remote_system, "remote-duskfall:duskfall_reach");
        assert!(registry.systems.contains_key(&remote_system));
        assert!(registry
            .planets
            .values()
            .any(|planet| planet.system == remote_system));

        let return_system = transition_target_system_id(&registry, &remote_system)
            .expect("loaded content should provide a return transition target");
        assert_eq!(return_system, STARTER_SYSTEM_ID);

        let remote_planet = registry
            .planets
            .values()
            .find(|planet| planet.system == remote_system)
            .expect("remote system should provide a planet")
            .id
            .clone();
        let mut game = test_game_with_systems(
            registry,
            vec![
                test_planet("core:near", STARTER_SYSTEM_ID, vec2(10.0, 0.0), true),
                test_planet(
                    &remote_planet,
                    "remote-duskfall:duskfall_reach",
                    vec2(1.0, 0.0),
                    true,
                ),
            ],
        );
        game.selected_planet = Some(0);
        game.destination_planet = Some(0);
        game.system_destinations
            .insert(remote_system.clone(), remote_planet.clone());
        game.ship.position = vec2(300.0, -200.0);
        game.ship.velocity = vec2(12.0, -4.0);
        game.ship.angular_velocity = 1.4;

        switch_current_system(&mut game, &remote_system);

        let arrival = game
            .content_registry
            .systems
            .get("remote-duskfall:duskfall_reach")
            .expect("remote system should exist")
            .arrival;
        assert_eq!(game.current_system_id, "remote-duskfall:duskfall_reach");
        assert_eq!(game.selected_planet, None);
        assert_eq!(game.destination_planet, Some(1));
        assert_eq!(game.ship.position, vec2(arrival[0], arrival[1]));
        assert_eq!(game.ship.velocity, Vec2::ZERO);
        assert_eq!(game.ship.angular_velocity, 0.0);
        assert_eq!(
            game.system_destinations.get(STARTER_SYSTEM_ID),
            Some(&"core:near".to_string())
        );
    }

    #[test]
    fn target_selection_ignores_bodies_outside_the_active_system() {
        let ship = Ship::starter();
        let planets = vec![
            test_planet("core:near", "core:frontier", vec2(10.0, 0.0), true),
            test_planet(
                "remote-duskfall:far",
                "remote-duskfall:duskfall_reach",
                vec2(1.0, 0.0),
                true,
            ),
        ];

        let target = target_planet(&ship, &planets, "core:frontier", None, Some(1))
            .expect("active system should have a fallback POI target");
        assert_eq!(target.id, "core:near");

        let target = target_planet(
            &ship,
            &planets,
            "remote-duskfall:duskfall_reach",
            Some(0),
            None,
        )
        .expect("remote active system should ignore core selection and choose remote POI");
        assert_eq!(target.id, "remote-duskfall:far");
    }

    #[test]
    fn known_systems_are_discoverable_routes_only() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let known_systems = known_system_ids(&registry);

        assert!(known_systems.contains(&STARTER_SYSTEM_ID.to_string()));
        assert!(known_systems.contains(&"remote-duskfall:duskfall_reach".to_string()));
        assert!(known_systems
            .iter()
            .all(|system_id| system_is_known(&registry, system_id)));
    }

    #[test]
    fn player_warp_charges_then_spends_fuel_and_switches_system() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let remote_system = "remote-duskfall:duskfall_reach".to_string();
        let mut game = test_game_with_systems(
            registry,
            vec![
                test_planet("core:near", STARTER_SYSTEM_ID, vec2(10.0, 0.0), true),
                test_planet("remote-duskfall:far", &remote_system, vec2(1.0, 0.0), true),
            ],
        );
        let fuel_canister = core_item(&game.content_registry, "fuel_canister")
            .expect("core fuel canister should exist");

        start_player_warp_charge(&mut game, remote_system.clone());
        assert!(game.pending_warp.is_none());

        game.inventory.add_item(fuel_canister.clone(), 1);
        start_player_warp_charge(&mut game, remote_system.clone());
        assert!(game.pending_warp.is_some());
        assert_eq!(game.inventory.count(&fuel_canister), 1);

        update_pending_warp(&mut game, WARP_CHARGE_SECONDS + 0.1);
        assert_eq!(game.inventory.count(&fuel_canister), 0);
        assert!(game.scene_transition.is_some());

        apply_transition_action(
            &mut game,
            TransitionAction::SwitchSystem(remote_system.clone()),
        );
        assert_eq!(game.current_system_id, remote_system);
    }

    #[test]
    fn old_scanned_save_loads_as_composition_scan() {
        let saved_planet = SavePlanet {
            id: "core:near".to_string(),
            scanned: true,
            scan_level: 0,
            mining: Vec::new(),
        };

        assert_eq!(scan_level_from_save(&saved_planet), 2);
    }

    #[test]
    fn survey_drones_advance_scan_levels() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(
            registry,
            vec![test_planet(
                "core:near",
                STARTER_SYSTEM_ID,
                Vec2::ZERO,
                true,
            )],
        );
        let survey_drone = core_item(&game.content_registry, "survey_drone")
            .expect("core survey drone should exist");
        let improved_survey_drone = core_item(&game.content_registry, "improved_survey_drone")
            .expect("core improved survey drone should exist");

        game.inventory.add_item(survey_drone.clone(), 1);
        assert!(launch_planet_scan(&mut game, 0));
        assert_eq!(game.planets[0].scan_level, 1);
        assert_eq!(game.inventory.count(&survey_drone), 0);

        game.inventory.add_item(improved_survey_drone.clone(), 1);
        assert!(launch_planet_scan(&mut game, 0));
        assert_eq!(game.planets[0].scan_level, MAX_SCAN_LEVEL);
        assert_eq!(game.inventory.count(&improved_survey_drone), 0);
        assert!(planet_has_surface_scan(&game.planets[0]));
        assert!(planet_has_composition_scan(&game.planets[0]));
        assert!(planet_has_richness_scan(&game.planets[0]));
    }

    #[test]
    fn scanner_array_upgrade_increases_survey_depth() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(
            registry,
            vec![test_planet(
                "core:scan_target",
                STARTER_SYSTEM_ID,
                vec2(20.0, 0.0),
                true,
            )],
        );
        let survey_drone = core_item(&game.content_registry, "survey_drone")
            .expect("core survey drone should exist");
        game.inventory.add_item(survey_drone, 1);
        game.ship.position = vec2(20.0, 0.0);
        game.ship_upgrades
            .iter_mut()
            .find(|upgrade| upgrade.kind == ShipUpgradeKind::ScannerArray)
            .expect("scanner array upgrade should be registered")
            .level = 2;

        assert!(launch_planet_scan(&mut game, 0));
        assert_eq!(game.planets[0].scan_level, 2);
    }

    #[test]
    fn richness_scan_uses_per_resource_modifiers() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut planet = test_planet_with_mineables(
            &registry,
            "core:rich",
            STARTER_SYSTEM_ID,
            Vec2::ZERO,
            &["core:iron_ore", "core:copper_ore"],
        );

        assert_eq!(mineable_richness_multiplier(&planet, 0), 1.0);
        planet.scan_level = MAX_SCAN_LEVEL;
        let iron_multiplier = mineable_richness_multiplier(&planet, 0);
        let copper_multiplier = mineable_richness_multiplier(&planet, 1);

        assert!((0.85..=1.60).contains(&iron_multiplier));
        assert!((0.85..=1.60).contains(&copper_multiplier));
        assert_ne!(iron_multiplier, copper_multiplier);
    }

    #[test]
    fn drone_bay_upgrade_adds_survey_drone_return_chance() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut upgrades = make_ship_upgrades();

        assert!(registry
            .upgrades
            .contains_key(ShipUpgradeKind::DroneBay.id()));
        assert!(registry
            .upgrades
            .contains_key(ShipUpgradeKind::FuelSystems.id()));
        assert!(registry
            .upgrades
            .contains_key(ShipUpgradeKind::ScannerArray.id()));
        assert!(registry
            .upgrades
            .contains_key(ShipUpgradeKind::CargoHold.id()));
        assert_eq!(survey_drone_return_chance(&upgrades), 0.0);
        assert_eq!(scanner_survey_bonus(&upgrades), 0);
        assert_eq!(warp_charge_seconds(&upgrades), WARP_CHARGE_SECONDS);
        assert_eq!(cargo_rating_kg(&upgrades), 20_000.0);
        upgrades
            .iter_mut()
            .find(|upgrade| upgrade.kind == ShipUpgradeKind::DroneBay)
            .expect("drone bay upgrade should be registered")
            .level = 3;
        upgrades
            .iter_mut()
            .find(|upgrade| upgrade.kind == ShipUpgradeKind::ScannerArray)
            .expect("scanner array upgrade should be registered")
            .level = 4;
        upgrades
            .iter_mut()
            .find(|upgrade| upgrade.kind == ShipUpgradeKind::FuelSystems)
            .expect("fuel systems upgrade should be registered")
            .level = 2;
        upgrades
            .iter_mut()
            .find(|upgrade| upgrade.kind == ShipUpgradeKind::CargoHold)
            .expect("cargo hold upgrade should be registered")
            .level = 2;
        assert_eq!(survey_drone_return_chance(&upgrades), 0.3);
        assert_eq!(scanner_survey_bonus(&upgrades), 2);
        assert!((warp_charge_seconds(&upgrades) - WARP_CHARGE_SECONDS * 0.8).abs() < 0.01);
        assert_eq!(cargo_rating_kg(&upgrades), 40_000.0);
    }

    #[test]
    fn hazard_effects_are_explicit_planet_configuration() {
        let mut planet = test_planet("core:hazard", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        planet.info.hazards = vec![
            "Radiation wake turbulence".to_string(),
            "Magnetic navigation drift".to_string(),
        ];

        assert_eq!(planet_hazard_mining_slowdown(&planet), 1.0);
        assert_eq!(planet_hazard_shield_drain_per_second(&planet), 0.0);

        planet.info.hazard_effects = HazardEffects {
            shield_drain_per_second: 1.2,
            mining_speed_multiplier: 1.25,
        };
        assert!((planet_hazard_mining_slowdown(&planet) - 1.25).abs() < f32::EPSILON);
        assert!((planet_hazard_shield_drain_per_second(&planet) - 1.2).abs() < f32::EPSILON);
    }

    #[test]
    fn duplicate_output_recipes_keep_distinct_settings_and_labels() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let recipes = make_processing_recipes(&registry);
        let uranium_index = recipes
            .iter()
            .position(|recipe| recipe.id == "core:uranium_reactor_pellet")
            .expect("uranium reactor pellet recipe should exist");
        let thorium_index = recipes
            .iter()
            .position(|recipe| recipe.id == "core:thorium_reactor_pellet")
            .expect("thorium reactor pellet recipe should exist");

        assert_eq!(
            recipes[uranium_index].output.item.id,
            recipes[thorium_index].output.item.id
        );
        assert_eq!(
            recipe_row_label(&recipes, uranium_index),
            "Reactor pellet (uranium)"
        );
        assert_eq!(
            recipe_row_label(&recipes, thorium_index),
            "Reactor pellet (thorium)"
        );

        let mut settings = vec![CraftSetting::starter(); recipes.len()];
        settings[uranium_index].keep = 12;
        settings[thorium_index].keep = 34;
        let saved = save_work_settings(&recipes, &settings, |recipe| recipe.id.as_str());

        assert!(saved
            .iter()
            .any(|setting| { setting.id == "core:uranium_reactor_pellet" && setting.keep == 12 }));
        assert!(saved
            .iter()
            .any(|setting| { setting.id == "core:thorium_reactor_pellet" && setting.keep == 34 }));

        let mut restored = vec![CraftSetting::starter(); recipes.len()];
        apply_work_settings(
            &mut restored,
            &recipes,
            &saved,
            |recipe| recipe.id.as_str(),
            |recipe| recipe.output.item.id.as_str(),
        );
        assert_eq!(restored[uranium_index].keep, 12);
        assert_eq!(restored[thorium_index].keep, 34);
    }

    #[test]
    fn escape_closes_active_menu_before_opening_dialog() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(
            registry,
            vec![test_planet(
                "core:near",
                STARTER_SYSTEM_ID,
                Vec2::ZERO,
                true,
            )],
        );
        game.inventory_open = true;
        game.selected_planet = Some(0);

        handle_escape_pressed(&mut game);

        assert!(!game.inventory_open);
        assert_eq!(game.selected_planet, None);
        assert!(!game.escape_dialog_open);
    }

    #[test]
    fn escape_closes_only_the_topmost_gameplay_overlay() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.inventory_open = true;
        game.selected_planet = Some(0);
        game.selected_station = Some(0);
        game.selected_station_service = Some(0);
        game.map_open = true;
        game.skills_open = true;
        game.upgrades_open = true;
        game.content_open = true;

        handle_escape_pressed(&mut game);
        assert!(!game.content_open);
        assert!(game.upgrades_open);
        assert!(!game.escape_dialog_open);

        handle_escape_pressed(&mut game);
        assert!(!game.upgrades_open);
        assert!(game.skills_open);
        assert!(!game.escape_dialog_open);

        handle_escape_pressed(&mut game);
        assert!(!game.skills_open);
        assert!(game.map_open);
        assert!(!game.escape_dialog_open);

        handle_escape_pressed(&mut game);
        assert!(!game.map_open);
        assert!(game.inventory_open);
        assert!(!game.escape_dialog_open);

        handle_escape_pressed(&mut game);
        assert!(!game.inventory_open);
        assert_eq!(game.selected_planet, None);
        assert_eq!(game.selected_station, None);
        assert_eq!(game.selected_station_service, None);
        assert!(!game.escape_dialog_open);

        handle_escape_pressed(&mut game);
        assert!(game.escape_dialog_open);
    }

    #[test]
    fn escape_opens_dialog_when_no_menu_is_active() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(
            registry,
            vec![test_planet(
                "core:near",
                STARTER_SYSTEM_ID,
                Vec2::ZERO,
                true,
            )],
        );

        handle_escape_pressed(&mut game);

        assert!(game.escape_dialog_open);
    }

    #[test]
    fn escape_dialog_resume_closes_dialog() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.escape_dialog_open = true;

        let result = apply_escape_dialog_action(&mut game, EscapeDialogAction::Resume);

        assert_eq!(result, EscapeDialogResult::Continue);
        assert!(!game.escape_dialog_open);
        assert!(!game.quit_to_title_requested);
    }

    #[test]
    fn escape_dialog_save_now_preserves_dialog_and_manual_save_feedback() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.save_path = test_save_path("escape-save-now");
        game.escape_dialog_open = true;
        game.save_dirty = true;

        let result = apply_escape_dialog_action(&mut game, EscapeDialogAction::SaveNow);

        assert_eq!(result, EscapeDialogResult::Continue);
        assert!(game.escape_dialog_open);
        assert!(!game.quit_to_title_requested);
        assert!(!game.save_dirty);
        assert!(game.save_status_manual);
        assert!(game.save_status_timer > 0.0);
        assert!(game.save_path.exists());

        let _ = fs::remove_file(&game.save_path);
    }

    #[test]
    fn escape_dialog_save_to_title_saves_and_requests_title_menu() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.save_path = test_save_path("escape-save-to-title");
        game.escape_dialog_open = true;
        game.save_dirty = true;

        let result = apply_escape_dialog_action(&mut game, EscapeDialogAction::SaveToTitle);

        assert_eq!(result, EscapeDialogResult::Continue);
        assert!(!game.escape_dialog_open);
        assert!(game.quit_to_title_requested);
        assert!(!game.save_dirty);
        assert!(game.save_status_manual);
        assert!(game.save_path.exists());

        let _ = fs::remove_file(&game.save_path);
    }

    #[test]
    fn escape_dialog_quit_desktop_saves_before_quit_request() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.save_path = test_save_path("escape-quit-desktop");
        game.escape_dialog_open = true;
        game.save_dirty = true;

        let result = apply_escape_dialog_action(&mut game, EscapeDialogAction::QuitDesktop);

        assert_eq!(result, EscapeDialogResult::QuitDesktop);
        assert!(!game.quit_to_title_requested);
        assert!(!game.save_dirty);
        assert!(game.save_status_manual);
        assert!(game.save_path.exists());

        let _ = fs::remove_file(&game.save_path);
    }

    #[test]
    fn orbit_entry_requires_interaction_range() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let planet = test_planet("core:near", STARTER_SYSTEM_ID, vec2(900.0, 0.0), true);
        let mut game = test_game_with_systems(registry, vec![planet]);

        assert!(!enter_planet_orbit(&mut game, 0));
        assert_eq!(game.orbiting_planet, None);

        let planet_position = game.planets[0].position;
        game.ship.position =
            planet_position + vec2(planet_interaction_radius(&game.planets[0]) - 1.0, 0.0);

        assert!(enter_planet_orbit(&mut game, 0));
        assert_eq!(game.orbiting_planet, Some(0));
        assert_vec2_near(
            game.ship.position,
            planet_position + vec2(planet_safe_orbit_radius(&game.planets[0]), 0.0),
        );
    }

    #[test]
    fn orbit_follow_tracks_moving_planet() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut planet = test_planet("core:orbiter", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        planet.motion = PlanetMotion::Orbit(OrbitMotion {
            center: Vec2::ZERO,
            anchor_planet: None,
            radius: 300.0,
            semi_minor: 300.0,
            axis_rotation: 0.0,
            period_days: content::MIN_ORBIT_PERIOD_DAYS,
            phase: 0.0,
        });
        planet.position = runtime_planet_position(&planet, 0.0);
        let mut game = test_game_with_systems(registry, vec![planet]);
        game.ship.position = game.planets[0].position + vec2(30.0, 0.0);

        assert!(enter_planet_orbit(&mut game, 0));
        let initial_ship_position = game.ship.position;
        advance_world_time_and_planets(&mut game, GAME_DAY_SECONDS);
        update_ship_orbit(&mut game);

        assert_ne!(game.ship.position, initial_ship_position);
        assert!(
            (game.ship.position.distance(game.planets[0].position)
                - planet_safe_orbit_radius(&game.planets[0]))
            .abs()
                < 0.01
        );
        assert_eq!(game.ship.velocity, Vec2::ZERO);
    }

    #[test]
    fn orbit_breaks_on_destination_or_warp() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let remote_system = "remote-duskfall:duskfall_reach".to_string();
        let mut game = test_game_with_systems(
            registry,
            vec![
                test_planet("core:near", STARTER_SYSTEM_ID, Vec2::ZERO, true),
                test_planet(
                    "core:destination",
                    STARTER_SYSTEM_ID,
                    vec2(400.0, 0.0),
                    true,
                ),
                test_planet("remote-duskfall:far", &remote_system, Vec2::ZERO, true),
            ],
        );
        game.ship.position = Vec2::ZERO;

        assert!(enter_planet_orbit(&mut game, 0));
        set_destination_planet(&mut game, Some(1));
        assert_eq!(game.orbiting_planet, None);
        assert_eq!(game.destination_planet, Some(1));

        assert!(enter_planet_orbit(&mut game, 0));
        let fuel_canister = core_item(&game.content_registry, "fuel_canister")
            .expect("core fuel canister should exist");
        game.inventory.add_item(fuel_canister, 1);
        start_player_warp_charge(&mut game, remote_system);
        assert_eq!(game.orbiting_planet, None);
        assert!(game.pending_warp.is_some());
    }

    #[test]
    fn orbit_mitigates_configured_shield_drain() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut planet = test_planet("core:hazard", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        planet.info.hazard_effects = HazardEffects {
            shield_drain_per_second: 5.0,
            mining_speed_multiplier: 1.0,
        };
        let mut game = test_game_with_systems(registry, vec![planet]);
        let max_shields = game.ship.systems.shields.max;
        game.ship.systems.shields.current = max_shields;

        update_orbital_hazards(&mut game, 1.0);
        assert_eq!(game.ship.systems.shields.current, max_shields - 5.0);

        game.ship.systems.shields.current = max_shields;
        assert!(enter_planet_orbit(&mut game, 0));
        update_orbital_hazards(&mut game, 1.0);
        assert_eq!(game.ship.systems.shields.current, max_shields);
    }

    #[test]
    fn starmap_filters_match_scan_destination_and_resource_state() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut scanned = test_planet_with_mineables(
            &registry,
            "core:scanned",
            STARTER_SYSTEM_ID,
            Vec2::ZERO,
            &["core:iron_ore"],
        );
        scanned.scan_level = 2;
        let unscanned = test_planet_with_mineables(
            &registry,
            "core:unscanned",
            STARTER_SYSTEM_ID,
            Vec2::ZERO,
            &["core:copper_ore"],
        );
        let mut game = test_game_with_systems(registry, vec![scanned, unscanned]);
        game.destination_planet = Some(1);

        game.starmap_filter = StarmapFilter::Scanned;
        assert!(planet_matches_starmap_filter(&game, 0, &game.planets[0]));
        assert!(!planet_matches_starmap_filter(&game, 1, &game.planets[1]));

        game.starmap_filter = StarmapFilter::Unscanned;
        assert!(!planet_matches_starmap_filter(&game, 0, &game.planets[0]));
        assert!(planet_matches_starmap_filter(&game, 1, &game.planets[1]));

        game.starmap_filter = StarmapFilter::Destination;
        assert!(!planet_matches_starmap_filter(&game, 0, &game.planets[0]));
        assert!(planet_matches_starmap_filter(&game, 1, &game.planets[1]));

        game.starmap_filter = StarmapFilter::Resource;
        game.starmap_resource_filter_index = 0;
        let resource =
            selected_starmap_resource_filter(&game).expect("scanned resource filter should exist");
        assert_eq!(resource.id, "core:iron_ore");
        assert!(planet_matches_starmap_filter(&game, 0, &game.planets[0]));
        assert!(!planet_matches_starmap_filter(&game, 1, &game.planets[1]));
    }

    #[test]
    fn static_planets_keep_seeded_positions_without_orbit_metadata() {
        let planet = test_planet("core:static", STARTER_SYSTEM_ID, vec2(123.0, -456.0), true);

        assert_eq!(runtime_planet_position(&planet, 0.0), planet.position);
        assert_eq!(runtime_planet_position(&planet, 9999.0), planet.position);
    }

    #[test]
    fn orbit_motion_computes_runtime_position_from_elapsed_days() {
        let mut planet = test_planet("core:orbiter", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        planet.motion = PlanetMotion::Orbit(OrbitMotion {
            center: vec2(10.0, -20.0),
            anchor_planet: None,
            radius: 100.0,
            semi_minor: 100.0,
            axis_rotation: 0.0,
            period_days: 40.0,
            phase: 0.0,
        });

        assert_vec2_near(runtime_planet_position(&planet, 0.0), vec2(110.0, -20.0));
        assert_vec2_near(runtime_planet_position(&planet, 10.0), vec2(10.0, 80.0));
        assert_vec2_near(runtime_planet_position(&planet, 20.0), vec2(-90.0, -20.0));
    }

    #[test]
    fn core_orbit_metadata_produces_visible_runtime_variation() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let active_orbits = registry
            .planet_order
            .iter()
            .filter_map(|planet_id| registry.planets.get(planet_id))
            .filter(|planet| planet.system == STARTER_SYSTEM_ID)
            .filter_map(
                |planet| match planet_motion_from_def(&registry, planet, 1) {
                    PlanetMotion::Orbit(orbit) => Some(orbit),
                    PlanetMotion::Static => None,
                },
            )
            .collect::<Vec<_>>();

        assert_eq!(active_orbits.len(), 20);
        assert!(active_orbits
            .iter()
            .any(|orbit| (orbit.radius - orbit.semi_minor).abs() > 1.0));
        assert!(active_orbits
            .iter()
            .any(|orbit| orbit.axis_rotation.abs() > 0.01));
    }

    #[test]
    fn advancing_world_time_updates_runtime_planet_positions() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut planet = test_planet("core:orbiter", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        planet.motion = PlanetMotion::Orbit(OrbitMotion {
            center: Vec2::ZERO,
            anchor_planet: None,
            radius: 100.0,
            semi_minor: 100.0,
            axis_rotation: 0.0,
            period_days: content::MIN_ORBIT_PERIOD_DAYS,
            phase: 0.0,
        });
        planet.position = runtime_planet_position(&planet, 0.0);
        let mut game = test_game_with_systems(registry, vec![planet]);

        assert_vec2_near(game.planets[0].position, vec2(100.0, 0.0));
        advance_world_time_and_planets(&mut game, GAME_DAY_SECONDS * 0.25);
        assert_vec2_near(
            game.planets[0].position,
            orbit_position(
                OrbitMotion {
                    center: Vec2::ZERO,
                    anchor_planet: None,
                    radius: 100.0,
                    semi_minor: 100.0,
                    axis_rotation: 0.0,
                    period_days: content::MIN_ORBIT_PERIOD_DAYS,
                    phase: 0.0,
                },
                0.25,
            ),
        );
    }

    fn test_planet(id: &str, system: &str, position: Vec2, is_poi: bool) -> Planet {
        Planet {
            id: id.to_string(),
            system: system.to_string(),
            faction: None,
            base_position: position,
            position,
            motion: PlanetMotion::Static,
            radius: 64.0,
            is_poi,
            texture: None,
            info: PlanetInfo {
                classification: "Test Planet".to_string(),
                mineables: Vec::new(),
                hazards: Vec::new(),
                hazard_effects: HazardEffects {
                    shield_drain_per_second: 0.0,
                    mining_speed_multiplier: 1.0,
                },
                summary: "Test planet.".to_string(),
            },
            mining: Vec::new(),
            scan_level: 0,
        }
    }

    fn test_planet_with_mineables(
        content_registry: &content::ContentRegistry,
        id: &str,
        system: &str,
        position: Vec2,
        item_ids: &[&str],
    ) -> Planet {
        let mut planet = test_planet(id, system, position, true);
        planet.info.mineables = item_ids
            .iter()
            .map(|item_id| Mineable {
                item: registry_item(content_registry, item_id)
                    .expect("test mineable item should exist"),
            })
            .collect();
        planet.mining = vec![MiningSetting::starter(); planet.info.mineables.len()];
        planet
    }

    fn test_station_destination(id: &str, system: &str, position: Vec2) -> StationDestination {
        StationDestination {
            id: id.to_string(),
            system: system.to_string(),
            name: "Test Station".to_string(),
            position,
            radius: 48.0,
            texture: None,
            icon: "station".to_string(),
            culture: Some("Test Culture".to_string()),
            faction: Some("Test Faction".to_string()),
            summary: "A station used by tests.".to_string(),
            services: vec![
                StationService {
                    id: "core:test_market".to_string(),
                    name: "Test Market".to_string(),
                    kind: "shop".to_string(),
                    description: Some("A test shop service.".to_string()),
                    trade: Vec::new(),
                    recipe_unlocks: Vec::new(),
                },
                StationService {
                    id: "core:test_garage".to_string(),
                    name: "Test Garage".to_string(),
                    kind: "garage".to_string(),
                    description: Some("A test garage service.".to_string()),
                    trade: Vec::new(),
                    recipe_unlocks: Vec::new(),
                },
            ],
        }
    }

    fn test_defense_threat(
        id: &str,
        disposition: ThreatDisposition,
        position: Vec2,
        hull: f32,
    ) -> DefenseThreat {
        DefenseThreat {
            id: id.to_string(),
            name: "Test threat".to_string(),
            system: STARTER_SYSTEM_ID.to_string(),
            position,
            radius: DEFENSE_THREAT_RADIUS,
            disposition,
            hull: ShipResource::full(hull),
        }
    }

    fn test_npc_ship(behavior: NpcBehaviorMode, position: Vec2) -> NpcShip {
        NpcShip {
            id: "core:test_npc".to_string(),
            name: "Test NPC".to_string(),
            system: STARTER_SYSTEM_ID.to_string(),
            position,
            velocity: Vec2::ZERO,
            angle: 0.0,
            radius: 24.0,
            texture: None,
            archetype: "test".to_string(),
            role: behavior.label().to_string(),
            faction: None,
            behavior_tags: Vec::new(),
            behavior,
            route_index: 0,
            anchor: position,
            identified: false,
            cargo_capacity: 100.0,
            cargo_defaults: Vec::new(),
            hull: ShipResource::full(50.0),
            shields: ShipResource::full(25.0),
            energy: ShipResource::full(20.0),
            shield_slots: Vec::new(),
            weapon_slots: Vec::new(),
            summary: "Test NPC ship.".to_string(),
        }
    }

    fn assert_vec2_near(actual: Vec2, expected: Vec2) {
        assert!(
            actual.distance(expected) < 0.01,
            "expected {expected:?}, got {actual:?}"
        );
    }

    fn test_game_with_systems(
        content_registry: content::ContentRegistry,
        planets: Vec<Planet>,
    ) -> GameState {
        GameState {
            content_registry,
            content_pack_options: Vec::new(),
            transition_assets: Vec::new(),
            scene_transition: None,
            current_system_id: STARTER_SYSTEM_ID.to_string(),
            save_path: save_state_path(),
            world_seed: 1,
            world_elapsed_days: 0.0,
            credits: default_credits(),
            ship: Ship::starter(),
            installed_power_modules: Vec::new(),
            equipped_shields: Vec::new(),
            equipped_weapons: Vec::new(),
            npc_ships: Vec::new(),
            defense_threats: Vec::new(),
            weapon_fire_events: Vec::new(),
            ship_texture: None,
            system_light_haze_texture: None,
            system_stars: Vec::new(),
            planets,
            stations: Vec::new(),
            recipe_vendor_locked_recipes: Vec::new(),
            purchased_recipe_unlocks: Vec::new(),
            selected_planet: None,
            selected_station: None,
            selected_npc_ship: None,
            selected_station_service: None,
            destination_planet: None,
            orbiting_planet: None,
            system_destinations: HashMap::new(),
            pending_warp: None,
            camera_zoom: 1.0,
            starmap_zoom: 1.0,
            starmap_pan: Vec2::ZERO,
            starmap_drag_previous_mouse: None,
            action_rail_width_override: None,
            action_rail_resize_previous_mouse: None,
            inventory: Inventory {
                slots: std::array::from_fn(|_| None),
            },
            smelt_recipes: Vec::new(),
            smelt_settings: Vec::new(),
            craft_recipes: Vec::new(),
            craft_settings: Vec::new(),
            processing_recipes: Vec::new(),
            processing_settings: Vec::new(),
            production_mode: ProductionMode::Smelting,
            skills: make_skills(),
            ship_upgrades: make_ship_upgrades(),
            inventory_open: false,
            map_open: false,
            skills_open: false,
            upgrades_open: false,
            content_open: false,
            content_browser: ContentBrowserState::default(),
            escape_dialog_open: false,
            quit_to_title_requested: false,
            starmap_filter: StarmapFilter::All,
            starmap_resource_filter_index: 0,
            work_scroll: 0.0,
            inventory_scroll: 0.0,
            upgrades_scroll: 0.0,
            shield_recharge_delay_remaining: 0.0,
            last_window_size: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
            window_save_delay: None,
            save_delay: None,
            save_dirty: false,
            save_status_timer: 0.0,
            save_status_manual: false,
        }
    }
}
