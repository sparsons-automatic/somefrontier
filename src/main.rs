mod audio;
mod branding_icon;
mod content;
mod egui_art;
mod egui_content;
mod egui_context_menu;
mod egui_contracts;
mod egui_debug;
mod egui_destinations;
mod egui_escape;
mod egui_fleet;
mod egui_inventory;
mod egui_loading;
mod egui_npc;
mod egui_pin;
mod egui_planet;
mod egui_research;
mod egui_station;
mod egui_title;
mod egui_upgrades;
mod remote_assets;

use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet},
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread::JoinHandle,
    time::{SystemTime, UNIX_EPOCH},
};

const DEFAULT_WINDOW_WIDTH: i32 = 1100;
const DEFAULT_WINDOW_HEIGHT: i32 = 760;
const BRANDING_LOGO_PATH: &str = "assets/branding/some-frontier-logo.png";
const UI_PANEL_CORNER_PATH: &str = "assets/ui/research-panel-corner-top-left.png";
const TITLE_SHIP_GRID_COLUMNS: usize = 3;
const GAME_PANEL_CONTENT_PAD_X: f32 = 56.0;
const GAME_PANEL_HEADER_PAD_X: f32 = 92.0;
const GAME_PANEL_HEADER_BASELINE: f32 = 62.0;
const GAME_PANEL_BODY_TOP: f32 = 94.0;
const STARFIELD_RADIUS: f32 = 9000.0;
const SHIP_RADIUS: f32 = 22.0;
const SHIP_SPRITE_SIZE: f32 = 72.0;
const DEFENSE_THREAT_RADIUS: f32 = 18.0;
const WEAPON_FIRE_EVENT_SECONDS: f32 = 0.55;
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
const PLAYER_FOLLOW_DISTANCE: f32 = 120.0;
const PLAYER_APPROACH_SPEED: f32 = 240.0;
const AUTOPILOT_CLEARANCE_MARGIN: f32 = 18.0;
const AUTOPILOT_INTERACTION_INSET: f32 = 12.0;
const NPC_HOSTILE_STANDOFF_DISTANCE: f32 = 360.0;
const NPC_PRESSURE_RANGE: f32 = 520.0;
const REDWAKE_PROBE_PRESSURE_PER_SECOND: f32 = 2.4;
const REDWAKE_PRESSURE_HULL_SPILLOVER: f32 = 0.35;
const NPC_ROUTE_RADIUS: f32 = 520.0;
const NPC_ROUTE_POINTS: [[f32; 2]; 4] = [[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0]];
const NPC_INTERACTION_PADDING: f32 = 126.0;
const CONTEXT_TARGET_AMBIGUITY_PIXELS: f32 = 24.0;
const COLLISION_MIN_CLOSING_SPEED: f32 = 24.0;
const COLLISION_DAMAGE_COEFFICIENT: f32 = 0.006;
const COLLISION_REFERENCE_MASS: f32 = 100_000.0;
const COLLISION_COOLDOWN_SECONDS: f32 = 0.6;
const COLLISION_EVENT_SECONDS: f32 = 0.8;
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
const STARMAP_ZOOM_MIN: f32 = 0.45;
const STARMAP_ZOOM_MAX: f32 = 3.5;
const STARMAP_ZOOM_STEP: f32 = 1.15;
const STARMAP_PAN_PIXELS_TO_WORLD: f32 = 1.35;
const ARC_SEGMENTS: usize = 72;
const INVENTORY_SLOTS: usize = 200;
const SHIP_UPGRADE_COUNT: usize = 8;
const OWNED_SHIP_TRANSFER_RANGE: f32 = 180.0;
const DEBUG_CONSOLE_HISTORY_LIMIT: usize = 32;
const WORK_ROW_HEIGHT: f32 = 30.0;
const INVENTORY_ROW_HEIGHT: f32 = 30.0;
const SHIP_UPGRADE_ROW_HEIGHT: f32 = 54.0;
const SAVE_VERSION: u32 = 2;
const AUTOSAVE_SECONDS: f32 = 60.0;
const EGUI_LAYOUT_SAVE_DELAY: f32 = 0.75;
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
const STARTUP_BACKGROUND_HOLD_SECONDS: f32 = 3.0;
const STARTUP_BACKGROUND_FADE_SECONDS: f32 = 2.0;
const STATION_APPROACH_TRANSITION_ID: &str = "frontier-station-approach";
const KNOWN_SYSTEMS_PANEL_WIDTH: f32 = 280.0;
const KNOWN_SYSTEM_ROW_HEIGHT: f32 = 70.0;
const WARP_CHARGE_SECONDS: f32 = 2.0;
const MAX_SCAN_LEVEL: u8 = 3;
const OPERATION_FEEDBACK_LIMIT: usize = 6;
const STARTER_SYSTEM_ID: &str = "core:frontier";
const UI_FONT_PATH: &str = "assets/fonts/Junicode.ttf";

thread_local! {
    static UI_FONT: RefCell<Option<Font>> = const { RefCell::new(None) };
    static BRANDING_LOGO: RefCell<Option<Texture2D>> = const { RefCell::new(None) };
    static STARTUP_PROGRESS: RefCell<Option<StartupProgress>> = const { RefCell::new(None) };
    static EGUI_LAYOUT_PERSISTENCE: RefCell<EguiLayoutPersistence> = RefCell::new(EguiLayoutPersistence::default());
    static EGUI_PIN_PERSISTENCE: RefCell<EguiPinPersistence> = RefCell::new(EguiPinPersistence::default());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct StartupProgress {
    completed: usize,
    total: usize,
}

impl StartupProgress {
    fn fraction(self) -> f32 {
        self.completed as f32 / self.total.max(1) as f32
    }

    fn count(self) -> String {
        format!("{} / {} startup steps", self.completed, self.total)
    }
}

fn begin_startup_progress(total: usize) {
    STARTUP_PROGRESS.with(|progress| {
        *progress.borrow_mut() = Some(StartupProgress {
            completed: 0,
            total: total.max(1),
        });
    });
}

fn reset_startup_progress() {
    STARTUP_PROGRESS.with(|progress| *progress.borrow_mut() = None);
}

fn advance_startup_progress() {
    STARTUP_PROGRESS.with(|progress| {
        if let Some(progress) = progress.borrow_mut().as_mut() {
            progress.completed = progress.completed.saturating_add(1).min(progress.total);
        }
    });
}

fn startup_progress() -> Option<StartupProgress> {
    STARTUP_PROGRESS.with(|progress| *progress.borrow())
}

#[derive(Default)]
struct EguiLayoutPersistence {
    loaded: bool,
    last_written: String,
    pending: Option<String>,
    pending_seconds: f32,
}

#[derive(Default)]
struct EguiPinPersistence {
    loaded: bool,
    pinned: BTreeSet<String>,
}

#[derive(Default, Serialize, Deserialize)]
struct SavedEguiPins {
    #[serde(default)]
    pinned: BTreeSet<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EguiPane {
    Ship,
    Production,
    Inventory,
    Planet,
    Station,
    Contact,
    Fleet,
    Content,
    Contracts,
    Upgrades,
    Research,
    Destinations,
}

impl EguiPane {
    fn id(self) -> &'static str {
        match self {
            Self::Ship => "ship",
            Self::Production => "production",
            Self::Inventory => "inventory",
            Self::Planet => "planet",
            Self::Station => "station",
            Self::Contact => "contact",
            Self::Fleet => "fleet",
            Self::Content => "content",
            Self::Contracts => "contracts",
            Self::Upgrades => "upgrades",
            Self::Research => "research",
            Self::Destinations => "destinations",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct EguiPinSnapshot {
    ship: bool,
    production: bool,
    inventory: bool,
    planet: bool,
    station: bool,
    contact: bool,
    fleet: bool,
    content: bool,
    contracts: bool,
    upgrades: bool,
    research: bool,
    destinations: bool,
}

const PERSISTED_EGUI_WINDOW_IDS: [&str; 14] = [
    "inventory-ship-window",
    "inventory-production-window",
    "inventory-cargo-window",
    "planet-workspace-window",
    "station-workspace-window",
    "npc-contact-window",
    "fleet-workspace-window",
    "content-browser-window",
    "contracts-workspace-window",
    "ship-upgrades-window",
    "research-workspace-window",
    "starmap-destinations-window",
    "escape-menu-window",
    "debug-console-window",
];

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
    id: String,
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
    runtime_flags: RuntimeFlags,
    audio: audio::AudioManager,
    content_registry: content::ContentRegistry,
    content_pack_options: Vec<PackOptionSelection>,
    transition_assets: Vec<TransitionAsset>,
    scene_transition: Option<SceneTransition>,
    current_system_id: String,
    save_path: PathBuf,
    world_seed: u64,
    world_elapsed_days: f32,
    credits: u32,
    owned_ships: Vec<OwnedShip>,
    active_owned_ship_id: String,
    next_owned_ship_sequence: u64,
    ship_id: String,
    ship: Ship,
    installed_power_modules: Vec<PowerModule>,
    equipped_shields: Vec<ShieldSystem>,
    equipped_weapons: Vec<WeaponSystem>,
    npc_ships: Vec<NpcShip>,
    collision_cooldowns: HashMap<String, f32>,
    collision_impact_events: Vec<CollisionImpactEvent>,
    defense_threats: Vec<DefenseThreat>,
    weapon_fire_events: Vec<WeaponFireEvent>,
    weapon_area_effect_events: Vec<WeaponAreaEffectEvent>,
    weapon_projectile_textures: HashMap<String, Texture2D>,
    ship_textures: HashMap<String, Texture2D>,
    system_light_haze_texture: Option<Texture2D>,
    system_stars: Vec<SystemStar>,
    planets: Vec<Planet>,
    stations: Vec<StationDestination>,
    recipe_vendor_locked_recipes: Vec<String>,
    active_research: Option<ActiveResearch>,
    completed_research: Vec<String>,
    selected_planet: Option<usize>,
    selected_station: Option<usize>,
    selected_npc_ship: Option<usize>,
    selected_station_service: Option<usize>,
    active_contracts: Vec<ActiveContract>,
    faction_reputation: HashMap<String, i32>,
    selected_research: Option<String>,
    destination_planet: Option<usize>,
    orbiting_planet: Option<usize>,
    system_destinations: HashMap<String, String>,
    starmap_selected_planet: Option<usize>,
    navigation_route: Vec<NavigationStop>,
    navigation_route_current: Option<usize>,
    navigation_route_active: bool,
    pending_warp: Option<PendingWarp>,
    camera_zoom: f32,
    starmap_zoom: f32,
    starmap_pan: Vec2,
    starmap_drag_previous_mouse: Option<Vec2>,
    starmap_panel_position: Option<Vec2>,
    starmap_panel_size: Option<Vec2>,
    starmap_move_previous_mouse: Option<Vec2>,
    starmap_resize_previous_mouse: Option<Vec2>,
    open_turret_bank: Option<usize>,
    inventory: Inventory,
    smelt_recipes: Vec<Recipe>,
    smelt_settings: Vec<CraftSetting>,
    craft_recipes: Vec<Recipe>,
    craft_settings: Vec<CraftSetting>,
    processing_recipes: Vec<Recipe>,
    processing_settings: Vec<CraftSetting>,
    production_mode: ProductionMode,
    ship_upgrades: [ShipUpgrade; SHIP_UPGRADE_COUNT],
    inventory_open: bool,
    map_open: bool,
    research_open: bool,
    upgrades_open: bool,
    content_open: bool,
    contracts_open: bool,
    fleet_open: bool,
    selected_owned_ship_index: usize,
    fleet_transfer_quantity: u32,
    content_browser: ContentBrowserState,
    escape_dialog_open: bool,
    quit_to_title_requested: bool,
    starmap_filter: StarmapFilter,
    starmap_resource_filter_index: usize,
    work_scroll: f32,
    contract_menu_scroll: f32,
    selected_contract_index: Option<usize>,
    inventory_scroll: f32,
    upgrades_scroll: f32,
    shield_recharge_delay_remaining: f32,
    last_window_size: (i32, i32),
    window_save_delay: Option<f32>,
    save_delay: Option<f32>,
    save_dirty: bool,
    save_status_timer: f32,
    save_status_manual: bool,
    operation_feedback: Vec<OperationFeedback>,
    debug_console: DebugConsole,
    context_action_menu: Option<ContextActionMenu>,
    follow_target: Option<FollowTarget>,
    approach_target: Option<ApproachTarget>,
    npc_trade_quantity: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct OperationFeedback {
    category: String,
    message: String,
    aggregate_key: Option<String>,
    count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CollisionImpactSeverity {
    Minor,
    Significant,
    Critical,
    Destructive,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CollisionImpactEvent {
    position: Vec2,
    timer: f32,
    shield_damage: f32,
    hull_damage: f32,
    severity: CollisionImpactSeverity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ContextTarget {
    Planet(usize),
    Station(usize),
    NpcShip(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ContextAction {
    Inspect,
    Approach,
    Orbit,
    Scan,
    Mine,
    Dock,
    Contracts,
    Repair,
    Follow,
    StopFollowing,
    Hail,
    Trade,
}

#[derive(Clone, Debug, PartialEq)]
struct ContextActionMenu {
    content: ContextMenuContent,
    position: Vec2,
}

#[derive(Clone, Debug, PartialEq)]
enum ContextMenuContent {
    Actions(ContextTarget),
    ChooseTarget(Vec<ContextTarget>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ContextTargetCandidate {
    target: ContextTarget,
    cursor_distance: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FollowTarget {
    NpcShip(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ApproachTarget {
    Planet(usize),
    PlanetOrbit(usize),
    Station(usize),
    NpcShip(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NavigationStop {
    system: String,
    planet: String,
}

struct ContextMenuEntry {
    action: ContextAction,
    label: &'static str,
    enabled: bool,
    detail: &'static str,
}

#[derive(Clone, Default)]
struct ContentBrowserState {
    selected_pack_index: Option<usize>,
    selected_object: Option<egui_content::ObjectKey>,
    object_history: Vec<egui_content::ObjectKey>,
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
    Title(Box<TitleMenu>),
    Playing(Box<GameState>),
}

struct TitleMenu {
    view: TitleView,
    pending_action: Option<TitleAction>,
    branding_egui_image: Option<egui::ColorImage>,
    branding_egui_texture: Option<egui::TextureHandle>,
    new_game_seed_text: String,
    ships: Vec<TitleShip>,
    selected_ship_index: usize,
    ship_textures: HashMap<String, TitleShipPreview>,
    save_slots: Vec<TitleSaveSlot>,
    selected_save_index: usize,
    pending_delete_save_index: Option<usize>,
    delete_save_error: Option<String>,
    content_packs: Vec<TitleContentPack>,
    selected_pack_index: usize,
    settings: AppSettings,
    selected_settings_category: SettingsCategory,
}

#[derive(Clone, Debug, PartialEq)]
struct TitleShip {
    id: String,
    name: String,
    texture: Option<String>,
    hull_capacity: f32,
    shield_capacity: f32,
    weapon_banks: usize,
}

struct TitleShipPreview {
    egui_image: egui::ColorImage,
    egui_texture: Option<egui::TextureHandle>,
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
        ship_id: String,
        pack_options: Vec<PackOptionSelection>,
    },
    LoadGame {
        path: PathBuf,
    },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct RuntimeFlags {
    debug: bool,
}

struct RemoteAudioStartup {
    progress: remote_assets::RemoteAssetProgressHandle,
    handle: Option<JoinHandle<Result<remote_assets::DownloadReport, String>>>,
    report: Option<remote_assets::DownloadReport>,
}

impl RemoteAudioStartup {
    fn start() -> Self {
        let progress = Arc::new(Mutex::new(remote_assets::RemoteAssetProgress::checking()));
        let handle = match remote_assets::RemoteAssetClient::from_environment() {
            Ok(client) => Some(client.spawn_download_with_progress(
                remote_assets::UreqTransport,
                remote_assets::RemoteAssetCache::new(config_dir().join("remote-assets")),
                Vec::new(),
                Some(progress.clone()),
            )),
            Err(error) => {
                if let Ok(mut state) = progress.lock() {
                    state.phase = remote_assets::RemoteAssetPhase::Failed;
                    state.message = error;
                }
                None
            }
        };
        Self {
            progress,
            handle,
            report: None,
        }
    }

    fn poll(&mut self) {
        let finished = self.handle.as_ref().is_some_and(JoinHandle::is_finished);
        if !finished {
            return;
        }
        if let Some(handle) = self.handle.take() {
            match handle
                .join()
                .unwrap_or_else(|_| Err("Remote audio worker stopped unexpectedly".to_string()))
            {
                Ok(report) => self.report = Some(report),
                Err(error) => {
                    if let Ok(mut state) = self.progress.lock() {
                        state.phase = remote_assets::RemoteAssetPhase::Failed;
                        state.message = error;
                    }
                }
            }
        }
    }

    fn draw(&mut self) {
        self.poll();
        let progress = self.snapshot();
        draw_remote_audio_startup(&progress);
    }

    fn current(&mut self) -> remote_assets::RemoteAssetProgress {
        self.poll();
        self.snapshot()
    }

    fn snapshot(&self) -> remote_assets::RemoteAssetProgress {
        self.progress
            .lock()
            .map(|state| state.clone())
            .unwrap_or_else(|_| remote_assets::RemoteAssetProgress::checking())
    }

    async fn finish(mut self) -> Option<remote_assets::DownloadReport> {
        let started = get_time();
        while self.handle.is_some() && get_time() - started < 5.0 {
            self.draw();
            next_frame().await;
        }
        if self.handle.is_some() {
            self.handle.take();
            if let Ok(mut state) = self.progress.lock() {
                state.phase = remote_assets::RemoteAssetPhase::Offline;
                state.message =
                    "Remote audio is still loading; continuing with local assets".to_string();
            }
        } else {
            self.draw();
        }
        next_frame().await;
        self.report
    }
}

#[derive(Clone, Default)]
struct DebugConsole {
    open: bool,
    input: String,
    history: Vec<String>,
}

enum TitleAction {
    NewGame {
        seed: u64,
        ship_id: String,
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
    ResetMenus,
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
    generation: f32,
    heat: f32,
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
    resolved: bool,
    name: String,
    kind: content::WeaponKind,
    install_item: String,
    range: f32,
    cooldown_seconds: f32,
    damage: f32,
    energy_cost: f32,
    ammo_item: Option<String>,
    ammo_per_shot: u32,
    tracking_degrees: f32,
    targeting: content::WeaponTargeting,
    effect: content::WeaponEffect,
    beam_color: [u8; 4],
    core_color: [u8; 4],
    impact_color: [u8; 4],
    fire_duration_seconds: f32,
    path_curve_strength: f32,
    path_wobble: f32,
    path_cycles: f32,
    trail_length: f32,
    burst_count: u8,
    travel_speed: Option<f32>,
    projectile_texture: Option<String>,
    projectile_size: f32,
    impact: content::WeaponImpact,
    splash_radius: f32,
    splash_falloff: content::DamageFalloff,
    splash_min_multiplier: f32,
    chain_targets: u8,
    chain_range: f32,
    chain_damage_multiplier: f32,
    friendly_fire: content::FriendlyFire,
    fire_audio: Option<String>,
    cooldown_remaining: f32,
    status: WeaponStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WeaponStatus {
    Ready,
    NoThreat,
    Cooldown,
    InsufficientEnergy,
    OutOfAmmo,
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

#[derive(Clone)]
struct WeaponFireEvent {
    from: Vec2,
    to: Vec2,
    timer: f32,
    duration: f32,
    effect: content::WeaponEffect,
    beam_color: [u8; 4],
    core_color: [u8; 4],
    impact_color: [u8; 4],
    path_curve_strength: f32,
    path_wobble: f32,
    path_cycles: f32,
    trail_length: f32,
    burst_count: u8,
    projectile_texture: Option<String>,
    projectile_size: f32,
    origin: WeaponFireOrigin,
    pending_impact: Option<PendingWeaponImpact>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum WeaponTargetRef {
    Player,
    OwnedShip(String),
    Npc(String),
    Threat(String),
}

#[derive(Clone)]
enum WeaponImpactOwner {
    Player,
    OwnedShip(String),
    Npc { id: String, hostile: bool },
}

#[derive(Clone)]
struct PendingWeaponImpact {
    owner: WeaponImpactOwner,
    primary_target: WeaponTargetRef,
    system: String,
    damage: f32,
    targeting: content::WeaponTargeting,
    impact: content::WeaponImpact,
    splash_radius: f32,
    splash_falloff: content::DamageFalloff,
    splash_min_multiplier: f32,
    chain_targets: u8,
    chain_range: f32,
    chain_damage_multiplier: f32,
    friendly_fire: content::FriendlyFire,
}

struct WeaponAreaEffectEvent {
    position: Vec2,
    radius: f32,
    timer: f32,
    duration: f32,
    color: [u8; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WeaponFireOrigin {
    Player,
    Npc,
}

#[derive(Clone, Copy)]
enum PlayerTurretTarget {
    DefenseThreat(usize),
    NpcShip(usize),
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
    mass: f32,
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
    trade_open: bool,
    credit_reward_min: u32,
    credit_reward_max: u32,
    hull: ShipResource,
    shields: ShipResource,
    equipped_shields: Vec<ShieldSystem>,
    shield_recharge_delay_remaining: f32,
    energy: ShipResource,
    shield_slots: Vec<String>,
    weapon_slots: Vec<String>,
    equipped_weapons: Vec<WeaponSystem>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NpcTradeAction {
    Buy,
    Sell,
}

struct NpcTradeQuote {
    buy_price: u32,
    sell_price: u32,
    rumor: &'static str,
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
    vendor: Option<StationVendor>,
    trade: Vec<TradeOffer>,
    research: Vec<ResearchLead>,
    recipe_unlocks: Vec<RecipeUnlockOffer>,
    contracts: Vec<ContractOffer>,
    reputation_required: Option<i32>,
}

#[derive(Clone)]
struct ContractOffer {
    id: String,
    name: String,
    kind: String,
    description: Option<String>,
    origin_station: String,
    origin_service: String,
    target_station: Option<String>,
    target_planet: Option<String>,
    item: Option<ItemRef>,
    amount: u32,
    reward: u32,
    duration_days: f32,
    reputation_faction: Option<String>,
    reputation_required: i32,
    reputation_reward: i32,
}

#[derive(Clone)]
struct ActiveContract {
    id: String,
    origin_station: String,
    origin_service: String,
    expires_day: f32,
    target_reached: bool,
}

#[derive(Clone)]
struct StationVendor {
    id: String,
    name: String,
    faction: Option<String>,
    specialties: Vec<String>,
    rotation_days: f32,
    slots: usize,
    price_variance: f32,
    offers: Vec<VendorOffer>,
    rotation: u64,
    reputation_required: i32,
    price_reputation_scale: f32,
}

#[derive(Clone)]
struct VendorOffer {
    item: ItemRef,
    buy_price: u32,
    sell_price: u32,
    min_stock: u32,
    max_stock: u32,
    weight: f32,
}

#[derive(Clone)]
struct ResearchLead {
    research: String,
    unavailable: bool,
}

#[derive(Clone)]
struct TradeOffer {
    item: ItemRef,
    buy_price: u32,
    sell_price: u32,
    stock: Option<u32>,
    max_stock: Option<u32>,
    restock_days: Option<f32>,
    next_restock_day: Option<f32>,
    catalog_rotation: Option<u64>,
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
    unit_mass: f32,
}

#[derive(Clone)]
struct ItemStack {
    item: ItemRef,
    count: u32,
}

#[derive(Clone)]
struct Inventory {
    slots: [Option<ItemStack>; INVENTORY_SLOTS],
}

#[derive(Clone)]
struct OwnedShip {
    instance_id: String,
    callsign: String,
    ship_id: String,
    system_id: String,
    ship: Ship,
    installed_power_modules: Vec<PowerModule>,
    equipped_shields: Vec<ShieldSystem>,
    equipped_weapons: Vec<WeaponSystem>,
    inventory: Inventory,
    upgrades: [ShipUpgrade; SHIP_UPGRADE_COUNT],
    shield_recharge_delay_remaining: f32,
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
    #[serde(default)]
    owned_ships: Vec<SaveOwnedShip>,
    #[serde(default)]
    active_owned_ship_id: String,
    #[serde(default = "default_next_owned_ship_sequence")]
    next_owned_ship_sequence: u64,
    #[serde(default = "default_starter_ship_id")]
    ship_id: String,
    ship: SaveShip,
    inventory: Vec<SaveStack>,
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
    weapon_slot_loadout: Vec<Option<String>>,
    #[serde(default)]
    market_offers: Vec<SaveMarketOffer>,
    #[serde(default)]
    npc_cargo: Vec<SaveNpcCargo>,
    #[serde(default)]
    system_destinations: Vec<SaveSystemDestination>,
    #[serde(default)]
    navigation_route: Vec<SaveNavigationStop>,
    #[serde(default)]
    navigation_route_current: Option<usize>,
    #[serde(default)]
    navigation_route_active: bool,
    #[serde(default)]
    content_pack_options: Vec<PackOptionSelection>,
    #[serde(default)]
    completed_research: Vec<String>,
    #[serde(default)]
    active_research: Vec<SaveActiveResearch>,
    #[serde(default)]
    active_contracts: Vec<SaveActiveContract>,
    #[serde(default)]
    faction_reputation: Vec<SaveFactionReputation>,
    #[serde(default, skip_serializing)]
    purchased_recipe_unlocks: Vec<String>,
    production_mode: String,
    smelt_settings: Vec<SaveWorkSetting>,
    craft_settings: Vec<SaveWorkSetting>,
    processing_settings: Vec<SaveWorkSetting>,
    planets: Vec<SavePlanet>,
}

#[derive(Clone, Serialize, Deserialize)]
struct SaveOwnedShip {
    instance_id: String,
    callsign: String,
    #[serde(default = "default_starter_ship_id")]
    ship_id: String,
    system_id: String,
    ship: SaveShip,
    inventory: Vec<SaveStack>,
    upgrades: Vec<SaveUpgrade>,
    installed_power_modules: Vec<String>,
    shield_slots: Vec<String>,
    shield_recharge_delay_remaining: f32,
    weapon_slot_loadout: Vec<Option<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct PackOptionSelection {
    pack_id: String,
    option_id: String,
    value: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct SaveShip {
    position: [f32; 2],
    velocity: [f32; 2],
    angle: f32,
    angular_velocity: f32,
    hull: SaveResource,
    shields: SaveResource,
    energy: SaveResource,
}

#[derive(Clone, Serialize, Deserialize)]
struct SaveResource {
    current: f32,
    max: f32,
}

#[derive(Clone, Serialize, Deserialize)]
struct SaveStack {
    item: String,
    count: u32,
}

#[derive(Serialize, Deserialize)]
struct SaveMarketOffer {
    station: String,
    service: String,
    item: String,
    stock: Option<u32>,
    #[serde(default)]
    next_restock_day: Option<f32>,
    #[serde(default)]
    catalog_rotation: Option<u64>,
}

#[derive(Serialize, Deserialize)]
struct SaveNpcCargo {
    npc: String,
    cargo: Vec<SaveStack>,
}

#[derive(Clone, Serialize, Deserialize)]
struct SaveActiveResearch {
    research: String,
    remaining_seconds: f32,
}

#[derive(Clone, Serialize, Deserialize)]
struct SaveActiveContract {
    id: String,
    origin_station: String,
    origin_service: String,
    expires_day: f32,
    #[serde(default)]
    target_reached: bool,
}

#[derive(Clone, Serialize, Deserialize)]
struct SaveFactionReputation {
    faction: String,
    value: i32,
}

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
struct SaveNavigationStop {
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

#[derive(Clone)]
struct ActiveResearch {
    research: String,
    remaining_seconds: f32,
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
    Keep,
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
enum WorkKind {
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
    async fn new(start_mode: GameStartMode, runtime_flags: RuntimeFlags) -> Self {
        reset_startup_progress();
        let mut remote_audio_startup = RemoteAudioStartup::start();
        remote_audio_startup.draw();
        next_frame().await;
        let start_pack_options = match &start_mode {
            GameStartMode::NewGame { pack_options, .. } => pack_options.clone(),
            GameStartMode::LoadGame { .. } => Vec::new(),
        };
        let requested_new_game_ship = match &start_mode {
            GameStartMode::NewGame { ship_id, .. } => Some(ship_id.as_str()),
            GameStartMode::LoadGame { .. } => None,
        };
        let content_registry = load_game_content_registry_with_options(&start_pack_options);
        begin_startup_progress(startup_work_total(
            &content_registry,
            matches!(start_mode, GameStartMode::LoadGame { .. }),
        ));

        egui_art::install_sources(load_egui_art_sources(&content_registry).await);

        advance_startup_progress();
        let remote_progress = remote_audio_startup.current();
        draw_startup_transition_with_remote(
            None,
            "Loading save data ...",
            1.0,
            Some(&remote_progress),
        );
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

        advance_startup_progress();
        let remote_progress = remote_audio_startup.current();
        draw_startup_transition_with_remote(
            None,
            "Preparing production chains ...",
            1.0,
            Some(&remote_progress),
        );
        next_frame().await;
        let smelt_recipes = make_smelting_recipes(&content_registry);
        let craft_recipes = make_crafting_recipes(&content_registry);
        let processing_recipes = make_processing_recipes(&content_registry);
        let inventory = Inventory::starter(&content_registry);
        let smelt_settings = vec![CraftSetting::starter(); smelt_recipes.len()];
        let craft_settings = vec![CraftSetting::starter(); craft_recipes.len()];
        let processing_settings = vec![CraftSetting::starter(); processing_recipes.len()];

        let transition_assets = load_transition_assets(Path::new("assets/transitions")).await;
        let startup_system_id = save_data
            .as_ref()
            .map(|save| save.current_system_id.as_str())
            .unwrap_or(STARTER_SYSTEM_ID);
        let startup_preferred_transition_id =
            preferred_transition_asset_id_for_system(&content_registry, startup_system_id);
        let startup_world_elapsed_days = save_data
            .as_ref()
            .map(|save| finite_nonnegative_or(save.world_elapsed_days, 0.0))
            .unwrap_or(0.0);
        let startup_faction_reputation = faction_reputation_from_save(
            &content_registry,
            save_data
                .as_ref()
                .map(|save| save.faction_reputation.as_slice()),
        );

        let active_ship_id = if let Some(save) = save_data.as_ref() {
            if content_registry.ships.contains_key(&save.ship_id) {
                save.ship_id.clone()
            } else {
                default_starter_ship_id()
            }
        } else {
            resolved_new_game_ship_id(&content_registry, requested_new_game_ship)
        };
        let active_ship_def = content_registry.ships.get(&active_ship_id).cloned();
        let ship_textures = load_player_ship_textures(
            &content_registry,
            &transition_assets,
            startup_preferred_transition_id,
        )
        .await;
        let system_light_haze_texture = Some(make_system_light_haze_texture());
        let weapon_projectile_textures = load_weapon_projectile_textures(
            &content_registry,
            &transition_assets,
            startup_preferred_transition_id,
        )
        .await;
        let system_stars = make_system_stars(&content_registry);
        let planets = make_planets(
            &content_registry,
            world_seed,
            &transition_assets,
            startup_preferred_transition_id,
        )
        .await;
        let stations = make_station_destinations(
            &content_registry,
            &transition_assets,
            startup_preferred_transition_id,
            world_seed,
            startup_world_elapsed_days,
            &startup_faction_reputation,
        )
        .await;
        let npc_ships = make_npc_ships(
            &content_registry,
            &transition_assets,
            startup_preferred_transition_id,
        )
        .await;
        let recipe_vendor_locked_recipes = research_locked_recipes(&content_registry, &stations);
        let installed_power_modules = active_ship_def
            .as_ref()
            .map(|ship| installed_power_modules_from_ids(&content_registry, &ship.power_modules))
            .unwrap_or_default();
        let equipped_shields = active_ship_def
            .as_ref()
            .map(|ship| equipped_shields_from_ids(&content_registry, &ship.shield_slots))
            .unwrap_or_default();
        let equipped_weapons = active_ship_def
            .as_ref()
            .map(|ship| equipped_weapons_from_ids(&content_registry, &ship.weapon_slots))
            .unwrap_or_default();
        let defense_threats = make_defense_threats();

        let (starmap_panel_position, starmap_panel_size) = load_starmap_panel_geometry();
        let mut game = Self {
            runtime_flags,
            audio: audio::AudioManager::empty(read_app_settings().master_volume),
            content_registry,
            content_pack_options,
            transition_assets,
            scene_transition: None,
            current_system_id: STARTER_SYSTEM_ID.to_string(),
            save_path,
            world_seed,
            world_elapsed_days: 0.0,
            credits,
            owned_ships: Vec::new(),
            active_owned_ship_id: "ship-1".to_string(),
            next_owned_ship_sequence: 2,
            ship_id: active_ship_id,
            ship: active_ship_def
                .as_ref()
                .map(Ship::from_content)
                .unwrap_or_else(Ship::starter),
            installed_power_modules,
            equipped_shields,
            equipped_weapons,
            npc_ships,
            collision_cooldowns: HashMap::new(),
            collision_impact_events: Vec::new(),
            defense_threats,
            weapon_fire_events: Vec::new(),
            weapon_area_effect_events: Vec::new(),
            weapon_projectile_textures,
            ship_textures,
            system_light_haze_texture,
            system_stars,
            planets,
            stations,
            recipe_vendor_locked_recipes,
            active_research: None,
            completed_research: Vec::new(),
            selected_planet: None,
            selected_station: None,
            selected_npc_ship: None,
            selected_station_service: None,
            active_contracts: Vec::new(),
            faction_reputation: startup_faction_reputation,
            selected_research: None,
            destination_planet: Some(1),
            orbiting_planet: None,
            system_destinations: HashMap::new(),
            starmap_selected_planet: None,
            navigation_route: Vec::new(),
            navigation_route_current: None,
            navigation_route_active: false,
            pending_warp: None,
            camera_zoom: 1.0,
            starmap_zoom: 1.0,
            starmap_pan: Vec2::ZERO,
            starmap_drag_previous_mouse: None,
            starmap_panel_position,
            starmap_panel_size,
            starmap_move_previous_mouse: None,
            starmap_resize_previous_mouse: None,
            open_turret_bank: None,
            inventory,
            smelt_recipes,
            smelt_settings,
            craft_recipes,
            craft_settings,
            processing_recipes,
            processing_settings,
            production_mode: ProductionMode::Smelting,
            ship_upgrades: make_ship_upgrades(),
            inventory_open: false,
            map_open: false,
            research_open: false,
            upgrades_open: false,
            content_open: false,
            contracts_open: false,
            fleet_open: false,
            selected_owned_ship_index: 0,
            fleet_transfer_quantity: 1,
            content_browser: ContentBrowserState::default(),
            escape_dialog_open: false,
            quit_to_title_requested: false,
            starmap_filter: StarmapFilter::All,
            starmap_resource_filter_index: 0,
            work_scroll: 0.0,
            contract_menu_scroll: 0.0,
            selected_contract_index: None,
            inventory_scroll: 0.0,
            upgrades_scroll: 0.0,
            shield_recharge_delay_remaining: 0.0,
            last_window_size: current_window_size(),
            window_save_delay: None,
            save_delay: Some(AUTOSAVE_SECONDS),
            save_dirty: true,
            save_status_timer: 0.0,
            save_status_manual: false,
            operation_feedback: Vec::new(),
            debug_console: DebugConsole::default(),
            context_action_menu: None,
            follow_target: None,
            approach_target: None,
            npc_trade_quantity: 1,
        };
        if let Some(save_data) = save_data {
            advance_startup_progress();
            let remote_progress = remote_audio_startup.current();
            draw_startup_transition_assets_with_remote(
                &game.transition_assets,
                startup_preferred_transition_id,
                "Restoring saved flight state ...",
                1.0,
                Some(&remote_progress),
            );
            next_frame().await;
            game.apply_save(save_data);
            game.save_dirty = false;
        } else {
            game.owned_ships.push(game.capture_active_owned_ship());
        }
        advance_startup_progress();
        let remote_progress = remote_audio_startup.current();
        draw_startup_transition_assets_with_remote(
            &game.transition_assets,
            startup_preferred_transition_id,
            "Finalizing audio ...",
            1.0,
            Some(&remote_progress),
        );
        next_frame().await;
        let remote_audio_report = remote_audio_startup.finish().await;
        game.audio = audio::AudioManager::load_from_report(
            remote_audio_report.as_ref(),
            config_dir().join("remote-assets"),
            read_app_settings().master_volume,
        )
        .await;
        game.audio
            .load_custom_paths(
                game.content_registry
                    .weapons
                    .values()
                    .filter_map(|weapon| weapon.fire_audio.as_deref()),
            )
            .await;
        advance_startup_progress();
        draw_startup_transition_assets(
            &game.transition_assets,
            startup_preferred_transition_id,
            "Ready",
            1.0,
        );
        next_frame().await;
        game.audio.play(audio::AudioCue::Ready);
        run_startup_transition_out(&game.transition_assets, startup_preferred_transition_id).await;
        game
    }

    fn apply_save(&mut self, save: SaveData) {
        let saved_owned_ships = save.owned_ships.clone();
        let saved_active_owned_ship_id = save.active_owned_ship_id.clone();
        let saved_next_owned_ship_sequence = save.next_owned_ship_sequence;
        self.world_seed = save.world_seed;
        self.world_elapsed_days = finite_nonnegative_or(save.world_elapsed_days, 0.0);
        self.credits = save.credits;
        self.faction_reputation = faction_reputation_from_save(
            &self.content_registry,
            Some(save.faction_reputation.as_slice()),
        );
        self.completed_research = completed_research_from_save(
            &self.content_registry,
            save.completed_research,
            save.purchased_recipe_unlocks,
        );
        self.active_research = active_research_from_save(
            &self.content_registry,
            save.active_research,
            &self.completed_research,
        );
        self.active_contracts = save
            .active_contracts
            .into_iter()
            .filter(|contract| {
                contract.expires_day.is_finite() && contract.expires_day >= self.world_elapsed_days
            })
            .map(|contract| ActiveContract {
                id: contract.id,
                origin_station: contract.origin_station,
                origin_service: contract.origin_service,
                expires_day: contract.expires_day,
                target_reached: contract.target_reached,
            })
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
        self.ship_id = if self.content_registry.ships.contains_key(&save.ship_id) {
            save.ship_id.clone()
        } else {
            default_starter_ship_id()
        };
        self.ship = self
            .content_registry
            .ships
            .get(&self.ship_id)
            .map(Ship::from_content)
            .unwrap_or_else(Ship::starter);
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
            default_installed_power_modules(&self.content_registry, &self.ship_id)
        } else {
            installed_power_modules_from_ids(&self.content_registry, &save.installed_power_modules)
        };
        self.equipped_shields = if save.shield_slots.is_empty() {
            default_equipped_shields(&self.content_registry, &self.ship_id)
        } else {
            equipped_shields_from_ids(&self.content_registry, &save.shield_slots)
        };
        self.shield_recharge_delay_remaining =
            finite_nonnegative_or(save.shield_recharge_delay_remaining, 0.0);
        self.equipped_weapons = if !save.weapon_slot_loadout.is_empty() {
            equipped_weapons_from_slot_ids(&self.content_registry, &save.weapon_slot_loadout)
        } else if save.weapon_slots.is_empty() {
            default_equipped_weapons(&self.content_registry, &self.ship_id)
        } else {
            equipped_weapons_from_ids(&self.content_registry, &save.weapon_slots)
        };

        apply_market_save(
            &mut self.stations,
            &save.market_offers,
            self.world_elapsed_days,
        );
        apply_npc_cargo_save(&mut self.npc_ships, &save.npc_cargo, &self.content_registry);

        self.inventory = Inventory::from_save(&self.content_registry, &save.inventory);
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
        let saved_route_current = save.navigation_route_current;
        let mut valid_stops_before_current = 0;
        self.navigation_route = save
            .navigation_route
            .into_iter()
            .enumerate()
            .filter_map(|(index, stop)| {
                let valid = self
                    .planets
                    .iter()
                    .any(|planet| planet.id == stop.planet && planet.system == stop.system);
                valid.then(|| {
                    if saved_route_current.is_some_and(|current| index < current) {
                        valid_stops_before_current += 1;
                    }
                    NavigationStop {
                        system: stop.system,
                        planet: stop.planet,
                    }
                })
            })
            .collect();
        self.navigation_route_current = saved_route_current
            .map(|_| valid_stops_before_current)
            .filter(|current| *current <= self.navigation_route.len());
        self.navigation_route_active = save.navigation_route_active
            && self
                .navigation_route_current
                .is_some_and(|current| current < self.navigation_route.len());
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
        self.starmap_selected_planet = self.destination_planet;
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

        if saved_owned_ships.is_empty() {
            self.active_owned_ship_id = "ship-1".to_string();
            self.next_owned_ship_sequence = 2;
            self.owned_ships = vec![self.capture_active_owned_ship()];
        } else {
            self.owned_ships = saved_owned_ships
                .into_iter()
                .map(|ship| OwnedShip::from_save(&self.content_registry, ship))
                .collect();
            self.next_owned_ship_sequence = saved_next_owned_ship_sequence.max(
                self.owned_ships
                    .iter()
                    .filter_map(|ship| owned_ship_sequence(&ship.instance_id))
                    .max()
                    .unwrap_or(0)
                    + 1,
            );
            let active_index = self
                .owned_ships
                .iter()
                .position(|ship| {
                    ship.instance_id == saved_active_owned_ship_id
                        && self.content_registry.ships.contains_key(&ship.ship_id)
                })
                .or_else(|| {
                    self.owned_ships
                        .iter()
                        .position(|ship| self.content_registry.ships.contains_key(&ship.ship_id))
                });
            if let Some(active_index) = active_index {
                self.restore_owned_ship_at(active_index);
            } else {
                let sequence = self.next_owned_ship_sequence;
                self.active_owned_ship_id = format!("ship-{sequence}");
                self.next_owned_ship_sequence = sequence.saturating_add(1);
                self.owned_ships.push(self.capture_active_owned_ship());
                self.selected_owned_ship_index = self.owned_ships.len() - 1;
            }
        }
        if self.navigation_route_active && !start_current_navigation_stop(self) {
            self.navigation_route_active = false;
        }
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
            owned_ships: self
                .owned_ships
                .iter()
                .map(|owned| {
                    if owned.instance_id == self.active_owned_ship_id {
                        self.capture_active_owned_ship().to_save()
                    } else {
                        owned.to_save()
                    }
                })
                .collect(),
            active_owned_ship_id: self.active_owned_ship_id.clone(),
            next_owned_ship_sequence: self.next_owned_ship_sequence,
            ship_id: self.ship_id.clone(),
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
            weapon_slots: Vec::new(),
            weapon_slot_loadout: self
                .equipped_weapons
                .iter()
                .map(|weapon| (!weapon.id.is_empty()).then(|| weapon.id.clone()))
                .collect(),
            market_offers: save_market_offers(&self.stations),
            npc_cargo: save_npc_cargo(&self.npc_ships),
            system_destinations: save_system_destinations(self),
            navigation_route: self
                .navigation_route
                .iter()
                .map(|stop| SaveNavigationStop {
                    system: stop.system.clone(),
                    planet: stop.planet.clone(),
                })
                .collect(),
            navigation_route_current: self.navigation_route_current,
            navigation_route_active: self.navigation_route_active,
            content_pack_options: self.content_pack_options.clone(),
            completed_research: self.completed_research.clone(),
            active_research: self
                .active_research
                .iter()
                .map(|active| SaveActiveResearch {
                    research: active.research.clone(),
                    remaining_seconds: finite_nonnegative_or(active.remaining_seconds, 0.0),
                })
                .filter(|active| active.remaining_seconds > 0.0)
                .collect(),
            active_contracts: self
                .active_contracts
                .iter()
                .map(|active| SaveActiveContract {
                    id: active.id.clone(),
                    origin_station: active.origin_station.clone(),
                    origin_service: active.origin_service.clone(),
                    expires_day: finite_nonnegative_or(active.expires_day, 0.0),
                    target_reached: active.target_reached,
                })
                .collect(),
            faction_reputation: self
                .faction_reputation
                .iter()
                .map(|(faction, value)| SaveFactionReputation {
                    faction: faction.clone(),
                    value: *value,
                })
                .collect(),
            purchased_recipe_unlocks: Vec::new(),
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
        if let Some(starter_ship) = self.content_registry.ships.get(&self.ship_id) {
            self.ship.attributes = ShipAttributes::from_content(starter_ship);
            self.ship.systems = ShipSystems::from_content(starter_ship, self.ship.attributes);
        } else {
            self.ship.attributes = ShipAttributes::starter();
            self.ship.systems = ShipSystems::starter(self.ship.attributes);
        }
        if self.equipped_shields.is_empty() {
            self.equipped_shields = default_equipped_shields(&self.content_registry, &self.ship_id);
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
            self.equipped_weapons = default_equipped_weapons(&self.content_registry, &self.ship_id);
        }
    }

    fn capture_active_owned_ship(&self) -> OwnedShip {
        let callsign = self
            .owned_ships
            .iter()
            .find(|ship| ship.instance_id == self.active_owned_ship_id)
            .map(|ship| ship.callsign.clone())
            .or_else(|| {
                self.content_registry
                    .ships
                    .get(&self.ship_id)
                    .map(|ship| ship.name.clone())
            })
            .unwrap_or_else(|| "Frontier vessel".to_string());
        OwnedShip {
            instance_id: self.active_owned_ship_id.clone(),
            callsign,
            ship_id: self.ship_id.clone(),
            system_id: self.current_system_id.clone(),
            ship: self.ship,
            installed_power_modules: self.installed_power_modules.clone(),
            equipped_shields: self.equipped_shields.clone(),
            equipped_weapons: self.equipped_weapons.clone(),
            inventory: self.inventory.clone(),
            upgrades: self.ship_upgrades,
            shield_recharge_delay_remaining: self.shield_recharge_delay_remaining,
        }
    }

    fn sync_active_owned_ship(&mut self) {
        let snapshot = self.capture_active_owned_ship();
        if let Some(existing) = self
            .owned_ships
            .iter_mut()
            .find(|ship| ship.instance_id == self.active_owned_ship_id)
        {
            *existing = snapshot;
        } else {
            self.owned_ships.push(snapshot);
        }
    }

    fn restore_owned_ship_at(&mut self, index: usize) -> bool {
        let Some(owned) = self.owned_ships.get(index).cloned() else {
            return false;
        };
        if !self.content_registry.ships.contains_key(&owned.ship_id) {
            return false;
        }
        self.active_owned_ship_id = owned.instance_id;
        self.ship_id = owned.ship_id;
        self.current_system_id = owned.system_id;
        self.ship = owned.ship;
        self.installed_power_modules = owned.installed_power_modules;
        self.equipped_shields = owned.equipped_shields;
        self.equipped_weapons = owned.equipped_weapons;
        self.inventory = owned.inventory;
        self.ship_upgrades = owned.upgrades;
        self.shield_recharge_delay_remaining = owned.shield_recharge_delay_remaining;
        self.selected_owned_ship_index = index;
        self.destination_planet = destination_planet_for_system(
            &self.planets,
            &self.system_destinations,
            &self.current_system_id,
        );
        true
    }
}

impl OwnedShip {
    fn to_save(&self) -> SaveOwnedShip {
        SaveOwnedShip {
            instance_id: self.instance_id.clone(),
            callsign: self.callsign.clone(),
            ship_id: self.ship_id.clone(),
            system_id: self.system_id.clone(),
            ship: save_ship_state(&self.ship),
            inventory: self.inventory.to_save(),
            upgrades: self
                .upgrades
                .iter()
                .map(|upgrade| SaveUpgrade {
                    kind: upgrade.kind.id().to_string(),
                    level: upgrade.level,
                })
                .collect(),
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
            weapon_slot_loadout: self
                .equipped_weapons
                .iter()
                .map(|weapon| (!weapon.id.is_empty()).then(|| weapon.id.clone()))
                .collect(),
        }
    }

    fn from_save(registry: &content::ContentRegistry, saved: SaveOwnedShip) -> Self {
        let mut ship = registry
            .ships
            .get(&saved.ship_id)
            .map(Ship::from_content)
            .unwrap_or_else(Ship::starter);
        let mut upgrades = make_ship_upgrades();
        apply_upgrade_save(&mut upgrades, &saved.upgrades);
        for upgrade in upgrades {
            for _ in 0..upgrade.level {
                apply_ship_upgrade(&mut ship, upgrade.kind);
            }
        }
        apply_saved_ship_state(&mut ship, &saved.ship);
        Self {
            instance_id: saved.instance_id,
            callsign: saved.callsign,
            ship_id: saved.ship_id.clone(),
            system_id: saved.system_id,
            ship,
            installed_power_modules: if saved.installed_power_modules.is_empty() {
                default_installed_power_modules(registry, &saved.ship_id)
            } else {
                installed_power_modules_from_ids(registry, &saved.installed_power_modules)
            },
            equipped_shields: if saved.shield_slots.is_empty() {
                default_equipped_shields(registry, &saved.ship_id)
            } else {
                equipped_shields_from_ids(registry, &saved.shield_slots)
            },
            equipped_weapons: if saved.weapon_slot_loadout.is_empty() {
                default_equipped_weapons(registry, &saved.ship_id)
            } else {
                equipped_weapons_from_slot_ids(registry, &saved.weapon_slot_loadout)
            },
            inventory: Inventory::from_save(registry, &saved.inventory),
            upgrades,
            shield_recharge_delay_remaining: finite_nonnegative_or(
                saved.shield_recharge_delay_remaining,
                0.0,
            ),
        }
    }
}

fn save_ship_state(ship: &Ship) -> SaveShip {
    SaveShip {
        position: [
            finite_or(ship.position.x, 0.0),
            finite_or(ship.position.y, 0.0),
        ],
        velocity: [
            finite_or(ship.velocity.x, 0.0),
            finite_or(ship.velocity.y, 0.0),
        ],
        angle: finite_or(ship.angle, 0.0),
        angular_velocity: finite_or(ship.angular_velocity, 0.0),
        hull: ship.systems.hull.to_save(),
        shields: ship.systems.shields.to_save(),
        energy: ship.systems.energy.to_save(),
    }
}

fn apply_saved_ship_state(ship: &mut Ship, saved: &SaveShip) {
    ship.position = vec2(
        finite_or(saved.position[0], 0.0),
        finite_or(saved.position[1], 0.0),
    );
    ship.velocity = vec2(
        finite_or(saved.velocity[0], 0.0),
        finite_or(saved.velocity[1], 0.0),
    );
    ship.angle = finite_or(saved.angle, 0.0);
    ship.angular_velocity = finite_or(saved.angular_velocity, 0.0);
    ship.systems.hull = ShipResource::from_save(saved.hull.clone());
    ship.systems.shields = ShipResource::from_save(saved.shields.clone());
    ship.systems.energy = ShipResource::from_save(saved.energy.clone());
}

fn owned_ship_sequence(instance_id: &str) -> Option<u64> {
    instance_id.strip_prefix("ship-")?.parse().ok()
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
    transition_assets: &[TransitionAsset],
    preferred_transition_id: Option<&str>,
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
                advance_startup_progress();
                draw_startup_transition_assets(
                    transition_assets,
                    preferred_transition_id,
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
    transition_assets: &[TransitionAsset],
    preferred_transition_id: Option<&str>,
    world_seed: u64,
    world_elapsed_days: f32,
    faction_reputation: &HashMap<String, i32>,
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
                advance_startup_progress();
                draw_startup_transition_assets(
                    transition_assets,
                    preferred_transition_id,
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
                .map(|service| {
                    let vendor = content_registry.vendors.values().find(|vendor| {
                        vendor.station == station_def.id && vendor.service == service.id
                    });
                    let runtime_vendor = vendor.map(|vendor| {
                        runtime_vendor_from_def(
                            content_registry,
                            vendor,
                            world_seed,
                            world_elapsed_days,
                            faction_reputation,
                        )
                    });
                    let trade = runtime_vendor
                        .as_ref()
                        .map(|(_, trade)| trade.clone())
                        .unwrap_or_else(|| {
                            service
                                .trade
                                .iter()
                                .filter_map(|offer| {
                                    registry_item(content_registry, &offer.item).map(|item| {
                                        TradeOffer {
                                            item,
                                            buy_price: offer.buy_price,
                                            sell_price: offer.sell_price,
                                            stock: offer.stock,
                                            max_stock: offer.stock,
                                            restock_days: offer.restock_days,
                                            next_restock_day: offer
                                                .stock
                                                .zip(offer.restock_days)
                                                .map(|(_, days)| days),
                                            catalog_rotation: None,
                                            unavailable: offer.unavailable,
                                        }
                                    })
                                })
                                .collect()
                        });
                    StationService {
                        id: service.id.clone(),
                        name: service.name.clone(),
                        kind: service.kind.clone(),
                        description: service.description.clone(),
                        vendor: runtime_vendor.map(|(vendor, _)| vendor),
                        trade,
                        research: service
                            .research
                            .iter()
                            .map(|lead| ResearchLead {
                                research: lead.research.clone(),
                                unavailable: lead.unavailable,
                            })
                            .collect(),
                        reputation_required: service.reputation_required,

                        recipe_unlocks: service
                            .recipe_unlocks
                            .iter()
                            .map(|unlock| RecipeUnlockOffer {
                                recipe: unlock.recipe.clone(),
                                price: unlock.price,
                                unavailable: unlock.unavailable,
                            })
                            .collect(),
                        contracts: service
                            .contracts
                            .iter()
                            .map(|contract| ContractOffer {
                                id: contract.id.clone(),
                                name: contract.name.clone(),
                                kind: contract.kind.clone(),
                                description: contract.description.clone(),
                                origin_station: station_def.id.clone(),
                                origin_service: service.id.clone(),
                                target_station: contract.target_station.clone(),
                                target_planet: contract.target_planet.clone(),
                                item: contract
                                    .item
                                    .as_deref()
                                    .and_then(|item| registry_item(content_registry, item)),
                                amount: contract.amount,
                                reward: contract.reward,
                                duration_days: contract.duration_days,
                                reputation_faction: vendor
                                    .and_then(|vendor| vendor.faction.clone())
                                    .or_else(|| station_def.faction.clone()),
                                reputation_required: contract.reputation_required,
                                reputation_reward: contract.reputation_reward,
                            })
                            .collect(),
                    }
                })
                .collect(),
        });
    }
    stations
}

fn runtime_vendor_from_def(
    content_registry: &content::ContentRegistry,
    vendor: &content::VendorDef,
    world_seed: u64,
    world_elapsed_days: f32,
    faction_reputation: &HashMap<String, i32>,
) -> (StationVendor, Vec<TradeOffer>) {
    let offers = vendor
        .offers
        .iter()
        .filter_map(|offer| {
            registry_item(content_registry, &offer.item).map(|item| VendorOffer {
                item,
                buy_price: offer.buy_price,
                sell_price: offer.sell_price,
                min_stock: offer.min_stock,
                max_stock: offer.max_stock,
                weight: offer.weight,
            })
        })
        .collect::<Vec<_>>();
    let rotation = vendor_rotation(vendor.rotation_days, world_elapsed_days);
    let runtime = StationVendor {
        id: vendor.id.clone(),
        name: vendor.name.clone(),
        faction: vendor.faction.clone(),
        specialties: vendor.specialties.clone(),
        rotation_days: vendor.rotation_days,
        slots: vendor.slots,
        price_variance: vendor.price_variance,
        offers,
        rotation,
        reputation_required: vendor.reputation_required,
        price_reputation_scale: vendor.price_reputation_scale,
    };
    let trade = vendor_trade_offers(&runtime, world_seed, faction_reputation);
    (runtime, trade)
}

fn vendor_rotation(rotation_days: f32, world_elapsed_days: f32) -> u64 {
    (world_elapsed_days.max(0.0) / rotation_days.max(0.001)).floor() as u64
}

fn vendor_trade_offers(
    vendor: &StationVendor,
    world_seed: u64,
    faction_reputation: &HashMap<String, i32>,
) -> Vec<TradeOffer> {
    let mut ranked = vendor
        .offers
        .iter()
        .enumerate()
        .map(|(index, offer)| {
            let seed = hash_seeded_id(
                world_seed ^ vendor.rotation.wrapping_mul(0x9e37_79b9_7f4a_7c15),
                &format!("{}:{index}", vendor.id),
            );
            let random = seeded_unit(seed).max(0.0001);
            (random.powf(1.0 / offer.weight.max(0.001)), index)
        })
        .collect::<Vec<_>>();
    ranked.sort_by(|left, right| right.0.total_cmp(&left.0));
    let rotation_end = (vendor.rotation + 1) as f32 * vendor.rotation_days;
    ranked
        .into_iter()
        .take(vendor.slots.min(vendor.offers.len()))
        .map(|(_, index)| {
            let offer = &vendor.offers[index];
            let price_seed = hash_seeded_id(
                world_seed ^ vendor.rotation.wrapping_mul(0x5177_5eed_cafe_babe),
                &format!("{}:price:{index}", vendor.id),
            );
            let variance = (seeded_unit(price_seed) * 2.0 - 1.0) * vendor.price_variance;
            let reputation = vendor
                .faction
                .as_ref()
                .and_then(|faction| faction_reputation.get(faction))
                .copied()
                .unwrap_or_default();
            let reputation_multiplier =
                (1.0 + vendor.price_reputation_scale * reputation as f32 / 100.0).clamp(0.5, 1.5);
            let sell_multiplier = (2.0 - reputation_multiplier).clamp(0.5, 1.5);
            let buy_price = ((varied_price(offer.buy_price, variance) as f32
                * reputation_multiplier)
                .round() as u32)
                .max(1);
            let sell_price = ((varied_price(
                offer.sell_price,
                (seeded_unit(price_seed ^ 0xa5a5_a5a5_a5a5_a5a5) * 2.0 - 1.0)
                    * vendor.price_variance,
            ) as f32
                * sell_multiplier)
                .round() as u32)
                .max(1);
            let stock_seed = hash_seeded_id(price_seed, "stock");
            let stock_range = offer.max_stock.saturating_sub(offer.min_stock) as u64 + 1;
            let stock = offer.min_stock + (stock_seed % stock_range) as u32;
            TradeOffer {
                item: offer.item.clone(),
                buy_price,
                sell_price,
                stock: Some(stock),
                max_stock: Some(offer.max_stock),
                restock_days: Some(vendor.rotation_days),
                next_restock_day: Some(rotation_end),
                catalog_rotation: Some(vendor.rotation),
                unavailable: reputation < vendor.reputation_required,
            }
        })
        .collect()
}

fn varied_price(base: u32, variance: f32) -> u32 {
    ((base as f32 * (1.0 + variance)).round() as u32).max(1)
}

fn faction_reputation_from_save(
    registry: &content::ContentRegistry,
    saved: Option<&[SaveFactionReputation]>,
) -> HashMap<String, i32> {
    registry
        .factions
        .iter()
        .map(|(id, faction)| {
            let saved_value = saved
                .and_then(|entries| entries.iter().find(|entry| entry.faction == *id))
                .map(|entry| entry.value)
                .unwrap_or(faction.reputation_start);
            (
                id.clone(),
                saved_value.clamp(faction.reputation_min, faction.reputation_max),
            )
        })
        .collect()
}

fn faction_reputation(game: &GameState, faction: Option<&str>) -> i32 {
    faction
        .and_then(|faction| game.faction_reputation.get(faction))
        .copied()
        .unwrap_or_default()
}

fn adjust_faction_reputation(game: &mut GameState, faction: Option<&str>, delta: i32) {
    let Some(faction_id) = faction else {
        return;
    };
    let Some(faction_def) = game.content_registry.factions.get(faction_id) else {
        return;
    };
    let current = faction_reputation(game, Some(faction_id));
    let next = current
        .saturating_add(delta)
        .clamp(faction_def.reputation_min, faction_def.reputation_max);
    if next == current {
        return;
    }
    game.faction_reputation.insert(faction_id.to_string(), next);
    game.save_dirty = true;
    push_operation_feedback(
        game,
        "Reputation",
        format!("{} standing {:+}", faction_def.name, next - current),
    );
    refresh_station_economy(game);
}

fn refresh_station_economy(game: &mut GameState) {
    for station in &mut game.stations {
        for service in &mut station.services {
            if let Some(vendor) = service.vendor.clone() {
                let previous = service.trade.clone();
                let mut refreshed =
                    vendor_trade_offers(&vendor, game.world_seed, &game.faction_reputation);
                for offer in &mut refreshed {
                    if let Some(old) = previous.iter().find(|old| {
                        old.item.id == offer.item.id
                            && old.catalog_rotation == offer.catalog_rotation
                    }) {
                        offer.stock = old.stock;
                        offer.next_restock_day = old.next_restock_day;
                    }
                }
                service.trade = refreshed;
            }
        }
    }
}

fn service_reputation_faction<'a>(
    station: &'a StationDestination,
    service: &'a StationService,
) -> Option<&'a str> {
    service
        .vendor
        .as_ref()
        .and_then(|vendor| vendor.faction.as_deref())
        .or(station.faction.as_deref())
}

fn station_service_is_available(
    game: &GameState,
    station: &StationDestination,
    service: &StationService,
) -> bool {
    service.reputation_required.is_none_or(|required| {
        faction_reputation(game, service_reputation_faction(station, service)) >= required
    }) && service.vendor.as_ref().is_none_or(|vendor| {
        faction_reputation(game, vendor.faction.as_deref()) >= vendor.reputation_required
    })
}

fn save_market_offers(stations: &[StationDestination]) -> Vec<SaveMarketOffer> {
    stations
        .iter()
        .flat_map(|station| {
            station.services.iter().flat_map(move |service| {
                service.trade.iter().filter_map(move |offer| {
                    offer.restock_days.map(|_| SaveMarketOffer {
                        station: station.id.clone(),
                        service: service.id.clone(),
                        item: offer.item.id.clone(),
                        stock: offer.stock,
                        next_restock_day: offer
                            .next_restock_day
                            .filter(|day| day.is_finite() && *day >= 0.0),
                        catalog_rotation: offer.catalog_rotation,
                    })
                })
            })
        })
        .collect()
}

fn apply_market_save(
    stations: &mut [StationDestination],
    saved_offers: &[SaveMarketOffer],
    world_elapsed_days: f32,
) {
    for saved in saved_offers {
        let Some(offer) = stations
            .iter_mut()
            .find(|station| station.id == saved.station)
            .and_then(|station| {
                station
                    .services
                    .iter_mut()
                    .find(|service| service.id == saved.service)
            })
            .and_then(|service| {
                service
                    .trade
                    .iter_mut()
                    .find(|offer| offer.item.id == saved.item)
            })
        else {
            continue;
        };

        if offer.catalog_rotation != saved.catalog_rotation {
            continue;
        }
        if let Some(max_stock) = offer.max_stock {
            offer.stock = saved.stock.map(|stock| stock.min(max_stock));
        } else {
            offer.stock = saved.stock;
        }
        offer.next_restock_day = saved
            .next_restock_day
            .filter(|day| day.is_finite() && *day >= world_elapsed_days)
            .or_else(|| {
                offer
                    .restock_days
                    .filter(|days| days.is_finite() && *days > 0.0)
                    .map(|days| world_elapsed_days + days)
            });
    }
}

fn save_npc_cargo(npc_ships: &[NpcShip]) -> Vec<SaveNpcCargo> {
    npc_ships
        .iter()
        .map(|npc| SaveNpcCargo {
            npc: npc.id.clone(),
            cargo: npc
                .cargo_defaults
                .iter()
                .map(|stack| SaveStack {
                    item: stack.item.id.clone(),
                    count: stack.count,
                })
                .collect(),
        })
        .collect()
}

fn apply_npc_cargo_save(
    npc_ships: &mut [NpcShip],
    saved: &[SaveNpcCargo],
    registry: &content::ContentRegistry,
) {
    for npc in npc_ships {
        let Some(saved_npc) = saved.iter().find(|saved_npc| saved_npc.npc == npc.id) else {
            continue;
        };
        let cargo = saved_npc
            .cargo
            .iter()
            .filter_map(|stack| {
                let item = registry_item(registry, &stack.item)?;
                (stack.count > 0).then_some(ItemStack {
                    item,
                    count: stack.count,
                })
            })
            .collect::<Vec<_>>();
        let mass = cargo
            .iter()
            .map(|stack| stack.item.unit_mass * stack.count as f32)
            .sum::<f32>();
        if mass <= npc.cargo_capacity {
            npc.cargo_defaults = cargo;
        }
    }
}

async fn make_npc_ships(
    content_registry: &content::ContentRegistry,
    transition_assets: &[TransitionAsset],
    preferred_transition_id: Option<&str>,
) -> Vec<NpcShip> {
    let mut npc_ships = Vec::new();
    for npc_ship_id in &content_registry.npc_ship_order {
        let Some(npc_ship_def) = content_registry.npc_ships.get(npc_ship_id) else {
            continue;
        };
        let texture = match npc_ship_def.texture.as_deref() {
            Some(path) => {
                advance_startup_progress();
                draw_startup_transition_assets(
                    transition_assets,
                    preferred_transition_id,
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
            mass: npc_ship_def.mass,
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
            trade_open: false,
            credit_reward_min: npc_ship_def.credit_reward_min,
            credit_reward_max: npc_ship_def.credit_reward_max,
            hull: ShipResource::full(npc_ship_def.hull_capacity),
            shields: ShipResource::full(npc_ship_def.shield_capacity),
            equipped_shields: equipped_shields_from_ids(
                content_registry,
                &npc_ship_def.shield_slots,
            ),
            shield_recharge_delay_remaining: 0.0,
            energy: ShipResource::full(npc_ship_def.energy_capacity),
            shield_slots: npc_ship_def.shield_slots.clone(),
            weapon_slots: npc_ship_def.weapon_slots.clone(),
            equipped_weapons: equipped_weapons_from_ids(
                content_registry,
                &npc_ship_def.weapon_slots,
            ),
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

fn research_locked_recipes(
    registry: &content::ContentRegistry,
    stations: &[StationDestination],
) -> Vec<String> {
    let mut recipes = Vec::new();
    for research in registry.research.values() {
        for reward in &research.rewards {
            if reward.kind == "recipe_unlock" {
                if let Some(recipe) = reward.target.as_ref() {
                    if !recipes.contains(recipe) {
                        recipes.push(recipe.clone());
                    }
                }
            }
        }
    }
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

fn completed_research_from_save(
    registry: &content::ContentRegistry,
    completed_research: Vec<String>,
    legacy_purchased_recipe_unlocks: Vec<String>,
) -> Vec<String> {
    let mut migrated = completed_research
        .into_iter()
        .filter(|research| registry.research.contains_key(research))
        .collect::<Vec<_>>();
    for recipe in legacy_purchased_recipe_unlocks {
        if let Some(research_id) = research_id_that_unlocks_recipe(registry, &recipe) {
            migrated.push(research_id.to_string());
        }
    }
    migrated.sort();
    migrated.dedup();
    migrated
}

fn active_research_from_save(
    registry: &content::ContentRegistry,
    active_research: Vec<SaveActiveResearch>,
    completed_research: &[String],
) -> Option<ActiveResearch> {
    active_research
        .into_iter()
        .filter(|research| {
            registry.research.contains_key(&research.research)
                && !completed_research
                    .iter()
                    .any(|completed| completed == &research.research)
        })
        .filter_map(|research| {
            let remaining_seconds = finite_nonnegative_or(research.remaining_seconds, 0.0);
            (remaining_seconds > 0.0).then_some(ActiveResearch {
                research: research.research,
                remaining_seconds,
            })
        })
        .min_by(|left, right| left.research.cmp(&right.research))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ResearchNodeState {
    Completed,
    Researching,
    Affordable,
    Available,
    Locked,
}

fn research_node_state(
    research: &content::ResearchDef,
    active_research: Option<&ActiveResearch>,
    completed_research: &[String],
    credits: u32,
) -> ResearchNodeState {
    if completed_research.iter().any(|done| done == &research.id) {
        return ResearchNodeState::Completed;
    }
    if let Some(active) = active_research {
        if active.research == research.id {
            return ResearchNodeState::Researching;
        }
        return ResearchNodeState::Locked;
    }
    let prerequisites_met = research
        .requires
        .iter()
        .chain(research.revealed_by.iter())
        .all(|required| completed_research.iter().any(|done| done == required));
    if !prerequisites_met {
        return ResearchNodeState::Locked;
    }
    if credits >= research.price {
        ResearchNodeState::Affordable
    } else {
        ResearchNodeState::Available
    }
}

fn start_research(game: &mut GameState, research_id: &str) -> bool {
    let Some(research) = game.content_registry.research.get(research_id) else {
        return false;
    };
    if research_node_state(
        research,
        game.active_research.as_ref(),
        &game.completed_research,
        game.credits,
    ) != ResearchNodeState::Affordable
    {
        return true;
    }
    let research_name = research.name.clone();
    let price = research.price;
    let duration_seconds = research.duration_seconds;
    game.credits = game.credits.saturating_sub(price);
    game.active_research = Some(ActiveResearch {
        research: research_id.to_string(),
        remaining_seconds: duration_seconds,
    });
    game.save_dirty = true;
    push_operation_feedback(
        game,
        "Research",
        format!(
            "Started {research_name} for {price} cr ({})",
            format_seconds(duration_seconds)
        ),
    );
    true
}

fn update_active_research(game: &mut GameState, dt: f32) {
    let Some(active) = game.active_research.as_mut() else {
        return;
    };

    active.remaining_seconds = (active.remaining_seconds - dt).max(0.0);
    if active.remaining_seconds > 0.0 {
        return;
    }

    let research_id = active.research.clone();
    game.active_research = None;
    if game
        .completed_research
        .iter()
        .any(|completed| completed == &research_id)
    {
        return;
    }
    let research_name = research_display_name(&game.content_registry, &research_id);
    game.completed_research.push(research_id);
    game.completed_research.sort();
    game.completed_research.dedup();
    game.save_dirty = true;
    push_operation_feedback(game, "Research", format!("Completed {research_name}"));
}

fn recipe_is_unlocked(game: &GameState, recipe_id: &str) -> bool {
    !game
        .recipe_vendor_locked_recipes
        .iter()
        .any(|locked| locked == recipe_id)
        || completed_research_unlocks_recipe(
            &game.content_registry,
            &game.completed_research,
            recipe_id,
        )
}

fn completed_research_unlocks_recipe(
    registry: &content::ContentRegistry,
    completed_research: &[String],
    recipe_id: &str,
) -> bool {
    completed_research.iter().any(|research_id| {
        registry
            .research
            .get(research_id)
            .is_some_and(|research| research_unlocks_recipe(research, recipe_id))
    })
}

fn research_id_that_unlocks_recipe<'a>(
    registry: &'a content::ContentRegistry,
    recipe_id: &str,
) -> Option<&'a str> {
    registry
        .research_order
        .iter()
        .filter_map(|research_id| registry.research.get(research_id))
        .find(|research| research_unlocks_recipe(research, recipe_id))
        .map(|research| research.id.as_str())
}

fn research_unlocks_recipe(research: &content::ResearchDef, recipe_id: &str) -> bool {
    research
        .rewards
        .iter()
        .any(|reward| reward.kind == "recipe_unlock" && reward.target.as_deref() == Some(recipe_id))
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

async fn load_egui_art_sources(
    registry: &content::ContentRegistry,
) -> HashMap<String, egui::ColorImage> {
    let paths = egui_art_source_paths(registry);
    let mut images = HashMap::new();
    for path in paths {
        advance_startup_progress();
        draw_startup_transition(
            None,
            &format!("Preparing interface artwork ... {}", asset_file_name(&path)),
            1.0,
        );
        next_frame().await;
        match load_image(&path).await {
            Ok(image) => {
                images.insert(path, thumbnail_color_image(&image, 320));
            }
            Err(error) => eprintln!("Unable to load egui artwork `{path}`: {error}"),
        }
    }
    images
}

fn egui_art_source_paths(registry: &content::ContentRegistry) -> Vec<String> {
    let mut paths = BTreeSet::new();
    paths.extend(
        registry
            .ships
            .values()
            .filter_map(|value| value.texture.clone()),
    );
    paths.extend(
        registry
            .npc_ships
            .values()
            .filter_map(|value| value.texture.clone()),
    );
    paths.extend(
        registry
            .planets
            .values()
            .filter_map(|value| value.texture.clone()),
    );
    paths.extend(
        registry
            .stations
            .values()
            .filter_map(|value| value.texture.clone()),
    );
    paths.extend(
        registry
            .weapons
            .values()
            .filter_map(|value| value.projectile_texture.clone()),
    );
    paths.into_iter().collect()
}

fn startup_work_total(registry: &content::ContentRegistry, restoring_save: bool) -> usize {
    let transition_assets = transition_asset_paths(Path::new("assets/transitions")).len();
    let ship_assets = registry
        .ships
        .values()
        .filter(|ship| ship.texture.is_some())
        .count();
    let weapon_assets = registry
        .weapons
        .values()
        .filter_map(|weapon| weapon.projectile_texture.as_deref())
        .collect::<BTreeSet<_>>()
        .len();
    let planet_assets = registry
        .planets
        .values()
        .filter(|planet| {
            planet.texture.is_some()
                && planet
                    .mineables
                    .iter()
                    .any(|item| registry.items.contains_key(item))
        })
        .count();
    let station_assets = registry
        .stations
        .values()
        .filter(|station| {
            station.texture.is_some() && station.system.is_some() && station.position.is_some()
        })
        .count();
    let npc_assets = registry
        .npc_ships
        .values()
        .filter(|ship| ship.texture.is_some())
        .count();

    4 + egui_art_source_paths(registry).len()
        + transition_assets
        + ship_assets
        + weapon_assets
        + planet_assets
        + station_assets
        + npc_assets
        + usize::from(restoring_save)
}

fn thumbnail_color_image(image: &Image, max_edge: usize) -> egui::ColorImage {
    let source = image_alpha_bounds(image);
    let source_x = source.x.max(0.0) as usize;
    let source_y = source.y.max(0.0) as usize;
    let source_width = source.w.max(1.0) as usize;
    let source_height = source.h.max(1.0) as usize;
    let scale = (max_edge.max(1) as f32 / source_width.max(source_height) as f32).min(1.0);
    let width = (source_width as f32 * scale).round().max(1.0) as usize;
    let height = (source_height as f32 * scale).round().max(1.0) as usize;
    let image_width = usize::from(image.width);
    let pixels = image.get_image_data();
    let mut resized = Vec::with_capacity(width * height * 4);

    for y in 0..height {
        let sample_y = source_y + y * source_height / height;
        for x in 0..width {
            let sample_x = source_x + x * source_width / width;
            let pixel = pixels[sample_y * image_width + sample_x];
            resized.extend_from_slice(&pixel);
        }
    }

    egui::ColorImage::from_rgba_unmultiplied([width, height], &resized)
}

async fn load_player_ship_textures(
    content_registry: &content::ContentRegistry,
    transition_assets: &[TransitionAsset],
    preferred_transition_id: Option<&str>,
) -> HashMap<String, Texture2D> {
    let mut textures = HashMap::new();
    for ship_id in &content_registry.ship_order {
        let Some(ship) = content_registry.ships.get(ship_id) else {
            continue;
        };
        let Some(path) = ship.texture.as_deref() else {
            continue;
        };
        advance_startup_progress();
        draw_startup_transition_assets(
            transition_assets,
            preferred_transition_id,
            &format!("Loading ship asset ... {}", ship.name),
            1.0,
        );
        next_frame().await;
        if let Some(texture) = load_asset_texture(path).await {
            textures.insert(ship_id.clone(), texture);
        }
    }
    textures
}

async fn load_weapon_projectile_textures(
    registry: &content::ContentRegistry,
    transition_assets: &[TransitionAsset],
    preferred_transition_id: Option<&str>,
) -> HashMap<String, Texture2D> {
    let mut paths = registry
        .weapons
        .values()
        .filter_map(|weapon| weapon.projectile_texture.clone())
        .collect::<Vec<_>>();
    paths.sort();
    paths.dedup();

    let mut textures = HashMap::new();
    for path in paths {
        advance_startup_progress();
        draw_startup_transition_assets(
            transition_assets,
            preferred_transition_id,
            &format!("Loading weapon asset ... {}", asset_file_name(&path)),
            1.0,
        );
        next_frame().await;
        if let Some(texture) = load_asset_texture(&path).await {
            textures.insert(path, texture);
        } else {
            eprintln!("Unable to load weapon projectile texture `{path}`");
        }
    }
    textures
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

fn set_branding_logo(texture: Option<Texture2D>) {
    BRANDING_LOGO.with(|logo| {
        *logo.borrow_mut() = texture;
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
    let paths = transition_asset_paths(root);
    if paths.is_empty() && !root.is_dir() {
        eprintln!("Transition asset directory unavailable: {}", root.display());
    }

    let mut assets = Vec::new();
    for path in paths {
        let path_string = path.to_string_lossy().to_string();
        let startup_background = assets.first().map(|asset: &TransitionAsset| &asset.texture);
        advance_startup_progress();
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
            id: transition_asset_id_from_path(&path_string),
            path: path_string,
            texture,
        });
    }

    assets
}

fn transition_asset_paths(root: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };
    let mut paths = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && is_supported_transition_image(path))
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn transition_asset_id_from_path(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .and_then(|file_stem| file_stem.to_str())
        .unwrap_or(path)
        .to_string()
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

fn select_transition_texture_by_id(assets: &[TransitionAsset], id: &str) -> Option<Texture2D> {
    assets
        .iter()
        .find(|asset| asset.id == id)
        .map(|asset| asset.texture.clone())
}

fn select_transition_texture_for_action(
    assets: &[TransitionAsset],
    stations: &[StationDestination],
    action: &TransitionAction,
) -> Option<Texture2D> {
    preferred_transition_asset_id_for_action(stations, action)
        .and_then(|id| select_transition_texture_by_id(assets, id))
        .or_else(|| select_transition_texture(assets))
}

fn preferred_transition_asset_id_for_action(
    stations: &[StationDestination],
    action: &TransitionAction,
) -> Option<&'static str> {
    match action {
        TransitionAction::SwitchSystem(system_id)
            if system_has_station_destination(stations, system_id) =>
        {
            Some(STATION_APPROACH_TRANSITION_ID)
        }
        TransitionAction::SwitchSystem(_) => None,
    }
}

fn preferred_transition_asset_id_for_system(
    registry: &content::ContentRegistry,
    system_id: &str,
) -> Option<&'static str> {
    if registry
        .stations
        .values()
        .any(|station| station.system.as_deref() == Some(system_id) && station.position.is_some())
    {
        Some(STATION_APPROACH_TRANSITION_ID)
    } else {
        None
    }
}

fn system_has_station_destination(stations: &[StationDestination], system_id: &str) -> bool {
    stations.iter().any(|station| station.system == system_id)
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
            generation: module.generation,
            heat: module.heat,
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
            resolved: true,
            name: weapon.name.clone(),
            kind: weapon.kind,
            install_item: weapon.install_item.clone(),
            range: weapon.range,
            cooldown_seconds: weapon.cooldown_seconds,
            damage: weapon.damage,
            energy_cost: weapon.energy_cost,
            ammo_item: weapon.ammo_item.clone(),
            ammo_per_shot: weapon.ammo_per_shot,
            tracking_degrees: weapon.tracking_degrees,
            targeting: weapon.targeting,
            effect: weapon.effect,
            beam_color: weapon.beam_color,
            core_color: weapon.core_color,
            impact_color: weapon.impact_color,
            fire_duration_seconds: weapon.fire_duration_seconds,
            path_curve_strength: weapon.path_curve_strength,
            path_wobble: weapon.path_wobble,
            path_cycles: weapon.path_cycles,
            trail_length: weapon.trail_length,
            burst_count: weapon.burst_count,
            travel_speed: weapon.travel_speed,
            projectile_texture: weapon.projectile_texture.clone(),
            projectile_size: weapon.projectile_size,
            impact: weapon.impact,
            splash_radius: weapon.splash_radius,
            splash_falloff: weapon.splash_falloff,
            splash_min_multiplier: weapon.splash_min_multiplier,
            chain_targets: weapon.chain_targets,
            chain_range: weapon.chain_range,
            chain_damage_multiplier: weapon.chain_damage_multiplier,
            friendly_fire: weapon.friendly_fire,
            fire_audio: weapon.fire_audio.clone(),
            cooldown_remaining: 0.0,
            status: WeaponStatus::Ready,
        }
    }

    fn empty() -> Self {
        Self::unresolved(String::new())
    }

    fn unresolved(id: String) -> Self {
        Self {
            id,
            resolved: false,
            name: "Unavailable weapon".to_string(),
            kind: content::WeaponKind::TurretDefense,
            install_item: String::new(),
            range: 0.0,
            cooldown_seconds: 0.0,
            damage: 0.0,
            energy_cost: 0.0,
            ammo_item: None,
            ammo_per_shot: 1,
            tracking_degrees: 0.0,
            targeting: content::WeaponTargeting::AllHostiles,
            effect: content::WeaponEffect::Arc,
            beam_color: [61, 178, 255, 255],
            core_color: [184, 245, 255, 255],
            impact_color: [143, 235, 255, 255],
            fire_duration_seconds: WEAPON_FIRE_EVENT_SECONDS,
            path_curve_strength: 0.18,
            path_wobble: 8.0,
            path_cycles: 3.0,
            trail_length: 0.4,
            burst_count: 3,
            travel_speed: None,
            projectile_texture: None,
            projectile_size: 28.0,
            impact: content::WeaponImpact::Single,
            splash_radius: 0.0,
            splash_falloff: content::DamageFalloff::Linear,
            splash_min_multiplier: 0.2,
            chain_targets: 3,
            chain_range: 240.0,
            chain_damage_multiplier: 0.75,
            friendly_fire: content::FriendlyFire::HostilesOnly,
            fire_audio: None,
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
            WeaponStatus::OutOfAmmo => "out of ammo",
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

fn npc_weapon_ammo_count(npc_ship: &NpcShip, weapon: &WeaponSystem) -> Option<u32> {
    weapon.ammo_item.as_deref().map(|ammo_item| {
        npc_ship
            .cargo_defaults
            .iter()
            .filter(|stack| stack.item.id == ammo_item)
            .map(|stack| stack.count)
            .sum()
    })
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
    ship_id: &str,
) -> Vec<PowerModule> {
    content_registry
        .ships
        .get(ship_id)
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

fn default_equipped_shields(
    content_registry: &content::ContentRegistry,
    ship_id: &str,
) -> Vec<ShieldSystem> {
    content_registry
        .ships
        .get(ship_id)
        .map(|ship| equipped_shields_from_ids(content_registry, &ship.shield_slots))
        .unwrap_or_default()
}

fn equipped_weapons_from_ids(
    content_registry: &content::ContentRegistry,
    weapon_ids: &[String],
) -> Vec<WeaponSystem> {
    weapon_ids
        .iter()
        .map(|weapon_id| {
            content_registry.weapons.get(weapon_id).map_or_else(
                || WeaponSystem::unresolved(weapon_id.clone()),
                WeaponSystem::from_def,
            )
        })
        .collect()
}

fn equipped_weapons_from_slot_ids(
    content_registry: &content::ContentRegistry,
    weapon_ids: &[Option<String>],
) -> Vec<WeaponSystem> {
    weapon_ids
        .iter()
        .map(|weapon_id| match weapon_id {
            Some(weapon_id) => content_registry.weapons.get(weapon_id).map_or_else(
                || WeaponSystem::unresolved(weapon_id.clone()),
                WeaponSystem::from_def,
            ),
            None => WeaponSystem::empty(),
        })
        .collect()
}

fn default_equipped_weapons(
    content_registry: &content::ContentRegistry,
    ship_id: &str,
) -> Vec<WeaponSystem> {
    content_registry
        .ships
        .get(ship_id)
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
        .get(&game.ship_id)
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
    push_operation_feedback(
        game,
        "Install",
        format!("Shield installed: {}", shield_def.name),
    );
    Ok(())
}

fn weapon_slot_capacity(game: &GameState) -> usize {
    game.content_registry
        .ships
        .get(&game.ship_id)
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
        .is_some_and(|weapon| weapon.resolved && weapon.id == weapon_id)
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
    if let Some(previous_weapon) = game
        .equipped_weapons
        .get(slot_index)
        .filter(|weapon| weapon.resolved)
    {
        let previous_item = required_item(&game.content_registry, &previous_weapon.install_item);
        game.inventory.add_item(previous_item, 1);
    }

    let installed_weapon = WeaponSystem::from_def(&weapon_def);
    if slot_index < game.equipped_weapons.len() {
        game.equipped_weapons[slot_index] = installed_weapon;
    } else {
        game.equipped_weapons
            .resize_with(slot_index, WeaponSystem::empty);
        game.equipped_weapons.push(installed_weapon);
    }
    game.save_dirty = true;
    push_operation_feedback(
        game,
        "Install",
        format!("Weapon installed: {}", weapon_def.name),
    );
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

fn active_shield_damage_resistance(game: &GameState) -> f32 {
    active_shield(game)
        .map(|shield| shield.damage_resistance)
        .unwrap_or(0.0)
}

fn owned_ship_shield_recharge_delay(owned: &OwnedShip) -> f32 {
    owned
        .equipped_shields
        .first()
        .map(|shield| shield.recharge_delay)
        .unwrap_or(4.0)
        .max(0.0)
}

fn owned_ship_shield_recharge_rate(owned: &OwnedShip) -> f32 {
    owned
        .equipped_shields
        .first()
        .map(|shield| shield.recharge_rate)
        .unwrap_or(0.0)
        .max(0.0)
}

fn owned_ship_shield_damage_resistance(owned: &OwnedShip) -> f32 {
    owned
        .equipped_shields
        .first()
        .map(|shield| shield.damage_resistance)
        .unwrap_or(0.0)
        .clamp(0.0, 1.0)
}

fn apply_owned_ship_weapon_damage(owned: &mut OwnedShip, amount: f32) -> bool {
    let damage = amount.max(0.0) * (1.0 - owned_ship_shield_damage_resistance(owned));
    let application = apply_damage_to_resources(
        &mut owned.ship.systems.shields,
        &mut owned.ship.systems.hull,
        damage,
        1.0,
    );
    if application.changed() {
        owned.shield_recharge_delay_remaining = owned_ship_shield_recharge_delay(owned);
    }
    application.changed()
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

fn ship_pressure_damage_after_resistance(game: &GameState, damage: f32) -> f32 {
    damage.max(0.0) * (1.0 - active_shield_damage_resistance(game)).clamp(0.0, 1.0)
}

fn apply_ship_pressure_damage(game: &mut GameState, amount: f32) -> bool {
    let damage = ship_pressure_damage_after_resistance(game, amount);
    let application = apply_damage_to_resources(
        &mut game.ship.systems.shields,
        &mut game.ship.systems.hull,
        damage,
        REDWAKE_PRESSURE_HULL_SPILLOVER,
    );
    let changed = application.changed();
    if changed {
        game.shield_recharge_delay_remaining = active_shield_recharge_delay(game);
        game.save_dirty = true;
    }
    changed
}

fn apply_ship_weapon_damage(game: &mut GameState, amount: f32) -> bool {
    let damage = ship_pressure_damage_after_resistance(game, amount);
    let application = apply_damage_to_resources(
        &mut game.ship.systems.shields,
        &mut game.ship.systems.hull,
        damage,
        1.0,
    );
    let changed = application.changed();
    if changed {
        game.shield_recharge_delay_remaining = active_shield_recharge_delay(game);
        game.save_dirty = true;
    }
    changed
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DamageApplication {
    shield_absorbed: f32,
    hull_damage: f32,
}

impl DamageApplication {
    fn changed(self) -> bool {
        self.shield_absorbed > 0.0 || self.hull_damage > 0.0
    }
}

fn apply_damage_to_resources(
    shields: &mut ShipResource,
    hull: &mut ShipResource,
    amount: f32,
    hull_spillover: f32,
) -> DamageApplication {
    let damage = if amount.is_finite() {
        amount.max(0.0)
    } else {
        0.0
    };
    let shield_absorbed = damage.min(shields.current.max(0.0));
    shields.spend(shield_absorbed);
    let hull_damage = ((damage - shield_absorbed) * hull_spillover.max(0.0)).max(0.0);
    hull.spend(hull_damage);
    DamageApplication {
        shield_absorbed,
        hull_damage,
    }
}

fn active_npc_shield(npc_ship: &NpcShip) -> Option<&ShieldSystem> {
    npc_ship.equipped_shields.first()
}

fn npc_shield_damage_resistance(npc_ship: &NpcShip) -> f32 {
    active_npc_shield(npc_ship)
        .map(|shield| shield.damage_resistance)
        .unwrap_or(0.0)
        .clamp(0.0, 1.0)
}

fn npc_shield_recharge_delay(npc_ship: &NpcShip) -> f32 {
    active_npc_shield(npc_ship)
        .map(|shield| shield.recharge_delay)
        .unwrap_or(0.0)
        .max(0.0)
}

fn npc_shield_recharge_rate(npc_ship: &NpcShip) -> f32 {
    active_npc_shield(npc_ship)
        .map(|shield| shield.recharge_rate)
        .unwrap_or(0.0)
        .max(0.0)
}

fn apply_npc_collision_damage(npc_ship: &mut NpcShip, amount: f32) -> DamageApplication {
    let resisted_damage = amount.max(0.0) * (1.0 - npc_shield_damage_resistance(npc_ship));
    let application = apply_damage_to_resources(
        &mut npc_ship.shields,
        &mut npc_ship.hull,
        resisted_damage,
        1.0,
    );
    if application.changed() {
        npc_ship.shield_recharge_delay_remaining = npc_shield_recharge_delay(npc_ship);
    }
    application
}

fn update_npc_shield_recharge(npc_ship: &mut NpcShip, dt: f32) {
    if npc_ship.shields.current >= npc_ship.shields.max {
        npc_ship.shield_recharge_delay_remaining = 0.0;
        return;
    }
    if npc_ship.shield_recharge_delay_remaining > 0.0 {
        npc_ship.shield_recharge_delay_remaining =
            (npc_ship.shield_recharge_delay_remaining - dt).max(0.0);
        return;
    }
    npc_ship
        .shields
        .restore(npc_shield_recharge_rate(npc_ship) * dt);
}

fn collision_pair_key(left: &str, right: &str) -> String {
    if left <= right {
        format!("{left}|{right}")
    } else {
        format!("{right}|{left}")
    }
}

fn collision_closing_speed(
    left_position: Vec2,
    left_velocity: Vec2,
    left_radius: f32,
    right_position: Vec2,
    right_velocity: Vec2,
    right_radius: f32,
) -> Option<f32> {
    let separation = right_position - left_position;
    let contact_distance = left_radius.max(0.0) + right_radius.max(0.0);
    if !separation.is_finite()
        || !left_velocity.is_finite()
        || !right_velocity.is_finite()
        || separation.length() > contact_distance
    {
        return None;
    }
    let normal = safe_direction(
        separation,
        safe_direction(left_velocity - right_velocity, vec2(1.0, 0.0)),
    );
    let closing_speed = (left_velocity - right_velocity).dot(normal);
    (closing_speed.is_finite() && closing_speed >= COLLISION_MIN_CLOSING_SPEED)
        .then_some(closing_speed)
}

fn collision_impact_damage(closing_speed: f32, left_mass: f32, right_mass: f32) -> f32 {
    if !closing_speed.is_finite() || !left_mass.is_finite() || !right_mass.is_finite() {
        return 0.0;
    }
    let average_mass =
        ((left_mass.max(0.0) + right_mass.max(0.0)) * 0.5) / COLLISION_REFERENCE_MASS;
    (closing_speed.max(0.0).powi(2) * average_mass * COLLISION_DAMAGE_COEFFICIENT).max(0.0)
}

fn collision_impact_severity(
    application: DamageApplication,
    destroyed: bool,
) -> CollisionImpactSeverity {
    if destroyed {
        CollisionImpactSeverity::Destructive
    } else if application.hull_damage > 0.0 {
        CollisionImpactSeverity::Critical
    } else if application.shield_absorbed >= 20.0 {
        CollisionImpactSeverity::Significant
    } else {
        CollisionImpactSeverity::Minor
    }
}

fn collision_damage_summary(application: DamageApplication) -> String {
    let mut parts = Vec::new();
    if application.shield_absorbed > 0.0 {
        parts.push(format!("shield -{:.0}", application.shield_absorbed));
    }
    if application.hull_damage > 0.0 {
        parts.push(format!("hull -{:.0}", application.hull_damage));
    }
    if parts.is_empty() {
        "no damage".to_string()
    } else {
        parts.join(", ")
    }
}

fn record_collision_impact(
    game: &mut GameState,
    position: Vec2,
    subject: &str,
    application: DamageApplication,
    destroyed: bool,
) {
    if !application.changed() {
        return;
    }
    let severity = collision_impact_severity(application, destroyed);
    let suffix = if destroyed { " / destroyed" } else { "" };
    push_operation_feedback(
        game,
        "Collision",
        format!(
            "{subject}: {}{}",
            collision_damage_summary(application),
            suffix
        ),
    );
    if application.shield_absorbed > 0.0 {
        game.audio.play(audio::AudioCue::ShieldImpact);
    }
    if application.hull_damage > 0.0 {
        game.audio.play(audio::AudioCue::HullImpact);
    }
    game.collision_impact_events.push(CollisionImpactEvent {
        position,
        timer: COLLISION_EVENT_SECONDS,
        shield_damage: application.shield_absorbed,
        hull_damage: application.hull_damage,
        severity,
    });
}

fn update_collision_impact_events(game: &mut GameState, dt: f32) {
    game.collision_impact_events.retain_mut(|event| {
        event.timer -= dt;
        event.timer > 0.0
    });
}

fn update_ship_collisions(game: &mut GameState, dt: f32) {
    if dt <= 0.0 || !dt.is_finite() {
        return;
    }
    update_collision_impact_events(game, dt);
    game.collision_cooldowns.retain(|_, remaining| {
        *remaining = (*remaining - dt).max(0.0);
        *remaining > 0.0
    });

    let player_id = "__player__";
    let player_position = game.ship.position;
    let player_velocity = game.ship.velocity;
    let player_mass = game.ship.attributes.mass;
    let current_system_id = game.current_system_id.clone();

    for npc_index in 0..game.npc_ships.len() {
        let Some(npc) = game.npc_ships.get(npc_index) else {
            continue;
        };
        if npc.system != current_system_id || npc.hull.current <= 0.0 {
            continue;
        }
        let key = collision_pair_key(player_id, &npc.id);
        let Some(closing_speed) = collision_closing_speed(
            player_position,
            player_velocity,
            SHIP_RADIUS,
            npc.position,
            npc.velocity,
            npc.radius,
        ) else {
            continue;
        };
        if game.collision_cooldowns.contains_key(&key) {
            continue;
        }
        let damage = collision_impact_damage(closing_speed, player_mass, npc.mass);
        if damage <= 0.0 {
            continue;
        }
        let npc_name = npc.name.clone();
        let impact_position = (player_position + npc.position) * 0.5;
        let player_resistance = active_shield_damage_resistance(game);
        let player_application = apply_damage_to_resources(
            &mut game.ship.systems.shields,
            &mut game.ship.systems.hull,
            damage * (1.0 - player_resistance),
            1.0,
        );
        if player_application.changed() {
            game.shield_recharge_delay_remaining = active_shield_recharge_delay(game);
        }
        let npc_application = apply_npc_collision_damage(&mut game.npc_ships[npc_index], damage);
        if player_application.changed() || npc_application.changed() {
            game.save_dirty = true;
        }
        let npc_destroyed = game.npc_ships[npc_index].hull.current <= 0.0;
        record_collision_impact(
            game,
            impact_position,
            &format!("Impact with {npc_name}"),
            DamageApplication {
                shield_absorbed: player_application.shield_absorbed
                    + npc_application.shield_absorbed,
                hull_damage: player_application.hull_damage + npc_application.hull_damage,
            },
            npc_destroyed,
        );
        game.collision_cooldowns
            .insert(key, COLLISION_COOLDOWN_SECONDS);
    }

    for left_index in 0..game.npc_ships.len() {
        for right_index in (left_index + 1)..game.npc_ships.len() {
            let (left, right) = (&game.npc_ships[left_index], &game.npc_ships[right_index]);
            if left.system != current_system_id
                || right.system != current_system_id
                || left.hull.current <= 0.0
                || right.hull.current <= 0.0
            {
                continue;
            }
            let key = collision_pair_key(&left.id, &right.id);
            let Some(closing_speed) = collision_closing_speed(
                left.position,
                left.velocity,
                left.radius,
                right.position,
                right.velocity,
                right.radius,
            ) else {
                continue;
            };
            if game.collision_cooldowns.contains_key(&key) {
                continue;
            }
            let damage = collision_impact_damage(closing_speed, left.mass, right.mass);
            if damage <= 0.0 {
                continue;
            }
            let left_name = left.name.clone();
            let right_name = right.name.clone();
            let impact_position = (left.position + right.position) * 0.5;
            let (left_application, right_application, destroyed) = {
                let (left, right) = game.npc_ships.split_at_mut(right_index);
                let left_application = apply_npc_collision_damage(&mut left[left_index], damage);
                let right_application = apply_npc_collision_damage(&mut right[0], damage);
                let destroyed =
                    left[left_index].hull.current <= 0.0 || right[0].hull.current <= 0.0;
                (left_application, right_application, destroyed)
            };
            if left_application.changed() || right_application.changed() {
                game.save_dirty = true;
            }
            record_collision_impact(
                game,
                impact_position,
                &format!("NPC collision: {left_name} / {right_name}"),
                DamageApplication {
                    shield_absorbed: left_application.shield_absorbed
                        + right_application.shield_absorbed,
                    hull_damage: left_application.hull_damage + right_application.hull_damage,
                },
                destroyed,
            );
            game.collision_cooldowns
                .insert(key, COLLISION_COOLDOWN_SECONDS);
        }
    }
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

fn bonus_output_count(
    registry: &content::ContentRegistry,
    completed_research: &[String],
    count: u32,
) -> u32 {
    let chance =
        completed_research_reward_amount(registry, completed_research, "bonus_output_chance")
            / 100.0;
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

fn push_operation_feedback(
    game: &mut GameState,
    category: impl Into<String>,
    message: impl Into<String>,
) {
    let message = message.into();
    if ["failed", "cannot", "unable", "warning", "need "]
        .iter()
        .any(|marker| message.to_ascii_lowercase().contains(marker))
    {
        game.audio.play(audio::AudioCue::Warning);
    }
    let entry = OperationFeedback {
        category: category.into(),
        message,
        aggregate_key: None,
        count: 1,
    };
    game.operation_feedback.retain(|existing| {
        existing.category != entry.category || existing.message != entry.message
    });
    game.operation_feedback.insert(0, entry);
    game.operation_feedback.truncate(OPERATION_FEEDBACK_LIMIT);
}

fn push_aggregate_operation_feedback(
    game: &mut GameState,
    category: impl Into<String>,
    aggregate_key: impl Into<String>,
    count: u32,
    format_message: impl Fn(u32) -> String,
) {
    let category = category.into();
    let aggregate_key = aggregate_key.into();
    let mut total = count;
    game.operation_feedback.retain(|existing| {
        if existing.category == category
            && existing
                .aggregate_key
                .as_ref()
                .is_some_and(|key| key == &aggregate_key)
        {
            total = total.saturating_add(existing.count);
            false
        } else {
            true
        }
    });
    game.operation_feedback.insert(
        0,
        OperationFeedback {
            category,
            message: format_message(total),
            aggregate_key: Some(aggregate_key),
            count: total,
        },
    );
    game.operation_feedback.truncate(OPERATION_FEEDBACK_LIMIT);
}

fn route_ready_feedback(game: &GameState) -> Option<String> {
    known_system_ids(&game.content_registry)
        .into_iter()
        .filter(|system_id| system_id != &game.current_system_id)
        .find_map(|system_id| {
            let summary = route_readiness_summary(game, &system_id);
            matches!(
                summary.as_str(),
                "Route ready" | "Remote prep ready" | "Route ready; Scanner array 2 recommended"
            )
            .then(|| {
                format!(
                    "{}: {}",
                    system_display_name(&game.content_registry, &system_id),
                    summary
                )
            })
        })
}

fn push_route_ready_feedback(game: &mut GameState) {
    if let Some(message) = route_ready_feedback(game) {
        push_operation_feedback(game, "Route", message);
    }
}

fn recipe_display_name(registry: &content::ContentRegistry, recipe_id: &str) -> String {
    registry
        .recipes
        .get(recipe_id)
        .and_then(|recipe| {
            registry
                .items
                .get(&recipe.output.item)
                .map(|item| format!("{} x{}", item.name, recipe.output.count))
        })
        .unwrap_or_else(|| local_content_id(recipe_id).replace('_', " "))
}

fn research_display_name(registry: &content::ContentRegistry, research_id: &str) -> String {
    registry
        .research
        .get(research_id)
        .map(|research| research.name.clone())
        .unwrap_or_else(|| local_content_id(research_id).replace('_', " "))
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
    push_operation_feedback(
        game,
        "Upgrade",
        format!(
            "{} upgraded to level {}",
            upgrade.kind.name(),
            game.ship_upgrades[upgrade_index].level
        ),
    );
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

fn completed_research_reward_amount(
    registry: &content::ContentRegistry,
    completed_research: &[String],
    reward_kind: &str,
) -> f32 {
    completed_research
        .iter()
        .filter_map(|research_id| registry.research.get(research_id))
        .flat_map(|research| research.rewards.iter())
        .filter(|reward| reward.kind == reward_kind)
        .filter_map(|reward| reward.amount)
        .sum()
}

fn mining_operation_seconds(
    registry: &content::ContentRegistry,
    completed_research: &[String],
) -> f32 {
    BASE_MINING_SECONDS
        / (1.0
            + completed_research_reward_amount(
                registry,
                completed_research,
                "mining_speed_percent",
            ) / 100.0)
}

fn recipe_operation_seconds(
    registry: &content::ContentRegistry,
    completed_research: &[String],
    work_kind: WorkKind,
    recipe: &Recipe,
) -> f32 {
    let base_seconds =
        if work_kind == WorkKind::Fabrication && recipe.output.item.is_id("core:circuit") {
            recipe.base_seconds * 2.0
        } else {
            recipe.base_seconds
        };
    let reward_kind = match work_kind {
        WorkKind::Smelting => "smelting_speed_percent",
        WorkKind::Fabrication => "fabrication_speed_percent",
    };
    base_seconds
        / (1.0
            + completed_research_reward_amount(registry, completed_research, reward_kind) / 100.0)
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
    fn empty() -> Self {
        Self {
            slots: std::array::from_fn(|_| None),
        }
    }

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
        self.count_id(&item.id)
    }

    fn count_id(&self, item_id: &str) -> u32 {
        self.slots
            .iter()
            .filter_map(|slot| slot.as_ref())
            .filter(|stack| stack.item.id == item_id)
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

    fn remove_item(&mut self, item: &ItemRef, count: u32) {
        self.remove_item_id(&item.id, count);
    }

    fn remove_item_id(&mut self, item_id: &str, mut count: u32) {
        for slot in &mut self.slots {
            if count == 0 {
                return;
            }

            if let Some(stack) = slot {
                if stack.item.id == item_id {
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

fn inventory_receivable_count(
    inventory: &Inventory,
    upgrades: &[ShipUpgrade; SHIP_UPGRADE_COUNT],
    item: &ItemRef,
    requested: u32,
) -> u32 {
    let has_slot = inventory.count(item) > 0 || inventory.slots.iter().any(Option::is_none);
    if !has_slot {
        return 0;
    }
    let by_mass = if item.unit_mass > 0.0 {
        ((cargo_rating_kg(upgrades) - inventory.total_mass()).max(0.0) / item.unit_mass) as u32
    } else {
        requested
    };
    requested.min(by_mass)
}

fn owned_ship_in_transfer_range(game: &GameState, index: usize) -> bool {
    game.owned_ships.get(index).is_some_and(|owned| {
        owned.instance_id != game.active_owned_ship_id
            && owned.system_id == game.current_system_id
            && owned.ship.position.distance(game.ship.position) <= OWNED_SHIP_TRANSFER_RANGE
    })
}

fn switch_active_owned_ship(game: &mut GameState, index: usize) -> bool {
    if game
        .owned_ships
        .get(index)
        .is_some_and(|ship| ship.instance_id == game.active_owned_ship_id)
    {
        return true;
    }
    let Some(target) = game.owned_ships.get(index) else {
        return false;
    };
    if target.ship.systems.hull.current <= 0.0
        || !game.content_registry.ships.contains_key(&target.ship_id)
    {
        return false;
    }
    game.sync_active_owned_ship();
    game.pending_warp = None;
    game.scene_transition = None;
    game.approach_target = None;
    game.navigation_route_active = false;
    game.follow_target = None;
    game.orbiting_planet = None;
    game.selected_planet = None;
    game.selected_station = None;
    game.selected_npc_ship = None;
    game.selected_station_service = None;
    if !game.restore_owned_ship_at(index) {
        return false;
    }
    game.weapon_fire_events.clear();
    game.weapon_area_effect_events.clear();
    game.collision_impact_events.clear();
    game.save_dirty = true;
    let callsign = game.owned_ships[index].callsign.clone();
    push_operation_feedback(game, "Fleet", format!("Control transferred to {callsign}"));
    true
}

fn acquire_owned_ship(game: &mut GameState, station_index: usize, ship_id: &str) -> bool {
    let Some(station) = game.stations.get(station_index) else {
        return false;
    };
    if station.system != game.current_system_id
        || !station_in_interaction_range(&game.ship, station)
        || !station.services.iter().any(|service| {
            service.kind == "garage" && station_service_is_available(game, station, service)
        })
    {
        return false;
    }
    let Some(definition) = game.content_registry.ships.get(ship_id).cloned() else {
        return false;
    };
    let Some(price) = definition.purchase_price else {
        return false;
    };
    if game.credits < price {
        push_operation_feedback(
            game,
            "Shipyard",
            format!(
                "Need {} more credits for {}",
                price - game.credits,
                definition.name
            ),
        );
        return true;
    }
    let sequence = game.next_owned_ship_sequence;
    game.next_owned_ship_sequence = game.next_owned_ship_sequence.saturating_add(1);
    let instance_id = format!("ship-{sequence}");
    let mut ship = Ship::from_content(&definition);
    let berth_offset = vec2(80.0 + sequence as f32 * 18.0, 48.0);
    ship.position = station.position + berth_offset;
    let owned = OwnedShip {
        instance_id,
        callsign: format!("{} {sequence}", definition.name),
        ship_id: definition.id.clone(),
        system_id: station.system.clone(),
        ship,
        installed_power_modules: installed_power_modules_from_ids(
            &game.content_registry,
            &definition.power_modules,
        ),
        equipped_shields: equipped_shields_from_ids(
            &game.content_registry,
            &definition.shield_slots,
        ),
        equipped_weapons: equipped_weapons_from_ids(
            &game.content_registry,
            &definition.weapon_slots,
        ),
        inventory: Inventory::empty(),
        upgrades: make_ship_upgrades(),
        shield_recharge_delay_remaining: 0.0,
    };
    game.credits -= price;
    game.owned_ships.push(owned);
    game.selected_owned_ship_index = game.owned_ships.len() - 1;
    game.save_dirty = true;
    push_operation_feedback(
        game,
        "Shipyard",
        format!("Purchased {} for {price} cr", definition.name),
    );
    true
}

fn transfer_owned_ship_item(
    game: &mut GameState,
    target_index: usize,
    item_id: &str,
    to_target: bool,
    requested: u32,
) -> u32 {
    if requested == 0 || !owned_ship_in_transfer_range(game, target_index) {
        return 0;
    }
    let Some(item) = registry_item(&game.content_registry, item_id) else {
        return 0;
    };
    let Some(target) = game.owned_ships.get_mut(target_index) else {
        return 0;
    };
    let amount = if to_target {
        game.inventory.count(&item).min(inventory_receivable_count(
            &target.inventory,
            &target.upgrades,
            &item,
            requested,
        ))
    } else {
        target
            .inventory
            .count(&item)
            .min(inventory_receivable_count(
                &game.inventory,
                &game.ship_upgrades,
                &item,
                requested,
            ))
    };
    if amount == 0 {
        return 0;
    }
    if to_target {
        game.inventory.remove_item(&item, amount);
        target.inventory.add_item(item.clone(), amount);
    } else {
        target.inventory.remove_item(&item, amount);
        game.inventory.add_item(item.clone(), amount);
    }
    game.sync_active_owned_ship();
    game.save_dirty = true;
    push_operation_feedback(
        game,
        "Fleet",
        format!("Transferred {} x{amount}", item.name),
    );
    amount
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
    let previous_days = game.world_elapsed_days;
    game.world_elapsed_days = (game.world_elapsed_days + dt / GAME_DAY_SECONDS).max(0.0);
    update_planet_runtime_positions(&mut game.planets, game.world_elapsed_days);
    if game.world_elapsed_days > previous_days {
        game.save_dirty = true;
    }
    update_contract_progress(game);
    update_station_restock(game);
}

fn update_contract_progress(game: &mut GameState) {
    let mut reached_names = Vec::new();
    for active in &mut game.active_contracts {
        let Some(contract) = game
            .stations
            .iter()
            .flat_map(|station| station.services.iter())
            .flat_map(|service| service.contracts.iter())
            .find(|contract| {
                contract.id == active.id
                    && contract.origin_station == active.origin_station
                    && contract.origin_service == active.origin_service
            })
        else {
            continue;
        };
        let reached = if contract.kind == "hauling" {
            contract.target_station.as_deref().is_some_and(|target| {
                game.selected_station
                    .and_then(|index| game.stations.get(index))
                    .is_some_and(|station| {
                        station.id == target && station_in_interaction_range(&game.ship, station)
                    })
                    && contract
                        .item
                        .as_ref()
                        .is_some_and(|item| game.inventory.count(item) >= contract.amount)
            })
        } else {
            contract.target_planet.as_deref().is_some_and(|target| {
                game.planets
                    .iter()
                    .find(|planet| planet.id == target)
                    .is_some_and(|planet| planet.scan_level >= contract.amount as u8)
            })
        };
        if reached && !active.target_reached {
            active.target_reached = true;
            game.save_dirty = true;
            reached_names.push(contract.name.clone());
        }
    }
    for name in reached_names {
        push_operation_feedback(game, "Contract", format!("Target reached: {name}"));
    }
}

fn update_station_restock(game: &mut GameState) {
    let current_day = game.world_elapsed_days;
    let mut changed = false;
    for station in &mut game.stations {
        for service in &mut station.services {
            if let Some(vendor) = service.vendor.as_mut() {
                let rotation = vendor_rotation(vendor.rotation_days, current_day);
                if rotation != vendor.rotation {
                    vendor.rotation = rotation;
                    service.trade =
                        vendor_trade_offers(vendor, game.world_seed, &game.faction_reputation);
                    changed = true;
                }
            }
            for offer in &mut service.trade {
                let (Some(stock), Some(max_stock), Some(restock_days), Some(next_restock_day)) = (
                    offer.stock.as_mut(),
                    offer.max_stock,
                    offer.restock_days,
                    offer.next_restock_day,
                ) else {
                    continue;
                };
                if !restock_days.is_finite()
                    || restock_days <= 0.0
                    || !next_restock_day.is_finite()
                    || current_day < next_restock_day
                {
                    continue;
                }

                *stock = max_stock;
                let intervals = ((current_day - next_restock_day) / restock_days).floor() + 1.0;
                offer.next_restock_day = Some(next_restock_day + intervals * restock_days);
                changed = true;
            }
        }
    }
    if changed {
        game.save_dirty = true;
    }
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

fn egui_layout_path() -> PathBuf {
    config_dir().join("egui-layout.ron")
}

fn starmap_panel_size_path() -> PathBuf {
    config_dir().join("starmap-size.txt")
}

fn parse_starmap_panel_geometry(source: &str) -> Option<(Option<Vec2>, Vec2)> {
    let mut values = source.split_whitespace();
    let first = values.next()?.parse::<f32>().ok()?;
    let second = values.next()?.parse::<f32>().ok()?;
    let remaining = values
        .map(str::parse::<f32>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    let (position, size) = match remaining.as_slice() {
        [] => (None, vec2(first, second)),
        [width, height] => (Some(vec2(first, second)), vec2(*width, *height)),
        _ => return None,
    };
    let position_valid = position
        .map(|position| position.x.is_finite() && position.y.is_finite())
        .unwrap_or(true);
    (position_valid && size.x.is_finite() && size.y.is_finite() && size.x > 0.0 && size.y > 0.0)
        .then_some((position, size))
}

fn load_starmap_panel_geometry() -> (Option<Vec2>, Option<Vec2>) {
    fs::read_to_string(starmap_panel_size_path())
        .ok()
        .and_then(|source| parse_starmap_panel_geometry(&source))
        .map(|(position, size)| (position, Some(size)))
        .unwrap_or_default()
}

#[cfg(not(test))]
fn save_starmap_panel_geometry(position: Vec2, size: Vec2) {
    let path = starmap_panel_size_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(
        path,
        format!(
            "{:.0} {:.0} {:.0} {:.0}\n",
            position.x, position.y, size.x, size.y
        ),
    );
}

#[cfg(test)]
fn save_starmap_panel_geometry(_position: Vec2, _size: Vec2) {}

fn reset_starmap_panel_geometry(game: &mut GameState) {
    game.starmap_panel_position = None;
    game.starmap_panel_size = None;
    game.starmap_move_previous_mouse = None;
    game.starmap_resize_previous_mouse = None;
    remove_persisted_starmap_panel_size();
}

#[cfg(not(test))]
fn remove_persisted_starmap_panel_size() {
    let _ = fs::remove_file(starmap_panel_size_path());
}

#[cfg(test)]
fn remove_persisted_starmap_panel_size() {}

#[cfg(not(test))]
fn egui_pins_path() -> PathBuf {
    config_dir().join("egui-pins.ron")
}

#[cfg(not(test))]
fn load_egui_pins() {
    EGUI_PIN_PERSISTENCE.with(|persistence| {
        let mut persistence = persistence.borrow_mut();
        if persistence.loaded {
            return;
        }
        persistence.loaded = true;
        let path = egui_pins_path();
        let Ok(source) = fs::read_to_string(&path) else {
            return;
        };
        match ron::from_str::<SavedEguiPins>(&source) {
            Ok(saved) => persistence.pinned = saved.pinned,
            Err(error) => eprintln!("Failed to parse egui pin file {}: {error}", path.display()),
        }
    });
}

#[cfg(test)]
fn load_egui_pins() {
    EGUI_PIN_PERSISTENCE.with(|persistence| persistence.borrow_mut().loaded = true);
}

fn egui_pane_pinned(pane: EguiPane) -> bool {
    load_egui_pins();
    EGUI_PIN_PERSISTENCE.with(|persistence| persistence.borrow().pinned.contains(pane.id()))
}

fn egui_pin_snapshot() -> EguiPinSnapshot {
    EguiPinSnapshot {
        ship: egui_pane_pinned(EguiPane::Ship),
        production: egui_pane_pinned(EguiPane::Production),
        inventory: egui_pane_pinned(EguiPane::Inventory),
        planet: egui_pane_pinned(EguiPane::Planet),
        station: egui_pane_pinned(EguiPane::Station),
        contact: egui_pane_pinned(EguiPane::Contact),
        fleet: egui_pane_pinned(EguiPane::Fleet),
        content: egui_pane_pinned(EguiPane::Content),
        contracts: egui_pane_pinned(EguiPane::Contracts),
        upgrades: egui_pane_pinned(EguiPane::Upgrades),
        research: egui_pane_pinned(EguiPane::Research),
        destinations: egui_pane_pinned(EguiPane::Destinations),
    }
}

fn toggle_egui_pane_pin(pane: EguiPane) {
    load_egui_pins();
    EGUI_PIN_PERSISTENCE.with(|persistence| {
        let mut persistence = persistence.borrow_mut();
        if !persistence.pinned.remove(pane.id()) {
            persistence.pinned.insert(pane.id().to_string());
        }
        let saved = SavedEguiPins {
            pinned: persistence.pinned.clone(),
        };
        persist_egui_pins(&saved);
    });
}

#[cfg(not(test))]
fn persist_egui_pins(saved: &SavedEguiPins) {
    let path = egui_pins_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    match ron::to_string(saved) {
        Ok(serialized) => {
            if let Err(error) = fs::write(&path, serialized) {
                eprintln!("Failed to write egui pin file {}: {error}", path.display());
            }
        }
        Err(error) => eprintln!("Failed to serialize egui pins: {error}"),
    }
}

#[cfg(test)]
fn persist_egui_pins(_saved: &SavedEguiPins) {}

fn reset_egui_pins() {
    EGUI_PIN_PERSISTENCE.with(|persistence| {
        let mut persistence = persistence.borrow_mut();
        persistence.loaded = true;
        persistence.pinned.clear();
    });
    remove_egui_pins_file();
}

#[cfg(not(test))]
fn remove_egui_pins_file() {
    let path = egui_pins_path();
    if let Err(error) = fs::remove_file(&path) {
        if error.kind() != std::io::ErrorKind::NotFound {
            eprintln!("Failed to remove egui pin file {}: {error}", path.display());
        }
    }
}

#[cfg(test)]
fn remove_egui_pins_file() {}

fn prepare_egui_layout(ctx: &egui::Context, screen: [f32; 2]) {
    EGUI_LAYOUT_PERSISTENCE.with(|persistence| {
        let mut persistence = persistence.borrow_mut();
        if persistence.loaded {
            return;
        }
        persistence.loaded = true;

        let path = egui_layout_path();
        let Ok(source) = fs::read_to_string(&path) else {
            return;
        };
        let memory = match ron::from_str::<egui::Memory>(&source) {
            Ok(memory) => memory,
            Err(error) => {
                eprintln!(
                    "Failed to parse egui layout file {}: {error}",
                    path.display()
                );
                return;
            }
        };
        ctx.memory_mut(|current| *current = memory);

        let all_recoverable = PERSISTED_EGUI_WINDOW_IDS.iter().all(|id| {
            ctx.memory(|memory| {
                memory
                    .area_rect(egui::Id::new(*id))
                    .is_none_or(|rect| egui_window_has_reachable_area(rect, screen))
            })
        });
        if all_recoverable {
            persistence.last_written = source;
        } else {
            ctx.memory_mut(|memory| memory.reset_areas());
            let _ = fs::remove_file(path);
        }
    });
}

fn egui_window_has_reachable_area(rect: egui::Rect, screen: [f32; 2]) -> bool {
    let visible_width = rect.max.x.min(screen[0]) - rect.min.x.max(0.0);
    let visible_height = rect.max.y.min(screen[1]) - rect.min.y.max(0.0);
    visible_width >= 64.0 && visible_height >= 24.0
}

fn queue_egui_layout_save(ctx: &egui::Context, dt: f32) {
    let Some(serialized) = ctx.memory(ron::to_string).ok() else {
        return;
    };

    EGUI_LAYOUT_PERSISTENCE.with(|persistence| {
        let mut persistence = persistence.borrow_mut();
        if serialized == persistence.last_written {
            persistence.pending = None;
            persistence.pending_seconds = 0.0;
            return;
        }
        persistence.pending = Some(serialized);
        persistence.pending_seconds += dt.max(0.0);
        if persistence.pending_seconds < EGUI_LAYOUT_SAVE_DELAY {
            return;
        }
        write_pending_egui_layout(&mut persistence);
    });
}

fn write_pending_egui_layout(persistence: &mut EguiLayoutPersistence) {
    let Some(serialized) = persistence.pending.take() else {
        return;
    };
    let path = egui_layout_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Err(error) = fs::write(&path, &serialized) {
        eprintln!(
            "Failed to write egui layout file {}: {error}",
            path.display()
        );
        persistence.pending = Some(serialized);
        return;
    }
    persistence.last_written = serialized;
    persistence.pending_seconds = 0.0;
}

#[cfg(not(test))]
fn flush_egui_layout() {
    egui_macroquad::cfg(|ctx| {
        if let Ok(serialized) = ctx.memory(ron::to_string) {
            EGUI_LAYOUT_PERSISTENCE.with(|persistence| {
                let mut persistence = persistence.borrow_mut();
                if serialized != persistence.last_written {
                    persistence.pending = Some(serialized);
                }
                write_pending_egui_layout(&mut persistence);
            });
        }
    });
}

#[cfg(test)]
fn flush_egui_layout() {}

fn reset_egui_menu_layout() {
    egui_macroquad::cfg(|ctx| ctx.memory_mut(|memory| memory.reset_areas()));
    let path = egui_layout_path();
    if let Err(error) = fs::remove_file(&path) {
        if error.kind() != std::io::ErrorKind::NotFound {
            eprintln!(
                "Failed to remove egui layout file {}: {error}",
                path.display()
            );
        }
    }
    EGUI_LAYOUT_PERSISTENCE.with(|persistence| {
        let mut persistence = persistence.borrow_mut();
        persistence.loaded = true;
        persistence.last_written.clear();
        persistence.pending = None;
        persistence.pending_seconds = 0.0;
    });
    reset_egui_pins();
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
        Ok(save) if (1..=SAVE_VERSION).contains(&save.version) => Some(save),
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
    draw_startup_transition_with_remote(background, label, opacity, None);
}

fn draw_startup_transition_with_remote(
    background: Option<&Texture2D>,
    label: &str,
    opacity: f32,
    remote_progress: Option<&remote_assets::RemoteAssetProgress>,
) {
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
    draw_startup_branding_logo(opacity);
    draw_startup_loading_panel(label, opacity, remote_progress);
}

fn draw_remote_audio_startup(progress: &remote_assets::RemoteAssetProgress) {
    draw_startup_transition_with_remote(None, &progress.message, 1.0, Some(progress));
}

fn remote_audio_startup_detail(progress: &remote_assets::RemoteAssetProgress) -> &'static str {
    match progress.phase {
        remote_assets::RemoteAssetPhase::Checking => "Checking the optional remote audio release",
        remote_assets::RemoteAssetPhase::Downloading => "Downloading remote audio",
        remote_assets::RemoteAssetPhase::Verifying => "Verifying remote audio",
        remote_assets::RemoteAssetPhase::Ready => "Remote audio is ready",
        remote_assets::RemoteAssetPhase::Failed => {
            "Remote audio unavailable; startup will continue offline"
        }
        remote_assets::RemoteAssetPhase::Offline => "Continuing offline with local audio assets",
    }
}

fn remote_audio_startup_fraction(progress: &remote_assets::RemoteAssetProgress) -> Option<f32> {
    if progress.total_assets == 0 {
        None
    } else {
        let current = if progress.current_total_bytes == 0 {
            0.0
        } else {
            progress.current_bytes as f32 / progress.current_total_bytes as f32
        };
        Some(
            ((progress.completed_assets as f32 + current) / progress.total_assets as f32)
                .clamp(0.0, 1.0),
        )
    }
}

fn startup_loading_layout(screen: Vec2) -> (Rect, f32) {
    let logo_width = (screen.x - 48.0).clamp(280.0, 720.0);
    let logo_height = ((screen.y - 250.0).clamp(170.0, 400.0)).min(logo_width * 0.57);
    let composition_height = logo_height + 142.0;
    let logo_y = ((screen.y - composition_height) * 0.5).max(12.0);
    (
        Rect::new(
            (screen.x - logo_width) * 0.5,
            logo_y,
            logo_width,
            logo_height,
        ),
        (logo_y + logo_height + 76.0).min(screen.y - 72.0),
    )
}

fn draw_startup_branding_logo(opacity: f32) {
    let (destination, _) = startup_loading_layout(vec2(screen_width(), screen_height()));
    BRANDING_LOGO.with(|logo| {
        let logo = logo.borrow();
        if let Some(logo) = logo.as_ref() {
            let source = Rect::new(
                logo.width() * 0.20,
                logo.height() * 0.18,
                logo.width() * 0.61,
                logo.height() * 0.52,
            );
            draw_texture_source_contain(logo, source, destination, opacity);
        }
    });
}

fn draw_startup_loading_panel(
    label: &str,
    opacity: f32,
    remote_progress: Option<&remote_assets::RemoteAssetProgress>,
) {
    let (_, panel_center_y) = startup_loading_layout(vec2(screen_width(), screen_height()));
    let detail = remote_progress.map(remote_audio_startup_detail);
    let local_progress = startup_progress();
    let progress = local_progress
        .map(StartupProgress::fraction)
        .or_else(|| remote_progress.and_then(remote_audio_startup_fraction));
    let count = local_progress.map(StartupProgress::count).or_else(|| {
        remote_progress.and_then(|progress| {
            (progress.total_assets > 0).then(|| {
                format!(
                    "{} / {} assets",
                    progress.completed_assets, progress.total_assets
                )
            })
        })
    });
    egui_macroquad::ui(|ctx| {
        egui_loading::draw(
            ctx,
            [screen_width(), screen_height()],
            &egui_loading::LoadingView {
                label,
                detail,
                progress,
                count: count.as_deref(),
                opacity,
                panel_center_y,
            },
        );
    });
    egui_macroquad::draw();
}

fn draw_startup_transition_assets(
    assets: &[TransitionAsset],
    preferred_id: Option<&str>,
    label: &str,
    opacity: f32,
) {
    draw_startup_transition_assets_with_remote(assets, preferred_id, label, opacity, None);
}

fn draw_startup_transition_assets_with_remote(
    assets: &[TransitionAsset],
    preferred_id: Option<&str>,
    label: &str,
    opacity: f32,
    remote_progress: Option<&remote_assets::RemoteAssetProgress>,
) {
    let ordered_assets = ordered_startup_transition_assets(assets, preferred_id);
    draw_startup_transition_sequence(&ordered_assets, label, opacity, remote_progress);
}

fn draw_startup_transition_sequence(
    assets: &[&TransitionAsset],
    label: &str,
    opacity: f32,
    remote_progress: Option<&remote_assets::RemoteAssetProgress>,
) {
    let background = startup_transition_background_at_time(
        assets,
        get_time() as f32,
        STARTUP_BACKGROUND_HOLD_SECONDS,
        STARTUP_BACKGROUND_FADE_SECONDS,
    );
    let opacity = opacity.clamp(0.0, 1.0);
    let screen_w = screen_width();
    let screen_h = screen_height();

    clear_background(Color::from_rgba(5, 8, 18, 255));

    match background {
        StartupTransitionBackground::None => {}
        StartupTransitionBackground::Single(texture) => {
            draw_fullscreen_texture_cover(texture, opacity);
        }
        StartupTransitionBackground::Crossfade {
            current,
            next,
            progress,
        } => {
            draw_fullscreen_texture_cover(current, opacity * (1.0 - progress));
            draw_fullscreen_texture_cover(next, opacity * progress);
        }
    }

    draw_rectangle(
        0.0,
        0.0,
        screen_w,
        screen_h,
        Color::new(0.01, 0.02, 0.04, 0.34 + 0.28 * opacity),
    );
    draw_startup_branding_logo(opacity);
    draw_startup_loading_panel(label, opacity, remote_progress);
}

enum StartupTransitionBackground<'a> {
    None,
    Single(&'a Texture2D),
    Crossfade {
        current: &'a Texture2D,
        next: &'a Texture2D,
        progress: f32,
    },
}

fn startup_transition_background_at_time<'a>(
    assets: &[&'a TransitionAsset],
    time_seconds: f32,
    hold_seconds: f32,
    fade_seconds: f32,
) -> StartupTransitionBackground<'a> {
    match assets {
        [] => StartupTransitionBackground::None,
        [asset] => StartupTransitionBackground::Single(&asset.texture),
        _ => {
            let step_seconds = hold_seconds + fade_seconds;
            let cycle_seconds = step_seconds * assets.len() as f32;
            let cycle_time = time_seconds.rem_euclid(cycle_seconds);
            let current_index = (cycle_time / step_seconds).floor() as usize;
            let local_time = cycle_time - current_index as f32 * step_seconds;
            let current = &assets[current_index].texture;
            if local_time < hold_seconds || fade_seconds <= 0.0 {
                StartupTransitionBackground::Single(current)
            } else {
                let next_index = (current_index + 1) % assets.len();
                StartupTransitionBackground::Crossfade {
                    current,
                    next: &assets[next_index].texture,
                    progress: ((local_time - hold_seconds) / fade_seconds).clamp(0.0, 1.0),
                }
            }
        }
    }
}

fn ordered_startup_transition_assets<'a>(
    assets: &'a [TransitionAsset],
    preferred_id: Option<&str>,
) -> Vec<&'a TransitionAsset> {
    let mut ordered = Vec::with_capacity(assets.len());
    if let Some(preferred_id) = preferred_id {
        if let Some(asset) = assets.iter().find(|asset| asset.id == preferred_id) {
            ordered.push(asset);
        }
    }
    ordered.extend(assets.iter().filter(|asset| {
        preferred_id
            .map(|preferred_id| asset.id != preferred_id)
            .unwrap_or(true)
    }));
    ordered
}

async fn run_startup_transition_out(assets: &[TransitionAsset], preferred_id: Option<&str>) {
    let mut elapsed = 0.0;
    while elapsed < STARTUP_FADE_SECONDS {
        let dt = get_frame_time().min(1.0 / 30.0);
        elapsed += dt;
        let opacity = 1.0 - (elapsed / STARTUP_FADE_SECONDS).clamp(0.0, 1.0);
        draw_startup_transition_sequence(
            &ordered_startup_transition_assets(assets, preferred_id),
            "Launching ...",
            opacity,
            None,
        );
        next_frame().await;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let runtime_flags = RuntimeFlags::from_env_args();
    set_ui_font(load_ui_font().await);
    let branding_logo = load_asset_texture(BRANDING_LOGO_PATH).await;
    set_branding_logo(branding_logo.clone());
    let ui_panel_corner = load_asset_texture(UI_PANEL_CORNER_PATH).await;
    let background = make_background();
    let mut app = if fast_start_enabled() {
        let start_mode = latest_save_path()
            .map(|path| GameStartMode::LoadGame { path })
            .unwrap_or_else(|| GameStartMode::NewGame {
                seed: new_world_seed(),
                ship_id: default_new_game_ship_id(),
                pack_options: default_content_pack_option_selections(),
            });
        AppState::Playing(Box::new(GameState::new(start_mode, runtime_flags).await))
    } else {
        AppState::Title(loaded_title_menu().await)
    };

    loop {
        let dt = get_frame_time().min(1.0 / 30.0);
        match &mut app {
            AppState::Title(menu) => {
                if let Some(action) = update_title_menu(menu) {
                    match action {
                        TitleAction::NewGame {
                            seed,
                            ship_id,
                            pack_options,
                        } => {
                            let mut game = GameState::new(
                                GameStartMode::NewGame {
                                    seed,
                                    ship_id,
                                    pack_options,
                                },
                                runtime_flags,
                            )
                            .await;
                            save_game_now(&mut game, SaveFeedback::Manual);
                            app = AppState::Playing(Box::new(game));
                        }
                        TitleAction::LoadGame { path } => {
                            app = AppState::Playing(Box::new(
                                GameState::new(GameStartMode::LoadGame { path }, runtime_flags)
                                    .await,
                            ));
                        }
                        TitleAction::QuitDesktop => {
                            macroquad::miniquad::window::quit();
                        }
                    }
                } else {
                    draw_title_menu(menu, &background);
                }
            }
            AppState::Playing(game) => {
                let egui_debug_was_active = game.runtime_flags.debug && game.debug_console.open;
                let egui_escape_was_active = game.escape_dialog_open;
                let pins_before_update = egui_pin_snapshot();
                let egui_fleet_was_active = egui_fleet_visible(game, pins_before_update);
                let egui_inventory_was_active =
                    egui_inventory_windows_visible(game, pins_before_update);
                let egui_planet_was_active = egui_planet_visible(game, pins_before_update);
                let egui_station_was_active = egui_station_visible(game, pins_before_update);
                let egui_npc_was_active = egui_npc_visible(game, pins_before_update);
                let egui_content_was_active = egui_content_visible(game, pins_before_update);
                let egui_contracts_was_active = egui_contracts_visible(game, pins_before_update);
                let egui_upgrades_was_active = egui_upgrades_visible(game, pins_before_update);
                let egui_research_was_active = egui_research_visible(game, pins_before_update);
                let egui_destinations_was_active =
                    egui_destinations_visible(game, pins_before_update);
                let egui_context_menu_was_active = game.context_action_menu.is_some();
                update_game(game, dt);
                if game.quit_to_title_requested {
                    app = AppState::Title(loaded_title_menu().await);
                } else {
                    draw_scene(game, &background, ui_panel_corner.as_ref());
                    let pins = egui_pin_snapshot();
                    let keep_click_step = production_keep_click_step();
                    let quantity_step = keep_click_step.max(1) as u32;
                    let upgrade_purchase_count = work_setting_step().max(1) as u32;
                    let egui_windows_allowed = !game.escape_dialog_open;
                    let inventory_visible =
                        egui_windows_allowed && egui_inventory_windows_visible(game, pins);
                    let planet_visible = egui_windows_allowed && egui_planet_visible(game, pins);
                    let station_visible = egui_windows_allowed && egui_station_visible(game, pins);
                    let npc_visible = egui_windows_allowed && egui_npc_visible(game, pins);
                    let fleet_visible = egui_windows_allowed && egui_fleet_visible(game, pins);
                    let content_visible = egui_windows_allowed && egui_content_visible(game, pins);
                    let contracts_visible =
                        egui_windows_allowed && egui_contracts_visible(game, pins);
                    let upgrades_visible =
                        egui_windows_allowed && egui_upgrades_visible(game, pins);
                    let research_visible =
                        egui_windows_allowed && egui_research_visible(game, pins);
                    let destinations_visible =
                        egui_windows_allowed && egui_destinations_visible(game, pins);
                    let inventory_view =
                        inventory_visible.then(|| inventory_overlay_view(game, keep_click_step));
                    let planet_view =
                        planet_visible.then(|| planet_pane_view(game, keep_click_step));
                    let station_view = station_visible.then(|| station_pane_view(game));
                    let npc_view = npc_visible.then(|| npc_pane_view(game, quantity_step));
                    let fleet_view = fleet_visible.then(|| fleet_pane_view(game, quantity_step));
                    let content_view = content_visible.then(|| content_browser_view(game));
                    let contracts_view = contracts_visible.then(|| contracts_view(game));
                    let upgrades_view =
                        upgrades_visible.then(|| upgrades_view(game, upgrade_purchase_count));
                    let research_view = research_visible.then(|| research_view(game));
                    let destinations_view = destinations_visible.then(|| destinations_view(game));
                    let context_view = if egui_windows_allowed {
                        context_menu_view(game)
                    } else {
                        None
                    };
                    let escape_visible = game.escape_dialog_open;
                    let escape_view = escape_visible.then_some(egui_escape::EscapeView {
                        save_dirty: game.save_dirty,
                    });
                    let debug_visible = game.runtime_flags.debug && game.debug_console.open;
                    let debug_view = debug_visible.then(|| debug_console_view(game));
                    let any_egui_visible = debug_visible
                        || escape_visible
                        || inventory_visible
                        || planet_visible
                        || station_visible
                        || npc_visible
                        || fleet_visible
                        || content_visible
                        || contracts_visible
                        || upgrades_visible
                        || research_visible
                        || destinations_visible
                        || context_view.is_some();

                    if any_egui_visible {
                        let mut inventory_actions = Vec::new();
                        let mut planet_actions = Vec::new();
                        let mut station_actions = Vec::new();
                        let mut npc_actions = Vec::new();
                        let mut fleet_actions = Vec::new();
                        let mut content_actions = Vec::new();
                        let mut contracts_actions = Vec::new();
                        let mut upgrades_actions = Vec::new();
                        let mut research_actions = Vec::new();
                        let mut destinations_actions = Vec::new();
                        let mut escape_actions = Vec::new();
                        let mut debug_actions = Vec::new();
                        let mut context_outcome = None;
                        egui_macroquad::ui(|ctx| {
                            prepare_egui_layout(ctx, [screen_width(), screen_height()]);
                            if let Some(view) = &inventory_view {
                                inventory_actions = egui_inventory::draw(
                                    ctx,
                                    [screen_width(), screen_height()],
                                    view,
                                );
                            }
                            if let Some(view) = &planet_view {
                                planet_actions =
                                    egui_planet::draw(ctx, [screen_width(), screen_height()], view);
                            }
                            if let Some(view) = &station_view {
                                station_actions = egui_station::draw(
                                    ctx,
                                    [screen_width(), screen_height()],
                                    view,
                                );
                            }
                            if let Some(view) = &npc_view {
                                npc_actions =
                                    egui_npc::draw(ctx, [screen_width(), screen_height()], view);
                            }
                            if let Some(view) = &fleet_view {
                                fleet_actions =
                                    egui_fleet::draw(ctx, [screen_width(), screen_height()], view);
                            }
                            if let Some(view) = &content_view {
                                content_actions = egui_content::draw(
                                    ctx,
                                    [screen_width(), screen_height()],
                                    view,
                                );
                            }
                            if let Some(view) = &contracts_view {
                                contracts_actions = egui_contracts::draw(
                                    ctx,
                                    [screen_width(), screen_height()],
                                    view,
                                );
                            }
                            if let Some(view) = &upgrades_view {
                                upgrades_actions = egui_upgrades::draw(
                                    ctx,
                                    [screen_width(), screen_height()],
                                    view,
                                );
                            }
                            if let Some(view) = &research_view {
                                research_actions = egui_research::draw(
                                    ctx,
                                    [screen_width(), screen_height()],
                                    view,
                                );
                            }
                            if let Some(view) = &destinations_view {
                                destinations_actions = egui_destinations::draw(
                                    ctx,
                                    [screen_width(), screen_height()],
                                    view,
                                );
                            }
                            if let Some(view) = escape_view {
                                escape_actions =
                                    egui_escape::draw(ctx, [screen_width(), screen_height()], view);
                            }
                            if let Some(view) = &debug_view {
                                debug_actions =
                                    egui_debug::draw(ctx, [screen_width(), screen_height()], view);
                            }
                            if let Some(view) = &context_view {
                                context_outcome = egui_context_menu::draw(ctx, view);
                            }
                            queue_egui_layout_save(ctx, dt);
                        });
                        if egui_inventory_was_active && inventory_visible {
                            apply_inventory_ui_actions(game, inventory_actions, keep_click_step);
                        }
                        if egui_planet_was_active && planet_visible {
                            apply_planet_ui_actions(game, planet_actions, keep_click_step);
                        }
                        if egui_station_was_active && station_visible {
                            apply_station_ui_actions(game, station_actions);
                        }
                        if egui_npc_was_active && npc_visible {
                            apply_npc_ui_actions(game, npc_actions, quantity_step);
                        }
                        if egui_fleet_was_active && fleet_visible {
                            apply_fleet_ui_actions(game, fleet_actions, quantity_step);
                        }
                        if egui_content_was_active && content_visible {
                            apply_content_ui_actions(game, content_actions);
                        }
                        if egui_contracts_was_active && contracts_visible {
                            apply_contracts_ui_actions(game, contracts_actions);
                        }
                        if egui_upgrades_was_active && upgrades_visible {
                            apply_upgrades_ui_actions(
                                game,
                                upgrades_actions,
                                upgrade_purchase_count,
                            );
                        }
                        if egui_research_was_active && research_visible {
                            apply_research_ui_actions(game, research_actions);
                        }
                        if egui_destinations_was_active && destinations_visible {
                            apply_destinations_ui_actions(game, destinations_actions);
                        }
                        if egui_escape_was_active && escape_visible {
                            apply_escape_ui_actions(game, escape_actions);
                        }
                        if egui_debug_was_active && debug_visible {
                            apply_debug_console_ui_actions(game, debug_actions);
                        }
                        if egui_context_menu_was_active && context_view.is_some() {
                            apply_context_menu_outcome(game, context_outcome);
                        }
                        egui_macroquad::draw();
                    }
                }
            }
        }
        next_frame().await;
    }
}

impl RuntimeFlags {
    fn from_env_args() -> Self {
        Self::from_args(env::args().skip(1))
    }

    fn from_args(args: impl IntoIterator<Item = String>) -> Self {
        Self {
            debug: args.into_iter().any(|arg| arg == "--debug"),
        }
    }
}

fn fast_start_enabled() -> bool {
    env::var("SOME_FRONTIER_FAST_START")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn handle_debug_console_toggle(game: &mut GameState) -> bool {
    if !game.runtime_flags.debug {
        return false;
    }
    if is_key_pressed(KeyCode::GraveAccent) || is_key_pressed(KeyCode::F12) {
        game.debug_console.open = !game.debug_console.open;
        return true;
    }
    false
}

fn handle_debug_console_input(game: &mut GameState) {
    if !game.debug_console.open {
        return;
    }
    if is_key_pressed(KeyCode::Escape) {
        game.debug_console.open = false;
    }
}

fn debug_console_view(game: &GameState) -> egui_debug::DebugConsoleView {
    egui_debug::DebugConsoleView {
        input: game.debug_console.input.clone(),
        history: game.debug_console.history.clone(),
        help: debug_console_help().to_string(),
    }
}

fn apply_debug_console_ui_actions(
    game: &mut GameState,
    actions: Vec<egui_debug::DebugConsoleUiAction>,
) {
    for action in actions {
        match action {
            egui_debug::DebugConsoleUiAction::SetInput(input) => {
                game.debug_console.input = input;
            }
            egui_debug::DebugConsoleUiAction::Execute(command) => {
                let result = execute_debug_console_command(game, &command);
                game.debug_console.history.insert(0, format!("> {command}"));
                game.debug_console.history.insert(0, result);
                game.debug_console
                    .history
                    .truncate(DEBUG_CONSOLE_HISTORY_LIMIT);
                game.debug_console.input.clear();
            }
            egui_debug::DebugConsoleUiAction::Close => {
                game.debug_console.open = false;
            }
        }
    }
}

fn execute_debug_console_command(game: &mut GameState, command: &str) -> String {
    let parts = command.split_whitespace().collect::<Vec<_>>();
    match parts.as_slice() {
        [] => "No command entered".to_string(),
        ["help"] => debug_console_help().to_string(),
        ["give", item_id] => debug_console_give(game, item_id, 1),
        ["give", item_id, count] => match count.parse::<u32>() {
            Ok(count) if count > 0 => debug_console_give(game, item_id, count),
            _ => format!("Invalid item count `{count}`"),
        },
        ["credits", amount] => match amount.parse::<u32>() {
            Ok(amount) if amount > 0 => {
                game.credits = game.credits.saturating_add(amount);
                game.save_dirty = true;
                format!("Added {amount} credits")
            }
            _ => format!("Invalid credit amount `{amount}`"),
        },
        ["credits", "set", amount] => match amount.parse::<u32>() {
            Ok(amount) => {
                game.credits = amount;
                game.save_dirty = true;
                format!("Credits set to {amount}")
            }
            _ => format!("Invalid credit amount `{amount}`"),
        },
        ["research", "complete", "all"] => {
            game.completed_research = game.content_registry.research_order.clone();
            game.completed_research.sort();
            game.completed_research.dedup();
            game.active_research = None;
            game.save_dirty = true;
            format!("Completed {} research nodes", game.completed_research.len())
        }
        ["research", "complete", research_id] => {
            let Some(research_id) =
                resolve_console_content_id(&game.content_registry.research, research_id)
            else {
                return format!("Unknown research `{research_id}`");
            };
            if !game
                .completed_research
                .iter()
                .any(|done| done == &research_id)
            {
                game.completed_research.push(research_id.clone());
                game.completed_research.sort();
                game.completed_research.dedup();
            }
            if game
                .active_research
                .as_ref()
                .is_some_and(|active| active.research == research_id)
            {
                game.active_research = None;
            }
            game.save_dirty = true;
            format!(
                "Completed research {}",
                research_display_name(&game.content_registry, &research_id)
            )
        }
        ["recipes", "unlock", "all"] => {
            let mut unlocked = 0_u32;
            for research_id in &game.content_registry.research_order {
                let Some(research) = game.content_registry.research.get(research_id) else {
                    continue;
                };
                if research
                    .rewards
                    .iter()
                    .any(|reward| reward.kind == "recipe_unlock")
                    && !game
                        .completed_research
                        .iter()
                        .any(|done| done == research_id)
                {
                    game.completed_research.push(research_id.clone());
                    unlocked += 1;
                }
            }
            game.completed_research.sort();
            game.completed_research.dedup();
            game.save_dirty = true;
            format!("Unlocked recipes from {unlocked} research nodes")
        }
        ["warp", system_id] => {
            let Some(system_id) =
                resolve_console_content_id(&game.content_registry.systems, system_id)
            else {
                return format!("Unknown system `{system_id}`");
            };
            switch_current_system(game, &system_id);
            game.pending_warp = None;
            game.scene_transition = None;
            game.save_dirty = true;
            format!(
                "Warped to {}",
                system_display_name(&game.content_registry, &system_id)
            )
        }
        [unknown, ..] => format!("Unknown command `{unknown}`. Try `help`."),
    }
}

fn debug_console_give(game: &mut GameState, item_id: &str, count: u32) -> String {
    let Some(item_id) = resolve_console_content_id(&game.content_registry.items, item_id) else {
        return format!("Unknown item `{item_id}`");
    };
    let Some(item) = registry_item(&game.content_registry, &item_id) else {
        return format!("Unknown item `{item_id}`");
    };
    game.inventory.add_item(item.clone(), count);
    game.save_dirty = true;
    format!("Gave {} x{count}", item.name)
}

fn resolve_console_content_id<T>(registry: &HashMap<String, T>, input: &str) -> Option<String> {
    if registry.contains_key(input) {
        return Some(input.to_string());
    }
    if let Some((pack, local)) = input.split_once('.') {
        let candidate = format!("{pack}:{local}");
        if registry.contains_key(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn debug_console_help() -> &'static str {
    "Commands: give <item> [count], credits <amount>, credits set <amount>, research complete <id|all>, recipes unlock all, warp <system>"
}

impl Default for TitleMenu {
    fn default() -> Self {
        let registry = load_game_content_registry();
        let ships = title_ships(&registry);
        let default_ship_id = resolved_new_game_ship_id(&registry, None);
        let selected_ship_index = ships
            .iter()
            .position(|ship| ship.id == default_ship_id)
            .unwrap_or(0);
        Self {
            view: TitleView::Main,
            pending_action: None,
            branding_egui_image: None,
            branding_egui_texture: None,
            new_game_seed_text: new_world_seed().to_string(),
            ships,
            selected_ship_index,
            ship_textures: HashMap::new(),
            save_slots: title_save_slots(),
            selected_save_index: 0,
            pending_delete_save_index: None,
            delete_save_error: None,
            content_packs: title_content_packs(&registry),
            selected_pack_index: 0,
            settings: read_app_settings(),
            selected_settings_category: SettingsCategory::Display,
        }
    }
}

impl TitleMenu {
    async fn load_branding_image(&mut self) {
        if let Ok(image) = load_image(BRANDING_LOGO_PATH).await {
            self.branding_egui_image =
                Some(cropped_color_image(&image, image_alpha_bounds(&image)));
        }
    }

    async fn load_ship_textures(&mut self) {
        let ship_assets = self
            .ships
            .iter()
            .filter_map(|ship| {
                ship.texture
                    .as_ref()
                    .map(|texture| (ship.id.clone(), texture.clone()))
            })
            .collect::<Vec<_>>();
        for (ship_id, texture_path) in ship_assets {
            if let Ok(image) = load_image(&texture_path).await {
                let source = image_alpha_bounds(&image);
                let egui_image = quarter_turned_cropped_color_image(&image, source);
                self.ship_textures.insert(
                    ship_id,
                    TitleShipPreview {
                        egui_image,
                        egui_texture: None,
                    },
                );
            }
        }
    }
}

fn cropped_color_image(image: &Image, source: Rect) -> egui::ColorImage {
    let source_x = source.x.max(0.0) as usize;
    let source_y = source.y.max(0.0) as usize;
    let source_width = source.w.max(1.0) as usize;
    let source_height = source.h.max(1.0) as usize;
    let image_width = usize::from(image.width);
    let mut cropped = vec![0_u8; source_width * source_height * 4];

    for row in 0..source_height {
        let source_start = ((source_y + row) * image_width + source_x) * 4;
        let source_end = source_start + source_width * 4;
        let destination_start = row * source_width * 4;
        let destination_end = destination_start + source_width * 4;
        if source_end <= image.bytes.len() {
            cropped[destination_start..destination_end]
                .copy_from_slice(&image.bytes[source_start..source_end]);
        }
    }

    egui::ColorImage::from_rgba_unmultiplied([source_width, source_height], &cropped)
}

fn quarter_turned_cropped_color_image(image: &Image, source: Rect) -> egui::ColorImage {
    let source_x = source.x.max(0.0) as usize;
    let source_y = source.y.max(0.0) as usize;
    let source_width = source.w.max(1.0) as usize;
    let source_height = source.h.max(1.0) as usize;
    let image_width = usize::from(image.width);
    let mut rotated = vec![0_u8; source_width * source_height * 4];

    for source_row in 0..source_height {
        for source_column in 0..source_width {
            let source_index =
                ((source_y + source_row) * image_width + source_x + source_column) * 4;
            let destination_column = source_height - source_row - 1;
            let destination_row = source_column;
            let destination_index = (destination_row * source_height + destination_column) * 4;
            if source_index + 4 <= image.bytes.len() && destination_index + 4 <= rotated.len() {
                rotated[destination_index..destination_index + 4]
                    .copy_from_slice(&image.bytes[source_index..source_index + 4]);
            }
        }
    }

    egui::ColorImage::from_rgba_unmultiplied([source_height, source_width], &rotated)
}

async fn loaded_title_menu() -> Box<TitleMenu> {
    let mut menu = TitleMenu::default();
    menu.load_branding_image().await;
    menu.load_ship_textures().await;
    Box::new(menu)
}

fn image_alpha_bounds(image: &Image) -> Rect {
    let width = usize::from(image.width);
    let height = usize::from(image.height);
    let full = Rect::new(0.0, 0.0, width as f32, height as f32);
    if width == 0 || height == 0 || image.bytes.len() < width * height * 4 {
        return full;
    }

    let mut min_x = width;
    let mut min_y = height;
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    let mut found = false;
    for (index, pixel) in image.bytes.chunks_exact(4).take(width * height).enumerate() {
        if pixel[3] <= 4 {
            continue;
        }
        let x = index % width;
        let y = index / width;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        found = true;
    }
    if !found {
        return full;
    }
    Rect::new(
        min_x as f32,
        min_y as f32,
        (max_x - min_x + 1) as f32,
        (max_y - min_y + 1) as f32,
    )
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

fn title_ships(registry: &content::ContentRegistry) -> Vec<TitleShip> {
    registry
        .ship_order
        .iter()
        .filter_map(|ship_id| registry.ships.get(ship_id))
        .map(|ship| TitleShip {
            id: ship.id.clone(),
            name: ship.name.clone(),
            texture: ship.texture.clone(),
            hull_capacity: ship.hull_capacity,
            shield_capacity: ship.shield_capacity,
            weapon_banks: ship.weapon_slots.len(),
        })
        .collect()
}

fn title_content_packs(registry: &content::ContentRegistry) -> Vec<TitleContentPack> {
    registry
        .packs
        .iter()
        .map(|pack| TitleContentPack {
            id: pack.id.clone(),
            name: pack.name.clone(),
            version: pack.version.clone(),
            description: pack.description.clone(),
            options: pack
                .options
                .iter()
                .map(|option| {
                    let default_value = option.default.as_save_string();
                    TitlePackOption {
                        id: option.id.clone(),
                        label: option.label.clone(),
                        description: option.description.clone(),
                        value_type: option.value_type,
                        default_value: default_value.clone(),
                        current_value: default_value,
                        choices: option.choices.clone(),
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
    if menu.pending_action.is_some() {
        return menu.pending_action.take();
    }
    if is_key_pressed(KeyCode::Escape) {
        if menu.view == TitleView::Main {
            return Some(TitleAction::QuitDesktop);
        }
        menu.view = TitleView::Main;
        return None;
    }

    let mut egui_wants_keyboard = false;
    egui_macroquad::cfg(|ctx| egui_wants_keyboard = ctx.wants_keyboard_input());

    match menu.view {
        TitleView::Main => update_title_main_menu(menu),
        TitleView::NewGame => {
            if egui_wants_keyboard {
                return None;
            }
            let action = update_title_new_game(menu);
            if action.is_some() {
                return action;
            }
            None
        }
        TitleView::LoadGame => {
            let action = update_title_load_game(menu);
            if action.is_some() {
                return action;
            }
            if is_key_pressed(KeyCode::Backspace) {
                menu.view = TitleView::Main;
            }
            None
        }
        TitleView::ContentPacks => {
            update_title_content_packs(menu);
            if is_key_pressed(KeyCode::Backspace) {
                menu.view = TitleView::Main;
            }
            None
        }
        TitleView::Settings => {
            update_title_settings(menu);
            if is_key_pressed(KeyCode::Backspace) {
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
        menu.pending_delete_save_index = None;
        menu.delete_save_error = None;
        return None;
    }

    if is_key_pressed(KeyCode::Up) {
        menu.selected_save_index = menu.selected_save_index.saturating_sub(1);
        menu.pending_delete_save_index = None;
        menu.delete_save_error = None;
    }
    if is_key_pressed(KeyCode::Down) {
        menu.selected_save_index =
            (menu.selected_save_index + 1).min(menu.save_slots.len().saturating_sub(1));
        menu.pending_delete_save_index = None;
        menu.delete_save_error = None;
    }
    if is_key_pressed(KeyCode::Enter) {
        return menu
            .save_slots
            .get(menu.selected_save_index)
            .map(|slot| TitleAction::LoadGame {
                path: slot.path.clone(),
            });
    }

    None
}

fn handle_title_delete_save_click(menu: &mut TitleMenu) {
    let selected_index = menu.selected_save_index;
    if selected_index >= menu.save_slots.len() {
        return;
    }

    if menu.pending_delete_save_index != Some(selected_index) {
        menu.pending_delete_save_index = Some(selected_index);
        menu.delete_save_error = None;
        return;
    }

    let deleted = delete_title_save_at(menu, selected_index);
    if !deleted {
        menu.pending_delete_save_index = Some(selected_index);
    }
}

fn delete_title_save_at(menu: &mut TitleMenu, save_index: usize) -> bool {
    let Some(slot) = menu.save_slots.get(save_index) else {
        return false;
    };
    let path = slot.path.clone();
    if let Err(error) = delete_save_file(&path) {
        menu.delete_save_error = Some(error);
        return false;
    }

    let previous_index = menu.selected_save_index;
    menu.save_slots = title_save_slots();
    menu.selected_save_index =
        selected_save_index_after_delete(previous_index, save_index, menu.save_slots.len());
    menu.pending_delete_save_index = None;
    menu.delete_save_error = None;
    true
}

fn delete_save_file(path: &Path) -> Result<(), String> {
    fs::remove_file(path).map_err(|error| format!("Could not delete save: {error}"))
}

fn selected_save_index_after_delete(
    previous_index: usize,
    deleted_index: usize,
    remaining_count: usize,
) -> usize {
    if remaining_count == 0 {
        0
    } else if previous_index > deleted_index {
        previous_index - 1
    } else {
        previous_index.min(remaining_count - 1)
    }
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
    if is_key_pressed(KeyCode::R) {
        menu.new_game_seed_text = new_world_seed().to_string();
    }
    if is_key_pressed(KeyCode::Left) {
        cycle_title_ship(menu, -1);
    }
    if is_key_pressed(KeyCode::Right) {
        cycle_title_ship(menu, 1);
    }
    if is_key_pressed(KeyCode::Up) {
        cycle_title_ship(menu, -(TITLE_SHIP_GRID_COLUMNS as i32));
    }
    if is_key_pressed(KeyCode::Down) {
        cycle_title_ship(menu, TITLE_SHIP_GRID_COLUMNS as i32);
    }

    let parsed_seed = parse_title_seed(&menu.new_game_seed_text);
    if is_key_pressed(KeyCode::Enter) {
        if let (Some(seed), Some(ship)) = (parsed_seed, selected_title_ship(menu)) {
            return Some(TitleAction::NewGame {
                seed,
                ship_id: ship.id.clone(),
                pack_options: selected_title_pack_options(&menu.content_packs),
            });
        }
    }

    None
}

fn selected_title_ship(menu: &TitleMenu) -> Option<&TitleShip> {
    menu.ships.get(menu.selected_ship_index)
}

fn cycle_title_ship(menu: &mut TitleMenu, direction: i32) {
    menu.selected_ship_index = cycled_index(menu.selected_ship_index, menu.ships.len(), direction);
}

fn cycled_index(current: usize, len: usize, direction: i32) -> usize {
    if len == 0 {
        return 0;
    }
    let current = current.min(len - 1) as i64;
    (current + i64::from(direction)).rem_euclid(len as i64) as usize
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
    None
}

fn mouse_vec2() -> Vec2 {
    vec2(mouse_position().0, mouse_position().1)
}

fn draw_title_menu(menu: &mut TitleMenu, background: &UniverseBackground) {
    draw_title_background(background);
    match menu.view {
        TitleView::Main => draw_title_main_egui(menu),
        TitleView::NewGame
        | TitleView::LoadGame
        | TitleView::ContentPacks
        | TitleView::Settings => draw_title_submenu_egui(menu),
    }
}

fn draw_title_main_egui(menu: &mut TitleMenu) {
    let mut actions = Vec::new();
    egui_macroquad::ui(|ctx| {
        if menu.branding_egui_texture.is_none() {
            if let Some(image) = &menu.branding_egui_image {
                menu.branding_egui_texture = Some(ctx.load_texture(
                    "some-frontier-title-logo",
                    image.clone(),
                    egui::TextureOptions::LINEAR,
                ));
            }
        }
        let logo_aspect_ratio = menu
            .branding_egui_image
            .as_ref()
            .map(|image| image.size[0] as f32 / image.size[1].max(1) as f32)
            .unwrap_or(1.0);
        actions = egui_title::draw_main(
            ctx,
            [screen_width(), screen_height()],
            egui_title::TitleMainView {
                logo: menu
                    .branding_egui_texture
                    .as_ref()
                    .map(egui::TextureHandle::id),
                logo_aspect_ratio,
                load_enabled: !menu.save_slots.is_empty(),
                fast_start: fast_start_enabled(),
            },
        );
    });
    for action in actions {
        match action {
            egui_title::TitleMainAction::NewGame => menu.view = TitleView::NewGame,
            egui_title::TitleMainAction::LoadGame if !menu.save_slots.is_empty() => {
                menu.view = TitleView::LoadGame;
            }
            egui_title::TitleMainAction::LoadGame => {}
            egui_title::TitleMainAction::ContentPacks => menu.view = TitleView::ContentPacks,
            egui_title::TitleMainAction::Settings => menu.view = TitleView::Settings,
            egui_title::TitleMainAction::Quit => {
                menu.pending_action = Some(TitleAction::QuitDesktop);
            }
        }
    }
    egui_macroquad::draw();
}

fn draw_title_submenu_egui(menu: &mut TitleMenu) {
    let mut actions = Vec::new();
    egui_macroquad::ui(|ctx| {
        ensure_title_ship_egui_textures(menu, ctx);
        let view = title_submenu_view(menu);
        actions = egui_title::draw(ctx, [screen_width(), screen_height()], &view);
    });
    apply_title_submenu_actions(menu, actions);
    egui_macroquad::draw();
}

fn ensure_title_ship_egui_textures(menu: &mut TitleMenu, ctx: &egui::Context) {
    for (ship_id, preview) in &mut menu.ship_textures {
        if preview.egui_texture.is_none() {
            preview.egui_texture = Some(ctx.load_texture(
                format!("title-ship-{ship_id}"),
                preview.egui_image.clone(),
                egui::TextureOptions::LINEAR,
            ));
        }
    }
}

fn title_submenu_view(menu: &TitleMenu) -> egui_title::TitleSubmenuView {
    let kind = match menu.view {
        TitleView::NewGame => egui_title::SubmenuKind::NewGame,
        TitleView::LoadGame => egui_title::SubmenuKind::LoadGame,
        TitleView::ContentPacks => egui_title::SubmenuKind::ContentPacks,
        TitleView::Settings => egui_title::SubmenuKind::Settings,
        TitleView::Main => unreachable!("main title view does not use an egui submenu"),
    };
    let ships = menu
        .ships
        .iter()
        .map(|ship| egui_title::ShipChoice {
            name: ship.name.clone(),
            pack: ship.id.split(':').next().unwrap_or("content").to_string(),
            hull: ship.hull_capacity,
            shields: ship.shield_capacity,
            weapon_banks: ship.weapon_banks,
            texture: menu
                .ship_textures
                .get(&ship.id)
                .and_then(|preview| preview.egui_texture.as_ref())
                .map(egui::TextureHandle::id),
        })
        .collect();
    let saves = menu
        .save_slots
        .iter()
        .map(|save| egui_title::SaveChoice {
            label: save.label.clone(),
            system: save.current_system_id.clone(),
            last_played: format_last_played(save.modified_unix_seconds),
            seed: save.world_seed,
            elapsed_days: save.world_elapsed_days,
            legacy: save.is_legacy,
        })
        .collect();
    let packs = menu
        .content_packs
        .iter()
        .map(|pack| egui_title::PackChoice {
            name: pack.name.clone(),
            id: pack.id.clone(),
            version: pack.version.clone(),
            description: pack
                .description
                .clone()
                .unwrap_or_else(|| "No description provided.".to_string()),
            options: pack
                .options
                .iter()
                .map(|option| {
                    let interactive = title_pack_option_is_interactive(option);
                    egui_title::PackOptionView {
                        label: option.label.clone(),
                        description: option.description.clone().unwrap_or_else(|| {
                            "Saved with new games; application pending.".to_string()
                        }),
                        value: if interactive {
                            option.current_value.clone()
                        } else {
                            format!("{} pending", option.default_value)
                        },
                        interactive,
                    }
                })
                .collect(),
        })
        .collect();
    let settings = SettingsCategory::ALL
        .iter()
        .copied()
        .map(|category| {
            let (label, value, note) = title_selected_setting_text(&menu.settings, category);
            egui_title::SettingsChoice {
                category: category.label().to_string(),
                label: label.to_string(),
                value,
                note: note.to_string(),
            }
        })
        .collect();
    let selected_setting = SettingsCategory::ALL
        .iter()
        .position(|category| *category == menu.selected_settings_category)
        .unwrap_or(0);

    egui_title::TitleSubmenuView {
        kind,
        seed: menu.new_game_seed_text.clone(),
        seed_valid: parse_title_seed(&menu.new_game_seed_text).is_some(),
        ships,
        selected_ship: menu.selected_ship_index,
        saves,
        selected_save: menu.selected_save_index,
        confirming_delete: menu.pending_delete_save_index == Some(menu.selected_save_index),
        delete_error: menu.delete_save_error.clone(),
        packs,
        selected_pack: menu.selected_pack_index,
        settings,
        selected_setting,
    }
}

fn apply_title_submenu_actions(menu: &mut TitleMenu, actions: Vec<egui_title::TitleSubmenuAction>) {
    for action in actions {
        match action {
            egui_title::TitleSubmenuAction::Back => menu.view = TitleView::Main,
            egui_title::TitleSubmenuAction::SetSeed(seed) => menu.new_game_seed_text = seed,
            egui_title::TitleSubmenuAction::RandomizeSeed => {
                menu.new_game_seed_text = new_world_seed().to_string();
            }
            egui_title::TitleSubmenuAction::SelectShip(index) => {
                menu.selected_ship_index = index.min(menu.ships.len().saturating_sub(1));
            }
            egui_title::TitleSubmenuAction::StartGame => {
                if let (Some(seed), Some(ship)) = (
                    parse_title_seed(&menu.new_game_seed_text),
                    selected_title_ship(menu),
                ) {
                    menu.pending_action = Some(TitleAction::NewGame {
                        seed,
                        ship_id: ship.id.clone(),
                        pack_options: selected_title_pack_options(&menu.content_packs),
                    });
                }
            }
            egui_title::TitleSubmenuAction::SelectSave(index) => {
                menu.selected_save_index = index.min(menu.save_slots.len().saturating_sub(1));
                menu.pending_delete_save_index = None;
                menu.delete_save_error = None;
            }
            egui_title::TitleSubmenuAction::LoadSave => {
                if let Some(save) = menu.save_slots.get(menu.selected_save_index) {
                    menu.pending_action = Some(TitleAction::LoadGame {
                        path: save.path.clone(),
                    });
                }
            }
            egui_title::TitleSubmenuAction::DeleteSave => handle_title_delete_save_click(menu),
            egui_title::TitleSubmenuAction::SelectPack(index) => {
                menu.selected_pack_index = index.min(menu.content_packs.len().saturating_sub(1));
            }
            egui_title::TitleSubmenuAction::CyclePackOption(index) => {
                if let Some(pack) = menu.content_packs.get_mut(menu.selected_pack_index) {
                    cycle_title_pack_option(pack, index);
                }
            }
            egui_title::TitleSubmenuAction::SelectSetting(index) => {
                if let Some(category) = SettingsCategory::ALL.get(index) {
                    menu.selected_settings_category = *category;
                }
            }
            egui_title::TitleSubmenuAction::AdjustSetting(direction) => {
                menu.settings
                    .adjust_selected(menu.selected_settings_category, direction);
                save_app_settings(&menu.settings);
            }
        }
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

fn draw_panel_corner_art(rect: Rect, texture: Option<&Texture2D>) {
    let Some(texture) = texture else {
        return;
    };
    let size = (rect.w.min(rect.h) * 0.42).clamp(112.0, 260.0);
    draw_texture_ex(
        texture,
        rect.x - size * 0.14,
        rect.y - size * 0.12,
        Color::new(1.0, 1.0, 1.0, 0.9),
        DrawTextureParams {
            dest_size: Some(vec2(size, size)),
            ..Default::default()
        },
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
    if handle_debug_console_toggle(game) || game.debug_console.open {
        handle_debug_console_input(game);
        return;
    }
    if game.escape_dialog_open {
        handle_escape_dialog_input(game);
        return;
    }

    update_scene_transition(game, dt);
    update_pending_warp(game, dt);
    advance_world_time_and_planets(game, dt);
    update_active_research(game, dt);
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
        game.audio.play(if game.map_open {
            audio::AudioCue::Select
        } else {
            audio::AudioCue::Back
        });
        if game.map_open {
            game.starmap_pan = Vec2::ZERO;
            game.starmap_drag_previous_mouse = None;
            if game.starmap_selected_planet.is_none() {
                game.starmap_selected_planet = game.destination_planet;
            }
            game.inventory_open = false;
            game.research_open = false;
            game.upgrades_open = false;
            game.content_open = false;
            game.contracts_open = false;
            game.fleet_open = false;
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
    if !game.map_open && is_key_pressed(KeyCode::F) {
        game.fleet_open = !game.fleet_open;
        if game.fleet_open {
            game.sync_active_owned_ship();
            game.inventory_open = false;
            game.research_open = false;
            game.upgrades_open = false;
            game.content_open = false;
            game.contracts_open = false;
            game.selected_owned_ship_index = game
                .owned_ships
                .iter()
                .position(|ship| ship.instance_id == game.active_owned_ship_id)
                .unwrap_or(0);
        }
        game.audio.play(if game.fleet_open {
            audio::AudioCue::Select
        } else {
            audio::AudioCue::Back
        });
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
        handle_inventory_shortcut(game);
    }
    if is_key_pressed(KeyCode::K) {
        game.research_open = !game.research_open;
        game.audio.play(if game.research_open {
            audio::AudioCue::Select
        } else {
            audio::AudioCue::Back
        });
        if game.research_open {
            game.map_open = false;
            game.inventory_open = false;
            game.upgrades_open = false;
            game.content_open = false;
            game.contracts_open = false;
            game.fleet_open = false;
            game.selected_planet = None;
            game.selected_station = None;
            game.selected_npc_ship = None;
            game.selected_station_service = None;
        }
    }
    if is_key_pressed(KeyCode::C) {
        game.content_open = !game.content_open;
        game.audio.play(if game.content_open {
            audio::AudioCue::Select
        } else {
            audio::AudioCue::Back
        });
        if game.content_open {
            game.map_open = false;
            game.inventory_open = false;
            game.research_open = false;
            game.upgrades_open = false;
            game.contracts_open = false;
            game.fleet_open = false;
            clear_unpinned_inspection_selection(game, egui_pin_snapshot());
        }
    }
    if is_key_pressed(KeyCode::J) {
        game.contracts_open = !game.contracts_open;
        game.audio.play(if game.contracts_open {
            audio::AudioCue::Select
        } else {
            audio::AudioCue::Back
        });
        if game.contracts_open {
            game.map_open = false;
            game.inventory_open = false;
            game.research_open = false;
            game.upgrades_open = false;
            game.content_open = false;
            game.fleet_open = false;
            game.selected_contract_index = None;
            game.contract_menu_scroll = 0.0;
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
        && !game.research_open
        && !game.upgrades_open
        && !game.content_open
        && !game.contracts_open
        && !game.fleet_open
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
    update_inactive_owned_ship_systems(game, dt);
    update_npc_ships(game, dt);
    update_hostile_npc_pressure(game, dt);
    update_weapon_systems(game, dt);
    remove_destroyed_npc_ships(game);

    let mut egui_pointer_captured = false;
    egui_macroquad::cfg(|ctx| egui_pointer_captured = ctx.wants_pointer_input());
    let wheel = mouse_wheel().1;
    if game.map_open {
        update_starmap_view_input(game, dt, wheel);
    }
    if wheel != 0.0
        && !game.inventory_open
        && !game.map_open
        && !game.research_open
        && !game.upgrades_open
        && !game.content_open
        && !game.contracts_open
        && !game.fleet_open
        && !egui_pointer_captured
    {
        adjust_camera_zoom(game, wheel.signum() as i32);
    }

    let mut click_handled = egui_pointer_captured
        && (is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_pressed(MouseButton::Right));
    if game.research_open {
        click_handled = is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_pressed(MouseButton::Right);
    }

    if game.upgrades_open {
        click_handled = is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_pressed(MouseButton::Right);
    }

    if game.content_open {
        click_handled = is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_pressed(MouseButton::Right);
    }

    if game.contracts_open {
        click_handled = is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_pressed(MouseButton::Right);
    }

    if game.fleet_open {
        if is_key_pressed(KeyCode::Up) {
            game.selected_owned_ship_index = game.selected_owned_ship_index.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Down) && !game.owned_ships.is_empty() {
            game.selected_owned_ship_index =
                (game.selected_owned_ship_index + 1).min(game.owned_ships.len() - 1);
        }
        if is_key_pressed(KeyCode::Enter) {
            switch_active_owned_ship(game, game.selected_owned_ship_index);
        }
        click_handled = is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_pressed(MouseButton::Right);
    }

    if !game.map_open
        && !game.research_open
        && !game.upgrades_open
        && !game.content_open
        && !game.contracts_open
        && !game.fleet_open
        && !egui_pointer_captured
    {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        if handle_context_action_input(game, mouse) {
            click_handled = true;
        }
    }

    if game.inventory_open
        && !game.map_open
        && !game.research_open
        && !game.upgrades_open
        && !game.content_open
        && !game.contracts_open
        && !game.fleet_open
    {
        click_handled = is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_pressed(MouseButton::Right);
    }

    if game.map_open
        && !egui_pointer_captured
        && game.starmap_move_previous_mouse.is_none()
        && game.starmap_resize_previous_mouse.is_none()
        && is_mouse_button_pressed(MouseButton::Left)
    {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        if let Some(system_id) = clicked_known_system_id(mouse, game) {
            if system_id != game.current_system_id && game.scene_transition.is_none() {
                start_player_warp_charge(game, system_id);
            }
            click_handled = true;
        } else if let Some(planet_index) = clicked_starmap_planet_index(mouse, game) {
            game.starmap_selected_planet = Some(planet_index);
            game.audio.play(audio::AudioCue::Select);
            click_handled = true;
        }
    }

    if is_mouse_button_pressed(MouseButton::Left)
        && !click_handled
        && !game.map_open
        && !game.research_open
        && !game.upgrades_open
        && !game.content_open
        && !game.contracts_open
        && !game.fleet_open
    {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        if clicked_player_ship(mouse) {
            game.selected_planet = None;
            game.selected_station = None;
            game.selected_npc_ship = None;
            game.selected_station_service = None;
            game.inventory_open = true;
            game.audio.play(audio::AudioCue::Select);
        } else {
            select_clicked_destination(game, mouse);
            identify_selected_npc_ship(game);
            if game.selected_planet.is_some()
                || game.selected_station.is_some()
                || game.selected_npc_ship.is_some()
            {
                game.audio.play(audio::AudioCue::Select);
            }
        }
    }

    if game.fleet_open {
        game.ship
            .systems
            .energy
            .restore(ship_energy_recharge(&game.ship, &game.installed_power_modules) * dt);
    } else if game.orbiting_planet.is_some() {
        update_ship_orbit(game);
    } else if game.follow_target.is_some() {
        if movement_input_pressed() {
            stop_following(game, "Manual flight input cancelled follow");
            let energy_recharge = ship_energy_recharge(&game.ship, &game.installed_power_modules);
            update_ship(&mut game.ship, dt, energy_recharge);
        } else {
            update_ship_follow(game, dt);
        }
    } else if game.approach_target.is_some() {
        if movement_input_pressed() {
            if game.navigation_route_active {
                pause_navigation_route(game, Some("Manual flight input paused itinerary"));
            } else {
                game.approach_target = None;
            }
            let energy_recharge = ship_energy_recharge(&game.ship, &game.installed_power_modules);
            update_ship(&mut game.ship, dt, energy_recharge);
        } else {
            update_ship_approach(game, dt);
        }
    } else {
        let energy_recharge = ship_energy_recharge(&game.ship, &game.installed_power_modules);
        update_ship(&mut game.ship, dt, energy_recharge);
    }
    update_ship_collisions(game, dt);
    if game_save_snapshot(game) != save_snapshot {
        game.save_dirty = true;
    }
}

fn handle_inventory_shortcut(game: &mut GameState) {
    if game.context_action_menu.take().is_some() {
        game.audio.play(audio::AudioCue::Back);
        return;
    }

    let pins = egui_pin_snapshot();
    game.map_open = false;
    game.research_open = false;
    game.upgrades_open = false;
    game.content_open = false;
    game.contracts_open = false;
    game.fleet_open = false;
    game.inventory_open = !game.inventory_open;
    clear_unpinned_inspection_selection(game, pins);
    game.audio.play(if game.inventory_open {
        audio::AudioCue::Select
    } else {
        audio::AudioCue::Back
    });
}

fn clear_unpinned_inspection_selection(game: &mut GameState, pins: EguiPinSnapshot) {
    if !pins.planet {
        game.selected_planet = None;
    }
    if !pins.station {
        game.selected_station = None;
        game.selected_station_service = None;
    }
    if !pins.contact {
        game.selected_npc_ship = None;
    }
}

fn handle_escape_pressed(game: &mut GameState) {
    if close_topmost_gameplay_overlay(game) {
        game.audio.play(audio::AudioCue::Back);
        return;
    }

    game.escape_dialog_open = true;
    game.audio.play(audio::AudioCue::Select);
}

fn movement_input_pressed() -> bool {
    is_key_down(KeyCode::W)
        || is_key_down(KeyCode::S)
        || is_key_down(KeyCode::A)
        || is_key_down(KeyCode::D)
        || is_key_down(KeyCode::Up)
        || is_key_down(KeyCode::Down)
        || is_key_down(KeyCode::Left)
        || is_key_down(KeyCode::Right)
}

fn context_action_entries(game: &GameState, target: ContextTarget) -> Vec<ContextMenuEntry> {
    match target {
        ContextTarget::Planet(index) => {
            let Some(planet) = game.planets.get(index) else {
                return Vec::new();
            };
            let in_range = planet_in_interaction_range(&game.ship, planet);
            let has_drone = core_item(&game.content_registry, "survey_drone")
                .is_some_and(|item| game.inventory.count(&item) > 0)
                || core_item(&game.content_registry, "improved_survey_drone")
                    .is_some_and(|item| game.inventory.count(&item) > 0);
            let surveyed = planet_has_composition_scan(planet);
            vec![
                ContextMenuEntry {
                    action: ContextAction::Inspect,
                    label: "Inspect",
                    enabled: true,
                    detail: "Open planet survey record",
                },
                ContextMenuEntry {
                    action: ContextAction::Approach,
                    label: "Approach",
                    enabled: true,
                    detail: "Set navigation target",
                },
                ContextMenuEntry {
                    action: ContextAction::Orbit,
                    label: "Orbit",
                    enabled: in_range
                        && game.orbiting_planet != Some(index)
                        && game.approach_target != Some(ApproachTarget::PlanetOrbit(index)),
                    detail: if game.approach_target == Some(ApproachTarget::PlanetOrbit(index)) {
                        "Flying to orbital position"
                    } else if in_range {
                        "Enter stable orbit"
                    } else {
                        "Approach planet"
                    },
                },
                ContextMenuEntry {
                    action: ContextAction::Scan,
                    label: "Scan",
                    enabled: in_range && has_drone && planet.scan_level < MAX_SCAN_LEVEL,
                    detail: if !in_range {
                        "Approach planet"
                    } else if !has_drone {
                        "Requires survey drone"
                    } else if planet.scan_level >= MAX_SCAN_LEVEL {
                        "Survey complete"
                    } else {
                        "Launch survey drone"
                    },
                },
                ContextMenuEntry {
                    action: ContextAction::Mine,
                    label: "Mine",
                    enabled: in_range && surveyed,
                    detail: if !in_range {
                        "Approach planet"
                    } else if surveyed {
                        "Open mining controls"
                    } else {
                        "Complete composition scan"
                    },
                },
            ]
        }
        ContextTarget::Station(index) => {
            let Some(station) = game.stations.get(index) else {
                return Vec::new();
            };
            let in_range = station_in_interaction_range(&game.ship, station);
            let trade_service = station.services.iter().position(|service| {
                !service.trade.is_empty() && station_service_is_available(game, station, service)
            });
            let contract_service = station.services.iter().position(|service| {
                !service.contracts.is_empty()
                    && station_service_is_available(game, station, service)
            });
            let repair_service = station.services.iter().position(|service| {
                service.kind == "garage" && station_service_is_available(game, station, service)
            });
            vec![
                ContextMenuEntry {
                    action: ContextAction::Inspect,
                    label: "Inspect",
                    enabled: in_range,
                    detail: if in_range {
                        "Open station services"
                    } else {
                        "Approach dock range"
                    },
                },
                ContextMenuEntry {
                    action: ContextAction::Approach,
                    label: "Approach",
                    enabled: true,
                    detail: "Approach dock range",
                },
                ContextMenuEntry {
                    action: ContextAction::Dock,
                    label: "Dock",
                    enabled: in_range,
                    detail: if in_range {
                        "Open station services"
                    } else {
                        "Approach dock range"
                    },
                },
                ContextMenuEntry {
                    action: ContextAction::Trade,
                    label: "Trade",
                    enabled: in_range && trade_service.is_some(),
                    detail: if !in_range {
                        "Approach dock range"
                    } else if trade_service.is_some() {
                        "Open trade service"
                    } else {
                        "No available trade service"
                    },
                },
                ContextMenuEntry {
                    action: ContextAction::Contracts,
                    label: "Contracts",
                    enabled: in_range && contract_service.is_some(),
                    detail: if !in_range {
                        "Approach dock range"
                    } else if contract_service.is_some() {
                        "Open contracts service"
                    } else {
                        "No available contracts"
                    },
                },
                ContextMenuEntry {
                    action: ContextAction::Repair,
                    label: "Repair",
                    enabled: in_range && repair_service.is_some(),
                    detail: if !in_range {
                        "Approach dock range"
                    } else if repair_service.is_some() {
                        "Open repair service"
                    } else {
                        "No available repair service"
                    },
                },
            ]
        }
        ContextTarget::NpcShip(index) => {
            let Some(npc_ship) = game.npc_ships.get(index) else {
                return Vec::new();
            };
            let following = game.follow_target == Some(FollowTarget::NpcShip(index));
            let in_range = npc_ship_in_interaction_range(&game.ship, npc_ship);
            let can_trade = npc_ship.identified
                && in_range
                && !npc_ship_is_hostile(&game.content_registry, npc_ship)
                && npc_ship_can_trade(&game.content_registry, npc_ship)
                && npc_ship.cargo_defaults.iter().any(|stack| stack.count > 0);
            vec![
                ContextMenuEntry {
                    action: if following {
                        ContextAction::StopFollowing
                    } else {
                        ContextAction::Follow
                    },
                    label: if following {
                        "Stop following"
                    } else {
                        "Follow"
                    },
                    enabled: true,
                    detail: if following {
                        "Manual control"
                    } else {
                        "Match pace"
                    },
                },
                ContextMenuEntry {
                    action: ContextAction::Hail,
                    label: "Hail",
                    enabled: npc_ship.identified
                        && in_range
                        && !npc_ship_is_hostile(&game.content_registry, npc_ship),
                    detail: if npc_ship.identified && in_range {
                        "Request rumor"
                    } else {
                        "Identify and approach"
                    },
                },
                ContextMenuEntry {
                    action: ContextAction::Trade,
                    label: "Trade",
                    enabled: can_trade,
                    detail: if can_trade {
                        "Open cargo market"
                    } else {
                        "Identify and approach"
                    },
                },
                ContextMenuEntry {
                    action: ContextAction::Inspect,
                    label: "Inspect",
                    enabled: in_range,
                    detail: if in_range {
                        "Open contact detail"
                    } else {
                        "Approach contact"
                    },
                },
                ContextMenuEntry {
                    action: ContextAction::Approach,
                    label: "Approach",
                    enabled: true,
                    detail: "Approach to interact",
                },
            ]
        }
    }
}

fn context_menu_view(game: &GameState) -> Option<egui_context_menu::ContextMenuView> {
    let menu = game.context_action_menu.as_ref()?;
    let (heading, entries) = match &menu.content {
        ContextMenuContent::Actions(target) => (
            None,
            context_action_entries(game, *target)
                .into_iter()
                .map(|entry| egui_context_menu::ContextMenuEntryView {
                    label: entry.label.to_string(),
                    detail: entry.detail.to_string(),
                    enabled: entry.enabled,
                })
                .collect(),
        ),
        ContextMenuContent::ChooseTarget(targets) => (
            Some("Choose target".to_string()),
            targets
                .iter()
                .map(|target| {
                    let (label, detail) = context_target_label(game, *target);
                    egui_context_menu::ContextMenuEntryView {
                        label,
                        detail,
                        enabled: true,
                    }
                })
                .collect(),
        ),
    };
    Some(egui_context_menu::ContextMenuView {
        position: [menu.position.x, menu.position.y],
        heading,
        entries,
    })
}

fn context_target_label(game: &GameState, target: ContextTarget) -> (String, String) {
    match target {
        ContextTarget::Planet(index) => game.planets.get(index).map_or_else(
            || ("Unavailable planet".to_string(), "Planet".to_string()),
            |planet| {
                (
                    if planet_has_surface_scan(planet) {
                        planet.info.classification.clone()
                    } else {
                        "Unscanned body".to_string()
                    },
                    format!("Planet · {}", planet.id),
                )
            },
        ),
        ContextTarget::Station(index) => game.stations.get(index).map_or_else(
            || ("Unavailable station".to_string(), "Station".to_string()),
            |station| (station.name.clone(), format!("Station · {}", station.id)),
        ),
        ContextTarget::NpcShip(index) => game.npc_ships.get(index).map_or_else(
            || ("Unavailable contact".to_string(), "Contact".to_string()),
            |npc_ship| {
                (
                    if npc_ship.identified {
                        npc_ship.name.clone()
                    } else {
                        "Unidentified contact".to_string()
                    },
                    format!("Contact · {}", npc_ship.id),
                )
            },
        ),
    }
}

fn apply_context_menu_outcome(
    game: &mut GameState,
    outcome: Option<egui_context_menu::ContextMenuOutcome>,
) {
    let Some(outcome) = outcome else {
        return;
    };
    let Some(menu) = game.context_action_menu.clone() else {
        return;
    };

    match outcome {
        egui_context_menu::ContextMenuOutcome::Choose(index) => match menu.content {
            ContextMenuContent::Actions(target) => {
                let selected = context_action_entries(game, target)
                    .get(index)
                    .map(|entry| (entry.action, entry.enabled));
                game.context_action_menu = None;
                if let Some((action, true)) = selected {
                    execute_context_action(game, target, action);
                    game.audio.play(audio::AudioCue::Confirm);
                } else {
                    game.audio.play(audio::AudioCue::Warning);
                }
            }
            ContextMenuContent::ChooseTarget(targets) => {
                if let Some(target) = targets.get(index).copied() {
                    select_context_target(game, target);
                    game.context_action_menu = Some(ContextActionMenu {
                        content: ContextMenuContent::Actions(target),
                        position: menu.position,
                    });
                    game.audio.play(audio::AudioCue::Select);
                } else {
                    game.context_action_menu = None;
                    game.audio.play(audio::AudioCue::Warning);
                }
            }
        },
        egui_context_menu::ContextMenuOutcome::Dismiss => {
            game.context_action_menu = None;
        }
    }
}

fn handle_context_action_input(game: &mut GameState, mouse: Vec2) -> bool {
    if game.context_action_menu.is_some() {
        if is_mouse_button_pressed(MouseButton::Left) {
            // Egui owns hit-testing and click-away dismissal for an open menu.
            return true;
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            if !open_context_menu_at(game, mouse) {
                game.context_action_menu = None;
            }
            return true;
        }
        return false;
    }

    if !is_mouse_button_pressed(MouseButton::Right) {
        return false;
    }
    if !open_context_menu_at(game, mouse) {
        return false;
    }
    game.audio.play(audio::AudioCue::Select);
    true
}

fn open_context_menu_at(game: &mut GameState, mouse: Vec2) -> bool {
    let Some(content) = context_menu_content_at(game, mouse) else {
        return false;
    };
    if let ContextMenuContent::Actions(target) = &content {
        select_context_target(game, *target);
    }
    game.inventory_open = false;
    game.context_action_menu = Some(ContextActionMenu {
        content,
        position: mouse,
    });
    true
}

fn context_menu_content_at(game: &GameState, mouse: Vec2) -> Option<ContextMenuContent> {
    context_menu_content_for_candidates(context_target_candidates(game, mouse))
}

fn context_menu_content_for_candidates(
    mut candidates: Vec<ContextTargetCandidate>,
) -> Option<ContextMenuContent> {
    candidates.sort_by(|left, right| left.cursor_distance.total_cmp(&right.cursor_distance));
    let nearest = candidates.first()?;
    let ambiguous = candidates
        .iter()
        .take_while(|candidate| {
            candidate.cursor_distance - nearest.cursor_distance <= CONTEXT_TARGET_AMBIGUITY_PIXELS
        })
        .map(|candidate| candidate.target)
        .collect::<Vec<_>>();
    if ambiguous.len() > 1 {
        Some(ContextMenuContent::ChooseTarget(ambiguous))
    } else {
        Some(ContextMenuContent::Actions(nearest.target))
    }
}

fn context_target_candidates(game: &GameState, mouse: Vec2) -> Vec<ContextTargetCandidate> {
    let center = vec2(screen_width() * 0.5, screen_height() * 0.5);
    let mut candidates = Vec::new();

    for (index, planet) in game.planets.iter().enumerate() {
        if !planet_is_in_system(planet, &game.current_system_id) {
            continue;
        }
        let screen_position =
            world_to_screen(planet.position, center, &game.ship, game.camera_zoom);
        let cursor_distance = mouse.distance(screen_position);
        if cursor_distance <= planet_interaction_radius(planet) * game.camera_zoom {
            candidates.push(ContextTargetCandidate {
                target: ContextTarget::Planet(index),
                cursor_distance,
            });
        }
    }
    for (index, station) in game.stations.iter().enumerate() {
        if !station_is_in_system(station, &game.current_system_id) {
            continue;
        }
        let screen_position =
            world_to_screen(station.position, center, &game.ship, game.camera_zoom);
        let cursor_distance = mouse.distance(screen_position);
        if cursor_distance <= station_interaction_radius(station) * game.camera_zoom {
            candidates.push(ContextTargetCandidate {
                target: ContextTarget::Station(index),
                cursor_distance,
            });
        }
    }
    for (index, npc_ship) in game.npc_ships.iter().enumerate() {
        if !npc_ship_is_in_system(npc_ship, &game.current_system_id) {
            continue;
        }
        let screen_position =
            world_to_screen(npc_ship.position, center, &game.ship, game.camera_zoom);
        let cursor_distance = mouse.distance(screen_position);
        if cursor_distance <= npc_ship_click_radius(npc_ship, game.camera_zoom) {
            candidates.push(ContextTargetCandidate {
                target: ContextTarget::NpcShip(index),
                cursor_distance,
            });
        }
    }

    candidates
}

fn select_context_target(game: &mut GameState, target: ContextTarget) {
    match target {
        ContextTarget::Planet(index) => {
            game.selected_planet = Some(index);
            game.selected_station = None;
            game.selected_npc_ship = None;
            game.selected_station_service = None;
        }
        ContextTarget::Station(index) => {
            game.selected_planet = None;
            game.selected_station = Some(index);
            game.selected_npc_ship = None;
            game.selected_station_service = None;
        }
        ContextTarget::NpcShip(index) => select_npc_ship_for_context(game, index),
    }
}

fn select_npc_ship_for_context(game: &mut GameState, npc_index: usize) {
    game.selected_planet = None;
    game.selected_station = None;
    game.selected_npc_ship = Some(npc_index);
    game.selected_station_service = None;
    identify_selected_npc_ship(game);
}

fn execute_context_action(game: &mut GameState, target: ContextTarget, action: ContextAction) {
    match (target, action) {
        (ContextTarget::Planet(_), ContextAction::Inspect) => {
            select_context_target(game, target);
            game.inventory_open = true;
        }
        (ContextTarget::Planet(index), ContextAction::Approach) => {
            select_context_target(game, target);
            set_destination_planet(game, Some(index));
            begin_approach(game, ApproachTarget::Planet(index));
            push_operation_feedback(game, "Route", "Planet set as navigation target".to_string());
        }
        (ContextTarget::Planet(index), ContextAction::Orbit) => {
            select_context_target(game, target);
            enter_planet_orbit(game, index);
        }
        (ContextTarget::Planet(index), ContextAction::Scan) => {
            select_context_target(game, target);
            launch_planet_scan(game, index);
        }
        (ContextTarget::Planet(index), ContextAction::Mine) => {
            let can_mine = game.planets.get(index).is_some_and(|planet| {
                planet_in_interaction_range(&game.ship, planet)
                    && planet_has_composition_scan(planet)
            });
            if can_mine {
                select_context_target(game, target);
                game.inventory_open = true;
            }
        }
        (ContextTarget::Station(index), ContextAction::Inspect) => {
            if game
                .stations
                .get(index)
                .is_some_and(|station| station_in_interaction_range(&game.ship, station))
            {
                select_context_target(game, target);
                game.inventory_open = true;
            }
        }
        (ContextTarget::Station(index), ContextAction::Dock) => {
            select_context_target(game, target);
            if station_in_interaction_range(&game.ship, &game.stations[index]) {
                game.inventory_open = true;
            } else {
                push_operation_feedback(game, "Station", "Approach dock range to dock".to_string());
            }
        }
        (
            ContextTarget::Station(index),
            action @ (ContextAction::Trade | ContextAction::Contracts | ContextAction::Repair),
        ) => {
            let service_index = game.stations.get(index).and_then(|station| {
                station.services.iter().position(|service| {
                    let matches = match action {
                        ContextAction::Trade => !service.trade.is_empty(),
                        ContextAction::Contracts => !service.contracts.is_empty(),
                        ContextAction::Repair => service.kind == "garage",
                        _ => false,
                    };
                    matches && station_service_is_available(game, station, service)
                })
            });
            select_context_target(game, target);
            if let Some(service_index) = service_index {
                select_station_service(game, index, service_index);
                if station_in_interaction_range(&game.ship, &game.stations[index]) {
                    game.inventory_open = true;
                } else {
                    push_operation_feedback(
                        game,
                        "Station",
                        "Approach dock range to use service".to_string(),
                    );
                }
            }
        }
        (ContextTarget::Station(index), ContextAction::Approach) => {
            select_context_target(game, target);
            begin_approach(game, ApproachTarget::Station(index));
            push_operation_feedback(game, "Station", "Autopilot approaching station".to_string());
        }
        (ContextTarget::NpcShip(index), ContextAction::Follow) => {
            if game.npc_ships.get(index).is_some() {
                game.follow_target = Some(FollowTarget::NpcShip(index));
                select_npc_ship_for_context(game, index);
                push_operation_feedback(game, "Follow", "Matching NPC course and pace".to_string());
            }
        }
        (ContextTarget::NpcShip(index), ContextAction::StopFollowing) => {
            stop_following(game, "Follow cancelled");
            select_npc_ship_for_context(game, index);
        }
        (ContextTarget::NpcShip(index), ContextAction::Hail) => {
            if let Some(npc_ship) = game.npc_ships.get(index) {
                let rumor = npc_ship_rumor(game, npc_ship);
                push_operation_feedback(game, "Rumor", rumor);
            }
        }
        (ContextTarget::NpcShip(index), ContextAction::Trade) => {
            select_npc_ship_for_context(game, index);
            game.inventory_open = true;
            game.npc_trade_quantity = 1;
            if let Some(npc_ship) = game.npc_ships.get_mut(index) {
                npc_ship.trade_open = true;
            }
        }
        (ContextTarget::NpcShip(index), ContextAction::Inspect) => {
            if game
                .npc_ships
                .get(index)
                .is_some_and(|npc_ship| npc_ship_in_interaction_range(&game.ship, npc_ship))
            {
                select_npc_ship_for_context(game, index);
                game.inventory_open = true;
            }
        }
        (ContextTarget::NpcShip(index), ContextAction::Approach) => {
            select_npc_ship_for_context(game, index);
            begin_approach(game, ApproachTarget::NpcShip(index));
            push_operation_feedback(game, "Contact", "Autopilot approaching contact".to_string());
        }
        _ => {}
    }
}

fn begin_approach(game: &mut GameState, target: ApproachTarget) {
    game.follow_target = None;
    game.approach_target = Some(target);
    game.orbiting_planet = None;
}

fn update_ship_approach(game: &mut GameState, dt: f32) {
    let Some(target) = game.approach_target else {
        return;
    };

    let target_state = match target {
        ApproachTarget::Planet(index) => game.planets.get(index).and_then(|planet| {
            planet_is_in_system(planet, &game.current_system_id).then_some((
                planet.position,
                body_approach_stop_radius(planet.radius, planet_interaction_radius(planet)),
            ))
        }),
        ApproachTarget::PlanetOrbit(index) => game.planets.get(index).and_then(|planet| {
            planet_is_in_system(planet, &game.current_system_id)
                .then_some((planet_orbit_position(planet, game.ship.position), 0.0))
        }),
        ApproachTarget::Station(index) => game.stations.get(index).and_then(|station| {
            station_is_in_system(station, &game.current_system_id).then_some((
                station.position,
                body_approach_stop_radius(station.radius, station_interaction_radius(station)),
            ))
        }),
        ApproachTarget::NpcShip(index) => game.npc_ships.get(index).and_then(|npc_ship| {
            (npc_ship.system == game.current_system_id && npc_ship.hull.current > 0.0)
                .then_some((npc_ship.position, npc_ship_interaction_radius(npc_ship)))
        }),
    };

    let Some((target_position, safe_distance)) = target_state else {
        if game.navigation_route_active {
            pause_navigation_route(
                game,
                Some("Itinerary paused: approach target is no longer available"),
            );
        } else {
            game.approach_target = None;
            push_operation_feedback(
                game,
                "Autopilot",
                "Approach target is no longer available".to_string(),
            );
        }
        return;
    };

    let offset = target_position - game.ship.position;
    let distance = offset.length();
    let remaining_distance = distance - safe_distance;
    if remaining_distance <= 0.0 {
        game.ship.velocity = Vec2::ZERO;
        game.ship.angular_velocity = 0.0;
        complete_ship_approach(game, target);
        return;
    }

    let direction = offset / distance.max(0.001);
    let current_braking_distance = autopilot_braking_distance(
        game.ship.velocity.length(),
        game.ship.forward_acceleration(),
    );
    let approach_speed = if current_braking_distance > remaining_distance {
        0.0
    } else {
        autopilot_speed_for_distance(
            remaining_distance,
            game.ship.forward_acceleration(),
            PLAYER_APPROACH_SPEED,
        )
    };
    let desired_velocity = direction * approach_speed;
    game.ship.velocity = move_toward_vec2(
        game.ship.velocity,
        desired_velocity,
        game.ship.forward_acceleration() * dt,
    );
    if game.ship.velocity.length_squared() > 0.01 {
        game.ship.angle = game.ship.velocity.y.atan2(game.ship.velocity.x) + std::f32::consts::PI;
    }
    let movement = game.ship.velocity * dt;
    let movement_distance = movement.length();
    if movement_distance >= remaining_distance {
        game.ship.position = target_position - direction * safe_distance;
        game.ship.velocity = Vec2::ZERO;
        game.ship.angular_velocity = 0.0;
        complete_ship_approach(game, target);
    } else {
        game.ship.position += movement;
    }
    game.ship
        .systems
        .energy
        .restore(ship_energy_recharge(&game.ship, &game.installed_power_modules) * dt);
}

fn complete_ship_approach(game: &mut GameState, target: ApproachTarget) {
    game.ship.velocity = Vec2::ZERO;
    game.ship.angular_velocity = 0.0;
    game.approach_target = None;
    if let ApproachTarget::PlanetOrbit(planet_index) = target {
        if game
            .planets
            .get(planet_index)
            .is_some_and(|planet| planet_is_in_system(planet, &game.current_system_id))
        {
            game.orbiting_planet = Some(planet_index);
            game.save_dirty = true;
            push_operation_feedback(game, "Orbit", "Stable orbit established".to_string());
            return;
        }
    }
    push_operation_feedback(game, "Autopilot", "Approach target reached".to_string());
    if let ApproachTarget::Planet(planet_index) = target {
        advance_navigation_route_after_arrival(game, planet_index);
    }
}

fn body_approach_stop_radius(target_radius: f32, interaction_radius: f32) -> f32 {
    let non_overlapping_radius = target_radius + SHIP_RADIUS;
    let preferred_radius = non_overlapping_radius + AUTOPILOT_CLEARANCE_MARGIN;
    let inside_interaction_radius = interaction_radius - AUTOPILOT_INTERACTION_INSET;
    preferred_radius.min(inside_interaction_radius.max(non_overlapping_radius))
}

fn stop_following(game: &mut GameState, message: &str) {
    if game.follow_target.take().is_some() {
        push_operation_feedback(game, "Follow", message.to_string());
    }
}

fn update_ship_follow(game: &mut GameState, dt: f32) {
    let Some(FollowTarget::NpcShip(index)) = game.follow_target else {
        return;
    };
    let Some(npc_ship) = game.npc_ships.get(index) else {
        stop_following(game, "Target lost");
        return;
    };
    if npc_ship.system != game.current_system_id || npc_ship.hull.current <= 0.0 {
        stop_following(game, "Target left local space");
        return;
    }
    let travel_direction = if npc_ship.velocity.length_squared() > 0.01 {
        npc_ship.velocity.normalize()
    } else {
        vec2(npc_ship.angle.cos(), npc_ship.angle.sin())
    };
    let follow_distance =
        PLAYER_FOLLOW_DISTANCE.max(npc_ship.radius + SHIP_RADIUS + AUTOPILOT_CLEARANCE_MARGIN);
    let desired_position = npc_ship.position - travel_direction * follow_distance;
    let offset = desired_position - game.ship.position;
    let offset_distance = offset.length();
    let correction_speed = autopilot_speed_for_distance(
        offset_distance,
        game.ship.forward_acceleration(),
        npc_ship.behavior.max_speed(),
    );
    let correction = if offset_distance > 0.001 {
        offset / offset_distance * correction_speed
    } else {
        Vec2::ZERO
    };
    let desired_velocity = npc_ship.velocity + correction;
    game.ship.velocity = move_toward_vec2(
        game.ship.velocity,
        desired_velocity,
        game.ship.forward_acceleration() * dt,
    );
    if game.ship.velocity.length_squared() > 0.01 {
        game.ship.angle = game.ship.velocity.y.atan2(game.ship.velocity.x);
    }
    let movement = game.ship.velocity * dt;
    let minimum_distance = npc_ship.radius + SHIP_RADIUS + AUTOPILOT_CLEARANCE_MARGIN;
    let next_position = game.ship.position + movement;
    if next_position.distance(npc_ship.position) < minimum_distance {
        game.ship.position = npc_ship.position - travel_direction * minimum_distance;
        game.ship.velocity = npc_ship.velocity;
    } else {
        game.ship.position = next_position;
    }
    game.ship
        .systems
        .energy
        .restore(ship_energy_recharge(&game.ship, &game.installed_power_modules) * dt);
    let in_range =
        game.ship.position.distance(npc_ship.position) <= npc_ship_interaction_radius(npc_ship);
    if in_range && !npc_ship.identified {
        game.selected_npc_ship = Some(index);
        identify_selected_npc_ship(game);
    }
}

fn autopilot_braking_distance(speed: f32, acceleration: f32) -> f32 {
    if !speed.is_finite() || !acceleration.is_finite() || acceleration <= 0.0 {
        return 0.0;
    }
    speed.max(0.0).powi(2) / (2.0 * acceleration)
}

fn autopilot_speed_for_distance(distance: f32, acceleration: f32, max_speed: f32) -> f32 {
    if !distance.is_finite() || !acceleration.is_finite() || !max_speed.is_finite() {
        return 0.0;
    }
    let safe_distance = distance.max(0.0);
    let safe_acceleration = acceleration.max(0.0);
    let safe_max_speed = max_speed.max(0.0);
    (2.0 * safe_acceleration * safe_distance)
        .sqrt()
        .min(safe_max_speed)
}

fn move_toward_vec2(current: Vec2, target: Vec2, max_delta: f32) -> Vec2 {
    let delta = target - current;
    if delta.length() <= max_delta {
        target
    } else {
        current + delta.normalize() * max_delta
    }
}

fn close_topmost_gameplay_overlay(game: &mut GameState) -> bool {
    if game.open_turret_bank.take().is_some() {
        true
    } else if game.context_action_menu.is_some() {
        game.context_action_menu = None;
        true
    } else if game.content_open {
        game.content_open = false;
        true
    } else if game.contracts_open {
        game.contracts_open = false;
        true
    } else if game.fleet_open {
        game.fleet_open = false;
        true
    } else if game.upgrades_open {
        game.upgrades_open = false;
        true
    } else if game.research_open {
        game.research_open = false;
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

    if is_key_pressed(KeyCode::R) {
        apply_escape_dialog_action(game, EscapeDialogAction::ResetMenus);
        return;
    }

    if is_key_pressed(KeyCode::T) {
        apply_escape_dialog_action(game, EscapeDialogAction::SaveToTitle);
        return;
    }

    if is_key_pressed(KeyCode::Q)
        && apply_escape_dialog_action(game, EscapeDialogAction::QuitDesktop)
            == EscapeDialogResult::QuitDesktop
    {
        macroquad::miniquad::window::quit();
    }
}

fn apply_escape_ui_actions(game: &mut GameState, actions: Vec<egui_escape::EscapeUiAction>) {
    for action in actions {
        let action = match action {
            egui_escape::EscapeUiAction::Resume => EscapeDialogAction::Resume,
            egui_escape::EscapeUiAction::SaveNow => EscapeDialogAction::SaveNow,
            egui_escape::EscapeUiAction::ResetMenus => EscapeDialogAction::ResetMenus,
            egui_escape::EscapeUiAction::SaveToTitle => EscapeDialogAction::SaveToTitle,
            egui_escape::EscapeUiAction::QuitDesktop => EscapeDialogAction::QuitDesktop,
        };
        if apply_escape_dialog_action(game, action) == EscapeDialogResult::QuitDesktop {
            macroquad::miniquad::window::quit();
        }
        if !game.escape_dialog_open {
            break;
        }
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
        EscapeDialogAction::ResetMenus => {
            reset_egui_menu_layout();
            reset_starmap_panel_geometry(game);
            push_operation_feedback(
                game,
                "UI",
                "Menu positions and map geometry restored; pins cleared".to_string(),
            );
            EscapeDialogResult::Continue
        }
        EscapeDialogAction::SaveToTitle => {
            flush_egui_layout();
            save_game_now(game, SaveFeedback::Manual);
            game.escape_dialog_open = false;
            game.quit_to_title_requested = true;
            EscapeDialogResult::Continue
        }
        EscapeDialogAction::QuitDesktop => {
            flush_egui_layout();
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
    let texture = select_transition_texture_for_action(
        &game.transition_assets,
        &game.stations,
        &pending_action,
    );
    game.scene_transition = Some(SceneTransition {
        texture,
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
    let target_name = system_display_name(&game.content_registry, &target_system_id).to_string();
    let charge_seconds = warp_charge_seconds(&game.ship_upgrades);
    game.pending_warp = Some(PendingWarp {
        target_system_id,
        timer: charge_seconds,
        cost,
    });
    push_operation_feedback(
        game,
        "Travel",
        format!("Warp charging for {target_name} ({charge_seconds:.1}s)"),
    );
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
    let target_name =
        system_display_name(&game.content_registry, &warp.target_system_id).to_string();
    let cost_label = format_warp_cost(&warp.cost);
    push_operation_feedback(
        game,
        "Travel",
        format!("Warp committed to {target_name}; spent {cost_label}"),
    );
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

fn station_stock_source_for_item<'a>(
    stations: &'a [StationDestination],
    system_id: &str,
    item_id: &str,
) -> Option<&'a str> {
    stations
        .iter()
        .filter(|station| station.system == system_id)
        .find(|station| {
            station.services.iter().any(|service| {
                service
                    .trade
                    .iter()
                    .any(|offer| !offer.unavailable && offer.item.id == item_id)
            })
        })
        .map(|station| station.name.as_str())
}

fn route_readiness_summary(game: &GameState, target_system_id: &str) -> String {
    if target_system_id == game.current_system_id {
        return "Operating locally".to_string();
    }

    let Some(target_system) = game.content_registry.systems.get(target_system_id) else {
        return "Route data unavailable".to_string();
    };
    let cost = warp_cost(
        &game.content_registry,
        &game.current_system_id,
        target_system_id,
    );
    let can_warp = can_afford_cost(&game.inventory, &cost);
    let is_remote = target_system.tags.iter().any(|tag| tag == "remote");

    if !can_warp {
        if let Some(stack) = cost
            .iter()
            .find(|stack| game.inventory.count(&stack.item) < stack.count)
        {
            let missing = stack
                .count
                .saturating_sub(game.inventory.count(&stack.item));
            if let Some(source) = station_stock_source_for_item(
                &game.stations,
                &game.current_system_id,
                &stack.item.id,
            ) {
                return format!(
                    "Need {} x{}; {} stocks it",
                    stack.item.name, missing, source
                );
            }

            return format!(
                "Need {} x{}; craft or buy before warp",
                stack.item.name, missing
            );
        }

        return "Route needs supplies".to_string();
    }

    if is_remote && ship_upgrade_level(&game.ship_upgrades, ShipUpgradeKind::ScannerArray) < 2 {
        return "Route ready; Scanner array 2 recommended".to_string();
    }

    if is_remote {
        return "Remote prep ready".to_string();
    }

    "Route ready".to_string()
}

fn switch_current_system(game: &mut GameState, target_system_id: &str) {
    let Some(target_system) = game.content_registry.systems.get(target_system_id) else {
        eprintln!("Cannot switch to missing system `{target_system_id}`");
        return;
    };
    let arrival = target_system.arrival;
    let target_name = target_system.name.clone();

    remember_current_system_destination(game);

    game.current_system_id = target_system_id.to_string();
    game.selected_planet = None;
    game.selected_station = None;
    game.selected_npc_ship = None;
    game.selected_station_service = None;
    game.context_action_menu = None;
    game.follow_target = None;
    game.approach_target = None;
    game.navigation_route_active = false;
    game.orbiting_planet = None;
    game.destination_planet = destination_planet_for_system(
        &game.planets,
        &game.system_destinations,
        &game.current_system_id,
    );
    game.starmap_selected_planet = game.destination_planet;
    game.ship.position = vec2(arrival[0], arrival[1]);
    game.ship.velocity = Vec2::ZERO;
    game.ship.angular_velocity = 0.0;
    game.save_dirty = true;
    push_operation_feedback(game, "Travel", format!("Arrived in {target_name}"));
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

fn navigation_stop_planet_index(game: &GameState, stop: &NavigationStop) -> Option<usize> {
    game.planets
        .iter()
        .position(|planet| planet.id == stop.planet && planet.system == stop.system)
}

fn selected_starmap_navigation_stop(game: &GameState) -> Option<NavigationStop> {
    game.starmap_selected_planet
        .and_then(|index| game.planets.get(index))
        .filter(|planet| planet.system == game.current_system_id)
        .map(|planet| NavigationStop {
            system: planet.system.clone(),
            planet: planet.id.clone(),
        })
}

fn fly_to_selected_starmap_planet(game: &mut GameState) -> bool {
    let Some(stop) = selected_starmap_navigation_stop(game) else {
        return false;
    };
    let Some(planet_index) = navigation_stop_planet_index(game, &stop) else {
        return false;
    };
    pause_navigation_route(game, None);
    set_destination_planet(game, Some(planet_index));
    begin_approach(game, ApproachTarget::Planet(planet_index));
    game.map_open = false;
    game.save_dirty = true;
    push_operation_feedback(game, "Route", "Flying to selected destination".to_string());
    true
}

fn add_selected_starmap_planet_to_route(game: &mut GameState) -> bool {
    let Some(stop) = selected_starmap_navigation_stop(game) else {
        return false;
    };
    game.navigation_route.push(stop);
    game.save_dirty = true;
    push_operation_feedback(game, "Route", "Destination added to itinerary".to_string());
    true
}

fn activate_navigation_route(game: &mut GameState) -> bool {
    if game.navigation_route.is_empty() {
        return false;
    }
    if game
        .navigation_route_current
        .is_none_or(|current| current >= game.navigation_route.len())
    {
        game.navigation_route_current = Some(0);
    }
    game.navigation_route_active = true;
    if !start_current_navigation_stop(game) {
        game.navigation_route_active = false;
        return false;
    }
    game.map_open = false;
    game.save_dirty = true;
    true
}

fn start_current_navigation_stop(game: &mut GameState) -> bool {
    let Some(current) = game.navigation_route_current else {
        return false;
    };
    let Some(stop) = game.navigation_route.get(current).cloned() else {
        return false;
    };
    if stop.system != game.current_system_id {
        push_operation_feedback(
            game,
            "Route",
            "Route paused: the next stop is in another system".to_string(),
        );
        return false;
    }
    let Some(planet_index) = navigation_stop_planet_index(game, &stop) else {
        push_operation_feedback(
            game,
            "Route",
            "Route paused: the next destination is unavailable".to_string(),
        );
        return false;
    };
    game.starmap_selected_planet = Some(planet_index);
    set_destination_planet(game, Some(planet_index));
    begin_approach(game, ApproachTarget::Planet(planet_index));
    push_operation_feedback(
        game,
        "Route",
        format!("Approaching itinerary stop {}", current + 1),
    );
    true
}

fn pause_navigation_route(game: &mut GameState, message: Option<&str>) {
    if !game.navigation_route_active {
        return;
    }
    game.navigation_route_active = false;
    game.approach_target = None;
    game.save_dirty = true;
    if let Some(message) = message {
        push_operation_feedback(game, "Route", message.to_string());
    }
}

fn advance_navigation_route_after_arrival(game: &mut GameState, planet_index: usize) {
    if !game.navigation_route_active {
        return;
    }
    let Some(current) = game.navigation_route_current else {
        return;
    };
    let Some(stop) = game.navigation_route.get(current) else {
        game.navigation_route_active = false;
        return;
    };
    let Some(planet) = game.planets.get(planet_index) else {
        return;
    };
    if stop.system != planet.system || stop.planet != planet.id {
        return;
    }

    let next = current + 1;
    game.navigation_route_current = Some(next);
    game.save_dirty = true;
    if next >= game.navigation_route.len() {
        game.navigation_route_active = false;
        push_operation_feedback(game, "Route", "Itinerary complete".to_string());
    } else if !start_current_navigation_stop(game) {
        game.navigation_route_active = false;
    }
}

fn clear_navigation_route(game: &mut GameState) {
    if game.navigation_route_active {
        game.approach_target = None;
    }
    game.navigation_route.clear();
    game.navigation_route_current = None;
    game.navigation_route_active = false;
    game.save_dirty = true;
}

fn remove_navigation_route_stop(game: &mut GameState, index: usize) -> bool {
    if game.navigation_route_active || index >= game.navigation_route.len() {
        return false;
    }
    game.navigation_route.remove(index);
    game.navigation_route_current = game.navigation_route_current.and_then(|current| {
        if game.navigation_route.is_empty() {
            None
        } else if index < current {
            Some(current - 1)
        } else if index == current {
            Some(current.min(game.navigation_route.len() - 1))
        } else {
            Some(current.min(game.navigation_route.len()))
        }
    });
    game.save_dirty = true;
    true
}

fn move_navigation_route_stop(game: &mut GameState, from: usize, to: usize) -> bool {
    if game.navigation_route_active
        || from >= game.navigation_route.len()
        || to >= game.navigation_route.len()
        || from == to
    {
        return false;
    }
    game.navigation_route.swap(from, to);
    game.navigation_route_current = game.navigation_route_current.map(|current| {
        if current == from {
            to
        } else if current == to {
            from
        } else {
            current
        }
    });
    game.save_dirty = true;
    true
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
    if let Some(index) = game.selected_npc_ship {
        if let Some(npc_ship) = game.npc_ships.get_mut(index) {
            npc_ship.trade_open = false;
        }
    }
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
    if let Some(index) = game.selected_npc_ship {
        if let Some(npc_ship) = game.npc_ships.get_mut(index) {
            npc_ship.trade_open = false;
        }
    }
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
    let Some(name) = game.npc_ships.get_mut(npc_ship_index).and_then(|npc_ship| {
        if !npc_ship_is_in_system(npc_ship, &game.current_system_id)
            || !npc_ship_in_interaction_range(&game.ship, npc_ship)
        {
            None
        } else {
            npc_ship.identified = true;
            Some(npc_ship.name.clone())
        }
    }) else {
        return false;
    };
    push_operation_feedback(game, "Contact", format!("Identified {name}"));
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

fn recipe_is_unlocked_from_sets(
    content_registry: &content::ContentRegistry,
    recipe_id: &str,
    locked_recipes: &[String],
    completed_research: &[String],
) -> bool {
    !locked_recipes.iter().any(|locked| locked == recipe_id)
        || completed_research_unlocks_recipe(content_registry, completed_research, recipe_id)
}

fn repair_ship_at_station(game: &mut GameState, station_index: usize) -> bool {
    let in_range = game
        .stations
        .get(station_index)
        .is_some_and(|station| station_in_interaction_range(&game.ship, station));
    if !in_range {
        return true;
    }
    let service_available = game.stations.get(station_index).and_then(|station| {
        game.selected_station_service
            .and_then(|index| station.services.get(index))
            .map(|service| station_service_is_available(game, station, service))
    });
    if service_available != Some(true) {
        push_operation_feedback(
            game,
            "Repair",
            "Repair service locked by faction standing".to_string(),
        );
        return true;
    }
    let missing_hull = (game.ship.systems.hull.max - game.ship.systems.hull.current).ceil();
    let missing_shields =
        (game.ship.systems.shields.max - game.ship.systems.shields.current).ceil();
    let cost = (missing_hull * 3.0) as u32 + missing_shields as u32;
    if cost == 0 {
        push_operation_feedback(
            game,
            "Repair",
            "Ship systems are already nominal".to_string(),
        );
        return true;
    }
    if game.credits < cost {
        push_operation_feedback(
            game,
            "Repair",
            format!("Need {} more credits for repairs", cost - game.credits),
        );
        return true;
    }
    game.credits -= cost;
    game.ship.systems.hull.current = game.ship.systems.hull.max;
    game.ship.systems.shields.current = game.ship.systems.shields.max;
    game.save_dirty = true;
    push_operation_feedback(
        game,
        "Repair",
        format!("Hull and shields restored for {cost} cr"),
    );
    let station_faction = game
        .stations
        .get(station_index)
        .and_then(|station| station.faction.clone());
    adjust_faction_reputation(game, station_faction.as_deref(), 1);
    true
}

fn accept_or_complete_contract(
    game: &mut GameState,
    station_index: usize,
    service_index: usize,
    contract_index: usize,
) -> bool {
    let in_range = game
        .stations
        .get(station_index)
        .is_some_and(|station| station_in_interaction_range(&game.ship, station));
    if !in_range {
        return true;
    }
    update_contract_progress(game);
    let Some(contract) = game
        .stations
        .get(station_index)
        .and_then(|station| station.services.get(service_index))
        .and_then(|service| service.contracts.get(contract_index))
        .cloned()
    else {
        return false;
    };
    if !station_service_is_available(
        game,
        &game.stations[station_index],
        &game.stations[station_index].services[service_index],
    ) || faction_reputation(game, contract.reputation_faction.as_deref())
        < contract.reputation_required
    {
        push_operation_feedback(
            game,
            "Contract",
            "Contract locked by faction standing".to_string(),
        );
        return true;
    }
    let active_index = game.active_contracts.iter().position(|active| {
        active.id == contract.id
            && active.origin_station == contract.origin_station
            && active.origin_service == contract.origin_service
    });
    if let Some(active_index) = active_index {
        if game.world_elapsed_days > game.active_contracts[active_index].expires_day {
            game.active_contracts.remove(active_index);
            game.save_dirty = true;
            push_operation_feedback(game, "Contract", format!("{} expired", contract.name));
            return true;
        }
        if !contract_is_complete(game, &contract) {
            push_operation_feedback(game, "Contract", contract_progress_text(game, &contract));
            return true;
        }
        if let Some(item) = &contract.item {
            game.inventory.remove_item(item, contract.amount);
        }
        let reputation_faction = contract.reputation_faction.clone();
        let reputation_reward = contract.reputation_reward;
        game.credits = game.credits.saturating_add(contract.reward);
        game.active_contracts.remove(active_index);
        game.save_dirty = true;
        push_operation_feedback(
            game,
            "Contract",
            format!("Completed {} for {} cr", contract.name, contract.reward),
        );
        adjust_faction_reputation(game, reputation_faction.as_deref(), reputation_reward);
        return true;
    }
    if game.active_contracts.len() >= 3 {
        push_operation_feedback(
            game,
            "Contract",
            "Active contract limit reached".to_string(),
        );
        return true;
    }
    game.active_contracts.push(ActiveContract {
        id: contract.id.clone(),
        origin_station: contract.origin_station.clone(),
        origin_service: contract.origin_service.clone(),
        expires_day: game.world_elapsed_days + contract.duration_days,
        target_reached: false,
    });
    game.save_dirty = true;
    push_operation_feedback(game, "Contract", format!("Accepted {}", contract.name));
    true
}

fn contract_is_complete(game: &GameState, contract: &ContractOffer) -> bool {
    let target_reached = game.active_contracts.iter().any(|active| {
        active.id == contract.id
            && active.origin_station == contract.origin_station
            && active.origin_service == contract.origin_service
            && active.target_reached
    });
    let at_origin = game
        .selected_station
        .and_then(|index| game.stations.get(index))
        .is_some_and(|station| station.id == contract.origin_station);
    if !at_origin || !target_reached {
        return false;
    }
    if contract.kind == "hauling" {
        contract
            .item
            .as_ref()
            .is_some_and(|item| game.inventory.count(item) >= contract.amount)
    } else {
        true
    }
}

fn contract_progress_text(game: &GameState, contract: &ContractOffer) -> String {
    if contract.kind == "hauling" {
        let count = contract
            .item
            .as_ref()
            .map(|item| game.inventory.count(item))
            .unwrap_or_default();
        format!(
            "Cargo {count}/{}; deliver at target station",
            contract.amount
        )
    } else {
        format!("Scan target planet to level {}", contract.amount)
    }
}

fn npc_interaction_rows(
    content_registry: &content::ContentRegistry,
    ship: &Ship,
    npc_ship: &NpcShip,
) -> Vec<NpcInteractionRow> {
    let in_range = npc_ship_in_interaction_range(ship, npc_ship);
    let identified = npc_ship.identified;
    let hostile = npc_ship_is_hostile(content_registry, npc_ship);
    let trade_role = npc_ship_can_trade(content_registry, npc_ship);

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
            state: if in_range
                && identified
                && !hostile
                && trade_role
                && !npc_ship.cargo_defaults.is_empty()
            {
                NpcInteractionState::Available
            } else {
                NpcInteractionState::Unavailable
            },
            status: if !identified {
                "Identify"
            } else if hostile {
                "Hostile"
            } else if trade_role && !npc_ship.cargo_defaults.is_empty() {
                "Open market"
            } else if trade_role {
                "No stock"
            } else {
                "No market"
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

fn npc_ship_can_trade(content_registry: &content::ContentRegistry, npc_ship: &NpcShip) -> bool {
    !npc_ship_is_hostile(content_registry, npc_ship)
        && (matches!(npc_ship.role.as_str(), "hauler" | "trader")
            || npc_ship
                .behavior_tags
                .iter()
                .any(|tag| tag == "trade-route"))
}

fn npc_trade_quote(npc_ship: &NpcShip, item: &ItemRef, world_seed: u64, day: u64) -> NpcTradeQuote {
    let salt = world_seed ^ day.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    let noise = stable_unit_noise(&format!("{}:{}", npc_ship.id, item.id), salt);
    let base = (item.unit_mass * 12.0).round() as u32;
    let buy_price = ((base.max(4) as f32) * (0.85 + noise * 0.35)).round() as u32;
    NpcTradeQuote {
        buy_price: buy_price.max(4),
        sell_price: ((buy_price as f32) * 0.72).round() as u32,
        rumor: match ((noise * 3.0) as usize).min(2) {
            0 => "Supply is tight; buyers are paying up.",
            1 => "The route is steady; prices look ordinary.",
            _ => "Freighters are moving extra stock through this lane.",
        },
    }
}

fn npc_ship_rumor(game: &GameState, npc_ship: &NpcShip) -> String {
    let Some(stack) = npc_ship.cargo_defaults.first() else {
        return format!("{} has no cargo market news.", npc_ship.name);
    };
    let quote = npc_trade_quote(
        npc_ship,
        &stack.item,
        game.world_seed,
        game.world_elapsed_days.floor().max(0.0) as u64,
    );
    format!(
        "{} reports {}: {} buys {} cr / sells {} cr.",
        npc_ship.name, quote.rumor, stack.item.name, quote.buy_price, quote.sell_price
    )
}

fn execute_npc_trade(
    game: &mut GameState,
    npc_ship_index: usize,
    row: usize,
    action: NpcTradeAction,
) -> bool {
    let Some(npc_ship) = game.npc_ships.get(npc_ship_index) else {
        return false;
    };
    if !npc_ship.identified
        || !npc_ship_in_interaction_range(&game.ship, npc_ship)
        || !npc_ship_can_trade(&game.content_registry, npc_ship)
    {
        return true;
    }
    let Some(stack) = npc_ship.cargo_defaults.get(row).cloned() else {
        return false;
    };
    let npc_name = npc_ship.name.clone();
    let item = stack.item.clone();
    let quote = npc_trade_quote(
        npc_ship,
        &item,
        game.world_seed,
        game.world_elapsed_days.floor().max(0.0) as u64,
    );
    let amount = match action {
        NpcTradeAction::Buy => npc_trade_buy_amount(
            game.credits,
            &game.inventory,
            &game.ship_upgrades,
            &stack,
            &quote,
            game.npc_trade_quantity,
        ),
        NpcTradeAction::Sell => npc_trade_sell_amount(
            npc_ship,
            &stack,
            game.inventory.count(&item),
            game.npc_trade_quantity,
        ),
    };
    if amount == 0 {
        return true;
    }
    match action {
        NpcTradeAction::Buy => {
            game.credits = game
                .credits
                .saturating_sub(quote.buy_price.saturating_mul(amount));
            game.inventory.add_item(item.clone(), amount);
            if let Some(npc) = game.npc_ships.get_mut(npc_ship_index) {
                npc.cargo_defaults[row].count =
                    npc.cargo_defaults[row].count.saturating_sub(amount);
            }
            game.save_dirty = true;
            push_operation_feedback(
                game,
                "Trade",
                format!(
                    "Bought {} x{} from {} for {} cr",
                    item.name,
                    amount,
                    npc_name,
                    quote.buy_price.saturating_mul(amount)
                ),
            );
        }
        NpcTradeAction::Sell => {
            game.inventory.remove_item(&item, amount);
            if let Some(npc) = game.npc_ships.get_mut(npc_ship_index) {
                npc.cargo_defaults[row].count += amount;
            }
            game.credits = game
                .credits
                .saturating_add(quote.sell_price.saturating_mul(amount));
            game.save_dirty = true;
            push_operation_feedback(
                game,
                "Trade",
                format!(
                    "Sold {} x{} to {} for {} cr",
                    item.name,
                    amount,
                    npc_name,
                    quote.sell_price.saturating_mul(amount)
                ),
            );
        }
    }
    true
}

fn npc_trade_buy_amount(
    credits: u32,
    inventory: &Inventory,
    ship_upgrades: &[ShipUpgrade; SHIP_UPGRADE_COUNT],
    stack: &ItemStack,
    quote: &NpcTradeQuote,
    requested: u32,
) -> u32 {
    let by_mass = if stack.item.unit_mass > 0.0 {
        ((cargo_rating_kg(ship_upgrades) - inventory.total_mass()).max(0.0) / stack.item.unit_mass)
            as u32
    } else {
        requested
    };
    let by_slot = if inventory.count(&stack.item) > 0 || inventory.slots.iter().any(Option::is_none)
    {
        requested
    } else {
        0
    };
    requested
        .min(stack.count)
        .min(credits / quote.buy_price.max(1))
        .min(by_mass)
        .min(by_slot)
}

fn npc_trade_sell_amount(
    npc_ship: &NpcShip,
    stack: &ItemStack,
    inventory_count: u32,
    requested: u32,
) -> u32 {
    let by_mass = if stack.item.unit_mass > 0.0 {
        ((npc_ship.cargo_capacity
            - npc_ship
                .cargo_defaults
                .iter()
                .map(|cargo| cargo.item.unit_mass * cargo.count as f32)
                .sum::<f32>())
        .max(0.0)
            / stack.item.unit_mass) as u32
    } else {
        requested
    };
    requested.min(inventory_count).min(by_mass)
}

fn npc_ship_is_hostile(content_registry: &content::ContentRegistry, npc_ship: &NpcShip) -> bool {
    matches!(npc_ship.behavior, NpcBehaviorMode::HostileIntercept)
        || npc_ship.role.eq_ignore_ascii_case("hostile")
        || npc_ship
            .behavior_tags
            .iter()
            .any(|tag| tag.eq_ignore_ascii_case("hostile"))
        || npc_ship.faction.as_deref().is_some_and(|faction_id| {
            content_registry
                .factions
                .get(faction_id)
                .is_some_and(|faction| {
                    faction.default_disposition == content::FactionDisposition::Hostile
                })
        })
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
        || completed_research_unlocks_recipe(
            &game.content_registry,
            &game.completed_research,
            &unlock.recipe,
        )
    {
        return true;
    }

    let Some(research_id) =
        research_id_that_unlocks_recipe(&game.content_registry, &unlock.recipe).map(str::to_string)
    else {
        return true;
    };
    game.credits -= unlock.price;
    game.completed_research.push(research_id);
    game.completed_research.sort();
    game.completed_research.dedup();
    game.save_dirty = true;
    push_operation_feedback(
        game,
        "Unlock",
        format!(
            "Recipe available: {}",
            recipe_display_name(&game.content_registry, &unlock.recipe)
        ),
    );
    true
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

    let station_name = station.name.clone();
    let service_name = station.services[service_index].name.clone();
    game.selected_station = Some(station_index);
    game.selected_planet = None;
    game.selected_npc_ship = None;
    game.selected_station_service = Some(service_index);
    push_operation_feedback(
        game,
        "Station",
        format!("{service_name} service selected at {station_name}"),
    );
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
    if !game
        .stations
        .get(station_index)
        .and_then(|station| station.services.get(service_index))
        .is_some_and(|service| {
            game.stations
                .get(station_index)
                .is_some_and(|station| station_service_is_available(game, station, service))
        })
    {
        return true;
    }
    let Some(station) = game.stations.get_mut(station_index) else {
        return false;
    };
    let station_name = station.name.clone();
    let Some(offer) = station
        .services
        .get_mut(service_index)
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
    let item = offer.item.clone();
    let buy_price = offer.buy_price;

    game.inventory.add_item(item.clone(), 1);
    game.save_dirty = true;
    push_operation_feedback(
        game,
        "Trade",
        format!(
            "Bought {} from {} for {} cr",
            item.name, station_name, buy_price
        ),
    );
    push_route_ready_feedback(game);
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
    if !game
        .stations
        .get(station_index)
        .and_then(|station| station.services.get(service_index))
        .is_some_and(|service| {
            game.stations
                .get(station_index)
                .is_some_and(|station| station_service_is_available(game, station, service))
        })
    {
        return true;
    }
    let Some(station) = game.stations.get_mut(station_index) else {
        return false;
    };
    let station_name = station.name.clone();
    let Some(offer) = station
        .services
        .get_mut(service_index)
        .and_then(|service| service.trade.get_mut(offer_index))
    else {
        return false;
    };
    if game.inventory.count(&offer.item) == 0 {
        return true;
    }

    let item = offer.item.clone();
    let sell_price = offer.sell_price;
    game.inventory.remove_item(&item, 1);
    game.credits = game.credits.saturating_add(sell_price);
    if let Some(stock) = offer.stock.as_mut() {
        *stock = stock.saturating_add(1);
        if let Some(max_stock) = offer.max_stock {
            *stock = (*stock).min(max_stock);
        }
    }

    game.save_dirty = true;
    push_operation_feedback(
        game,
        "Trade",
        format!(
            "Sold {} to {} for {} cr",
            item.name, station_name, sell_price
        ),
    );
    true
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
        let mut feedback = None;
        if let Some(planet) = game.planets.get_mut(planet_index) {
            let previous_level = planet.scan_level;
            let scan_steps = scan_steps.saturating_add(scanner_survey_bonus(&game.ship_upgrades));
            planet.scan_level = planet
                .scan_level
                .saturating_add(scan_steps)
                .min(MAX_SCAN_LEVEL);
            let detail = if planet.scan_level >= MAX_SCAN_LEVEL {
                "survey complete"
            } else if previous_level < 2 && planet.scan_level >= 2 {
                "composition revealed"
            } else if previous_level < 1 && planet.scan_level >= 1 {
                "surface record updated"
            } else {
                "scan data updated"
            };
            feedback = Some(format!("{}: {detail}", planet.info.classification));
        }
        if let Some(message) = feedback {
            push_operation_feedback(game, "Survey", message);
        }
        return true;
    }

    false
}

fn active_production_row_count(game: &GameState) -> usize {
    match game.production_mode {
        ProductionMode::Smelting => game.smelt_recipes.len(),
        ProductionMode::Crafting => game.craft_recipes.len(),
        ProductionMode::Processing => game.processing_recipes.len(),
    }
}

fn max_scroll_offset(row_count: usize, row_height: f32, viewport_height: f32) -> f32 {
    let content_height = row_count as f32 * row_height;
    (content_height - viewport_height).max(0.0)
}

fn update_production(game: &mut GameState, dt: f32) {
    let smelted = update_recipes(
        RecipeUpdate {
            content_registry: &game.content_registry,
            inventory: &mut game.inventory,
            recipes: &game.smelt_recipes,
            settings: &mut game.smelt_settings,
            locked_recipes: &game.recipe_vendor_locked_recipes,
            completed_research: &game.completed_research,
            work_kind: WorkKind::Smelting,
        },
        dt,
    );
    for stack in smelted {
        let item_name = stack.item.name.clone();
        push_aggregate_operation_feedback(
            game,
            "Production",
            format!("smelt:{}", stack.item.id),
            stack.count,
            |count| format!("Produced {item_name} x{count}"),
        );
        push_route_ready_feedback(game);
    }

    let crafted = update_recipes(
        RecipeUpdate {
            content_registry: &game.content_registry,
            inventory: &mut game.inventory,
            recipes: &game.craft_recipes,
            settings: &mut game.craft_settings,
            locked_recipes: &game.recipe_vendor_locked_recipes,
            completed_research: &game.completed_research,
            work_kind: WorkKind::Fabrication,
        },
        dt,
    );
    for stack in crafted {
        let item_name = stack.item.name.clone();
        push_aggregate_operation_feedback(
            game,
            "Production",
            format!("craft:{}", stack.item.id),
            stack.count,
            |count| format!("Built {item_name} x{count}"),
        );
        push_route_ready_feedback(game);
    }

    let processed = update_recipes(
        RecipeUpdate {
            content_registry: &game.content_registry,
            inventory: &mut game.inventory,
            recipes: &game.processing_recipes,
            settings: &mut game.processing_settings,
            locked_recipes: &game.recipe_vendor_locked_recipes,
            completed_research: &game.completed_research,
            work_kind: WorkKind::Fabrication,
        },
        dt,
    );
    for stack in processed {
        let item_name = stack.item.name.clone();
        push_aggregate_operation_feedback(
            game,
            "Production",
            format!("process:{}", stack.item.id),
            stack.count,
            |count| format!("Processed {item_name} x{count}"),
        );
        push_route_ready_feedback(game);
    }
}

struct RecipeUpdate<'a> {
    content_registry: &'a content::ContentRegistry,
    inventory: &'a mut Inventory,
    recipes: &'a [Recipe],
    settings: &'a mut [CraftSetting],
    locked_recipes: &'a [String],
    completed_research: &'a [String],
    work_kind: WorkKind,
}

fn update_recipes(update: RecipeUpdate<'_>, dt: f32) -> Vec<ItemStack> {
    let RecipeUpdate {
        content_registry,
        inventory,
        recipes,
        settings,
        locked_recipes,
        completed_research,
        work_kind,
    } = update;

    clear_blocked_recipe_progress(
        inventory,
        content_registry,
        recipes,
        settings,
        locked_recipes,
        completed_research,
    );
    let Some(recipe_index) = next_recipe_bill_index_for_sets(
        inventory,
        content_registry,
        recipes,
        settings,
        locked_recipes,
        completed_research,
    ) else {
        return Vec::new();
    };
    let recipe = &recipes[recipe_index];
    let operation_seconds =
        recipe_operation_seconds(content_registry, completed_research, work_kind, recipe);
    let setting = &mut settings[recipe_index];
    setting.progress += dt / operation_seconds;
    if setting.progress < 1.0 {
        return Vec::new();
    }
    if !inventory.can_craft(recipe) {
        setting.progress = 0.0;
        return Vec::new();
    }

    setting.progress -= 1.0;
    if setting.queued > 0 {
        setting.queued = setting.queued.saturating_sub(recipe.output.count);
    }
    if inventory.craft(recipe) {
        let bonus = bonus_output_count(content_registry, completed_research, recipe.output.count);
        if bonus > 0 {
            inventory.add_item(recipe.output.item.clone(), bonus);
        }
        return vec![ItemStack {
            item: recipe.output.item.clone(),
            count: recipe.output.count.saturating_add(bonus),
        }];
    }
    Vec::new()
}

fn clear_blocked_recipe_progress(
    inventory: &Inventory,
    content_registry: &content::ContentRegistry,
    recipes: &[Recipe],
    settings: &mut [CraftSetting],
    locked_recipes: &[String],
    completed_research: &[String],
) {
    for (recipe, setting) in recipes.iter().zip(settings.iter_mut()) {
        if recipe_has_bill(inventory, recipe, setting)
            && (!recipe_is_unlocked_from_sets(
                content_registry,
                &recipe.id,
                locked_recipes,
                completed_research,
            ) || !inventory.can_craft(recipe))
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
        &game.content_registry,
        recipes,
        settings,
        &game.recipe_vendor_locked_recipes,
        &game.completed_research,
    )
}

fn next_recipe_bill_index_for_sets(
    inventory: &Inventory,
    content_registry: &content::ContentRegistry,
    recipes: &[Recipe],
    settings: &[CraftSetting],
    locked_recipes: &[String],
    completed_research: &[String],
) -> Option<usize> {
    recipes
        .iter()
        .zip(settings.iter())
        .enumerate()
        .find_map(|(index, (recipe, setting))| {
            (recipe_is_unlocked_from_sets(
                content_registry,
                &recipe.id,
                locked_recipes,
                completed_research,
            ) && recipe_has_bill(inventory, recipe, setting)
                && inventory.can_craft(recipe))
            .then_some(index)
        })
}

fn recipe_has_bill(inventory: &Inventory, recipe: &Recipe, setting: &CraftSetting) -> bool {
    setting.queued > 0 || (setting.keep > 0 && inventory.count(&recipe.output.item) < setting.keep)
}

fn update_mining(game: &mut GameState, dt: f32) {
    let base_operation_seconds =
        mining_operation_seconds(&game.content_registry, &game.completed_research);

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
        let mut total_mined = mined;
        let bonus = bonus_output_count(&game.content_registry, &game.completed_research, mined);
        if bonus > 0 {
            game.inventory.add_item(mineable.item.clone(), bonus);
            total_mined = total_mined.saturating_add(bonus);
        }
        if rand::gen_range(0.0, 1.0) < richness_bonus_chance {
            game.inventory.add_item(mineable.item.clone(), mined);
            total_mined = total_mined.saturating_add(mined);
        }
        let item_name = mineable.item.name.clone();
        push_aggregate_operation_feedback(
            game,
            "Mining",
            format!("mine:{}", mineable.item.id),
            total_mined,
            |count| format!("Recovered {item_name} x{count}"),
        );
        push_route_ready_feedback(game);
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

fn update_inactive_owned_ship_systems(game: &mut GameState, dt: f32) {
    for owned in &mut game.owned_ships {
        if owned.instance_id == game.active_owned_ship_id
            || owned.system_id != game.current_system_id
            || owned.ship.systems.hull.current <= 0.0
        {
            continue;
        }

        let energy_recharge = ship_energy_recharge(&owned.ship, &owned.installed_power_modules);
        owned.ship.systems.energy.restore(energy_recharge * dt);

        if owned.ship.systems.shields.current >= owned.ship.systems.shields.max {
            owned.shield_recharge_delay_remaining = 0.0;
            continue;
        }
        if owned.shield_recharge_delay_remaining > 0.0 {
            owned.shield_recharge_delay_remaining =
                (owned.shield_recharge_delay_remaining - dt).max(0.0);
            continue;
        }
        let shield_recharge = owned_ship_shield_recharge_rate(owned);
        owned.ship.systems.shields.restore(shield_recharge * dt);
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
        update_npc_shield_recharge(&mut game.npc_ships[index], dt);
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

fn update_hostile_npc_pressure(game: &mut GameState, dt: f32) {
    if dt <= 0.0 {
        return;
    }

    let pressure_count = active_hostile_pressure_count(
        &game.content_registry,
        &game.ship,
        &game.npc_ships,
        &game.current_system_id,
    );
    if pressure_count == 0 {
        return;
    }

    apply_ship_pressure_damage(
        game,
        REDWAKE_PROBE_PRESSURE_PER_SECOND * pressure_count as f32 * dt,
    );
}

fn active_hostile_pressure_count(
    content_registry: &content::ContentRegistry,
    ship: &Ship,
    npc_ships: &[NpcShip],
    current_system_id: &str,
) -> usize {
    npc_ships
        .iter()
        .filter(|npc_ship| {
            npc_ship_exerts_pressure(content_registry, ship, npc_ship, current_system_id)
        })
        .count()
}

fn npc_ship_exerts_pressure(
    content_registry: &content::ContentRegistry,
    ship: &Ship,
    npc_ship: &NpcShip,
    current_system_id: &str,
) -> bool {
    npc_ship.system == current_system_id
        && npc_ship.hull.current > 0.0
        && npc_ship.behavior_tags.iter().any(|tag| tag == "pressure")
        && npc_ship_is_hostile(content_registry, npc_ship)
        && npc_ship_surface_distance(ship, npc_ship) <= NPC_PRESSURE_RANGE
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
    let mut completed_impacts = Vec::new();
    let mut active_fire_events = Vec::new();
    for mut event in std::mem::take(&mut game.weapon_fire_events) {
        refresh_weapon_fire_target(game, &mut event);
        event.timer -= dt;
        if event.timer <= 0.0 {
            if let Some(impact) = event.pending_impact.take() {
                completed_impacts.push((event.clone(), impact));
            }
        } else {
            active_fire_events.push(event);
        }
    }
    game.weapon_fire_events = active_fire_events;
    game.weapon_area_effect_events.retain_mut(|event| {
        event.timer -= dt;
        event.timer > 0.0
    });

    for (event, impact) in completed_impacts {
        resolve_weapon_impact(game, &event, impact);
    }

    update_player_weapon_systems(game, dt);
    update_inactive_owned_ship_weapon_systems(game, dt);
    update_npc_weapon_systems(game, dt);
}

#[derive(Clone)]
struct ImpactCandidate {
    target: WeaponTargetRef,
    position: Vec2,
    radius: f32,
    hostile_to_owner: bool,
}

fn refresh_weapon_fire_target(game: &GameState, event: &mut WeaponFireEvent) {
    let Some(impact) = event.pending_impact.as_ref() else {
        return;
    };
    if let Some(position) = weapon_target_position(game, &impact.primary_target, &impact.system) {
        event.to = position;
    }
}

fn weapon_target_position(
    game: &GameState,
    target: &WeaponTargetRef,
    system: &str,
) -> Option<Vec2> {
    match target {
        WeaponTargetRef::Player => (game.current_system_id == system
            && game.ship.systems.hull.current > 0.0)
            .then_some(game.ship.position),
        WeaponTargetRef::OwnedShip(instance_id) => game
            .owned_ships
            .iter()
            .find(|owned| {
                owned.instance_id == *instance_id
                    && owned.instance_id != game.active_owned_ship_id
                    && owned.system_id == system
                    && owned.ship.systems.hull.current > 0.0
            })
            .map(|owned| owned.ship.position),
        WeaponTargetRef::Npc(id) => game
            .npc_ships
            .iter()
            .find(|npc| npc.id == *id && npc.system == system && npc.hull.current > 0.0)
            .map(|npc| npc.position),
        WeaponTargetRef::Threat(id) => game
            .defense_threats
            .iter()
            .find(|threat| threat.id == *id && threat.system == system && threat.hull.current > 0.0)
            .map(|threat| threat.position),
    }
}

fn impact_candidates(game: &GameState, impact: &PendingWeaponImpact) -> Vec<ImpactCandidate> {
    let mut candidates = Vec::new();
    if impact.targeting != content::WeaponTargeting::ThreatsOnly
        && game.current_system_id == impact.system
        && game.ship.systems.hull.current > 0.0
    {
        candidates.push(ImpactCandidate {
            target: WeaponTargetRef::Player,
            position: game.ship.position,
            radius: SHIP_RADIUS,
            hostile_to_owner: matches!(&impact.owner, WeaponImpactOwner::Npc { hostile: true, .. }),
        });
    }
    if impact.targeting != content::WeaponTargeting::ThreatsOnly {
        candidates.extend(
            game.owned_ships
                .iter()
                .filter(|owned| {
                    owned.instance_id != game.active_owned_ship_id
                        && owned.system_id == impact.system
                        && owned.ship.systems.hull.current > 0.0
                })
                .map(|owned| ImpactCandidate {
                    target: WeaponTargetRef::OwnedShip(owned.instance_id.clone()),
                    position: owned.ship.position,
                    radius: SHIP_RADIUS,
                    hostile_to_owner: matches!(
                        &impact.owner,
                        WeaponImpactOwner::Npc { hostile: true, .. }
                    ),
                }),
        );
    }
    for npc in &game.npc_ships {
        if impact.targeting == content::WeaponTargeting::ThreatsOnly
            || npc.system != impact.system
            || npc.hull.current <= 0.0
        {
            continue;
        }
        let npc_is_hostile = npc_ship_is_hostile(&game.content_registry, npc);
        let hostile_to_owner = match &impact.owner {
            WeaponImpactOwner::Player | WeaponImpactOwner::OwnedShip(_) => npc_is_hostile,
            WeaponImpactOwner::Npc { hostile, .. } => {
                if *hostile {
                    !npc_is_hostile
                } else {
                    npc_is_hostile
                }
            }
        };
        candidates.push(ImpactCandidate {
            target: WeaponTargetRef::Npc(npc.id.clone()),
            position: npc.position,
            radius: npc.radius,
            hostile_to_owner,
        });
    }
    for threat in &game.defense_threats {
        if impact.targeting == content::WeaponTargeting::ShipsOnly
            || threat.system != impact.system
            || threat.hull.current <= 0.0
        {
            continue;
        }
        candidates.push(ImpactCandidate {
            target: WeaponTargetRef::Threat(threat.id.clone()),
            position: threat.position,
            radius: threat.radius,
            hostile_to_owner: threat.disposition == ThreatDisposition::Hostile
                && !matches!(&impact.owner, WeaponImpactOwner::Npc { hostile: true, .. }),
        });
    }
    candidates
}

fn impact_target_is_owner(target: &WeaponTargetRef, owner: &WeaponImpactOwner) -> bool {
    matches!(
        (target, owner),
        (WeaponTargetRef::Player, WeaponImpactOwner::Player)
    ) || matches!(
        (target, owner),
        (WeaponTargetRef::OwnedShip(target_id), WeaponImpactOwner::OwnedShip(owner_id))
            if target_id == owner_id
    ) || matches!(
        (target, owner),
        (WeaponTargetRef::Npc(target_id), WeaponImpactOwner::Npc { id, .. }) if target_id == id
    )
}

fn impact_candidate_is_eligible(candidate: &ImpactCandidate, impact: &PendingWeaponImpact) -> bool {
    match impact.friendly_fire {
        content::FriendlyFire::HostilesOnly => candidate.hostile_to_owner,
        content::FriendlyFire::AllExceptOwner => {
            !impact_target_is_owner(&candidate.target, &impact.owner)
        }
        content::FriendlyFire::Everyone => true,
    }
}

fn resolve_weapon_impact(
    game: &mut GameState,
    source_event: &WeaponFireEvent,
    impact: PendingWeaponImpact,
) {
    let candidates = impact_candidates(game, &impact);
    match impact.impact {
        content::WeaponImpact::Single => {
            if candidates.iter().any(|candidate| {
                candidate.target == impact.primary_target
                    && impact_candidate_is_eligible(candidate, &impact)
            }) {
                apply_weapon_damage_to_target(game, &impact.primary_target, impact.damage);
            }
        }
        content::WeaponImpact::Chain => {
            let chain = weapon_chain_targets(&candidates, &impact);
            apply_weapon_chain(game, source_event, &impact, &chain, false);
        }
        content::WeaponImpact::Splash => {
            apply_weapon_splash(
                game,
                source_event.to,
                source_event,
                &impact,
                impact.damage,
                &candidates,
                None,
            );
        }
        content::WeaponImpact::ChainSplash => {
            let chain = weapon_chain_targets(&candidates, &impact);
            apply_weapon_chain(game, source_event, &impact, &chain, true);
        }
    }
}

fn weapon_chain_targets(
    candidates: &[ImpactCandidate],
    impact: &PendingWeaponImpact,
) -> Vec<ImpactCandidate> {
    let Some(primary) = candidates
        .iter()
        .find(|candidate| {
            candidate.target == impact.primary_target
                && impact_candidate_is_eligible(candidate, impact)
        })
        .cloned()
    else {
        return Vec::new();
    };
    let mut chain = vec![primary];
    while chain.len() < impact.chain_targets as usize {
        let from = chain.last().expect("chain has a primary target").position;
        let next = candidates
            .iter()
            .filter(|candidate| {
                impact_candidate_is_eligible(candidate, impact)
                    && !chain
                        .iter()
                        .any(|selected| selected.target == candidate.target)
                    && from.distance(candidate.position) <= impact.chain_range + candidate.radius
            })
            .min_by(|a, b| {
                from.distance_squared(a.position)
                    .total_cmp(&from.distance_squared(b.position))
            })
            .cloned();
        let Some(next) = next else {
            break;
        };
        chain.push(next);
    }
    chain
}

fn apply_weapon_chain(
    game: &mut GameState,
    source_event: &WeaponFireEvent,
    impact: &PendingWeaponImpact,
    chain: &[ImpactCandidate],
    splash_each_target: bool,
) {
    let candidates = impact_candidates(game, impact);
    let mut previous_position = source_event.from;
    let mut damaged = HashSet::new();
    for (index, target) in chain.iter().enumerate() {
        let damage = impact.damage * impact.chain_damage_multiplier.powi(index as i32);
        if splash_each_target {
            apply_weapon_splash(
                game,
                target.position,
                source_event,
                impact,
                damage,
                &candidates,
                Some(&mut damaged),
            );
        } else {
            apply_weapon_damage_to_target(game, &target.target, damage);
        }
        if index > 0 {
            push_chain_visual(game, source_event, previous_position, target.position);
        }
        previous_position = target.position;
    }
}

fn push_chain_visual(game: &mut GameState, source: &WeaponFireEvent, from: Vec2, to: Vec2) {
    let mut link = source.clone();
    link.from = from;
    link.to = to;
    link.duration = (from.distance(to) / 1200.0).clamp(0.08, 0.35);
    link.timer = link.duration;
    link.pending_impact = None;
    game.weapon_fire_events.push(link);
}

fn apply_weapon_splash(
    game: &mut GameState,
    center: Vec2,
    source_event: &WeaponFireEvent,
    impact: &PendingWeaponImpact,
    base_damage: f32,
    candidates: &[ImpactCandidate],
    mut damaged: Option<&mut HashSet<WeaponTargetRef>>,
) {
    for candidate in candidates {
        if !impact_candidate_is_eligible(candidate, impact) {
            continue;
        }
        if damaged
            .as_ref()
            .is_some_and(|targets| targets.contains(&candidate.target))
        {
            continue;
        }
        let distance = (center.distance(candidate.position) - candidate.radius).max(0.0);
        if distance > impact.splash_radius {
            continue;
        }
        let normalized = (distance / impact.splash_radius.max(f32::EPSILON)).clamp(0.0, 1.0);
        let multiplier = splash_damage_multiplier(
            impact.splash_falloff,
            normalized,
            impact.splash_min_multiplier,
        );
        apply_weapon_damage_to_target(game, &candidate.target, base_damage * multiplier);
        if let Some(targets) = damaged.as_deref_mut() {
            targets.insert(candidate.target.clone());
        }
    }
    game.weapon_area_effect_events.push(WeaponAreaEffectEvent {
        position: center,
        radius: impact.splash_radius,
        timer: 0.65,
        duration: 0.65,
        color: source_event.impact_color,
    });
    game.audio.play(audio::AudioCue::Explosion);
}

fn splash_damage_multiplier(
    falloff: content::DamageFalloff,
    normalized_distance: f32,
    minimum: f32,
) -> f32 {
    let remaining = 1.0 - normalized_distance.clamp(0.0, 1.0);
    match falloff {
        content::DamageFalloff::None => 1.0,
        content::DamageFalloff::Linear => minimum + (1.0 - minimum) * remaining,
        content::DamageFalloff::Quadratic => minimum + (1.0 - minimum) * remaining.powi(2),
    }
}

fn apply_weapon_damage_to_target(
    game: &mut GameState,
    target: &WeaponTargetRef,
    damage: f32,
) -> bool {
    match target {
        WeaponTargetRef::Player => apply_ship_weapon_damage(game, damage),
        WeaponTargetRef::OwnedShip(instance_id) => game
            .owned_ships
            .iter_mut()
            .find(|owned| {
                owned.instance_id == *instance_id && owned.ship.systems.hull.current > 0.0
            })
            .is_some_and(|owned| apply_owned_ship_weapon_damage(owned, damage)),
        WeaponTargetRef::Npc(id) => game
            .npc_ships
            .iter_mut()
            .find(|npc| npc.id == *id && npc.hull.current > 0.0)
            .is_some_and(|npc| {
                let before = npc.shields.current + npc.hull.current;
                apply_npc_weapon_damage(npc, damage);
                npc.shields.current + npc.hull.current < before
            }),
        WeaponTargetRef::Threat(id) => game
            .defense_threats
            .iter_mut()
            .find(|threat| threat.id == *id && threat.hull.current > 0.0)
            .is_some_and(|threat| {
                let before = threat.hull.current;
                threat.hull.spend(damage.max(0.0));
                threat.hull.current < before
            }),
    }
}

fn remove_destroyed_npc_ships(game: &mut GameState) {
    if game
        .npc_ships
        .iter()
        .all(|npc_ship| npc_ship.hull.current > 0.0)
    {
        return;
    }

    let previous_selection = game.selected_npc_ship;
    let previous_follow = game.follow_target;
    let mut surviving_npc_ships = Vec::with_capacity(game.npc_ships.len());
    let mut next_selected_npc_ship = None;
    let mut next_follow_target = None;
    let npc_ships = std::mem::take(&mut game.npc_ships);

    for (old_index, npc_ship) in npc_ships.into_iter().enumerate() {
        if npc_ship.hull.current > 0.0 {
            let new_index = surviving_npc_ships.len();
            if previous_selection == Some(old_index) {
                next_selected_npc_ship = Some(new_index);
            }
            if previous_follow == Some(FollowTarget::NpcShip(old_index)) {
                next_follow_target = Some(FollowTarget::NpcShip(new_index));
            }
            surviving_npc_ships.push(npc_ship);
        } else {
            game.audio.play(audio::AudioCue::Explosion);
            let cargo_items = transfer_destroyed_npc_loot(
                &mut game.inventory,
                &game.ship_upgrades,
                &npc_ship.cargo_defaults,
            );
            let credit_reward = destroyed_npc_credit_reward(&game.content_registry, &npc_ship);
            if credit_reward > 0 {
                game.credits = game.credits.saturating_add(credit_reward);
            }
            push_destroyed_npc_loot_feedback(game, &npc_ship.name, cargo_items, credit_reward);
        }
    }

    game.npc_ships = surviving_npc_ships;
    game.selected_npc_ship = next_selected_npc_ship;
    game.context_action_menu = None;
    game.follow_target = next_follow_target;
    if previous_follow.is_some() && next_follow_target.is_none() {
        push_operation_feedback(game, "Follow", "Target destroyed".to_string());
    }
    game.save_dirty = true;
}

fn transfer_destroyed_npc_loot(
    inventory: &mut Inventory,
    ship_upgrades: &[ShipUpgrade; SHIP_UPGRADE_COUNT],
    cargo_defaults: &[ItemStack],
) -> u32 {
    let cargo_capacity = cargo_rating_kg(ship_upgrades);
    let mut cargo_mass = inventory.total_mass();
    let mut transferred = 0;
    for stack in cargo_defaults {
        let stack_mass = stack.item.unit_mass * stack.count as f32;
        if cargo_mass + stack_mass > cargo_capacity {
            continue;
        }

        inventory.add_item(stack.item.clone(), stack.count);
        cargo_mass += stack_mass;
        transferred += stack.count;
    }
    transferred
}

fn destroyed_npc_credit_reward(
    content_registry: &content::ContentRegistry,
    npc_ship: &NpcShip,
) -> u32 {
    if !npc_ship_is_hostile(content_registry, npc_ship) || npc_ship.credit_reward_max == 0 {
        return 0;
    }
    let min = npc_ship.credit_reward_min.min(npc_ship.credit_reward_max);
    let max = npc_ship.credit_reward_max;
    if min == max {
        return min;
    }
    rand::gen_range(min as f32, max.saturating_add(1) as f32).floor() as u32
}

fn push_destroyed_npc_loot_feedback(
    game: &mut GameState,
    npc_name: &str,
    cargo_items: u32,
    credit_reward: u32,
) {
    if cargo_items == 0 && credit_reward == 0 {
        return;
    }

    let cargo_label = match cargo_items {
        0 => None,
        1 => Some("1 cargo".to_string()),
        count => Some(format!("{count} cargo")),
    };
    let credit_label = (credit_reward > 0).then(|| format!("{credit_reward} cr"));
    let parts = [cargo_label, credit_label]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(", ");
    push_operation_feedback(game, "Loot", format!("{npc_name}: {parts}"));
}

fn weapon_fire_event(
    from: Vec2,
    to: Vec2,
    weapon: &WeaponSystem,
    origin: WeaponFireOrigin,
    owner: WeaponImpactOwner,
    primary_target: WeaponTargetRef,
    system: String,
) -> WeaponFireEvent {
    let duration = weapon
        .travel_speed
        .map_or(weapon.fire_duration_seconds, |speed| {
            (from.distance(to) / speed).clamp(0.05, 5.0)
        });
    WeaponFireEvent {
        from,
        to,
        timer: duration,
        duration,
        effect: weapon.effect,
        beam_color: weapon.beam_color,
        core_color: weapon.core_color,
        impact_color: weapon.impact_color,
        path_curve_strength: weapon.path_curve_strength,
        path_wobble: weapon.path_wobble,
        path_cycles: weapon.path_cycles,
        trail_length: weapon.trail_length,
        burst_count: weapon.burst_count,
        projectile_texture: weapon.projectile_texture.clone(),
        projectile_size: weapon.projectile_size,
        origin,
        pending_impact: Some(PendingWeaponImpact {
            owner,
            primary_target,
            system,
            damage: weapon.damage,
            targeting: weapon.targeting,
            impact: weapon.impact,
            splash_radius: weapon.splash_radius,
            splash_falloff: weapon.splash_falloff,
            splash_min_multiplier: weapon.splash_min_multiplier,
            chain_targets: weapon.chain_targets,
            chain_range: weapon.chain_range,
            chain_damage_multiplier: weapon.chain_damage_multiplier,
            friendly_fire: weapon.friendly_fire,
        }),
    }
}

fn update_player_weapon_systems(game: &mut GameState, dt: f32) {
    for weapon_index in 0..game.equipped_weapons.len() {
        if !game.equipped_weapons[weapon_index].resolved {
            continue;
        }
        {
            let weapon = &mut game.equipped_weapons[weapon_index];
            weapon.cooldown_remaining = (weapon.cooldown_remaining - dt).max(0.0);
            if weapon.cooldown_remaining > 0.0 {
                weapon.status = WeaponStatus::Cooldown;
                continue;
            }
        }

        let target = player_turret_target(
            &game.content_registry,
            &game.ship,
            &game.equipped_weapons[weapon_index],
            &game.defense_threats,
            &game.npc_ships,
            &game.current_system_id,
        );
        let Some(target) = target else {
            game.equipped_weapons[weapon_index].status = WeaponStatus::NoThreat;
            continue;
        };

        let ammo_item = game.equipped_weapons[weapon_index].ammo_item.clone();
        let ammo_per_shot = game.equipped_weapons[weapon_index].ammo_per_shot;
        if ammo_item
            .as_deref()
            .is_some_and(|item| game.inventory.count_id(item) < ammo_per_shot)
        {
            game.equipped_weapons[weapon_index].status = WeaponStatus::OutOfAmmo;
            continue;
        }

        let weapon = &mut game.equipped_weapons[weapon_index];
        if game.ship.systems.energy.current < weapon.energy_cost {
            weapon.status = WeaponStatus::InsufficientEnergy;
            continue;
        }

        let (target_position, target_ref) = match target {
            PlayerTurretTarget::DefenseThreat(target_index) => {
                let target = &game.defense_threats[target_index];
                (target.position, WeaponTargetRef::Threat(target.id.clone()))
            }
            PlayerTurretTarget::NpcShip(npc_ship_index) => {
                let target = &game.npc_ships[npc_ship_index];
                (target.position, WeaponTargetRef::Npc(target.id.clone()))
            }
        };
        if let Some(ammo_item) = ammo_item.as_deref() {
            game.inventory.remove_item_id(ammo_item, ammo_per_shot);
        }
        game.ship.systems.energy.spend(weapon.energy_cost);
        weapon.cooldown_remaining = weapon.cooldown_seconds;
        weapon.status = WeaponStatus::Fired;
        game.weapon_fire_events.push(weapon_fire_event(
            game.ship.position,
            target_position,
            weapon,
            WeaponFireOrigin::Player,
            WeaponImpactOwner::Player,
            target_ref,
            game.current_system_id.clone(),
        ));
        game.audio
            .play_custom_or(weapon.fire_audio.as_deref(), audio::AudioCue::WeaponFire);
        game.save_dirty = true;
    }
}

fn update_inactive_owned_ship_weapon_systems(game: &mut GameState, dt: f32) {
    for owned_index in 0..game.owned_ships.len() {
        if game.owned_ships[owned_index].instance_id == game.active_owned_ship_id
            || game.owned_ships[owned_index].system_id != game.current_system_id
            || game.owned_ships[owned_index].ship.systems.hull.current <= 0.0
        {
            continue;
        }

        let weapon_count = game.owned_ships[owned_index].equipped_weapons.len();
        for weapon_index in 0..weapon_count {
            if !game.owned_ships[owned_index].equipped_weapons[weapon_index].resolved {
                continue;
            }
            {
                let weapon = &mut game.owned_ships[owned_index].equipped_weapons[weapon_index];
                weapon.cooldown_remaining = (weapon.cooldown_remaining - dt).max(0.0);
                if weapon.cooldown_remaining > 0.0 {
                    weapon.status = WeaponStatus::Cooldown;
                    continue;
                }
            }

            if !fire_inactive_owned_ship_weapon(game, owned_index, weapon_index) {
                game.owned_ships[owned_index].equipped_weapons[weapon_index].status =
                    WeaponStatus::NoThreat;
            }
        }
    }
}

fn fire_inactive_owned_ship_weapon(
    game: &mut GameState,
    owned_index: usize,
    weapon_index: usize,
) -> bool {
    let Some(owned) = game.owned_ships.get(owned_index) else {
        return false;
    };
    let Some(weapon) = owned.equipped_weapons.get(weapon_index) else {
        return false;
    };
    let Some(target) = player_turret_target(
        &game.content_registry,
        &owned.ship,
        weapon,
        &game.defense_threats,
        &game.npc_ships,
        &owned.system_id,
    ) else {
        return false;
    };
    if owned.ship.systems.energy.current < weapon.energy_cost {
        game.owned_ships[owned_index].equipped_weapons[weapon_index].status =
            WeaponStatus::InsufficientEnergy;
        return true;
    }
    if weapon
        .ammo_item
        .as_deref()
        .is_some_and(|item| owned.inventory.count_id(item) < weapon.ammo_per_shot)
    {
        game.owned_ships[owned_index].equipped_weapons[weapon_index].status =
            WeaponStatus::OutOfAmmo;
        return true;
    }

    let (target_position, target_ref) = match target {
        PlayerTurretTarget::DefenseThreat(target_index) => {
            let target = &game.defense_threats[target_index];
            (target.position, WeaponTargetRef::Threat(target.id.clone()))
        }
        PlayerTurretTarget::NpcShip(npc_ship_index) => {
            let target = &game.npc_ships[npc_ship_index];
            (target.position, WeaponTargetRef::Npc(target.id.clone()))
        }
    };
    let origin = owned.ship.position;
    let owner_id = owned.instance_id.clone();
    let system_id = owned.system_id.clone();
    let energy_cost = weapon.energy_cost;
    let cooldown_seconds = weapon.cooldown_seconds;
    let ammo_item = weapon.ammo_item.clone();
    let ammo_per_shot = weapon.ammo_per_shot;
    let fire_audio = weapon.fire_audio.clone();
    let fire_event = weapon_fire_event(
        origin,
        target_position,
        weapon,
        WeaponFireOrigin::Player,
        WeaponImpactOwner::OwnedShip(owner_id),
        target_ref,
        system_id,
    );

    let owned = &mut game.owned_ships[owned_index];
    owned.ship.systems.energy.spend(energy_cost);
    if let Some(ammo_item) = ammo_item.as_deref() {
        owned.inventory.remove_item_id(ammo_item, ammo_per_shot);
    }
    owned.equipped_weapons[weapon_index].cooldown_remaining = cooldown_seconds;
    owned.equipped_weapons[weapon_index].status = WeaponStatus::Fired;
    game.weapon_fire_events.push(fire_event);
    game.audio
        .play_custom_or(fire_audio.as_deref(), audio::AudioCue::WeaponFire);
    game.save_dirty = true;
    true
}

fn update_npc_weapon_systems(game: &mut GameState, dt: f32) {
    for npc_index in 0..game.npc_ships.len() {
        if game.npc_ships[npc_index].system != game.current_system_id
            || game.npc_ships[npc_index].hull.current <= 0.0
        {
            continue;
        }

        let hostile = npc_ship_is_hostile(&game.content_registry, &game.npc_ships[npc_index]);
        let weapon_count = game.npc_ships[npc_index].equipped_weapons.len();
        for weapon_index in 0..weapon_count {
            if !game.npc_ships[npc_index].equipped_weapons[weapon_index].resolved {
                continue;
            }
            {
                let weapon = &mut game.npc_ships[npc_index].equipped_weapons[weapon_index];
                weapon.cooldown_remaining = (weapon.cooldown_remaining - dt).max(0.0);
                if weapon.cooldown_remaining > 0.0 {
                    weapon.status = WeaponStatus::Cooldown;
                    continue;
                }
            }

            if hostile && fire_npc_weapon_at_owned_ship(game, npc_index, weapon_index) {
                continue;
            }
            if !hostile && fire_npc_weapon_at_defense_threat(game, npc_index, weapon_index) {
                continue;
            }

            game.npc_ships[npc_index].equipped_weapons[weapon_index].status =
                WeaponStatus::NoThreat;
        }
    }
}

fn npc_weapon_has_ammo(npc_ship: &NpcShip, weapon: &WeaponSystem) -> bool {
    weapon.ammo_item.as_deref().is_none_or(|ammo_item| {
        npc_ship
            .cargo_defaults
            .iter()
            .filter(|stack| stack.item.id == ammo_item)
            .map(|stack| stack.count)
            .sum::<u32>()
            >= weapon.ammo_per_shot
    })
}

fn consume_npc_weapon_ammo(npc_ship: &mut NpcShip, ammo_item: &str, mut count: u32) {
    for stack in &mut npc_ship.cargo_defaults {
        if count == 0 {
            break;
        }
        if stack.item.id == ammo_item {
            let removed = stack.count.min(count);
            stack.count -= removed;
            count -= removed;
        }
    }
    npc_ship.cargo_defaults.retain(|stack| stack.count > 0);
}

fn fire_npc_weapon_at_owned_ship(
    game: &mut GameState,
    npc_index: usize,
    weapon_index: usize,
) -> bool {
    let Some(npc_ship) = game.npc_ships.get(npc_index) else {
        return false;
    };
    let Some(weapon) = npc_ship.equipped_weapons.get(weapon_index) else {
        return false;
    };
    let Some((target_ref, target_position)) = hostile_npc_owned_ship_target(game, npc_ship, weapon)
    else {
        return false;
    };
    if npc_ship.energy.current < weapon.energy_cost {
        game.npc_ships[npc_index].equipped_weapons[weapon_index].status =
            WeaponStatus::InsufficientEnergy;
        return true;
    }
    if !npc_weapon_has_ammo(npc_ship, weapon) {
        game.npc_ships[npc_index].equipped_weapons[weapon_index].status = WeaponStatus::OutOfAmmo;
        return true;
    }

    let origin = npc_ship.position;
    let energy_cost = weapon.energy_cost;
    let cooldown_seconds = weapon.cooldown_seconds;
    let ammo_item = weapon.ammo_item.clone();
    let ammo_per_shot = weapon.ammo_per_shot;
    let owner = WeaponImpactOwner::Npc {
        id: npc_ship.id.clone(),
        hostile: npc_ship_is_hostile(&game.content_registry, npc_ship),
    };
    let fire_event = weapon_fire_event(
        origin,
        target_position,
        weapon,
        WeaponFireOrigin::Npc,
        owner,
        target_ref,
        game.current_system_id.clone(),
    );
    let fire_audio = weapon.fire_audio.clone();
    game.npc_ships[npc_index].energy.spend(energy_cost);
    if let Some(ammo_item) = ammo_item.as_deref() {
        consume_npc_weapon_ammo(&mut game.npc_ships[npc_index], ammo_item, ammo_per_shot);
        game.save_dirty = true;
    }
    game.npc_ships[npc_index].equipped_weapons[weapon_index].cooldown_remaining = cooldown_seconds;
    game.npc_ships[npc_index].equipped_weapons[weapon_index].status = WeaponStatus::Fired;
    game.weapon_fire_events.push(fire_event);
    game.audio
        .play_custom_or(fire_audio.as_deref(), audio::AudioCue::WeaponFire);
    true
}

fn hostile_npc_owned_ship_target(
    game: &GameState,
    npc_ship: &NpcShip,
    weapon: &WeaponSystem,
) -> Option<(WeaponTargetRef, Vec2)> {
    let active = npc_weapon_can_target_ship(npc_ship, weapon, &game.ship, &game.current_system_id)
        .then(|| {
            (
                WeaponTargetRef::Player,
                game.ship.position,
                npc_ship.position.distance_squared(game.ship.position),
            )
        });
    let inactive = game
        .owned_ships
        .iter()
        .filter(|owned| {
            owned.instance_id != game.active_owned_ship_id
                && owned.system_id == game.current_system_id
                && npc_weapon_can_target_ship(npc_ship, weapon, &owned.ship, &owned.system_id)
        })
        .map(|owned| {
            (
                WeaponTargetRef::OwnedShip(owned.instance_id.clone()),
                owned.ship.position,
                npc_ship.position.distance_squared(owned.ship.position),
            )
        })
        .min_by(|(_, _, a), (_, _, b)| a.total_cmp(b));

    [active, inactive]
        .into_iter()
        .flatten()
        .min_by(|(_, _, a), (_, _, b)| a.total_cmp(b))
        .map(|(target, position, _)| (target, position))
}

fn fire_npc_weapon_at_defense_threat(
    game: &mut GameState,
    npc_index: usize,
    weapon_index: usize,
) -> bool {
    let Some(npc_ship) = game.npc_ships.get(npc_index) else {
        return false;
    };
    let Some(weapon) = npc_ship.equipped_weapons.get(weapon_index) else {
        return false;
    };
    let Some(target_index) = npc_defense_turret_target_index(
        npc_ship,
        weapon,
        &game.defense_threats,
        &game.current_system_id,
    ) else {
        return false;
    };
    if npc_ship.energy.current < weapon.energy_cost {
        game.npc_ships[npc_index].equipped_weapons[weapon_index].status =
            WeaponStatus::InsufficientEnergy;
        return true;
    }
    if !npc_weapon_has_ammo(npc_ship, weapon) {
        game.npc_ships[npc_index].equipped_weapons[weapon_index].status = WeaponStatus::OutOfAmmo;
        return true;
    }

    let origin = npc_ship.position;
    let target_position = game.defense_threats[target_index].position;
    let target_ref = WeaponTargetRef::Threat(game.defense_threats[target_index].id.clone());
    let energy_cost = weapon.energy_cost;
    let cooldown_seconds = weapon.cooldown_seconds;
    let ammo_item = weapon.ammo_item.clone();
    let ammo_per_shot = weapon.ammo_per_shot;
    let owner = WeaponImpactOwner::Npc {
        id: npc_ship.id.clone(),
        hostile: npc_ship_is_hostile(&game.content_registry, npc_ship),
    };
    let fire_event = weapon_fire_event(
        origin,
        target_position,
        weapon,
        WeaponFireOrigin::Npc,
        owner,
        target_ref,
        game.current_system_id.clone(),
    );
    let fire_audio = weapon.fire_audio.clone();
    game.npc_ships[npc_index].energy.spend(energy_cost);
    if let Some(ammo_item) = ammo_item.as_deref() {
        consume_npc_weapon_ammo(&mut game.npc_ships[npc_index], ammo_item, ammo_per_shot);
        game.save_dirty = true;
    }
    game.npc_ships[npc_index].equipped_weapons[weapon_index].cooldown_remaining = cooldown_seconds;
    game.npc_ships[npc_index].equipped_weapons[weapon_index].status = WeaponStatus::Fired;
    game.weapon_fire_events.push(fire_event);
    game.audio
        .play_custom_or(fire_audio.as_deref(), audio::AudioCue::WeaponFire);
    true
}

fn apply_npc_weapon_damage(npc_ship: &mut NpcShip, damage: f32) {
    let damage = damage.max(0.0);
    let shield_absorbed = damage.min(npc_ship.shields.current);
    npc_ship.shields.spend(shield_absorbed);

    let spillover = damage - shield_absorbed;
    if spillover > 0.0 {
        npc_ship.hull.spend(spillover);
    }
}

fn player_turret_target(
    content_registry: &content::ContentRegistry,
    ship: &Ship,
    weapon: &WeaponSystem,
    threats: &[DefenseThreat],
    npc_ships: &[NpcShip],
    current_system_id: &str,
) -> Option<PlayerTurretTarget> {
    if weapon.kind != content::WeaponKind::TurretDefense {
        return None;
    }

    let threat_target = (weapon.targeting != content::WeaponTargeting::ShipsOnly)
        .then(|| defense_turret_target_index(ship, weapon, threats, current_system_id))
        .flatten()
        .map(|index| {
            (
                PlayerTurretTarget::DefenseThreat(index),
                threats[index].position.distance_squared(ship.position),
            )
        });
    let npc_target = (weapon.targeting != content::WeaponTargeting::ThreatsOnly)
        .then(|| {
            hostile_npc_turret_target_index(
                content_registry,
                ship,
                weapon,
                npc_ships,
                current_system_id,
            )
        })
        .flatten()
        .map(|index| {
            (
                PlayerTurretTarget::NpcShip(index),
                npc_ships[index].position.distance_squared(ship.position),
            )
        });

    [threat_target, npc_target]
        .into_iter()
        .flatten()
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(target, _)| target)
}

fn defense_turret_target_index(
    ship: &Ship,
    weapon: &WeaponSystem,
    threats: &[DefenseThreat],
    current_system_id: &str,
) -> Option<usize> {
    if weapon.kind != content::WeaponKind::TurretDefense
        || weapon.targeting == content::WeaponTargeting::ShipsOnly
    {
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

fn hostile_npc_turret_target_index(
    content_registry: &content::ContentRegistry,
    ship: &Ship,
    weapon: &WeaponSystem,
    npc_ships: &[NpcShip],
    current_system_id: &str,
) -> Option<usize> {
    if weapon.kind != content::WeaponKind::TurretDefense
        || weapon.targeting == content::WeaponTargeting::ThreatsOnly
    {
        return None;
    }

    npc_ships
        .iter()
        .enumerate()
        .filter(|(_, npc_ship)| {
            hostile_npc_is_valid_player_turret_target(
                content_registry,
                ship,
                weapon,
                npc_ship,
                current_system_id,
            )
        })
        .min_by(|(_, a), (_, b)| {
            a.position
                .distance_squared(ship.position)
                .total_cmp(&b.position.distance_squared(ship.position))
        })
        .map(|(index, _)| index)
}

fn npc_defense_turret_target_index(
    npc_ship: &NpcShip,
    weapon: &WeaponSystem,
    threats: &[DefenseThreat],
    current_system_id: &str,
) -> Option<usize> {
    if weapon.kind != content::WeaponKind::TurretDefense
        || weapon.targeting == content::WeaponTargeting::ShipsOnly
    {
        return None;
    }

    threats
        .iter()
        .enumerate()
        .filter(|(_, threat)| {
            npc_defense_threat_is_valid_target(npc_ship, weapon, threat, current_system_id)
        })
        .min_by(|(_, a), (_, b)| {
            a.position
                .distance_squared(npc_ship.position)
                .total_cmp(&b.position.distance_squared(npc_ship.position))
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

fn hostile_npc_is_valid_player_turret_target(
    content_registry: &content::ContentRegistry,
    ship: &Ship,
    weapon: &WeaponSystem,
    npc_ship: &NpcShip,
    current_system_id: &str,
) -> bool {
    npc_ship_is_hostile(content_registry, npc_ship)
        && npc_ship.system == current_system_id
        && npc_ship.hull.current > 0.0
        && ship.position.distance(npc_ship.position) <= weapon.range + npc_ship.radius
        && target_within_tracking_arc(ship, weapon, npc_ship.position)
}

fn npc_defense_threat_is_valid_target(
    npc_ship: &NpcShip,
    weapon: &WeaponSystem,
    threat: &DefenseThreat,
    current_system_id: &str,
) -> bool {
    threat.disposition == ThreatDisposition::Hostile
        && threat.system == current_system_id
        && threat.hull.current > 0.0
        && npc_ship.position.distance(threat.position) <= weapon.range + threat.radius
        && target_within_tracking_arc_from(
            npc_ship.position,
            npc_ship.angle,
            weapon,
            threat.position,
        )
}

fn npc_weapon_can_target_ship(
    npc_ship: &NpcShip,
    weapon: &WeaponSystem,
    ship: &Ship,
    current_system_id: &str,
) -> bool {
    weapon.kind == content::WeaponKind::TurretDefense
        && weapon.targeting != content::WeaponTargeting::ThreatsOnly
        && npc_ship.system == current_system_id
        && npc_ship.hull.current > 0.0
        && ship.systems.hull.current > 0.0
        && npc_ship.position.distance(ship.position) <= weapon.range + SHIP_RADIUS + npc_ship.radius
        && target_within_tracking_arc_from(npc_ship.position, npc_ship.angle, weapon, ship.position)
}

fn target_within_tracking_arc(ship: &Ship, weapon: &WeaponSystem, target_position: Vec2) -> bool {
    target_within_tracking_arc_from(ship.position, ship.angle, weapon, target_position)
}

fn target_within_tracking_arc_from(
    source_position: Vec2,
    source_angle: f32,
    weapon: &WeaponSystem,
    target_position: Vec2,
) -> bool {
    if weapon.tracking_degrees >= 359.0 {
        return true;
    }
    let to_target = target_position - source_position;
    if to_target.length_squared() <= f32::EPSILON {
        return true;
    }
    let forward = vec2(source_angle.cos(), source_angle.sin());
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
    let mut egui_pointer_captured = false;
    egui_macroquad::cfg(|ctx| egui_pointer_captured = ctx.wants_pointer_input());
    let mouse = mouse_vec2();
    let adjusting_panel =
        game.starmap_move_previous_mouse.is_some() || game.starmap_resize_previous_mouse.is_some();
    if egui_pointer_captured && !adjusting_panel {
        game.starmap_drag_previous_mouse = None;
        game.starmap_move_previous_mouse = None;
        game.starmap_resize_previous_mouse = None;
        return;
    }
    if handle_starmap_panel_geometry_input(game, mouse) {
        game.starmap_drag_previous_mouse = None;
        return;
    }
    let (map_x, map_y, map_width, map_height) = starmap_panel_rect(game);
    let mouse_over_map = Rect::new(map_x, map_y, map_width, map_height).contains(mouse);
    if !mouse_over_map {
        game.starmap_drag_previous_mouse = None;
        return;
    }

    if wheel > 0.0 {
        game.starmap_zoom = (game.starmap_zoom * STARMAP_ZOOM_STEP).min(STARMAP_ZOOM_MAX);
    } else if wheel < 0.0 {
        game.starmap_zoom = (game.starmap_zoom / STARMAP_ZOOM_STEP).max(STARMAP_ZOOM_MIN);
    }

    if is_key_pressed(KeyCode::Home) {
        game.starmap_pan = Vec2::ZERO;
    }

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

fn handle_starmap_panel_geometry_input(game: &mut GameState, mouse: Vec2) -> bool {
    if is_mouse_button_down(MouseButton::Left) {
        if let Some(previous_mouse) = game.starmap_resize_previous_mouse {
            let (_, _, width, height) = starmap_panel_rect(game);
            let resized = starmap_panel_size_for(
                vec2(screen_width(), screen_height()),
                Some(vec2(width, height) + mouse - previous_mouse),
            );
            game.starmap_panel_size = Some(resized);
            game.starmap_resize_previous_mouse = Some(mouse);
            return true;
        }
        if let Some(previous_mouse) = game.starmap_move_previous_mouse {
            let (x, y, width, height) = starmap_panel_rect(game);
            let position = starmap_panel_position_for(
                vec2(screen_width(), screen_height()),
                vec2(width, height),
                Some(vec2(x, y) + mouse - previous_mouse),
            );
            game.starmap_panel_position = Some(position);
            game.starmap_move_previous_mouse = Some(mouse);
            return true;
        }
        if starmap_resize_handle_rect(game).contains(mouse) {
            let (_, _, width, height) = starmap_panel_rect(game);
            game.starmap_panel_size = Some(vec2(width, height));
            game.starmap_resize_previous_mouse = Some(mouse);
            return true;
        }
        if starmap_move_handle_rect(game).contains(mouse) {
            let (x, y, _, _) = starmap_panel_rect(game);
            game.starmap_panel_position = Some(vec2(x, y));
            game.starmap_move_previous_mouse = Some(mouse);
            return true;
        }
    } else if game.starmap_move_previous_mouse.take().is_some()
        || game.starmap_resize_previous_mouse.take().is_some()
    {
        let (x, y, width, height) = starmap_panel_rect(game);
        let position = vec2(x, y);
        let size = vec2(width, height);
        game.starmap_panel_position = Some(position);
        game.starmap_panel_size = Some(size);
        save_starmap_panel_geometry(position, size);
        return true;
    }
    false
}

fn default_camera_zoom() -> f32 {
    1.0
}

fn default_credits() -> u32 {
    500
}

fn default_next_owned_ship_sequence() -> u64 {
    2
}

fn default_current_system_id() -> String {
    STARTER_SYSTEM_ID.to_string()
}

fn default_starter_ship_id() -> String {
    STARTER_SHIP_ID.to_string()
}

fn resolved_new_game_ship_id(
    registry: &content::ContentRegistry,
    requested_ship_id: Option<&str>,
) -> String {
    requested_ship_id
        .filter(|ship_id| registry.ships.contains_key(*ship_id))
        .map(str::to_owned)
        .or_else(|| {
            registry
                .starter_ship
                .as_ref()
                .filter(|ship_id| registry.ships.contains_key(*ship_id))
                .cloned()
        })
        .or_else(|| {
            registry
                .ships
                .contains_key(STARTER_SHIP_ID)
                .then(default_starter_ship_id)
        })
        .or_else(|| registry.ship_order.first().cloned())
        .unwrap_or_else(default_starter_ship_id)
}

fn default_new_game_ship_id() -> String {
    let registry = load_game_content_registry();
    resolved_new_game_ship_id(&registry, None)
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

fn draw_scene(game: &GameState, background: &UniverseBackground, panel_corner: Option<&Texture2D>) {
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
    for owned in game.owned_ships.iter().filter(|owned| {
        owned.instance_id != game.active_owned_ship_id && owned.system_id == game.current_system_id
    }) {
        draw_inactive_owned_ship(
            center,
            ship,
            owned,
            game.ship_textures.get(&owned.ship_id),
            zoom,
        );
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
    draw_ship(
        center,
        ship,
        game.ship_textures.get(&game.ship_id),
        zoom,
        game.approach_target.is_some() || game.follow_target.is_some(),
    );
    for event in &game.weapon_fire_events {
        draw_weapon_fire_event(center, ship, event, zoom, &game.weapon_projectile_textures);
    }
    for event in &game.weapon_area_effect_events {
        draw_weapon_area_effect_event(center, ship, event, zoom);
    }
    for event in &game.collision_impact_events {
        draw_collision_impact_event(center, ship, event, zoom);
    }
    let turn = ship.angular_velocity;
    draw_hud(HudView {
        ship,
        planets: &game.planets,
        stations: &game.stations,
        npc_ships: &game.npc_ships,
        pressure_contacts: active_hostile_pressure_count(
            &game.content_registry,
            ship,
            &game.npc_ships,
            &game.current_system_id,
        ),
        incoming_weapon_fire: incoming_weapon_fire_count(ship, &game.weapon_fire_events),
        collision_warning: game
            .collision_impact_events
            .iter()
            .find(|event| event.severity >= CollisionImpactSeverity::Critical),
        selected_planet: game.selected_planet,
        selected_station: game.selected_station,
        selected_npc_ship: game.selected_npc_ship,
        destination_planet: game.destination_planet,
        orbiting_planet: game.orbiting_planet,
        current_system_id: &game.current_system_id,
        speed,
        turn,
    });
    draw_inventory_hint(game);
    draw_interaction_prompt(game);

    if game.map_open {
        draw_starmap_overlay(game, panel_corner);
    }
    if game.escape_dialog_open {
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 145),
        );
    }
    if game.save_status_timer > 0.0 {
        draw_save_confirmation(game.save_status_timer, game.save_status_manual);
    }
    if game.runtime_flags.debug {
        draw_debug_console_hint(game);
    }
    if let Some(transition) = &game.scene_transition {
        draw_scene_transition_overlay(transition);
    }
}

fn nearby_garage_index(game: &GameState) -> Option<usize> {
    game.stations.iter().position(|station| {
        station.system == game.current_system_id
            && station_in_interaction_range(&game.ship, station)
            && station.services.iter().any(|service| {
                service.kind == "garage" && station_service_is_available(game, station, service)
            })
    })
}

fn draw_debug_console_hint(game: &GameState) {
    if game.debug_console.open {
        return;
    }
    let hint_color = Color::from_rgba(150, 221, 226, 210);
    draw_rectangle(
        18.0,
        screen_height() - 46.0,
        210.0,
        28.0,
        Color::from_rgba(5, 10, 16, 160),
    );
    draw_rectangle_lines(
        18.0,
        screen_height() - 46.0,
        210.0,
        28.0,
        1.0,
        Color::from_rgba(95, 137, 155, 120),
    );
    draw_text(
        "Debug console ` or F12",
        30.0,
        screen_height() - 27.0,
        16.0,
        hint_color,
    );
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

fn draw_texture_source_contain(texture: &Texture2D, source: Rect, rect: Rect, opacity: f32) {
    let opacity = opacity.clamp(0.0, 1.0);
    let dest_size = contained_texture_size(vec2(source.w, source.h), vec2(rect.w, rect.h));
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

fn contained_texture_size(source: Vec2, bounds: Vec2) -> Vec2 {
    if source.x <= 0.0 || source.y <= 0.0 || bounds.x <= 0.0 || bounds.y <= 0.0 {
        return Vec2::ZERO;
    }
    let scale = (bounds.x / source.x).min(bounds.y / source.y);
    source * scale
}

fn draw_rotated_texture_contain(texture: &Texture2D, rect: Rect, tint: Color, rotation: f32) {
    let dest_size = contained_texture_size(
        vec2(texture.width(), texture.height()),
        vec2(rect.w, rect.h),
    );
    let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
    let position = center - dest_size * 0.5;
    draw_texture_ex(
        texture,
        position.x,
        position.y,
        tint,
        DrawTextureParams {
            dest_size: Some(dest_size),
            rotation,
            pivot: Some(center),
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
        (mouse.distance(screen_pos) <= planet_interaction_radius(planet) * zoom).then_some(index)
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
            (mouse.distance(screen_pos) <= station_interaction_radius(station) * zoom)
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
            let hit_radius = npc_ship_click_radius(npc_ship, zoom);
            (mouse.distance(screen_pos) <= hit_radius)
                .then_some((index, mouse.distance_squared(screen_pos)))
        })
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index)
}

fn npc_ship_click_radius(npc_ship: &NpcShip, zoom: f32) -> f32 {
    (npc_ship.radius * zoom).clamp(14.0, 36.0) + 6.0
}

fn clicked_starmap_planet_index(mouse: Vec2, game: &GameState) -> Option<usize> {
    let (x, y, width, height) = starmap_panel_rect(game);
    if mouse.x < x || mouse.x > x + width || mouse.y < y || mouse.y > y + height {
        return None;
    }
    if known_systems_panel_rect(game).contains(mouse) {
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
    if !known_systems_panel_rect(game).contains(mouse) {
        return None;
    }

    known_system_ids(&game.content_registry)
        .iter()
        .enumerate()
        .find_map(|(index, system_id)| {
            known_system_row_rect(game, index)
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

    begin_approach(game, ApproachTarget::PlanetOrbit(planet_index));
    game.selected_planet = Some(planet_index);
    game.destination_planet = None;
    remember_current_system_destination(game);
    push_operation_feedback(game, "Orbit", "Flying to orbital position".to_string());
    true
}

fn break_planet_orbit(game: &mut GameState) {
    if matches!(game.approach_target, Some(ApproachTarget::PlanetOrbit(_))) {
        game.approach_target = None;
    }
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
        draw_rotated_texture_contain(
            texture,
            Rect::new(
                screen_pos.x - size * 0.5,
                screen_pos.y - size * 0.5,
                size,
                size,
            ),
            Color { a: 0.92, ..WHITE },
            npc_ship.angle + std::f32::consts::FRAC_PI_2,
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
        npc_ship
            .equipped_weapons
            .len()
            .max(npc_ship.weapon_slots.len())
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

fn draw_weapon_fire_event(
    center: Vec2,
    ship: &Ship,
    event: &WeaponFireEvent,
    zoom: f32,
    projectile_textures: &HashMap<String, Texture2D>,
) {
    let from = world_to_screen(event.from, center, ship, zoom);
    let to = world_to_screen(event.to, center, ship, zoom);
    let alpha = (event.timer / event.duration.max(f32::EPSILON)).clamp(0.0, 1.0);
    let travel = 1.0 - alpha;
    let beam = weapon_effect_color(event.beam_color, alpha);
    let core = weapon_effect_color(event.core_color, alpha);
    let impact = weapon_effect_color(event.impact_color, alpha * 0.9);
    let path_direction = match event.origin {
        WeaponFireOrigin::Player => 1.0,
        WeaponFireOrigin::Npc => -1.0,
    };
    let projectile_texture = event
        .projectile_texture
        .as_ref()
        .and_then(|path| projectile_textures.get(path));
    if event.effect == content::WeaponEffect::Beam {
        draw_line(from.x, from.y, to.x, to.y, 7.0, beam);
        draw_line(from.x, from.y, to.x, to.y, 2.5, core);
    }
    let projectile_count = if event.effect == content::WeaponEffect::Burst {
        event.burst_count.max(1)
    } else {
        1
    };
    let elapsed = event.duration - event.timer;
    let trail_steps = 9;
    for projectile_index in 0..projectile_count {
        let lane = projectile_index as f32 - (projectile_count - 1) as f32 * 0.5;
        let head = weapon_effect_path_point(from, to, event, travel, elapsed, path_direction, lane);
        if event.effect != content::WeaponEffect::Beam {
            for step in 0..trail_steps {
                let trail_step = event.trail_length / trail_steps as f32;
                let trail_end_t = (travel - step as f32 * trail_step).clamp(0.0, 1.0);
                let trail_start_t = (trail_end_t - trail_step * 1.15).clamp(0.0, 1.0);
                if trail_end_t <= 0.0 {
                    continue;
                }
                let trail_start = weapon_effect_path_point(
                    from,
                    to,
                    event,
                    trail_start_t,
                    elapsed,
                    path_direction,
                    lane,
                );
                let trail_end = weapon_effect_path_point(
                    from,
                    to,
                    event,
                    trail_end_t,
                    elapsed,
                    path_direction,
                    lane,
                );
                let fade = alpha * (1.0 - step as f32 / trail_steps as f32).powf(1.4);
                draw_line(
                    trail_start.x,
                    trail_start.y,
                    trail_end.x,
                    trail_end.y,
                    (6.0 - step as f32 * 0.45).max(1.2),
                    Color { a: fade, ..beam },
                );
            }
        }
        if let Some(texture) = projectile_texture {
            draw_weapon_projectile_sprite(
                texture,
                head,
                from,
                to,
                event,
                travel,
                elapsed,
                path_direction,
                lane,
                event.projectile_size * zoom,
                alpha,
            );
        } else if event.effect != content::WeaponEffect::Beam {
            draw_circle(head.x, head.y, 5.0 + alpha * 5.0, beam);
            draw_circle(head.x, head.y, 2.6 + alpha * 2.2, core);
        }
    }
    if travel > 0.72 {
        let flare = ((travel - 0.72) / 0.28).clamp(0.0, 1.0) * alpha;
        draw_circle(to.x, to.y, 8.0 + flare * 13.0, Color { a: flare, ..impact });
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_weapon_projectile_sprite(
    texture: &Texture2D,
    head: Vec2,
    from: Vec2,
    to: Vec2,
    event: &WeaponFireEvent,
    travel: f32,
    elapsed: f32,
    path_direction: f32,
    lane: f32,
    requested_length: f32,
    alpha: f32,
) {
    let previous = weapon_effect_path_point(
        from,
        to,
        event,
        (travel - 0.015).max(0.0),
        elapsed,
        path_direction,
        lane,
    );
    let direction = safe_direction(head - previous, to - from);
    let length = requested_length.clamp(12.0, 96.0);
    let height = length * texture.height() / texture.width().max(1.0);
    draw_texture_ex(
        texture,
        head.x - length * 0.5,
        head.y - height * 0.5,
        Color::new(1.0, 1.0, 1.0, alpha.clamp(0.25, 1.0)),
        DrawTextureParams {
            dest_size: Some(vec2(length, height)),
            rotation: direction.y.atan2(direction.x),
            ..Default::default()
        },
    );
}

fn draw_weapon_area_effect_event(
    center: Vec2,
    ship: &Ship,
    event: &WeaponAreaEffectEvent,
    zoom: f32,
) {
    let position = world_to_screen(event.position, center, ship, zoom);
    let life = (event.timer / event.duration.max(f32::EPSILON)).clamp(0.0, 1.0);
    let progress = 1.0 - life;
    let radius = (event.radius * zoom * progress.sqrt()).max(2.0);
    let color = weapon_effect_color(event.color, life * 0.7);
    draw_circle(
        position.x,
        position.y,
        radius,
        Color {
            a: life * 0.08,
            ..color
        },
    );
    draw_circle_lines(position.x, position.y, radius, 3.0, color);
    draw_circle_lines(
        position.x,
        position.y,
        radius * 0.72,
        1.5,
        Color {
            a: life * 0.45,
            ..color
        },
    );
}

fn weapon_effect_path_point(
    from: Vec2,
    to: Vec2,
    event: &WeaponFireEvent,
    t: f32,
    elapsed: f32,
    path_direction: f32,
    burst_lane: f32,
) -> Vec2 {
    let t = t.clamp(0.0, 1.0);
    let delta = to - from;
    let distance = delta.length().max(1.0);
    let normal = vec2(-delta.y, delta.x).normalize_or_zero();
    let envelope = (std::f32::consts::PI * t).sin();
    let phase = std::f32::consts::TAU * event.path_cycles * t;
    let base = from.lerp(to, t);
    let offset = match event.effect {
        content::WeaponEffect::Arc => {
            let curve = distance * event.path_curve_strength * envelope * path_direction;
            let shimmer = (elapsed * 18.0 + distance * 0.03).sin() * event.path_wobble * envelope;
            curve + shimmer
        }
        content::WeaponEffect::Spiral => {
            let radius =
                (distance * event.path_curve_strength * 0.35).max(event.path_wobble) * envelope;
            phase.sin() * radius * path_direction
        }
        content::WeaponEffect::Zigzag => {
            let triangle = (phase.sin().asin() * (2.0 / std::f32::consts::PI)).clamp(-1.0, 1.0);
            triangle * event.path_wobble * envelope * path_direction
        }
        content::WeaponEffect::Homing => {
            let early_curve = 4.0 * t * (1.0 - t).powi(2);
            distance * event.path_curve_strength * early_curve * path_direction
                + phase.sin() * event.path_wobble * (1.0 - t) * envelope
        }
        content::WeaponEffect::Burst => {
            burst_lane * event.path_wobble * envelope
                + phase.sin() * event.path_wobble * 0.2 * path_direction
        }
        content::WeaponEffect::Straight | content::WeaponEffect::Beam => 0.0,
    };
    base + normal * offset
}

fn weapon_effect_color(rgba: [u8; 4], alpha: f32) -> Color {
    Color::new(
        rgba[0] as f32 / 255.0,
        rgba[1] as f32 / 255.0,
        rgba[2] as f32 / 255.0,
        rgba[3] as f32 / 255.0 * alpha,
    )
}

fn draw_collision_impact_event(center: Vec2, ship: &Ship, event: &CollisionImpactEvent, zoom: f32) {
    let screen_pos = world_to_screen(event.position, center, ship, zoom);
    let alpha = (event.timer / COLLISION_EVENT_SECONDS).clamp(0.0, 1.0);
    let progress = 1.0 - alpha;
    let (outer, core) = match event.severity {
        CollisionImpactSeverity::Minor => (
            Color::new(0.35, 0.80, 1.0, alpha * 0.75),
            Color::new(0.80, 0.98, 1.0, alpha),
        ),
        CollisionImpactSeverity::Significant => (
            Color::new(0.98, 0.72, 0.25, alpha * 0.85),
            Color::new(1.0, 0.94, 0.62, alpha),
        ),
        CollisionImpactSeverity::Critical | CollisionImpactSeverity::Destructive => (
            Color::new(1.0, 0.25, 0.18, alpha * 0.9),
            Color::new(1.0, 0.82, 0.48, alpha),
        ),
    };
    let radius = (10.0 + progress * 28.0) * zoom.clamp(0.45, 2.0);
    draw_circle_lines(screen_pos.x, screen_pos.y, radius, 3.0, outer);
    draw_circle(
        screen_pos.x,
        screen_pos.y,
        (3.0 + alpha * 6.0) * zoom.clamp(0.45, 2.0),
        core,
    );
    if event.hull_damage > 0.0 {
        draw_line(
            screen_pos.x - radius,
            screen_pos.y - radius,
            screen_pos.x + radius,
            screen_pos.y + radius,
            2.0,
            outer,
        );
        draw_line(
            screen_pos.x + radius,
            screen_pos.y - radius,
            screen_pos.x - radius,
            screen_pos.y + radius,
            2.0,
            outer,
        );
    }
}

fn incoming_weapon_fire_count(ship: &Ship, weapon_fire_events: &[WeaponFireEvent]) -> usize {
    weapon_fire_events
        .iter()
        .filter(|event| event.to.distance(ship.position) <= SHIP_RADIUS + 12.0)
        .count()
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

fn draw_ship(
    center: Vec2,
    ship: &Ship,
    texture: Option<&Texture2D>,
    zoom: f32,
    autopilot_thrusting: bool,
) {
    let thrusting = autopilot_thrusting || is_key_down(KeyCode::W) || is_key_down(KeyCode::S);
    draw_ship_sprite(
        center,
        texture,
        ship_screen_size(zoom),
        thrusting,
        ship_screen_rotation(ship),
    );
}

fn draw_inactive_owned_ship(
    center: Vec2,
    active_ship: &Ship,
    owned: &OwnedShip,
    texture: Option<&Texture2D>,
    zoom: f32,
) {
    let screen_pos = world_to_screen(owned.ship.position, center, active_ship, zoom);
    let size = ship_screen_size(zoom);
    let cull_padding = size + 120.0;
    if screen_pos.x < -cull_padding
        || screen_pos.x > screen_width() + cull_padding
        || screen_pos.y < -cull_padding
        || screen_pos.y > screen_height() + cull_padding
    {
        return;
    }

    let destroyed = owned.ship.systems.hull.current <= 0.0;
    let fleet_color = if destroyed {
        Color::from_rgba(220, 126, 116, 185)
    } else {
        Color::from_rgba(150, 221, 226, 185)
    };
    draw_circle_lines(screen_pos.x, screen_pos.y, size * 0.58, 1.5, fleet_color);
    draw_ship_sprite(
        screen_pos,
        texture,
        size,
        false,
        ship_screen_rotation(&owned.ship),
    );
    draw_ship_status_arcs(screen_pos, &owned.ship, zoom);

    let label = fit_debug_text(&owned.callsign, 180.0, 15);
    let label_size = measure_text(&label, None, 15, 1.0);
    draw_text(
        &label,
        screen_pos.x - label_size.width * 0.5,
        screen_pos.y + size * 0.66 + 16.0,
        15.0,
        fleet_color,
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

        draw_rotated_texture_contain(
            texture,
            Rect::new(center.x - size * 0.5, center.y - size * 0.5, size, size),
            WHITE,
            rotation,
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

fn draw_inventory_hint(game: &GameState) {
    let mut text = if game.fleet_open {
        "F close fleet"
    } else if game.map_open {
        "M close map"
    } else if game.contracts_open {
        "J close contracts"
    } else if game.content_open {
        "C close content"
    } else if game.upgrades_open {
        "Esc close upgrades"
    } else if game.research_open {
        "K close research"
    } else if game.inventory_open {
        "E/Tab close inventory   F fleet   M map   K research   C content   J contracts   Esc close"
    } else {
        "E/Tab inventory   F fleet   M map   K research   C content   J contracts   Esc menu"
    }
    .to_string();
    if game.save_status_timer > 0.0 {
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

fn draw_starmap_overlay(game: &GameState, panel_corner: Option<&Texture2D>) {
    let (x, y, width, height) = starmap_panel_rect(game);

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
            game.starmap_selected_planet == Some(index),
            game.starmap_zoom,
            game.starmap_pan,
        );
    }
    draw_starmap_ship_3d(&game.ship, game.starmap_zoom, game.starmap_pan);

    set_default_camera();

    draw_rectangle(x, y, width, height, Color::from_rgba(4, 12, 18, 24));
    draw_panel_corner_art(Rect::new(x, y, width, height), panel_corner);
    draw_starmap_planet_markers(game, &camera);
    draw_text(
        "3D Starmap",
        x + GAME_PANEL_HEADER_PAD_X,
        y + GAME_PANEL_HEADER_BASELINE,
        28.0,
        Color::from_rgba(235, 242, 226, 255),
    );
    let help = fit_debug_text(
        "Drag header move   Wheel zoom   Right-drag pan   Drag corner resize   Click select   F/R filters",
        (width - 280.0).max(40.0),
        16,
    );
    draw_text(
        &help,
        x + 260.0,
        y + 36.0,
        16.0,
        Color::from_rgba(126, 156, 164, 220),
    );
    draw_starmap_filter_readout(
        game,
        x + GAME_PANEL_HEADER_PAD_X,
        y + GAME_PANEL_BODY_TOP - 24.0,
    );
    draw_starmap_readout(
        x + GAME_PANEL_CONTENT_PAD_X,
        y + height - 82.0,
        &game.ship,
        &game.planets,
        &game.current_system_id,
        game.destination_planet,
    );
    draw_known_systems_panel(game);
    draw_starmap_resize_handle(game);
}

fn starmap_resize_handle_rect(game: &GameState) -> Rect {
    let (x, y, width, height) = starmap_panel_rect(game);
    Rect::new(x + width - 24.0, y + height - 24.0, 24.0, 24.0)
}

fn starmap_move_handle_rect(game: &GameState) -> Rect {
    let (x, y, width, _) = starmap_panel_rect(game);
    Rect::new(x, y, width, 58.0)
}

fn draw_starmap_resize_handle(game: &GameState) {
    let handle = starmap_resize_handle_rect(game);
    let hovered = handle.contains(mouse_vec2()) || game.starmap_resize_previous_mouse.is_some();
    let color = if hovered {
        Color::from_rgba(150, 221, 226, 235)
    } else {
        Color::from_rgba(112, 151, 163, 170)
    };
    for inset in [5.0, 10.0, 15.0] {
        draw_line(
            handle.x + handle.w - inset,
            handle.y + handle.h,
            handle.x + handle.w,
            handle.y + handle.h - inset,
            1.5,
            color,
        );
    }
}

fn starmap_panel_rect(game: &GameState) -> (f32, f32, f32, f32) {
    let screen = vec2(screen_width(), screen_height());
    let size = starmap_panel_size_for(screen, game.starmap_panel_size);
    let position = starmap_panel_position_for(screen, size, game.starmap_panel_position);
    (position.x, position.y, size.x, size.y)
}

fn starmap_panel_size_for(screen: Vec2, requested: Option<Vec2>) -> Vec2 {
    let margin = 24.0;
    let destinations_space = 390.0;
    let default_width = (screen.x - destinations_space - margin * 3.0)
        .clamp(520.0, 1100.0)
        .min((screen.x - margin * 2.0).max(320.0));
    let default_height = (screen.y * 0.82)
        .clamp(420.0, 820.0)
        .min((screen.y - margin * 2.0).max(280.0));
    let maximum = vec2(
        (screen.x - margin * 2.0).max(320.0),
        (screen.y - margin * 2.0).max(280.0),
    );
    let minimum = vec2(520.0_f32.min(maximum.x), 420.0_f32.min(maximum.y));
    let requested = requested.unwrap_or_else(|| vec2(default_width, default_height));
    vec2(
        requested.x.clamp(minimum.x, maximum.x),
        requested.y.clamp(minimum.y, maximum.y),
    )
}

fn starmap_panel_position_for(screen: Vec2, size: Vec2, requested: Option<Vec2>) -> Vec2 {
    let requested = requested.unwrap_or_else(|| vec2(24.0, 24.0));
    let maximum = vec2((screen.x - size.x).max(0.0), (screen.y - size.y).max(0.0));
    vec2(
        requested.x.clamp(0.0, maximum.x),
        requested.y.clamp(0.0, maximum.y),
    )
}

fn known_systems_panel_rect(game: &GameState) -> Rect {
    let (x, y, width, height) = starmap_panel_rect(game);
    Rect::new(
        x + width - KNOWN_SYSTEMS_PANEL_WIDTH - 18.0,
        y + 66.0,
        KNOWN_SYSTEMS_PANEL_WIDTH,
        height - 150.0,
    )
}

fn known_system_row_rect(game: &GameState, index: usize) -> Rect {
    let panel = known_systems_panel_rect(game);
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
        aspect: Some(width / height.max(1.0)),
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
        let is_selected = game.starmap_selected_planet == Some(index);
        let route_position = game
            .navigation_route
            .iter()
            .position(|stop| stop.system == planet.system && stop.planet == planet.id);
        let color = if is_destination {
            Color::from_rgba(113, 235, 138, 220)
        } else if is_selected {
            Color::from_rgba(255, 206, 88, 220)
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
        if let Some(route_position) = route_position {
            draw_text(
                &(route_position + 1).to_string(),
                screen_pos.x + radius + 5.0,
                screen_pos.y - radius - 3.0,
                18.0,
                Color::from_rgba(235, 242, 226, 235),
            );
        }
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
    is_selected: bool,
    zoom: f32,
    pan: Vec2,
) {
    let relative = planet.position - ship.position;
    let position = world_to_starmap_3d(relative, zoom, pan);
    let floor = vec3(position.x, 0.0, position.z);
    let radius = starmap_planet_radius(planet, zoom) * 0.64;
    let color = if is_destination {
        Color::from_rgba(82, 194, 214, 235)
    } else if is_selected {
        Color::from_rgba(196, 151, 63, 235)
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
    } else if is_selected {
        draw_starmap_ring_at_3d(position, radius + 8.0, Color::from_rgba(255, 206, 88, 190));
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
    let panel = known_systems_panel_rect(game);
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
        let row = known_system_row_rect(game, index);
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
        draw_text(
            &fit_debug_text(&route_readiness_summary(game, system_id), row.w - 20.0, 13),
            row.x + 10.0,
            row.y + 59.0,
            13.0,
            if can_warp {
                Color::from_rgba(150, 221, 226, 220)
            } else {
                Color::from_rgba(220, 126, 116, 220)
            },
        );
    }
}

fn inventory_panel_height() -> f32 {
    inventory_panel_height_for(screen_height())
}

fn inventory_panel_height_for(screen_height: f32) -> f32 {
    let maximum = (screen_height - 56.0).max(260.0);
    (screen_height * 0.8).clamp(260.0, maximum)
}

fn egui_inventory_active(game: &GameState) -> bool {
    game.inventory_open
        && game.selected_planet.is_none()
        && game.selected_station.is_none()
        && game.selected_npc_ship.is_none()
}

fn egui_inventory_windows_visible(game: &GameState, pins: EguiPinSnapshot) -> bool {
    game.inventory_open || pins.ship || pins.production || pins.inventory
}

fn egui_planet_visible(game: &GameState, pins: EguiPinSnapshot) -> bool {
    game.selected_planet.is_some() && (game.inventory_open || pins.planet)
}

fn egui_station_visible(game: &GameState, pins: EguiPinSnapshot) -> bool {
    game.selected_station.is_some() && (game.inventory_open || pins.station)
}

fn egui_npc_visible(game: &GameState, pins: EguiPinSnapshot) -> bool {
    game.selected_npc_ship.is_some() && (game.inventory_open || pins.contact)
}

fn egui_fleet_visible(game: &GameState, pins: EguiPinSnapshot) -> bool {
    game.fleet_open || pins.fleet
}

fn egui_content_visible(game: &GameState, pins: EguiPinSnapshot) -> bool {
    game.content_open || pins.content
}

fn egui_contracts_visible(game: &GameState, pins: EguiPinSnapshot) -> bool {
    game.contracts_open || pins.contracts
}

fn egui_upgrades_visible(game: &GameState, pins: EguiPinSnapshot) -> bool {
    game.upgrades_open || pins.upgrades
}

fn egui_research_visible(game: &GameState, pins: EguiPinSnapshot) -> bool {
    game.research_open || pins.research
}

fn egui_destinations_visible(game: &GameState, pins: EguiPinSnapshot) -> bool {
    game.map_open || pins.destinations
}

fn inventory_overlay_view(
    game: &GameState,
    keep_click_step: i32,
) -> egui_inventory::InventoryOverlayView {
    let pins = egui_pin_snapshot();
    let (recipes, settings) = active_production_rows(game);
    let active_row = next_recipe_bill_index(game, recipes, settings);
    let production = recipes
        .iter()
        .zip(settings)
        .enumerate()
        .map(|(index, (recipe, setting))| {
            let unlocked = recipe_is_unlocked(game, &recipe.id);
            egui_inventory::ProductionRow {
                item: recipe_row_label(recipes, index),
                keep: setting.keep,
                status: if unlocked {
                    work_row_status(
                        game.inventory.count(&recipe.output.item),
                        setting.keep,
                        setting.queued,
                    )
                } else {
                    "Locked".to_string()
                },
                percent: if active_row == Some(index) {
                    format!("{:.0}%", setting.progress() * 100.0)
                } else {
                    String::new()
                },
                active: active_row == Some(index),
                enabled: unlocked && game.inventory.can_craft(recipe),
                unlocked,
                requirements: recipe
                    .ingredients
                    .iter()
                    .map(|ingredient| {
                        format!(
                            "{}: {} / {}",
                            ingredient.item.name,
                            game.inventory.count(&ingredient.item),
                            ingredient.count
                        )
                    })
                    .collect(),
            }
        })
        .collect();

    let inventory = game
        .inventory
        .slots
        .iter()
        .filter_map(|slot| slot.as_ref())
        .map(|stack| egui_inventory::InventoryRow {
            item: stack.item.name.clone(),
            quantity: stack.count,
            mass: format_mass(stack.item.unit_mass * stack.count as f32),
        })
        .collect();

    egui_inventory::InventoryOverlayView {
        credits: game.credits,
        keep_click_step,
        production_tab: production_tab_for_mode(game.production_mode),
        production,
        inventory,
        ship: ship_pane_view(game),
        show_ship: egui_inventory_active(game) || pins.ship,
        show_production: game.inventory_open || pins.production,
        show_inventory: game.inventory_open || pins.inventory,
        ship_pinned: pins.ship,
        production_pinned: pins.production,
        inventory_pinned: pins.inventory,
    }
}

fn planet_pane_view(game: &GameState, keep_click_step: i32) -> egui_planet::PlanetPaneView {
    let planet_index = game.selected_planet.unwrap_or(usize::MAX);
    let Some(planet) = game.planets.get(planet_index) else {
        return egui_planet::PlanetPaneView {
            pinned: egui_pane_pinned(EguiPane::Planet),
            title: "Planet unavailable".to_string(),
            artwork: None,
            content_id: String::new(),
            summary: "The selected planet is no longer available.".to_string(),
            ownership: None,
            scan_level: 0,
            max_scan_level: MAX_SCAN_LEVEL,
            mineables: Vec::new(),
            hazards: Vec::new(),
            hazard_effects: None,
            richness: "Unknown".to_string(),
            proximity: "Unavailable".to_string(),
            in_range: false,
            is_orbiting: false,
            orbit_pending: false,
            scan_enabled: false,
            scan_label: "No scan available".to_string(),
            basic_drones: 0,
            improved_drones: 0,
            drone_return: "Unknown".to_string(),
            latest_operation: None,
            keep_click_step,
            mining: Vec::new(),
            mining_revealed: false,
        };
    };

    let surface_scanned = planet_has_surface_scan(planet);
    let composition_scanned = planet_has_composition_scan(planet);
    let richness_scanned = planet_has_richness_scan(planet);
    let in_range = planet_in_interaction_range(&game.ship, planet);
    let is_orbiting = game.orbiting_planet == Some(planet_index);
    let orbit_pending = game.approach_target == Some(ApproachTarget::PlanetOrbit(planet_index));
    let basic_drones = core_item(&game.content_registry, "survey_drone")
        .map(|item| game.inventory.count(&item))
        .unwrap_or(0);
    let improved_drones = core_item(&game.content_registry, "improved_survey_drone")
        .map(|item| game.inventory.count(&item))
        .unwrap_or(0);
    let has_drone = basic_drones > 0 || improved_drones > 0;
    let ownership = surface_scanned.then(|| {
        planet
            .faction
            .as_deref()
            .map(|faction| {
                format!(
                    "{} / {}",
                    faction_name(&game.content_registry, faction),
                    faction_disposition_label(&game.content_registry, faction)
                )
            })
            .unwrap_or_else(|| "Unclaimed".to_string())
    });
    let hazard_effects = surface_scanned.then(|| {
        format!(
            "Mining: {:.0}% speed / {:.1} shield/s",
            100.0 / planet_hazard_mining_slowdown(planet),
            planet_hazard_shield_drain_per_second(planet)
        )
    });
    let richness = if richness_scanned {
        let minimum = planet
            .info
            .mineables
            .iter()
            .enumerate()
            .map(|(index, _)| mineable_richness_multiplier(planet, index))
            .fold(f32::INFINITY, f32::min);
        format!(
            "{:.0}-{:.0}% extraction by resource",
            minimum * 100.0,
            planet_richness_multiplier(planet) * 100.0
        )
    } else {
        "Unknown until deep scan".to_string()
    };
    let active_row = in_range
        .then(|| next_mining_bill_index(&game.inventory, planet))
        .flatten();
    let mining = if composition_scanned {
        planet
            .info
            .mineables
            .iter()
            .zip(&planet.mining)
            .enumerate()
            .map(|(index, (mineable, setting))| {
                let current = game.inventory.count(&mineable.item);
                egui_planet::MiningRow {
                    item: mineable.item.name.clone(),
                    keep: setting.keep,
                    status: work_row_status(current, setting.keep, setting.queued),
                    richness: if richness_scanned {
                        format!(
                            "{:.0}%",
                            mineable_richness_multiplier(planet, index) * 100.0
                        )
                    } else {
                        String::new()
                    },
                    active: active_row == Some(index),
                    enabled: in_range,
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    egui_planet::PlanetPaneView {
        pinned: egui_pane_pinned(EguiPane::Planet),
        title: if surface_scanned {
            planet.info.classification.clone()
        } else {
            "Unscanned body".to_string()
        },
        artwork: game
            .content_registry
            .planets
            .get(&planet.id)
            .and_then(|definition| definition.texture.clone()),
        content_id: planet.id.clone(),
        summary: if surface_scanned {
            planet.info.summary.clone()
        } else {
            "Classification, hazards, composition, and landing profiles are unknown. Send survey drones while in range to build the planet record."
                .to_string()
        },
        ownership,
        scan_level: planet.scan_level,
        max_scan_level: MAX_SCAN_LEVEL,
        mineables: if composition_scanned {
            planet
                .info
                .mineables
                .iter()
                .map(|mineable| mineable.item.name.clone())
                .collect()
        } else {
            Vec::new()
        },
        hazards: if surface_scanned {
            planet.info.hazards.clone()
        } else {
            Vec::new()
        },
        hazard_effects,
        richness,
        proximity: if is_orbiting {
            "Stable orbit locked".to_string()
        } else if orbit_pending {
            "Flying to orbital position".to_string()
        } else if in_range {
            "Interaction range reached".to_string()
        } else {
            "Approach required for mining".to_string()
        },
        in_range,
        is_orbiting,
        orbit_pending,
        scan_enabled: in_range && has_drone && planet.scan_level < MAX_SCAN_LEVEL,
        scan_label: if planet.scan_level >= MAX_SCAN_LEVEL {
            "Survey complete".to_string()
        } else if !in_range {
            "Approach to scan".to_string()
        } else if !has_drone {
            "No survey drones".to_string()
        } else {
            "Send survey drone".to_string()
        },
        basic_drones,
        improved_drones,
        drone_return: format!(
            "{:.0}%",
            survey_drone_return_chance(&game.ship_upgrades) * 100.0
        ),
        latest_operation: game
            .operation_feedback
            .iter()
            .find(|entry| {
                entry.category == "Survey"
                    || entry.category == "Mining"
                    || entry.category == "Orbit"
            })
            .map(|entry| format!("{} / {}", entry.category, entry.message)),
        keep_click_step,
        mining,
        mining_revealed: composition_scanned,
    }
}

fn station_pane_view(game: &GameState) -> egui_station::StationPaneView {
    let station_index = game.selected_station.unwrap_or(usize::MAX);
    let Some(station) = game.stations.get(station_index) else {
        return egui_station::StationPaneView {
            pinned: egui_pane_pinned(EguiPane::Station),
            name: "Station unavailable".to_string(),
            artwork: None,
            content_id: String::new(),
            summary: "The selected station is no longer available.".to_string(),
            ownership: "Unknown".to_string(),
            proximity: "Unavailable".to_string(),
            credits: game.credits,
            services: Vec::new(),
            selected_service: None,
            service_description: None,
            service_status: None,
            trade: Vec::new(),
            garage: false,
            repair_label: String::new(),
            repair_enabled: false,
            contracts: Vec::new(),
            recipes: Vec::new(),
            research: Vec::new(),
            latest_operation: None,
        };
    };

    let in_range = station_in_interaction_range(&game.ship, station);
    let distance = game.ship.position.distance(station.position);
    let ownership = match (&station.faction, &station.culture) {
        (Some(faction), Some(culture)) => format!(
            "{} / {} / {}",
            faction_name(&game.content_registry, faction),
            faction_name(&game.content_registry, culture),
            faction_disposition_label(&game.content_registry, faction)
        ),
        (Some(faction), None) => format!(
            "{} / {}",
            faction_name(&game.content_registry, faction),
            faction_disposition_label(&game.content_registry, faction)
        ),
        (None, Some(culture)) => faction_name(&game.content_registry, culture).to_string(),
        (None, None) => "Independent".to_string(),
    };
    let services = station
        .services
        .iter()
        .map(|service| {
            let context = service
                .vendor
                .as_ref()
                .map(|vendor| {
                    let specialties = vendor.specialties.join(", ");
                    if specialties.is_empty() {
                        vendor.name.clone()
                    } else {
                        format!("{} · {}", vendor.name, specialties)
                    }
                })
                .unwrap_or_else(|| service.kind.clone());
            let context = service
                .reputation_required
                .map_or(context.clone(), |required| {
                    let standing =
                        faction_reputation(game, service_reputation_faction(station, service));
                    format!("{context} · reputation {standing}/{required}")
                });
            egui_station::ServiceRow {
                name: service.name.clone(),
                context,
                available: true,
            }
        })
        .collect();

    let selected_service = game
        .selected_station_service
        .filter(|index| *index < station.services.len());
    let service = selected_service.and_then(|index| station.services.get(index));
    let service_available =
        service.is_some_and(|service| station_service_is_available(game, station, service));
    let service_status = service.map(|_| {
        if !service_available {
            "Locked by faction standing".to_string()
        } else if in_range {
            "Dock range reached · service ready".to_string()
        } else {
            "Approach within dock range to use this service".to_string()
        }
    });
    let trade = service
        .map(|service| {
            service
                .trade
                .iter()
                .map(|offer| {
                    let cargo = game.inventory.count(&offer.item);
                    egui_station::TradeRow {
                        item: offer.item.name.clone(),
                        stock: format_trade_stock(offer, game.world_elapsed_days),
                        cargo,
                        buy_price: offer.buy_price,
                        sell_price: offer.sell_price,
                        can_buy: in_range
                            && service_available
                            && !offer.unavailable
                            && offer.stock != Some(0)
                            && game.credits >= offer.buy_price,
                        can_sell: in_range && service_available && cargo > 0,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let missing_hull = (game.ship.systems.hull.max - game.ship.systems.hull.current).ceil();
    let missing_shields =
        (game.ship.systems.shields.max - game.ship.systems.shields.current).ceil();
    let repair_cost = (missing_hull * 3.0) as u32 + missing_shields as u32;
    let garage = service.is_some_and(|service| service.kind == "garage");
    let repair_label = if !in_range {
        "Approach to repair".to_string()
    } else if repair_cost == 0 {
        "Ship systems nominal".to_string()
    } else if game.credits < repair_cost {
        format!("Need {} cr", repair_cost.saturating_sub(game.credits))
    } else {
        format!("Repair hull + shields · {repair_cost} cr")
    };

    let contracts = service
        .map(|service| {
            service
                .contracts
                .iter()
                .map(|contract| {
                    let active = game.active_contracts.iter().find(|active| {
                        active.id == contract.id
                            && active.origin_station == contract.origin_station
                            && active.origin_service == contract.origin_service
                    });
                    let standing = faction_reputation(game, contract.reputation_faction.as_deref());
                    let status = if standing < contract.reputation_required {
                        "Need reputation"
                    } else if let Some(active) = active {
                        if game.world_elapsed_days > active.expires_day {
                            "Expired"
                        } else if contract_is_complete(game, contract) {
                            "Complete"
                        } else {
                            "Active"
                        }
                    } else {
                        "Accept"
                    };
                    let destination = contract_target_name_from_station_data(
                        station,
                        contract,
                        &game.stations,
                        &game.planets,
                    );
                    let objective = if contract.kind == "hauling" {
                        format!(
                            "Deliver {} × {} to {}",
                            contract
                                .item
                                .as_ref()
                                .map(|item| item.name.as_str())
                                .unwrap_or("cargo"),
                            contract.amount,
                            destination
                        )
                    } else {
                        format!("Scan {} to level {}", destination, contract.amount)
                    };
                    egui_station::ContractRow {
                        name: contract.name.clone(),
                        description: contract
                            .description
                            .clone()
                            .unwrap_or_else(|| match contract.kind.as_str() {
                                "hauling" => "Deliver cargo to the listed station".to_string(),
                                _ => "Complete the requested survey".to_string(),
                            }),
                        objective,
                        reward: contract.reward,
                        duration_days: contract.duration_days,
                        status: status.to_string(),
                        actionable: in_range,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let recipes = service
        .map(|service| {
            service
                .recipe_unlocks
                .iter()
                .map(|unlock| {
                    let purchased =
                        research_id_that_unlocks_recipe(&game.content_registry, &unlock.recipe)
                            .is_some_and(|research_id| {
                                game.completed_research
                                    .iter()
                                    .any(|done| done == research_id)
                            });
                    let (status, purchasable) = if purchased {
                        ("Owned".to_string(), false)
                    } else if unlock.unavailable {
                        ("Unavailable".to_string(), false)
                    } else if !in_range {
                        ("Approach".to_string(), false)
                    } else if game.credits < unlock.price {
                        (
                            format!("Need {} cr", unlock.price.saturating_sub(game.credits)),
                            false,
                        )
                    } else {
                        (format!("Unlock · {} cr", unlock.price), true)
                    };
                    egui_station::RecipeRow {
                        name: recipe_display_name(&game.content_registry, &unlock.recipe),
                        status,
                        purchasable,
                    }
                })
                .collect()
        })
        .unwrap_or_default();
    let research = service
        .map(|service| {
            service
                .research
                .iter()
                .map(|lead| {
                    let complete = game
                        .completed_research
                        .iter()
                        .any(|done| done == &lead.research);
                    egui_station::ResearchRow {
                        name: research_display_name(&game.content_registry, &lead.research),
                        status: if complete {
                            "Complete"
                        } else if lead.unavailable {
                            "Unavailable"
                        } else if !in_range {
                            "Approach"
                        } else {
                            "Available"
                        }
                        .to_string(),
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    egui_station::StationPaneView {
        pinned: egui_pane_pinned(EguiPane::Station),
        name: station.name.clone(),
        artwork: game
            .content_registry
            .stations
            .get(&station.id)
            .and_then(|definition| definition.texture.clone()),
        content_id: station.id.clone(),
        summary: station.summary.clone(),
        ownership,
        proximity: if in_range {
            format!("Dock range reached · {distance:.0}u")
        } else {
            format!("Approach required · {distance:.0}u")
        },
        credits: game.credits,
        services,
        selected_service,
        service_description: service.and_then(|service| service.description.clone()),
        service_status,
        trade,
        garage,
        repair_label,
        repair_enabled: garage && in_range && service_available,
        contracts,
        recipes,
        research,
        latest_operation: game
            .operation_feedback
            .iter()
            .find(|entry| {
                matches!(
                    entry.category.as_str(),
                    "Station" | "Trade" | "Repair" | "Contract" | "Unlock" | "Route"
                )
            })
            .map(|entry| format!("{} / {}", entry.category, entry.message)),
    }
}

fn fleet_cargo_rows(inventory: &Inventory) -> Vec<egui_fleet::CargoRow> {
    inventory
        .slots
        .iter()
        .flatten()
        .map(|stack| egui_fleet::CargoRow {
            item_id: stack.item.id.clone(),
            name: stack.item.name.clone(),
            count: stack.count,
        })
        .collect()
}

fn research_view(game: &GameState) -> egui_research::ResearchView {
    let registry = &game.content_registry;
    let nodes = registry
        .research_order
        .iter()
        .filter_map(|research_id| registry.research.get(research_id))
        .map(|research| {
            let state = research_node_state(
                research,
                game.active_research.as_ref(),
                &game.completed_research,
                game.credits,
            );
            let progress = game
                .active_research
                .as_ref()
                .filter(|active| active.research == research.id)
                .map(|active| {
                    1.0 - (active.remaining_seconds / research.duration_seconds).clamp(0.0, 1.0)
                });
            egui_research::ResearchNode {
                id: research.id.clone(),
                name: research.name.clone(),
                tier: research.tier,
                column: research.column,
                row: research.row,
                state: egui_research_state(state),
                progress,
            }
        })
        .collect();
    let mut seen_edges = HashSet::new();
    let mut edges = Vec::new();
    for research_id in &registry.research_order {
        let Some(research) = registry.research.get(research_id) else {
            continue;
        };
        for required in research.requires.iter().chain(&research.revealed_by) {
            if seen_edges.insert((required.clone(), research.id.clone())) {
                edges.push(egui_research::ResearchEdge {
                    from: required.clone(),
                    to: research.id.clone(),
                });
            }
        }
    }
    let selected = game
        .selected_research
        .clone()
        .filter(|id| registry.research.contains_key(id));
    let detail = selected.as_ref().and_then(|research_id| {
        let research = registry.research.get(research_id)?;
        let state = research_node_state(
            research,
            game.active_research.as_ref(),
            &game.completed_research,
            game.credits,
        );
        let active = game
            .active_research
            .as_ref()
            .filter(|active| active.research == research.id);
        let progress = active.map(|active| {
            1.0 - (active.remaining_seconds / research.duration_seconds).clamp(0.0, 1.0)
        });
        Some(egui_research::ResearchDetail {
            id: research.id.clone(),
            name: research.name.clone(),
            summary: research
                .summary
                .clone()
                .unwrap_or_else(|| "No research summary provided.".to_string()),
            state: egui_research_state(state),
            price: research.price,
            duration: format_seconds(research.duration_seconds),
            requirements: research_requirement_labels(registry, research),
            rewards: research
                .rewards
                .iter()
                .map(|reward| research_reward_label(registry, reward))
                .collect(),
            progress,
            remaining: active
                .map(|active| format!("{} remaining", format_seconds(active.remaining_seconds))),
            start_enabled: state == ResearchNodeState::Affordable,
            action_label: match state {
                ResearchNodeState::Affordable => "Start research",
                ResearchNodeState::Available => "Need credits",
                ResearchNodeState::Researching => "Researching",
                ResearchNodeState::Completed => "Completed",
                ResearchNodeState::Locked => "Locked",
            }
            .to_string(),
        })
    });

    egui_research::ResearchView {
        pinned: egui_pane_pinned(EguiPane::Research),
        credits: game.credits,
        nodes,
        edges,
        selected,
        detail,
        latest_operation: game
            .operation_feedback
            .iter()
            .find(|entry| entry.category == "Research")
            .map(|entry| format!("{} / {}", entry.category, entry.message)),
    }
}

fn destinations_view(game: &GameState) -> egui_destinations::DestinationsView {
    let selected = game
        .starmap_selected_planet
        .and_then(|index| game.planets.get(index))
        .filter(|planet| planet.system == game.current_system_id)
        .map(|planet| {
            let scanned = planet_has_surface_scan(planet);
            egui_destinations::DestinationCandidate {
                name: if scanned {
                    planet.info.classification.clone()
                } else {
                    "Unscanned body".to_string()
                },
                artwork: game
                    .content_registry
                    .planets
                    .get(&planet.id)
                    .and_then(|definition| definition.texture.clone()),
                content_id: planet.id.clone(),
                system: system_display_name(&game.content_registry, &planet.system).to_string(),
                summary: if scanned {
                    planet.info.summary.clone()
                } else {
                    "Survey data is unavailable. Approach the body to investigate it.".to_string()
                },
                distance: format!("{:.0}u", game.ship.position.distance(planet.position)),
                scan_status: format!("{}/{}", planet.scan_level, MAX_SCAN_LEVEL),
            }
        });
    let route = game
        .navigation_route
        .iter()
        .enumerate()
        .map(|(index, stop)| {
            let current = game.navigation_route_current;
            let state = if current.is_some_and(|current| index < current) {
                egui_destinations::RouteStopState::Completed
            } else if current == Some(index) && game.navigation_route_active {
                egui_destinations::RouteStopState::Active
            } else if current == Some(index) {
                egui_destinations::RouteStopState::Paused
            } else {
                egui_destinations::RouteStopState::Pending
            };
            let planet = navigation_stop_planet_index(game, stop)
                .and_then(|planet_index| game.planets.get(planet_index));
            egui_destinations::RouteStopView {
                name: planet
                    .map(|planet| {
                        if planet_has_surface_scan(planet) {
                            planet.info.classification.clone()
                        } else {
                            "Unscanned body".to_string()
                        }
                    })
                    .unwrap_or_else(|| stop.planet.clone()),
                system: system_display_name(&game.content_registry, &stop.system).to_string(),
                distance: planet
                    .filter(|planet| planet.system == game.current_system_id)
                    .map(|planet| format!("{:.0}u", game.ship.position.distance(planet.position)))
                    .unwrap_or_else(|| "Other system".to_string()),
                state,
            }
        })
        .collect();

    egui_destinations::DestinationsView {
        pinned: egui_pane_pinned(EguiPane::Destinations),
        map_open: game.map_open,
        selected,
        route,
        route_active: game.navigation_route_active,
        route_started: game
            .navigation_route_current
            .is_some_and(|current| current < game.navigation_route.len()),
    }
}

fn egui_research_state(state: ResearchNodeState) -> egui_research::ResearchState {
    match state {
        ResearchNodeState::Completed => egui_research::ResearchState::Completed,
        ResearchNodeState::Researching => egui_research::ResearchState::Researching,
        ResearchNodeState::Affordable => egui_research::ResearchState::Affordable,
        ResearchNodeState::Available => egui_research::ResearchState::Available,
        ResearchNodeState::Locked => egui_research::ResearchState::Locked,
    }
}

fn upgrades_view(game: &GameState, purchase_count: u32) -> egui_upgrades::UpgradesView {
    egui_upgrades::UpgradesView {
        pinned: egui_pane_pinned(EguiPane::Upgrades),
        ship_name: game
            .content_registry
            .ships
            .get(&game.ship_id)
            .map(|ship| ship.name.clone())
            .unwrap_or_else(|| local_content_id(&game.ship_id).replace('_', " ")),
        artwork: game
            .content_registry
            .ships
            .get(&game.ship_id)
            .and_then(|definition| definition.texture.clone()),
        purchase_count: purchase_count.max(1),
        rows: game
            .ship_upgrades
            .iter()
            .map(|upgrade| {
                let cost = upgrade.next_cost(&game.content_registry);
                egui_upgrades::UpgradeRow {
                    name: upgrade.kind.name().to_string(),
                    level: upgrade.level,
                    effect: upgrade.kind.effect_text().to_string(),
                    affordable: can_afford_cost(&game.inventory, &cost),
                    cost: cost
                        .iter()
                        .map(|stack| egui_upgrades::CostRow {
                            item: stack.item.name.clone(),
                            required: stack.count,
                            available: game.inventory.count(&stack.item),
                        })
                        .collect(),
                }
            })
            .collect(),
        latest_operation: game
            .operation_feedback
            .iter()
            .find(|entry| entry.category == "Upgrade")
            .map(|entry| format!("{} / {}", entry.category, entry.message)),
    }
}

fn contracts_view(game: &GameState) -> egui_contracts::ContractsView {
    let entries = active_contract_menu_entries(game);
    let selected = game
        .selected_contract_index
        .filter(|index| *index < entries.len());
    let detail = selected.map(|index| {
        let entry = &entries[index];
        let origin_station = game
            .stations
            .iter()
            .find(|station| station.id == entry.contract.origin_station);
        let focus_enabled =
            origin_station.is_some_and(|station| station.system == game.current_system_id);
        let focus_status = match origin_station {
            None => "Origin station is unavailable".to_string(),
            Some(station) if station.system != game.current_system_id => {
                format!(
                    "Origin is in {}",
                    system_display_name(&game.content_registry, &station.system)
                )
            }
            Some(_) => "Origin station is in the current system".to_string(),
        };
        egui_contracts::ContractDetail {
            name: entry.contract.name.clone(),
            description: entry
                .contract
                .description
                .clone()
                .unwrap_or_else(|| "No description provided.".to_string()),
            status: entry.status.clone(),
            deadline: entry.deadline.clone(),
            progress: entry.progress.clone(),
            origin: entry.origin_name.clone(),
            destination: entry.target_name.clone(),
            reward: format!(
                "{} credits · +{} reputation",
                entry.contract.reward, entry.contract.reputation_reward
            ),
            requirement: format!("{} reputation", entry.contract.reputation_required),
            focus_enabled,
            focus_status,
        }
    });

    egui_contracts::ContractsView {
        pinned: egui_pane_pinned(EguiPane::Contracts),
        rows: entries
            .iter()
            .map(|entry| egui_contracts::ContractRow {
                name: entry.contract.name.clone(),
                route: format!("{} → {}", entry.origin_name, entry.target_name),
                status: entry.status.clone(),
                deadline: entry.deadline.clone(),
            })
            .collect(),
        selected,
        detail,
        latest_operation: game
            .operation_feedback
            .iter()
            .find(|entry| entry.category == "Contract" || entry.category == "Contracts")
            .map(|entry| format!("{} / {}", entry.category, entry.message)),
    }
}

fn content_browser_view(game: &GameState) -> egui_content::ContentBrowserView {
    let registry = &game.content_registry;
    let selected_pack_id = selected_content_pack_id(game);
    let transition_pixels = game
        .transition_assets
        .iter()
        .map(|asset| asset.texture.width() * asset.texture.height())
        .sum::<f32>();
    let transition_summary = game.transition_assets.first().map(|asset| {
        if game.transition_assets.len() == 1 {
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
        }
    });

    egui_content::ContentBrowserView {
        pinned: egui_pane_pinned(EguiPane::Content),
        summary: format!(
            "{} packs · {} items · {} recipes · {} factions · {} NPC ships · {} shields · {} weapons · {} systems · {} stars · {} planets · {} stations · {} upgrades · {} transition images",
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
        ),
        transition_summary,
        filter_label: selected_pack_id
            .map(|pack_id| format!("Showing pack: {pack_id}"))
            .unwrap_or_else(|| "Showing all packs".to_string()),
        packs: registry
            .packs
            .iter()
            .map(|pack| egui_content::PackRow {
                name: pack.name.clone(),
                detail: format!("{} · {}", pack.id, pack.version),
            })
            .collect(),
        selected_pack: game
            .content_browser
            .selected_pack_index
            .filter(|index| *index < registry.packs.len()),
        selected_detail: game
            .content_browser
            .selected_pack_index
            .and_then(|index| registry.packs.get(index))
            .map(|pack| egui_content::PackDetail {
                name: pack.name.clone(),
                id: pack.id.clone(),
                version: pack.version.clone(),
                description: pack
                    .description
                    .clone()
                    .unwrap_or_else(|| "No pack description provided.".to_string()),
                path: pack.path.display().to_string(),
                dependencies: pack.depends_on.clone(),
                optional_dependencies: pack.optional_depends_on.clone(),
                options: pack
                    .options
                    .iter()
                    .map(|option| {
                        let mut label = format!(
                            "{} · default {} · {}",
                            option.label,
                            option.default.as_save_string(),
                            option.id
                        );
                        if let Some(description) = &option.description {
                            label.push_str(&format!("\n{description}"));
                        }
                        label
                    })
                    .collect(),
            }),
        object_groups: content_object_groups(registry, selected_pack_id),
        selected_object: game.content_browser.selected_object.clone(),
        object_detail: game
            .content_browser
            .selected_object
            .as_ref()
            .and_then(|key| content_object_detail(registry, key)),
        can_go_back: !game.content_browser.object_history.is_empty(),
    }
}

fn content_object_groups(
    registry: &content::ContentRegistry,
    selected_pack_id: Option<&str>,
) -> Vec<egui_content::ObjectGroup> {
    use egui_content::ObjectKind;

    let visible = |id: &str| {
        selected_pack_id
            .map(|pack_id| content_id_belongs_to_pack(id, pack_id))
            .unwrap_or(true)
    };
    let mut groups = Vec::new();
    macro_rules! group {
        ($title:literal, $kind:expr, $order:expr, $map:expr, $name:expr, $detail:expr) => {{
            let rows = $order
                .iter()
                .filter(|id| visible(id))
                .filter_map(|id| $map.get(id).map(|value| (id, value)))
                .map(|(id, value)| egui_content::ObjectRow {
                    key: egui_content::ObjectKey {
                        kind: $kind,
                        id: id.clone(),
                    },
                    name: ($name)(value),
                    detail: ($detail)(value),
                })
                .collect::<Vec<_>>();
            if !rows.is_empty() {
                groups.push(egui_content::ObjectGroup {
                    title: $title.to_string(),
                    rows,
                });
            }
        }};
    }

    group!(
        "Items",
        ObjectKind::Item,
        registry.item_order,
        registry.items,
        |v: &content::ItemDef| v.name.clone(),
        |v: &content::ItemDef| format!("{} · {} kg", v.tier, v.unit_mass)
    );
    group!(
        "Recipes",
        ObjectKind::Recipe,
        registry.recipe_order,
        registry.recipes,
        |v: &content::RecipeDef| registry
            .items
            .get(&v.output.item)
            .map(|item| item.name.clone())
            .unwrap_or_else(|| local_content_id(&v.output.item).to_string()),
        |v: &content::RecipeDef| format!("{} x{}", v.station, v.output.count)
    );
    group!(
        "Ships",
        ObjectKind::Ship,
        registry.ship_order,
        registry.ships,
        |v: &content::ShipDef| v.name.clone(),
        |v: &content::ShipDef| format!("{} weapon slot(s)", v.weapon_slots.len())
    );
    group!(
        "NPC Ships",
        ObjectKind::NpcShip,
        registry.npc_ship_order,
        registry.npc_ships,
        |v: &content::NpcShipDef| v.name.clone(),
        |v: &content::NpcShipDef| format!("{} · {}", v.archetype, v.role)
    );
    group!(
        "Shields",
        ObjectKind::Shield,
        registry.shield_order,
        registry.shields,
        |v: &content::ShieldDef| v.name.clone(),
        |v: &content::ShieldDef| format!("{:.0} capacity", v.capacity)
    );
    group!(
        "Weapons",
        ObjectKind::Weapon,
        registry.weapon_order,
        registry.weapons,
        |v: &content::WeaponDef| v.name.clone(),
        |v: &content::WeaponDef| format!("{:.0} damage · {:.0}u", v.damage, v.range)
    );
    group!(
        "Power Modules",
        ObjectKind::PowerModule,
        registry.power_module_order,
        registry.power_modules,
        |v: &content::PowerModuleDef| v.name.clone(),
        |v: &content::PowerModuleDef| format!("+{:.1}/s", v.generation)
    );
    group!(
        "Research",
        ObjectKind::Research,
        registry.research_order,
        registry.research,
        |v: &content::ResearchDef| v.name.clone(),
        |v: &content::ResearchDef| format!("Tier {} · {} cr", v.tier, v.price)
    );
    group!(
        "Factions",
        ObjectKind::Faction,
        registry.faction_order,
        registry.factions,
        |v: &content::FactionDef| v.name.clone(),
        |v: &content::FactionDef| v.kind.clone()
    );
    group!(
        "Universes",
        ObjectKind::Universe,
        registry.universe_order,
        registry.universes,
        |v: &content::UniverseDef| v.name.clone(),
        |_v: &content::UniverseDef| "Universe".to_string()
    );
    group!(
        "Galaxy Groups",
        ObjectKind::GalaxyGroup,
        registry.galaxy_group_order,
        registry.galaxy_groups,
        |v: &content::GalaxyGroupDef| v.name.clone(),
        |_v: &content::GalaxyGroupDef| "Galaxy group".to_string()
    );
    group!(
        "Galaxy Clusters",
        ObjectKind::GalaxyCluster,
        registry.galaxy_cluster_order,
        registry.galaxy_clusters,
        |v: &content::GalaxyClusterDef| v.name.clone(),
        |_v: &content::GalaxyClusterDef| "Galaxy cluster".to_string()
    );
    group!(
        "Galaxies",
        ObjectKind::Galaxy,
        registry.galaxy_order,
        registry.galaxies,
        |v: &content::GalaxyDef| v.name.clone(),
        |_v: &content::GalaxyDef| "Galaxy".to_string()
    );
    group!(
        "Regions",
        ObjectKind::Region,
        registry.region_order,
        registry.regions,
        |v: &content::RegionDef| v.name.clone(),
        |_v: &content::RegionDef| "Region".to_string()
    );
    group!(
        "Systems",
        ObjectKind::System,
        registry.system_order,
        registry.systems,
        |v: &content::SystemDef| v.name.clone(),
        |v: &content::SystemDef| format!("{} tag(s)", v.tags.len())
    );
    group!(
        "Stars",
        ObjectKind::Star,
        registry.star_order,
        registry.stars,
        |v: &content::StarDef| v.name.clone(),
        |v: &content::StarDef| v.classification.clone()
    );
    group!(
        "Planets",
        ObjectKind::Planet,
        registry.planet_order,
        registry.planets,
        |v: &content::PlanetDef| local_content_id(&v.id).to_string(),
        |v: &content::PlanetDef| format!(
            "{} · {} resource(s)",
            v.classification,
            v.mineables.len()
        )
    );
    group!(
        "Stations",
        ObjectKind::Station,
        registry.station_order,
        registry.stations,
        |v: &content::StationDef| v.name.clone(),
        |v: &content::StationDef| format!("{} service(s)", v.services.len())
    );
    group!(
        "Vendors",
        ObjectKind::Vendor,
        registry.vendor_order,
        registry.vendors,
        |v: &content::VendorDef| v.name.clone(),
        |v: &content::VendorDef| format!("{} offer(s)", v.offers.len())
    );
    group!(
        "Upgrades",
        ObjectKind::Upgrade,
        registry.upgrade_order,
        registry.upgrades,
        |v: &content::UpgradeDef| local_content_id(&v.id).to_string(),
        |v: &content::UpgradeDef| format!("{} cost(s)", v.costs.len())
    );
    groups
}

fn content_object_detail(
    registry: &content::ContentRegistry,
    key: &egui_content::ObjectKey,
) -> Option<egui_content::ObjectDetail> {
    use egui_content::ObjectKind;

    let field = |label: &str, value: String| egui_content::DetailField {
        label: label.to_string(),
        value,
    };
    let reference = |label: &str, kind, id: &str| egui_content::DetailReference {
        label: label.to_string(),
        target: egui_content::ObjectKey {
            kind,
            id: id.to_string(),
        },
    };
    let joined = |values: &[String]| {
        if values.is_empty() {
            "None".to_string()
        } else {
            values.join(", ")
        }
    };
    let position = |value: [f32; 2]| format!("{:.0}, {:.0}", value[0], value[1]);

    let (name, summary, fields, references) = match key.kind {
        ObjectKind::Item => {
            let value = registry.items.get(&key.id)?;
            (
                value.name.clone(),
                None,
                vec![
                    field("Tier", value.tier.clone()),
                    field("Unit mass", format!("{} kg", value.unit_mass)),
                ],
                vec![],
            )
        }
        ObjectKind::Recipe => {
            let value = registry.recipes.get(&key.id)?;
            let mut references = vec![reference(
                "Output item",
                ObjectKind::Item,
                &value.output.item,
            )];
            references.extend(value.ingredients.iter().map(|ingredient| {
                reference(
                    &format!("Ingredient x{}", ingredient.count),
                    ObjectKind::Item,
                    &ingredient.item,
                )
            }));
            (
                registry
                    .items
                    .get(&value.output.item)
                    .map(|item| format!("{} recipe", item.name))
                    .unwrap_or_else(|| local_content_id(&key.id).to_string()),
                value.purpose.clone(),
                vec![
                    field("Station", value.station.clone()),
                    field("Output count", value.output.count.to_string()),
                    field("Ingredients", value.ingredients.len().to_string()),
                    field("Duplicate output", value.allow_duplicate_output.to_string()),
                ],
                references,
            )
        }
        ObjectKind::Ship => {
            let value = registry.ships.get(&key.id)?;
            let mut references = Vec::new();
            references.extend(
                value
                    .power_modules
                    .iter()
                    .map(|id| reference("Power module", ObjectKind::PowerModule, id)),
            );
            references.extend(
                value
                    .shield_slots
                    .iter()
                    .map(|id| reference("Shield", ObjectKind::Shield, id)),
            );
            references.extend(
                value
                    .weapon_slots
                    .iter()
                    .map(|id| reference("Weapon", ObjectKind::Weapon, id)),
            );
            (
                value.name.clone(),
                None,
                vec![
                    field("Mass", format!("{} t", value.mass)),
                    field("Hull", value.hull_capacity.to_string()),
                    field("Shield", value.shield_capacity.to_string()),
                    field(
                        "Energy",
                        format!("{} · +{}/s", value.energy_capacity, value.energy_recharge),
                    ),
                    field(
                        "Acceleration",
                        format!(
                            "{} / {} reverse",
                            value.forward_acceleration, value.reverse_acceleration
                        ),
                    ),
                    field("Turn", value.turn_acceleration.to_string()),
                    field(
                        "Purchase price",
                        value
                            .purchase_price
                            .map(|price| format!("{price} cr"))
                            .unwrap_or_else(|| "Not sold".to_string()),
                    ),
                    field(
                        "Texture",
                        value.texture.clone().unwrap_or_else(|| "None".to_string()),
                    ),
                ],
                references,
            )
        }
        ObjectKind::NpcShip => {
            let value = registry.npc_ships.get(&key.id)?;
            let mut references = vec![reference("System", ObjectKind::System, &value.system)];
            if let Some(id) = &value.faction {
                references.push(reference("Faction", ObjectKind::Faction, id));
            }
            references.extend(value.cargo_defaults.iter().map(|stack| {
                reference(
                    &format!("Cargo x{}", stack.count),
                    ObjectKind::Item,
                    &stack.item,
                )
            }));
            references.extend(
                value
                    .shield_slots
                    .iter()
                    .map(|id| reference("Shield", ObjectKind::Shield, id)),
            );
            references.extend(
                value
                    .weapon_slots
                    .iter()
                    .map(|id| reference("Weapon", ObjectKind::Weapon, id)),
            );
            (
                value.name.clone(),
                value.summary.clone(),
                vec![
                    field(
                        "Archetype / role",
                        format!("{} / {}", value.archetype, value.role),
                    ),
                    field("Position", position(value.position)),
                    field("Radius", value.radius.to_string()),
                    field(
                        "Spawn",
                        format!("{} x{}", value.spawn_weight, value.spawn_count),
                    ),
                    field(
                        "Mass / cargo",
                        format!("{} t / {} t", value.mass, value.cargo_capacity),
                    ),
                    field(
                        "Hull / shield / energy",
                        format!(
                            "{} / {} / {}",
                            value.hull_capacity, value.shield_capacity, value.energy_capacity
                        ),
                    ),
                    field("Behavior tags", joined(&value.behavior_tags)),
                    field(
                        "Credit reward",
                        format!("{}–{}", value.credit_reward_min, value.credit_reward_max),
                    ),
                ],
                references,
            )
        }
        ObjectKind::Shield => {
            let value = registry.shields.get(&key.id)?;
            (
                value.name.clone(),
                value.summary.clone(),
                vec![
                    field("Capacity", value.capacity.to_string()),
                    field(
                        "Recharge",
                        format!("{}/s after {}s", value.recharge_rate, value.recharge_delay),
                    ),
                    field(
                        "Damage resistance",
                        format!("{:.0}%", value.damage_resistance * 100.0),
                    ),
                    field(
                        "Hazard resistance",
                        format!("{:.0}%", value.hazard_resistance * 100.0),
                    ),
                ],
                vec![reference(
                    "Install item",
                    ObjectKind::Item,
                    &value.install_item,
                )],
            )
        }
        ObjectKind::Weapon => {
            let value = registry.weapons.get(&key.id)?;
            let mut references = vec![reference(
                "Install item",
                ObjectKind::Item,
                &value.install_item,
            )];
            if let Some(id) = &value.ammo_item {
                references.push(reference("Ammunition", ObjectKind::Item, id));
            }
            (
                value.name.clone(),
                value.summary.clone(),
                vec![
                    field("Range", format!("{}u", value.range)),
                    field("Damage", value.damage.to_string()),
                    field("Cooldown", format!("{}s", value.cooldown_seconds)),
                    field(
                        "Energy / ammo",
                        format!("{} / {}", value.energy_cost, value.ammo_per_shot),
                    ),
                    field("Targeting", format!("{:?}", value.targeting)),
                    field("Path effect", format!("{:?}", value.effect)),
                    field("Impact", format!("{:?}", value.impact)),
                    field(
                        "Splash",
                        format!("{}u · {:?}", value.splash_radius, value.splash_falloff),
                    ),
                    field(
                        "Chain",
                        format!("{} target(s) · {}u", value.chain_targets, value.chain_range),
                    ),
                    field("Friendly fire", format!("{:?}", value.friendly_fire)),
                    field(
                        "Projectile",
                        value
                            .projectile_texture
                            .clone()
                            .unwrap_or_else(|| "Beam".to_string()),
                    ),
                ],
                references,
            )
        }
        ObjectKind::PowerModule => {
            let value = registry.power_modules.get(&key.id)?;
            let mut references = vec![reference(
                "Install item",
                ObjectKind::Item,
                &value.install_item,
            )];
            if let Some(id) = &value.fuel_item {
                references.push(reference("Fuel", ObjectKind::Item, id));
            }
            (
                value.name.clone(),
                value.summary.clone(),
                vec![
                    field("Family", value.family.clone()),
                    field("Generation", format!("+{}/s", value.generation)),
                    field("Mass", format!("{} t", value.mass)),
                    field("Fuel rate", format!("{}/min", value.fuel_per_minute)),
                    field("Heat / risk", format!("{} / {}", value.heat, value.risk)),
                ],
                references,
            )
        }
        ObjectKind::Research => {
            let value = registry.research.get(&key.id)?;
            let mut references = Vec::new();
            references.extend(
                value
                    .requires
                    .iter()
                    .map(|id| reference("Requires", ObjectKind::Research, id)),
            );
            references.extend(
                value
                    .revealed_by
                    .iter()
                    .map(|id| reference("Revealed by", ObjectKind::Research, id)),
            );
            for reward in &value.rewards {
                if let Some(id) = &reward.target {
                    let kind = match reward.kind.as_str() {
                        "recipe" => ObjectKind::Recipe,
                        "item" => ObjectKind::Item,
                        _ => continue,
                    };
                    references.push(reference(&format!("Reward: {}", reward.kind), kind, id));
                }
            }
            (
                value.name.clone(),
                value.summary.clone(),
                vec![
                    field("Tier", value.tier.to_string()),
                    field("Grid", format!("{}, {}", value.column, value.row)),
                    field("Price", format!("{} cr", value.price)),
                    field("Duration", format!("{}s", value.duration_seconds)),
                    field("Rewards", value.rewards.len().to_string()),
                ],
                references,
            )
        }
        ObjectKind::Faction => {
            let value = registry.factions.get(&key.id)?;
            (
                value.name.clone(),
                value.summary.clone(),
                vec![
                    field("Kind", value.kind.clone()),
                    field("Disposition", value.default_disposition.id().to_string()),
                    field("Starting reputation", value.reputation_start.to_string()),
                    field(
                        "Reputation range",
                        format!("{} to {}", value.reputation_min, value.reputation_max),
                    ),
                    field("Tags", joined(&value.tags)),
                ],
                vec![],
            )
        }
        ObjectKind::Universe => {
            let value = registry.universes.get(&key.id)?;
            (
                value.name.clone(),
                value.description.clone(),
                vec![],
                vec![],
            )
        }
        ObjectKind::GalaxyGroup => {
            let value = registry.galaxy_groups.get(&key.id)?;
            let references = value
                .universe
                .as_ref()
                .map(|id| reference("Universe", ObjectKind::Universe, id))
                .into_iter()
                .collect();
            (
                value.name.clone(),
                value.description.clone(),
                vec![],
                references,
            )
        }
        ObjectKind::GalaxyCluster => {
            let value = registry.galaxy_clusters.get(&key.id)?;
            let mut references = Vec::new();
            if let Some(id) = &value.galaxy_group {
                references.push(reference("Galaxy group", ObjectKind::GalaxyGroup, id));
            }
            if let Some(id) = &value.universe {
                references.push(reference("Universe", ObjectKind::Universe, id));
            }
            (
                value.name.clone(),
                value.description.clone(),
                vec![],
                references,
            )
        }
        ObjectKind::Galaxy => {
            let value = registry.galaxies.get(&key.id)?;
            let mut references = Vec::new();
            if let Some(id) = &value.galaxy_cluster {
                references.push(reference("Galaxy cluster", ObjectKind::GalaxyCluster, id));
            }
            if let Some(id) = &value.galaxy_group {
                references.push(reference("Galaxy group", ObjectKind::GalaxyGroup, id));
            }
            if let Some(id) = &value.universe {
                references.push(reference("Universe", ObjectKind::Universe, id));
            }
            (
                value.name.clone(),
                value.description.clone(),
                vec![],
                references,
            )
        }
        ObjectKind::Region => {
            let value = registry.regions.get(&key.id)?;
            let mut references = Vec::new();
            if let Some(id) = &value.galaxy {
                references.push(reference("Galaxy", ObjectKind::Galaxy, id));
            }
            if let Some(id) = &value.galaxy_cluster {
                references.push(reference("Galaxy cluster", ObjectKind::GalaxyCluster, id));
            }
            if let Some(id) = &value.galaxy_group {
                references.push(reference("Galaxy group", ObjectKind::GalaxyGroup, id));
            }
            if let Some(id) = &value.universe {
                references.push(reference("Universe", ObjectKind::Universe, id));
            }
            (
                value.name.clone(),
                value.description.clone(),
                vec![],
                references,
            )
        }
        ObjectKind::System => {
            let value = registry.systems.get(&key.id)?;
            let mut references = Vec::new();
            if let Some(id) = &value.region {
                references.push(reference("Region", ObjectKind::Region, id));
            }
            if let Some(id) = &value.galaxy {
                references.push(reference("Galaxy", ObjectKind::Galaxy, id));
            }
            if let Some(id) = &value.universe {
                references.push(reference("Universe", ObjectKind::Universe, id));
            }
            if let Some(id) = &value.primary_star {
                references.push(reference("Primary star", ObjectKind::Star, id));
            }
            if let Some(id) = &value.faction {
                references.push(reference("Faction", ObjectKind::Faction, id));
            }
            (
                value.name.clone(),
                value.description.clone(),
                vec![
                    field("Arrival", position(value.arrival)),
                    field("Tags", joined(&value.tags)),
                ],
                references,
            )
        }
        ObjectKind::Star => {
            let value = registry.stars.get(&key.id)?;
            (
                value.name.clone(),
                None,
                vec![
                    field("Classification", value.classification.clone()),
                    field("Position", position(value.position)),
                    field("Radius", value.radius.to_string()),
                ],
                vec![reference("System", ObjectKind::System, &value.system)],
            )
        }
        ObjectKind::Planet => {
            let value = registry.planets.get(&key.id)?;
            let mut references = vec![reference("System", ObjectKind::System, &value.system)];
            if let Some(id) = &value.faction {
                references.push(reference("Faction", ObjectKind::Faction, id));
            }
            references.extend(
                value
                    .mineables
                    .iter()
                    .map(|id| reference("Mineable", ObjectKind::Item, id)),
            );
            if let Some(id) = value.orbit.as_ref().and_then(|orbit| orbit.around.as_ref()) {
                references.push(reference("Orbits", ObjectKind::Planet, id));
            }
            (
                local_content_id(&value.id).to_string(),
                Some(value.summary.clone()),
                vec![
                    field("Classification", value.classification.clone()),
                    field("Position", position(value.position)),
                    field("Radius", value.radius.to_string()),
                    field("Point of interest", value.is_poi.to_string()),
                    field("Hazards", joined(&value.hazards)),
                ],
                references,
            )
        }
        ObjectKind::Station => {
            let value = registry.stations.get(&key.id)?;
            let mut references = Vec::new();
            if let Some(id) = &value.system {
                references.push(reference("System", ObjectKind::System, id));
            }
            if let Some(id) = &value.faction {
                references.push(reference("Faction", ObjectKind::Faction, id));
            }
            (
                value.name.clone(),
                value.summary.clone(),
                vec![
                    field(
                        "Position",
                        value
                            .position
                            .map(position)
                            .unwrap_or_else(|| "Generated".to_string()),
                    ),
                    field("Radius", value.radius.to_string()),
                    field("Icon", value.icon.clone()),
                    field(
                        "Culture",
                        value.culture.clone().unwrap_or_else(|| "None".to_string()),
                    ),
                    field(
                        "Services",
                        value
                            .services
                            .iter()
                            .map(|service| service.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", "),
                    ),
                ],
                references,
            )
        }
        ObjectKind::Vendor => {
            let value = registry.vendors.get(&key.id)?;
            let mut references = vec![reference("Station", ObjectKind::Station, &value.station)];
            if let Some(id) = &value.faction {
                references.push(reference("Faction", ObjectKind::Faction, id));
            }
            references.extend(
                value
                    .offers
                    .iter()
                    .map(|offer| reference("Offered item", ObjectKind::Item, &offer.item)),
            );
            (
                value.name.clone(),
                None,
                vec![
                    field("Service", value.service.clone()),
                    field("Specialties", joined(&value.specialties)),
                    field("Rotation", format!("{} day(s)", value.rotation_days)),
                    field("Slots", value.slots.to_string()),
                    field("Price variance", value.price_variance.to_string()),
                    field("Reputation required", value.reputation_required.to_string()),
                ],
                references,
            )
        }
        ObjectKind::Upgrade => {
            let value = registry.upgrades.get(&key.id)?;
            let references = value
                .costs
                .iter()
                .map(|cost| {
                    reference(
                        &format!(
                            "Cost {} +{}/{} levels",
                            cost.base_count, cost.per_level, cost.per_levels
                        ),
                        ObjectKind::Item,
                        &cost.item,
                    )
                })
                .collect();
            (
                local_content_id(&value.id).to_string(),
                None,
                vec![field("Cost entries", value.costs.len().to_string())],
                references,
            )
        }
    };

    Some(egui_content::ObjectDetail {
        key: key.clone(),
        name,
        artwork: content_object_artwork(registry, key),
        summary,
        fields,
        references,
    })
}

fn content_object_artwork(
    registry: &content::ContentRegistry,
    key: &egui_content::ObjectKey,
) -> Option<String> {
    use egui_content::ObjectKind;

    match key.kind {
        ObjectKind::Ship => registry
            .ships
            .get(&key.id)
            .and_then(|value| value.texture.clone()),
        ObjectKind::NpcShip => registry
            .npc_ships
            .get(&key.id)
            .and_then(|value| value.texture.clone()),
        ObjectKind::Weapon => registry
            .weapons
            .get(&key.id)
            .and_then(|value| value.projectile_texture.clone()),
        ObjectKind::Planet => registry
            .planets
            .get(&key.id)
            .and_then(|value| value.texture.clone()),
        ObjectKind::Station => registry
            .stations
            .get(&key.id)
            .and_then(|value| value.texture.clone()),
        _ => None,
    }
}

fn fleet_pane_view(game: &GameState, quantity_step: u32) -> egui_fleet::FleetPaneView {
    let ships = game
        .owned_ships
        .iter()
        .map(|owned| {
            let active = owned.instance_id == game.active_owned_ship_id;
            let ship = if active { &game.ship } else { &owned.ship };
            let inventory = if active {
                &game.inventory
            } else {
                &owned.inventory
            };
            let hull_name = game
                .content_registry
                .ships
                .get(&owned.ship_id)
                .map(|definition| definition.name.clone())
                .unwrap_or_else(|| "Unavailable hull".to_string());
            egui_fleet::ShipRow {
                callsign: owned.callsign.clone(),
                hull_name,
                system: local_content_id(&owned.system_id).to_string(),
                condition: format!(
                    "Hull {:.0}/{:.0} · Cargo {}",
                    ship.systems.hull.current,
                    ship.systems.hull.max,
                    format_mass(inventory.total_mass())
                ),
                active,
                resolved: game.content_registry.ships.contains_key(&owned.ship_id),
            }
        })
        .collect();

    let selected_index = game
        .selected_owned_ship_index
        .min(game.owned_ships.len().saturating_sub(1));
    let selected_ship = game.owned_ships.get(selected_index).map(|owned| {
        let active = owned.instance_id == game.active_owned_ship_id;
        let ship = if active { &game.ship } else { &owned.ship };
        let inventory = if active {
            &game.inventory
        } else {
            &owned.inventory
        };
        let upgrades = if active {
            &game.ship_upgrades
        } else {
            &owned.upgrades
        };
        let weapons = if active {
            &game.equipped_weapons
        } else {
            &owned.equipped_weapons
        };
        let resolved = game.content_registry.ships.contains_key(&owned.ship_id);
        let transfer_available = owned_ship_in_transfer_range(game, selected_index);
        let distance = if owned.system_id == game.current_system_id {
            format!(
                "{:.0} units from active ship",
                ship.position.distance(game.ship.position)
            )
        } else {
            format!("Remote · {}", local_content_id(&owned.system_id))
        };
        egui_fleet::SelectedShip {
            callsign: owned.callsign.clone(),
            hull_name: game
                .content_registry
                .ships
                .get(&owned.ship_id)
                .map(|definition| definition.name.clone())
                .unwrap_or_else(|| "Unavailable hull".to_string()),
            artwork: game
                .content_registry
                .ships
                .get(&owned.ship_id)
                .and_then(|definition| definition.texture.clone()),
            system: local_content_id(&owned.system_id).to_string(),
            distance,
            hull: format!(
                "{:.0}/{:.0}",
                ship.systems.hull.current, ship.systems.hull.max
            ),
            shields: format!(
                "{:.0}/{:.0}",
                ship.systems.shields.current, ship.systems.shields.max
            ),
            energy: format!(
                "{:.0}/{:.0}",
                ship.systems.energy.current, ship.systems.energy.max
            ),
            cargo: format!(
                "{} / {}",
                format_mass(inventory.total_mass()),
                format_mass(cargo_rating_kg(upgrades))
            ),
            turrets: weapons.len(),
            active,
            switch_enabled: !active && resolved && ship.systems.hull.current > 0.0,
            transfer_available,
            transfer_status: if active {
                "Select another ship to transfer cargo.".to_string()
            } else if transfer_available {
                "Cargo transfer available.".to_string()
            } else if owned.system_id != game.current_system_id {
                "Cargo transfer requires both ships in the same system.".to_string()
            } else {
                format!(
                    "Move within {:.0} units to transfer cargo.",
                    OWNED_SHIP_TRANSFER_RANGE
                )
            },
            cargo_rows: fleet_cargo_rows(inventory),
        }
    });

    let shipyard = nearby_garage_index(game).map(|station_index| {
        let station = &game.stations[station_index];
        let offers = game
            .content_registry
            .ship_order
            .iter()
            .filter_map(|ship_id| game.content_registry.ships.get(ship_id))
            .filter_map(|ship| {
                let price = ship.purchase_price?;
                Some(egui_fleet::ShipOffer {
                    ship_id: ship.id.clone(),
                    name: ship.name.clone(),
                    price,
                    affordable: game.credits >= price,
                })
            })
            .collect();
        egui_fleet::ShipyardView {
            station_name: station.name.clone(),
            offers,
        }
    });

    egui_fleet::FleetPaneView {
        pinned: egui_pane_pinned(EguiPane::Fleet),
        ships,
        selected: (!game.owned_ships.is_empty()).then_some(selected_index),
        selected_ship,
        active_cargo: fleet_cargo_rows(&game.inventory),
        transfer_quantity: game.fleet_transfer_quantity,
        quantity_step,
        credits: game.credits,
        shipyard,
        latest_operation: game
            .operation_feedback
            .iter()
            .find(|entry| matches!(entry.category.as_str(), "Fleet" | "Shipyard"))
            .map(|entry| format!("{} / {}", entry.category, entry.message)),
    }
}

fn npc_pane_view(game: &GameState, quantity_step: u32) -> egui_npc::NpcPaneView {
    let npc_index = game.selected_npc_ship.unwrap_or(usize::MAX);
    let Some(npc) = game.npc_ships.get(npc_index) else {
        return egui_npc::NpcPaneView {
            pinned: egui_pane_pinned(EguiPane::Contact),
            name: "Contact unavailable".to_string(),
            artwork: None,
            content_id: String::new(),
            proximity: "Unavailable".to_string(),
            identified: false,
            hostile: false,
            identity: "Unknown".to_string(),
            disposition: "Unknown".to_string(),
            systems: "Unknown".to_string(),
            loadout: "Unknown".to_string(),
            defenses: Vec::new(),
            cargo_summary: "Unknown".to_string(),
            summary: "The selected contact is no longer available.".to_string(),
            interactions: Vec::new(),
            trade_open: false,
            trade_available: false,
            trade_quantity: game.npc_trade_quantity,
            quantity_step,
            credits: game.credits,
            trade: Vec::new(),
            latest_operation: None,
        };
    };

    let identified = npc.identified;
    let hostile = npc_ship_is_hostile(&game.content_registry, npc);
    let in_range = npc_ship_in_interaction_range(&game.ship, npc);
    let trade_available =
        identified && in_range && !hostile && npc_ship_can_trade(&game.content_registry, npc);
    let interactions = npc_interaction_rows(&game.content_registry, &game.ship, npc)
        .into_iter()
        .map(|row| egui_npc::InteractionRow {
            label: row.action.label().to_string(),
            status: row.status.to_string(),
            enabled: row.state == NpcInteractionState::Available,
            complete: row.state == NpcInteractionState::Complete,
        })
        .collect();
    let day = game.world_elapsed_days.floor().max(0.0) as u64;
    let trade = npc
        .cargo_defaults
        .iter()
        .map(|stack| {
            let quote = npc_trade_quote(npc, &stack.item, game.world_seed, day);
            let cargo = game.inventory.count(&stack.item);
            egui_npc::TradeRow {
                item: stack.item.name.clone(),
                stock: stack.count,
                cargo,
                buy_price: quote.buy_price,
                sell_price: quote.sell_price,
                buy_amount: if trade_available {
                    npc_trade_buy_amount(
                        game.credits,
                        &game.inventory,
                        &game.ship_upgrades,
                        stack,
                        &quote,
                        game.npc_trade_quantity,
                    )
                } else {
                    0
                },
                sell_amount: if trade_available {
                    npc_trade_sell_amount(npc, stack, cargo, game.npc_trade_quantity)
                } else {
                    0
                },
            }
        })
        .collect();

    let defenses = if identified {
        npc.equipped_weapons
            .iter()
            .map(|weapon| {
                let mut defense = format!(
                    "{} · {} · range {:.0} · damage {:.0}",
                    weapon.name,
                    weapon.readiness_label(),
                    weapon.range,
                    weapon.damage
                );
                if let Some(ammo) = npc_weapon_ammo_count(npc, weapon) {
                    defense.push_str(&format!(" · ammo {ammo}"));
                }
                defense
            })
            .collect()
    } else {
        Vec::new()
    };
    let cargo_units = npc
        .cargo_defaults
        .iter()
        .map(|stack| stack.count)
        .sum::<u32>();

    egui_npc::NpcPaneView {
        pinned: egui_pane_pinned(EguiPane::Contact),
        name: if identified {
            npc.name.clone()
        } else {
            "Unidentified transponder".to_string()
        },
        artwork: game
            .content_registry
            .npc_ships
            .get(&npc.id)
            .and_then(|definition| definition.texture.clone()),
        content_id: if identified {
            npc.id.clone()
        } else {
            "Identity scan required".to_string()
        },
        proximity: format!(
            "{} · {:.0}u · {}",
            if in_range {
                "Interaction range"
            } else {
                "Approach"
            },
            npc_ship_surface_distance(&game.ship, npc),
            npc.behavior.label()
        ),
        identified,
        hostile,
        identity: if identified {
            format!(
                "{} / {} / {}",
                local_content_id(&npc.id),
                npc.archetype,
                npc.role
            )
        } else {
            "Run identification scan from interaction range".to_string()
        },
        disposition: if identified {
            npc.faction
                .as_deref()
                .map(|faction| {
                    format!(
                        "{} / {}",
                        faction_name(&game.content_registry, faction),
                        faction_disposition_label(&game.content_registry, faction)
                    )
                })
                .unwrap_or_else(|| "Independent / unknown".to_string())
        } else {
            "Unknown".to_string()
        },
        systems: if identified {
            format!(
                "Hull {:.0}/{:.0} · Shield {:.0}/{:.0} · Energy {:.0}/{:.0}",
                npc.hull.current,
                npc.hull.max,
                npc.shields.current,
                npc.shields.max,
                npc.energy.current,
                npc.energy.max
            )
        } else {
            "Unscanned".to_string()
        },
        loadout: if identified {
            format!(
                "{} shield · {} turret · cargo {}",
                npc.shield_slots.len(),
                npc.equipped_weapons.len(),
                format_mass(npc.cargo_capacity)
            )
        } else {
            "Unknown".to_string()
        },
        defenses,
        cargo_summary: if identified {
            format!(
                "{} manifest item(s) · {} unit(s)",
                npc.cargo_defaults.len(),
                cargo_units
            )
        } else {
            "Unknown".to_string()
        },
        summary: if identified {
            npc.summary.clone()
        } else {
            "Contact details are unavailable until identification completes.".to_string()
        },
        interactions,
        trade_open: npc.trade_open,
        trade_available,
        trade_quantity: game.npc_trade_quantity,
        quantity_step,
        credits: game.credits,
        trade,
        latest_operation: game
            .operation_feedback
            .iter()
            .find(|entry| {
                matches!(
                    entry.category.as_str(),
                    "Contact" | "Rumor" | "Trade" | "Follow"
                )
            })
            .map(|entry| format!("{} / {}", entry.category, entry.message)),
    }
}

fn active_production_rows(game: &GameState) -> (&[Recipe], &[CraftSetting]) {
    match game.production_mode {
        ProductionMode::Smelting => (&game.smelt_recipes, &game.smelt_settings),
        ProductionMode::Crafting => (&game.craft_recipes, &game.craft_settings),
        ProductionMode::Processing => (&game.processing_recipes, &game.processing_settings),
    }
}

fn production_tab_for_mode(mode: ProductionMode) -> egui_inventory::ProductionTab {
    match mode {
        ProductionMode::Smelting => egui_inventory::ProductionTab::Smelt,
        ProductionMode::Crafting => egui_inventory::ProductionTab::Craft,
        ProductionMode::Processing => egui_inventory::ProductionTab::Process,
    }
}

fn production_mode_for_tab(tab: egui_inventory::ProductionTab) -> ProductionMode {
    match tab {
        egui_inventory::ProductionTab::Smelt => ProductionMode::Smelting,
        egui_inventory::ProductionTab::Craft => ProductionMode::Crafting,
        egui_inventory::ProductionTab::Process => ProductionMode::Processing,
    }
}

fn production_keep_click_step() -> i32 {
    production_keep_click_step_for_keys(
        is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift),
        is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl),
    )
}

fn production_keep_click_step_for_keys(shift: bool, control: bool) -> i32 {
    if control {
        20
    } else if shift {
        10
    } else {
        1
    }
}

fn ship_pane_view(game: &GameState) -> egui_inventory::ShipPane {
    let ship_name = game
        .content_registry
        .ships
        .get(&game.ship_id)
        .map(|ship| ship.name.clone())
        .unwrap_or_else(|| local_content_id(&game.ship_id).replace('_', " "));
    let shield_choices = inventory_shield_choices(game);
    let weapon_choices = inventory_weapon_choices(game);
    let shields = (0..shield_slot_capacity(game))
        .map(|slot_index| {
            let installed = game.equipped_shields.get(slot_index);
            egui_inventory::EquipmentSlot {
                label: format!("Shield Bank #{}", slot_index + 1),
                installed_id: installed.map(|shield| shield.id.clone()),
                installed_name: installed
                    .map(|shield| shield.name.clone())
                    .unwrap_or_else(|| "Empty shield slot".to_string()),
                choices: shield_choices.clone(),
            }
        })
        .collect();
    let weapons = (0..weapon_slot_capacity(game))
        .map(|slot_index| {
            let installed = game
                .equipped_weapons
                .get(slot_index)
                .filter(|weapon| weapon.resolved);
            egui_inventory::EquipmentSlot {
                label: format!("Turret Bank #{}", slot_index + 1),
                installed_id: installed.map(|weapon| weapon.id.clone()),
                installed_name: installed
                    .map(|weapon| weapon.name.clone())
                    .unwrap_or_else(|| "Empty weapon slot".to_string()),
                choices: weapon_choices.clone(),
            }
        })
        .collect();
    let power = game
        .installed_power_modules
        .iter()
        .map(|module| {
            format!(
                "{} · +{:.1}/s · {:.0}% heat",
                module.name,
                module.generation,
                module.heat * 100.0
            )
        })
        .collect();

    egui_inventory::ShipPane {
        ship_name,
        artwork: game
            .content_registry
            .ships
            .get(&game.ship_id)
            .and_then(|definition| definition.texture.clone()),
        cargo: format!(
            "Cargo {} / {}",
            format_mass(game.inventory.total_mass()),
            format_mass(cargo_rating_kg(&game.ship_upgrades))
        ),
        hull: format!("{:.0}%", game.ship.systems.hull.fraction() * 100.0),
        shield: format!("{:.0}%", game.ship.systems.shields.fraction() * 100.0),
        energy: format!("{:.0}%", game.ship.systems.energy.fraction() * 100.0),
        mass: format_mass(game.ship.attributes.mass),
        engine: format!("{:.0}", game.ship.forward_acceleration()),
        turn: format!("{:.1}", game.ship.turn_acceleration()),
        power,
        shields,
        weapons,
        feedback: game
            .operation_feedback
            .iter()
            .map(|entry| format!("{} · {}", entry.category, entry.message))
            .collect(),
    }
}

fn inventory_shield_choices(game: &GameState) -> Vec<egui_inventory::EquipmentChoice> {
    let installed = game
        .equipped_shields
        .iter()
        .map(|shield| shield.id.as_str())
        .collect::<HashSet<_>>();
    game.content_registry
        .shield_order
        .iter()
        .filter_map(|shield_id| {
            let shield = game.content_registry.shields.get(shield_id)?;
            let item = registry_item(&game.content_registry, &shield.install_item)?;
            (installed.contains(shield_id.as_str()) || game.inventory.count(&item) > 0).then(|| {
                egui_inventory::EquipmentChoice {
                    id: shield_id.clone(),
                    name: shield.name.clone(),
                }
            })
        })
        .collect()
}

fn inventory_weapon_choices(game: &GameState) -> Vec<egui_inventory::EquipmentChoice> {
    let installed = game
        .equipped_weapons
        .iter()
        .filter(|weapon| weapon.resolved)
        .map(|weapon| weapon.id.as_str())
        .collect::<HashSet<_>>();
    game.content_registry
        .weapon_order
        .iter()
        .filter_map(|weapon_id| {
            let weapon = game.content_registry.weapons.get(weapon_id)?;
            if weapon.kind != content::WeaponKind::TurretDefense {
                return None;
            }
            let item = registry_item(&game.content_registry, &weapon.install_item)?;
            (installed.contains(weapon_id.as_str()) || game.inventory.count(&item) > 0).then(|| {
                egui_inventory::EquipmentChoice {
                    id: weapon_id.clone(),
                    name: weapon.name.clone(),
                }
            })
        })
        .collect()
}

fn apply_inventory_ui_actions(
    game: &mut GameState,
    actions: Vec<egui_inventory::InventoryUiAction>,
    keep_click_step: i32,
) {
    for action in actions {
        match action {
            egui_inventory::InventoryUiAction::SetProductionTab(tab) => {
                let mode = production_mode_for_tab(tab);
                if game.production_mode != mode {
                    game.production_mode = mode;
                    game.work_scroll = 0.0;
                    game.save_dirty = true;
                    game.audio.play(audio::AudioCue::Select);
                }
            }
            egui_inventory::InventoryUiAction::AdjustKeep { row, direction } => {
                let unlocked = active_production_rows(game)
                    .0
                    .get(row)
                    .is_some_and(|recipe| recipe_is_unlocked(game, &recipe.id));
                if unlocked {
                    let settings = match game.production_mode {
                        ProductionMode::Smelting => &mut game.smelt_settings,
                        ProductionMode::Crafting => &mut game.craft_settings,
                        ProductionMode::Processing => &mut game.processing_settings,
                    };
                    if let Some(setting) = settings.get_mut(row) {
                        adjust_work_setting(
                            &mut setting.keep,
                            WorkColumn::Keep,
                            direction.signum() * keep_click_step,
                        );
                        game.save_dirty = true;
                    }
                }
            }
            egui_inventory::InventoryUiAction::QueueRecipe { row } => {
                let unlocked = active_production_rows(game)
                    .0
                    .get(row)
                    .is_some_and(|recipe| recipe_is_unlocked(game, &recipe.id));
                if unlocked {
                    let settings = match game.production_mode {
                        ProductionMode::Smelting => &mut game.smelt_settings,
                        ProductionMode::Crafting => &mut game.craft_settings,
                        ProductionMode::Processing => &mut game.processing_settings,
                    };
                    if let Some(setting) = settings.get_mut(row) {
                        setting.queued = setting.queued.saturating_add(1).min(999);
                        game.save_dirty = true;
                    }
                }
            }
            egui_inventory::InventoryUiAction::InstallShield { slot, shield_id } => {
                let installed = install_shield_in_slot(game, slot, &shield_id).is_ok();
                game.audio.play(if installed {
                    audio::AudioCue::Confirm
                } else {
                    audio::AudioCue::Warning
                });
            }
            egui_inventory::InventoryUiAction::InstallWeapon { slot, weapon_id } => {
                let installed = install_weapon_in_slot(game, slot, &weapon_id).is_ok();
                game.audio.play(if installed {
                    audio::AudioCue::Confirm
                } else {
                    audio::AudioCue::Warning
                });
            }
            egui_inventory::InventoryUiAction::OpenUpgrades => {
                game.upgrades_open = true;
                game.inventory_open = false;
                game.audio.play(audio::AudioCue::Select);
            }
            egui_inventory::InventoryUiAction::ToggleShipPin => {
                toggle_egui_pane_pin(EguiPane::Ship);
            }
            egui_inventory::InventoryUiAction::ToggleProductionPin => {
                toggle_egui_pane_pin(EguiPane::Production);
            }
            egui_inventory::InventoryUiAction::ToggleInventoryPin => {
                toggle_egui_pane_pin(EguiPane::Inventory);
            }
        }
    }
}

fn apply_planet_ui_actions(
    game: &mut GameState,
    actions: Vec<egui_planet::PlanetUiAction>,
    keep_click_step: i32,
) {
    let Some(planet_index) = game.selected_planet else {
        return;
    };

    for action in actions {
        match action {
            egui_planet::PlanetUiAction::TogglePin => {
                toggle_egui_pane_pin(EguiPane::Planet);
            }
            egui_planet::PlanetUiAction::EnterOrbit => {
                if enter_planet_orbit(game, planet_index) {
                    game.audio.play(audio::AudioCue::Confirm);
                } else {
                    game.audio.play(audio::AudioCue::Warning);
                }
            }
            egui_planet::PlanetUiAction::LaunchScan => {
                let scanned = launch_planet_scan(game, planet_index);
                game.audio.play(if scanned {
                    audio::AudioCue::Confirm
                } else {
                    audio::AudioCue::Warning
                });
                if scanned {
                    game.save_dirty = true;
                }
            }
            egui_planet::PlanetUiAction::AdjustMiningKeep { row, direction } => {
                let available = game.planets.get(planet_index).is_some_and(|planet| {
                    planet_has_composition_scan(planet)
                        && planet_in_interaction_range(&game.ship, planet)
                });
                if available {
                    if let Some(setting) = game
                        .planets
                        .get_mut(planet_index)
                        .and_then(|planet| planet.mining.get_mut(row))
                    {
                        adjust_work_setting(
                            &mut setting.keep,
                            WorkColumn::Keep,
                            direction.signum() * keep_click_step,
                        );
                        game.save_dirty = true;
                    }
                }
            }
            egui_planet::PlanetUiAction::QueueMining { row } => {
                let available = game.planets.get(planet_index).is_some_and(|planet| {
                    planet_has_composition_scan(planet)
                        && planet_in_interaction_range(&game.ship, planet)
                });
                if available {
                    if let Some(setting) = game
                        .planets
                        .get_mut(planet_index)
                        .and_then(|planet| planet.mining.get_mut(row))
                    {
                        setting.queued = setting.queued.saturating_add(1).min(999);
                        game.save_dirty = true;
                    }
                }
            }
        }
    }
}

fn apply_station_ui_actions(game: &mut GameState, actions: Vec<egui_station::StationUiAction>) {
    let Some(station_index) = game.selected_station else {
        return;
    };

    for action in actions {
        let handled = match action {
            egui_station::StationUiAction::TogglePin => {
                toggle_egui_pane_pin(EguiPane::Station);
                true
            }
            egui_station::StationUiAction::SelectService(service_index) => {
                select_station_service(game, station_index, service_index)
            }
            egui_station::StationUiAction::Buy(offer_index) => {
                game.selected_station_service.is_some_and(|service_index| {
                    buy_station_trade_offer(game, station_index, service_index, offer_index)
                })
            }
            egui_station::StationUiAction::Sell(offer_index) => {
                game.selected_station_service.is_some_and(|service_index| {
                    sell_station_trade_offer(game, station_index, service_index, offer_index)
                })
            }
            egui_station::StationUiAction::Repair => repair_ship_at_station(game, station_index),
            egui_station::StationUiAction::Contract(contract_index) => {
                game.selected_station_service.is_some_and(|service_index| {
                    accept_or_complete_contract(game, station_index, service_index, contract_index)
                })
            }
            egui_station::StationUiAction::PurchaseRecipe(unlock_index) => {
                game.selected_station_service.is_some_and(|service_index| {
                    purchase_recipe_unlock(game, station_index, service_index, unlock_index)
                })
            }
        };
        game.audio.play(if handled {
            audio::AudioCue::Confirm
        } else {
            audio::AudioCue::Warning
        });
    }
}

fn apply_fleet_ui_actions(
    game: &mut GameState,
    actions: Vec<egui_fleet::FleetUiAction>,
    quantity_step: u32,
) {
    for action in actions {
        let handled = match action {
            egui_fleet::FleetUiAction::TogglePin => {
                toggle_egui_pane_pin(EguiPane::Fleet);
                true
            }
            egui_fleet::FleetUiAction::SelectShip(index) => {
                if index < game.owned_ships.len() {
                    game.selected_owned_ship_index = index;
                    true
                } else {
                    false
                }
            }
            egui_fleet::FleetUiAction::SwitchControl => {
                switch_active_owned_ship(game, game.selected_owned_ship_index)
            }
            egui_fleet::FleetUiAction::AdjustTransferQuantity(direction) => {
                if !owned_ship_in_transfer_range(game, game.selected_owned_ship_index)
                    || direction == 0
                {
                    false
                } else {
                    let amount = quantity_step.max(1);
                    game.fleet_transfer_quantity = if direction < 0 {
                        game.fleet_transfer_quantity.saturating_sub(amount).max(1)
                    } else {
                        game.fleet_transfer_quantity.saturating_add(amount).min(999)
                    };
                    true
                }
            }
            egui_fleet::FleetUiAction::TransferToSelected(item_id) => {
                transfer_owned_ship_item(
                    game,
                    game.selected_owned_ship_index,
                    &item_id,
                    true,
                    game.fleet_transfer_quantity,
                ) > 0
            }
            egui_fleet::FleetUiAction::TransferToActive(item_id) => {
                transfer_owned_ship_item(
                    game,
                    game.selected_owned_ship_index,
                    &item_id,
                    false,
                    game.fleet_transfer_quantity,
                ) > 0
            }
            egui_fleet::FleetUiAction::BuyShip(ship_id) => nearby_garage_index(game)
                .is_some_and(|station_index| acquire_owned_ship(game, station_index, &ship_id)),
        };
        game.audio.play(if handled {
            audio::AudioCue::Confirm
        } else {
            audio::AudioCue::Warning
        });
    }
}

fn apply_content_ui_actions(game: &mut GameState, actions: Vec<egui_content::ContentUiAction>) {
    for action in actions {
        match action {
            egui_content::ContentUiAction::TogglePin => {
                toggle_egui_pane_pin(EguiPane::Content);
            }
            egui_content::ContentUiAction::SelectPack(index) => {
                if index.is_none_or(|index| index < game.content_registry.packs.len()) {
                    game.content_browser.selected_pack_index = index;
                    game.content_browser.selected_object = None;
                    game.content_browser.object_history.clear();
                    game.content_browser.items_scroll = 0.0;
                    game.content_browser.recipes_scroll = 0.0;
                    game.content_browser.npc_ships_scroll = 0.0;
                    game.content_browser.planets_scroll = 0.0;
                    game.audio.play(audio::AudioCue::Select);
                } else {
                    game.audio.play(audio::AudioCue::Warning);
                }
            }
            egui_content::ContentUiAction::SelectObject(key) => {
                if content_object_detail(&game.content_registry, &key).is_some() {
                    if game.content_browser.selected_object.as_ref() != Some(&key) {
                        if let Some(previous) = game.content_browser.selected_object.take() {
                            game.content_browser.object_history.push(previous);
                        }
                        game.content_browser.selected_pack_index =
                            key.id.split_once(':').and_then(|(pack_id, _)| {
                                game.content_registry
                                    .packs
                                    .iter()
                                    .position(|pack| pack.id == pack_id)
                            });
                        game.content_browser.selected_object = Some(key);
                    }
                    game.audio.play(audio::AudioCue::Select);
                } else {
                    game.audio.play(audio::AudioCue::Warning);
                }
            }
            egui_content::ContentUiAction::BackObject => {
                if let Some(previous) = game.content_browser.object_history.pop() {
                    game.content_browser.selected_pack_index =
                        previous.id.split_once(':').and_then(|(pack_id, _)| {
                            game.content_registry
                                .packs
                                .iter()
                                .position(|pack| pack.id == pack_id)
                        });
                    game.content_browser.selected_object = Some(previous);
                    game.audio.play(audio::AudioCue::Select);
                } else {
                    game.audio.play(audio::AudioCue::Warning);
                }
            }
        }
    }
}

fn apply_contracts_ui_actions(
    game: &mut GameState,
    actions: Vec<egui_contracts::ContractsUiAction>,
) {
    for action in actions {
        match action {
            egui_contracts::ContractsUiAction::TogglePin => {
                toggle_egui_pane_pin(EguiPane::Contracts);
            }
            egui_contracts::ContractsUiAction::Select(index) => {
                if index < active_contract_menu_entries(game).len() {
                    game.selected_contract_index = Some(index);
                    game.audio.play(audio::AudioCue::Select);
                } else {
                    game.audio.play(audio::AudioCue::Warning);
                }
            }
            egui_contracts::ContractsUiAction::FocusOrigin(index) => {
                let entry = active_contract_menu_entries(game).get(index).cloned();
                if let Some(entry) = entry {
                    game.selected_contract_index = Some(index);
                    let contracts_were_open = game.contracts_open;
                    focus_active_contract(game, &entry);
                    game.audio
                        .play(if contracts_were_open && !game.contracts_open {
                            audio::AudioCue::Confirm
                        } else {
                            audio::AudioCue::Warning
                        });
                } else {
                    game.audio.play(audio::AudioCue::Warning);
                }
            }
        }
    }
}

fn apply_upgrades_ui_actions(
    game: &mut GameState,
    actions: Vec<egui_upgrades::UpgradesUiAction>,
    purchase_count: u32,
) {
    for action in actions {
        let handled = match action {
            egui_upgrades::UpgradesUiAction::TogglePin => {
                toggle_egui_pane_pin(EguiPane::Upgrades);
                true
            }
            egui_upgrades::UpgradesUiAction::Purchase(index) => {
                let mut purchased = false;
                for _ in 0..purchase_count.max(1) {
                    if !buy_ship_upgrade(game, index) {
                        break;
                    }
                    purchased = true;
                }
                purchased
            }
        };
        game.audio.play(if handled {
            audio::AudioCue::Confirm
        } else {
            audio::AudioCue::Warning
        });
    }
}

fn apply_research_ui_actions(game: &mut GameState, actions: Vec<egui_research::ResearchUiAction>) {
    for action in actions {
        let handled = match action {
            egui_research::ResearchUiAction::TogglePin => {
                toggle_egui_pane_pin(EguiPane::Research);
                true
            }
            egui_research::ResearchUiAction::Select(research_id) => {
                if game.content_registry.research.contains_key(&research_id) {
                    game.selected_research = Some(research_id);
                    true
                } else {
                    false
                }
            }
            egui_research::ResearchUiAction::Start(research_id) => {
                start_research(game, &research_id)
            }
        };
        game.audio.play(if handled {
            audio::AudioCue::Confirm
        } else {
            audio::AudioCue::Warning
        });
    }
}

fn apply_destinations_ui_actions(
    game: &mut GameState,
    actions: Vec<egui_destinations::DestinationsUiAction>,
) {
    for action in actions {
        let handled = match action {
            egui_destinations::DestinationsUiAction::TogglePin => {
                toggle_egui_pane_pin(EguiPane::Destinations);
                true
            }
            egui_destinations::DestinationsUiAction::FlySelected => {
                fly_to_selected_starmap_planet(game)
            }
            egui_destinations::DestinationsUiAction::AddSelected => {
                add_selected_starmap_planet_to_route(game)
            }
            egui_destinations::DestinationsUiAction::Remove(index) => {
                remove_navigation_route_stop(game, index)
            }
            egui_destinations::DestinationsUiAction::MoveUp(index) => {
                move_navigation_route_stop(game, index, index.saturating_sub(1))
            }
            egui_destinations::DestinationsUiAction::MoveDown(index) => {
                move_navigation_route_stop(game, index, index.saturating_add(1))
            }
            egui_destinations::DestinationsUiAction::Activate => activate_navigation_route(game),
            egui_destinations::DestinationsUiAction::Pause => {
                let was_active = game.navigation_route_active;
                pause_navigation_route(game, Some("Itinerary paused"));
                was_active
            }
            egui_destinations::DestinationsUiAction::Clear => {
                let had_route = !game.navigation_route.is_empty();
                clear_navigation_route(game);
                had_route
            }
        };
        game.audio.play(if handled {
            audio::AudioCue::Confirm
        } else {
            audio::AudioCue::Warning
        });
    }
}

fn apply_npc_ui_actions(
    game: &mut GameState,
    actions: Vec<egui_npc::NpcUiAction>,
    quantity_step: u32,
) {
    let Some(npc_index) = game.selected_npc_ship else {
        return;
    };

    for action in actions {
        let handled = match action {
            egui_npc::NpcUiAction::TogglePin => {
                toggle_egui_pane_pin(EguiPane::Contact);
                true
            }
            egui_npc::NpcUiAction::Interaction(row_index) => {
                let action = game.npc_ships.get(npc_index).and_then(|npc| {
                    npc_interaction_rows(&game.content_registry, &game.ship, npc)
                        .get(row_index)
                        .filter(|row| row.state == NpcInteractionState::Available)
                        .map(|row| row.action)
                });
                match action {
                    Some(NpcInteractionAction::Identify) => identify_selected_npc_ship(game),
                    Some(NpcInteractionAction::Hail) => {
                        let rumor = game
                            .npc_ships
                            .get(npc_index)
                            .map(|npc| npc_ship_rumor(game, npc));
                        if let Some(rumor) = rumor {
                            push_operation_feedback(game, "Rumor", rumor);
                            true
                        } else {
                            false
                        }
                    }
                    Some(NpcInteractionAction::Trade) => {
                        if let Some(npc) = game.npc_ships.get_mut(npc_index) {
                            npc.trade_open = true;
                            game.npc_trade_quantity = 1;
                            true
                        } else {
                            false
                        }
                    }
                    Some(NpcInteractionAction::Dock | NpcInteractionAction::Conflict) | None => {
                        false
                    }
                }
            }
            egui_npc::NpcUiAction::CloseTrade => {
                if let Some(npc) = game.npc_ships.get_mut(npc_index) {
                    npc.trade_open = false;
                    true
                } else {
                    false
                }
            }
            egui_npc::NpcUiAction::AdjustTradeQuantity(direction) => {
                let amount = quantity_step.max(1);
                game.npc_trade_quantity = if direction < 0 {
                    game.npc_trade_quantity.saturating_sub(amount).max(1)
                } else {
                    game.npc_trade_quantity.saturating_add(amount).min(999)
                };
                true
            }
            egui_npc::NpcUiAction::Buy(row) => {
                execute_npc_trade(game, npc_index, row, NpcTradeAction::Buy)
            }
            egui_npc::NpcUiAction::Sell(row) => {
                execute_npc_trade(game, npc_index, row, NpcTradeAction::Sell)
            }
        };
        game.audio.play(if handled {
            audio::AudioCue::Confirm
        } else {
            audio::AudioCue::Warning
        });
    }
}

#[derive(Clone)]
struct ContractMenuEntry {
    contract: ContractOffer,
    origin_name: String,
    target_name: String,
    status: String,
    progress: String,
    deadline: String,
}

fn active_contract_menu_entries(game: &GameState) -> Vec<ContractMenuEntry> {
    game.active_contracts
        .iter()
        .filter_map(|active| {
            let station = game
                .stations
                .iter()
                .find(|station| station.id == active.origin_station)?;
            let contract = station
                .services
                .iter()
                .find(|service| service.id == active.origin_service)
                .and_then(|service| {
                    service.contracts.iter().find(|contract| {
                        contract.id == active.id
                            && contract.origin_station == active.origin_station
                            && contract.origin_service == active.origin_service
                    })
                })?
                .clone();
            let target_name = contract_target_name(game, &contract);
            let expired = game.world_elapsed_days > active.expires_day;
            let completion_ready = active.target_reached
                && (contract.kind != "hauling"
                    || contract
                        .item
                        .as_ref()
                        .is_some_and(|item| game.inventory.count(item) >= contract.amount));
            let status = if expired {
                "Expired"
            } else if completion_ready {
                "Ready to complete"
            } else {
                "Active"
            };
            let remaining_days = (active.expires_day - game.world_elapsed_days).max(0.0);
            Some(ContractMenuEntry {
                progress: if contract.kind == "hauling" {
                    let count = contract
                        .item
                        .as_ref()
                        .map(|item| game.inventory.count(item))
                        .unwrap_or_default();
                    format!("Cargo {count}/{}", contract.amount)
                } else if active.target_reached {
                    format!("Survey level {}/{}", contract.amount, contract.amount)
                } else {
                    let scan_level = contract
                        .target_planet
                        .as_deref()
                        .and_then(|target| {
                            game.planets
                                .iter()
                                .find(|planet| planet.id == target)
                                .map(|planet| planet.scan_level as u32)
                        })
                        .unwrap_or_default();
                    format!(
                        "Survey level {}/{}",
                        scan_level.min(contract.amount),
                        contract.amount
                    )
                },
                deadline: format!("{remaining_days:.1} days remaining"),
                contract,
                origin_name: station.name.clone(),
                target_name,
                status: status.to_string(),
            })
        })
        .collect()
}

fn contract_target_name(game: &GameState, contract: &ContractOffer) -> String {
    if contract.kind == "hauling" {
        contract
            .target_station
            .as_deref()
            .and_then(|target| game.stations.iter().find(|station| station.id == target))
            .map(|station| station.name.clone())
            .or_else(|| contract.target_station.clone())
            .unwrap_or_else(|| "Target station".to_string())
    } else {
        contract
            .target_planet
            .as_deref()
            .and_then(|target| game.planets.iter().find(|planet| planet.id == target))
            .map(|planet| planet.id.clone())
            .or_else(|| contract.target_planet.clone())
            .unwrap_or_else(|| "Target planet".to_string())
    }
}

fn focus_active_contract(game: &mut GameState, entry: &ContractMenuEntry) {
    let Some(station_index) = game
        .stations
        .iter()
        .position(|station| station.id == entry.contract.origin_station)
    else {
        push_operation_feedback(
            game,
            "Contracts",
            "Origin station is unavailable".to_string(),
        );
        return;
    };
    let station = &game.stations[station_index];
    if station.system != game.current_system_id {
        push_operation_feedback(game, "Contracts", format!("Origin: {}", entry.origin_name));
        return;
    }
    let Some(service_index) = station
        .services
        .iter()
        .position(|service| service.id == entry.contract.origin_service)
    else {
        return;
    };
    game.selected_station = Some(station_index);
    game.selected_station_service = Some(service_index);
    game.selected_planet = None;
    game.selected_npc_ship = None;
    game.contracts_open = false;
    game.inventory_open = true;
    push_operation_feedback(
        game,
        "Contracts",
        format!("Focused {} at {}", entry.contract.name, entry.origin_name),
    );
}

fn fit_debug_text(text: &str, width: f32, font_size: u16) -> String {
    let mut fitted = text.to_string();
    let original_char_count = text.chars().count();
    while fitted.chars().count() > 3
        && measure_text(&fitted, None, font_size, 1.0).width > width - 12.0
    {
        fitted.pop();
    }
    append_debug_ellipsis(fitted, original_char_count)
}

fn append_debug_ellipsis(mut fitted: String, original_char_count: usize) -> String {
    if fitted.chars().count() < original_char_count && fitted.chars().count() > 3 {
        for _ in 0..3 {
            fitted.pop();
        }
        while fitted.chars().last().is_some_and(char::is_whitespace) {
            fitted.pop();
        }
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

fn ship_upgrades_table_viewport_height() -> f32 {
    let (_, _, _, panel_height) = ship_upgrades_panel_rect();
    (panel_height - GAME_PANEL_BODY_TOP - 42.0).max(0.0)
}

fn format_seconds(seconds: f32) -> String {
    let seconds = finite_nonnegative_or(seconds, 0.0).ceil() as u32;
    if seconds < 60 {
        format!("{seconds}s")
    } else {
        format!("{}m {}s", seconds / 60, seconds % 60)
    }
}

fn research_requirement_labels(
    registry: &content::ContentRegistry,
    research: &content::ResearchDef,
) -> Vec<String> {
    let mut labels = research
        .requires
        .iter()
        .chain(research.revealed_by.iter())
        .map(|research_id| research_display_name(registry, research_id))
        .collect::<Vec<_>>();
    labels.sort();
    labels.dedup();
    labels
}

fn research_reward_label(
    registry: &content::ContentRegistry,
    reward: &content::ResearchRewardDef,
) -> String {
    match reward.kind.as_str() {
        "recipe_unlock" => reward
            .target
            .as_deref()
            .map(|recipe| format!("Unlock recipe: {}", recipe_display_name(registry, recipe)))
            .unwrap_or_else(|| "Unlock recipe".to_string()),
        "item_visibility" => reward
            .target
            .as_deref()
            .map(|item_id| {
                let name = registry
                    .items
                    .get(item_id)
                    .map(|item| item.name.as_str())
                    .unwrap_or(item_id);
                format!("Reveal item: {name}")
            })
            .unwrap_or_else(|| "Reveal item".to_string()),
        "station_visibility" => reward
            .target
            .as_deref()
            .map(|station_id| {
                let name = registry
                    .stations
                    .get(station_id)
                    .map(|station| station.name.as_str())
                    .unwrap_or(station_id);
                format!("Reveal station: {name}")
            })
            .unwrap_or_else(|| "Reveal station".to_string()),
        "mining_speed_percent" => format!("Mining speed +{:.0}%", reward.amount.unwrap_or(0.0)),
        "smelting_speed_percent" => {
            format!("Smelting speed +{:.0}%", reward.amount.unwrap_or(0.0))
        }
        "fabrication_speed_percent" => {
            format!("Fabrication speed +{:.0}%", reward.amount.unwrap_or(0.0))
        }
        "bonus_output_chance" => {
            format!("Bonus output chance +{:.0}%", reward.amount.unwrap_or(0.0))
        }
        _ => reward.kind.clone(),
    }
}

fn work_table_height() -> f32 {
    let panel_height = inventory_panel_height();
    panel_height - 132.0
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

fn contract_target_name_from_station_data(
    station: &StationDestination,
    contract: &ContractOffer,
    stations: &[StationDestination],
    planets: &[Planet],
) -> String {
    if contract.kind == "hauling" {
        contract
            .target_station
            .as_deref()
            .and_then(|target| stations.iter().find(|station| station.id == target))
            .map(|station| station.name.clone())
            .or_else(|| contract.target_station.clone())
            .unwrap_or_else(|| station.name.clone())
    } else {
        contract
            .target_planet
            .as_deref()
            .and_then(|target| planets.iter().find(|planet| planet.id == target))
            .map(|planet| planet.id.clone())
            .or_else(|| contract.target_planet.clone())
            .unwrap_or_else(|| "target planet".to_string())
    }
}

fn format_trade_stock(offer: &TradeOffer, world_elapsed_days: f32) -> String {
    if offer.unavailable {
        return "unavail".to_string();
    }
    let stock = offer
        .stock
        .map(|stock| stock.to_string())
        .unwrap_or_else(|| "open".to_string());
    if let (Some(max_stock), Some(next_restock_day)) = (offer.max_stock, offer.next_restock_day) {
        let remaining = (next_restock_day - world_elapsed_days).max(0.0);
        format!("Stock {stock}/{max_stock} · Restock {remaining:.1}d")
    } else {
        stock
    }
}

#[cfg(test)]
fn operation_feedback_contains(game: &GameState, category: &str, text: &str) -> bool {
    game.operation_feedback
        .iter()
        .any(|entry| entry.category == category && entry.message.contains(text))
}

#[cfg(test)]
fn latest_operation_feedback(game: &GameState) -> Option<(&str, &str)> {
    game.operation_feedback
        .first()
        .map(|entry| (entry.category.as_str(), entry.message.as_str()))
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
    pressure_contacts: usize,
    incoming_weapon_fire: usize,
    collision_warning: Option<&'a CollisionImpactEvent>,
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
        pressure_contacts,
        incoming_weapon_fire,
        collision_warning,
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

    if incoming_weapon_fire > 0 {
        draw_text(
            &format!("Incoming weapon fire x{incoming_weapon_fire}"),
            34.0,
            184.0,
            20.0,
            Color::from_rgba(226, 104, 96, 255),
        );
        return;
    }

    if let Some(event) = collision_warning {
        let label = match event.severity {
            CollisionImpactSeverity::Critical => "Critical collision impact",
            CollisionImpactSeverity::Destructive => "Destructive collision impact",
            CollisionImpactSeverity::Minor | CollisionImpactSeverity::Significant => {
                "Collision impact"
            }
        };
        draw_text(
            label,
            34.0,
            184.0,
            20.0,
            Color::from_rgba(247, 116, 96, 255),
        );
        return;
    }

    if pressure_contacts > 0 {
        draw_text(
            &format!("Redwake probe pressure x{pressure_contacts}"),
            34.0,
            184.0,
            20.0,
            Color::from_rgba(226, 104, 96, 255),
        );
        return;
    }

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
    if game.map_open || game.research_open || game.upgrades_open || game.content_open {
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
    fn new_game_ship_choices_include_pack_hulls_and_highlight_the_pack_default() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let ships = title_ships(&registry);
        let default_ship = resolved_new_game_ship_id(&registry, None);

        assert!(ships.iter().any(|ship| ship.id == STARTER_SHIP_ID));
        assert!(ships.iter().any(|ship| {
            ship.id == "turrets-galore:twinspire_gunship"
                && ship.weapon_banks == 2
                && ship
                    .texture
                    .as_deref()
                    .is_some_and(|path| path.ends_with("assets/ships/twinspire-gunship.png"))
        }));
        assert_eq!(default_ship, "turrets-galore:twinspire_gunship");
    }

    #[test]
    fn explicit_new_game_ship_selection_overrides_the_pack_default() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");

        assert_eq!(
            resolved_new_game_ship_id(&registry, Some(STARTER_SHIP_ID)),
            STARTER_SHIP_ID
        );
        assert_eq!(
            resolved_new_game_ship_id(&registry, Some("missing-pack:missing-ship")),
            "turrets-galore:twinspire_gunship"
        );
    }

    #[test]
    fn ship_picker_cycles_in_both_directions() {
        assert_eq!(cycled_index(0, 2, 1), 1);
        assert_eq!(cycled_index(1, 2, 1), 0);
        assert_eq!(cycled_index(0, 2, -1), 1);
        assert_eq!(cycled_index(1, 2, -1), 0);
        assert_eq!(cycled_index(1, 8, 3), 4);
        assert_eq!(cycled_index(1, 8, -3), 6);
        assert_eq!(cycled_index(7, 0, 1), 0);
    }

    #[test]
    fn contained_textures_preserve_their_aspect_ratio() {
        let fitted = contained_texture_size(vec2(353.0, 512.0), vec2(64.0, 64.0));

        assert!((fitted.x - 44.125).abs() < 0.001);
        assert_eq!(fitted.y, 64.0);
        assert!((fitted.x / fitted.y - 353.0 / 512.0).abs() < 0.001);
        assert_eq!(
            contained_texture_size(Vec2::ZERO, vec2(64.0, 64.0)),
            Vec2::ZERO
        );
    }

    #[test]
    fn ship_preview_ignores_transparent_canvas_padding() {
        let mut image = Image {
            bytes: vec![0; 8 * 6 * 4],
            width: 8,
            height: 6,
        };
        for y in 1..=4 {
            for x in 2..=5 {
                image.bytes[(y * 8 + x) * 4 + 3] = 255;
            }
        }

        assert_eq!(image_alpha_bounds(&image), Rect::new(2.0, 1.0, 4.0, 4.0));

        let transparent = Image {
            bytes: vec![0; 3 * 2 * 4],
            width: 3,
            height: 2,
        };
        assert_eq!(
            image_alpha_bounds(&transparent),
            Rect::new(0.0, 0.0, 3.0, 2.0)
        );
    }

    #[test]
    fn egui_art_thumbnails_crop_padding_and_cap_the_long_edge() {
        let mut image = Image {
            bytes: vec![0; 12 * 8 * 4],
            width: 12,
            height: 8,
        };
        for y in 2..=5 {
            for x in 2..=9 {
                let pixel = (y * 12 + x) * 4;
                image.bytes[pixel..pixel + 4].copy_from_slice(&[20, 40, 60, 255]);
            }
        }

        let thumbnail = thumbnail_color_image(&image, 4);

        assert_eq!(thumbnail.size, [4, 2]);
        assert!(thumbnail.pixels.iter().all(|pixel| pixel.a() == 255));
    }

    #[test]
    fn startup_progress_reports_real_fraction_and_step_count() {
        let progress = StartupProgress {
            completed: 7,
            total: 28,
        };

        assert!((progress.fraction() - 0.25).abs() < f32::EPSILON);
        assert_eq!(progress.count(), "7 / 28 startup steps");
    }

    #[test]
    fn runtime_flags_parse_debug_cli_flag() {
        assert!(RuntimeFlags::from_args(["--debug".to_string()]).debug);
        assert!(RuntimeFlags::from_args(["--fast".to_string(), "--debug".to_string()]).debug);
        assert!(!RuntimeFlags::from_args(["debug".to_string()]).debug);
        assert!(!RuntimeFlags::from_args(Vec::<String>::new()).debug);
    }

    #[test]
    fn debug_console_starts_closed_with_empty_input() {
        let console = DebugConsole::default();

        assert!(!console.open);
        assert!(console.input.is_empty());
    }

    #[test]
    fn selected_save_index_clamps_after_delete() {
        assert_eq!(selected_save_index_after_delete(0, 0, 0), 0);
        assert_eq!(selected_save_index_after_delete(2, 2, 2), 1);
        assert_eq!(selected_save_index_after_delete(2, 1, 2), 1);
        assert_eq!(selected_save_index_after_delete(0, 1, 2), 0);
    }

    #[test]
    fn delete_save_file_removes_existing_file() {
        let path = test_save_path("delete-save-file");
        fs::write(&path, "temporary save").expect("test save file should be writable");

        assert!(delete_save_file(&path).is_ok());
        assert!(!path.exists());
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
    fn persisted_egui_windows_require_a_reachable_grab_area() {
        let screen = [1100.0, 760.0];
        assert!(egui_window_has_reachable_area(
            egui::Rect::from_min_size(egui::pos2(980.0, 40.0), egui::vec2(300.0, 400.0)),
            screen,
        ));
        assert!(!egui_window_has_reachable_area(
            egui::Rect::from_min_size(egui::pos2(1080.0, 40.0), egui::vec2(300.0, 400.0)),
            screen,
        ));
        assert!(!egui_window_has_reachable_area(
            egui::Rect::from_min_size(egui::pos2(40.0, 750.0), egui::vec2(300.0, 400.0)),
            screen,
        ));
    }

    #[test]
    fn egui_memory_round_trips_window_geometry() {
        let ctx = egui::Context::default();
        ctx.begin_pass(egui::RawInput::default());
        egui::Window::new("Ship")
            .id(egui::Id::new("inventory-ship-window"))
            .default_pos(egui::pos2(30.0, 40.0))
            .show(&ctx, |ui| {
                ui.label("test");
            });
        let _ = ctx.end_pass();
        let memory = ctx.memory(Clone::clone);
        let serialized = ron::to_string(&memory).expect("egui memory should serialize");
        let restored: egui::Memory =
            ron::from_str(&serialized).expect("egui memory should deserialize");

        assert!(restored
            .area_rect(egui::Id::new("inventory-ship-window"))
            .is_some());
    }

    #[test]
    fn pinned_egui_panes_remain_visible_after_parent_workspaces_close() {
        let mut game = test_game_with_systems(content::ContentRegistry::default(), Vec::new());
        game.inventory_open = false;
        game.fleet_open = false;
        game.selected_planet = Some(0);
        let pins = EguiPinSnapshot {
            production: true,
            planet: true,
            fleet: true,
            ..EguiPinSnapshot::default()
        };

        assert!(egui_inventory_windows_visible(&game, pins));
        assert!(egui_planet_visible(&game, pins));
        assert!(egui_fleet_visible(&game, pins));
        assert!(!egui_station_visible(&game, pins));
        assert!(!egui_npc_visible(&game, pins));
    }

    #[test]
    fn closing_inventory_preserves_only_pinned_inspection_targets() {
        let mut game = test_game_with_systems(content::ContentRegistry::default(), Vec::new());
        game.selected_planet = Some(0);
        game.selected_station = Some(1);
        game.selected_station_service = Some(2);
        game.selected_npc_ship = Some(3);

        clear_unpinned_inspection_selection(
            &mut game,
            EguiPinSnapshot {
                station: true,
                ..EguiPinSnapshot::default()
            },
        );

        assert_eq!(game.selected_planet, None);
        assert_eq!(game.selected_station, Some(1));
        assert_eq!(game.selected_station_service, Some(2));
        assert_eq!(game.selected_npc_ship, None);
    }

    #[test]
    fn egui_pin_file_round_trips_stable_pane_ids() {
        let saved = SavedEguiPins {
            pinned: [EguiPane::Production.id(), EguiPane::Fleet.id()]
                .into_iter()
                .map(str::to_string)
                .collect(),
        };

        let serialized = ron::to_string(&saved).expect("pins should serialize");
        let restored: SavedEguiPins = ron::from_str(&serialized).expect("pins should deserialize");

        assert_eq!(restored.pinned, saved.pinned);
    }

    #[test]
    fn egui_pane_pin_toggle_is_independent_per_window() {
        reset_egui_pins();

        toggle_egui_pane_pin(EguiPane::Production);
        assert!(egui_pane_pinned(EguiPane::Production));
        assert!(!egui_pane_pinned(EguiPane::Inventory));

        toggle_egui_pane_pin(EguiPane::Inventory);
        toggle_egui_pane_pin(EguiPane::Production);
        assert!(!egui_pane_pinned(EguiPane::Production));
        assert!(egui_pane_pinned(EguiPane::Inventory));
        reset_egui_pins();
    }

    #[test]
    fn content_browser_egui_actions_filter_the_existing_registry_rows() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let pack_index = game
            .content_registry
            .packs
            .iter()
            .position(|pack| pack.id == "turrets-galore")
            .expect("turrets-galore should be loaded");

        apply_content_ui_actions(
            &mut game,
            vec![egui_content::ContentUiAction::SelectPack(Some(pack_index))],
        );
        let view = content_browser_view(&game);

        assert_eq!(view.selected_pack, Some(pack_index));
        assert_eq!(
            view.selected_detail
                .as_ref()
                .map(|detail| detail.id.as_str()),
            Some("turrets-galore")
        );
        assert_eq!(view.filter_label, "Showing pack: turrets-galore");
        let item_group = view
            .object_groups
            .iter()
            .find(|group| group.title == "Items")
            .expect("filtered item group should be present");
        assert!(!item_group.rows.is_empty());
        assert!(item_group
            .rows
            .iter()
            .any(|row| row.name.contains("SUPER NUKE")));

        apply_content_ui_actions(
            &mut game,
            vec![egui_content::ContentUiAction::SelectPack(None)],
        );
        assert_eq!(game.content_browser.selected_pack_index, None);
        assert_eq!(
            content_browser_view(&game).filter_label,
            "Showing all packs"
        );
    }

    #[test]
    fn content_browser_object_references_are_inspectable_and_have_back_history() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let weapon_key = content_object_groups(&game.content_registry, Some("turrets-galore"))
            .into_iter()
            .find(|group| group.title == "Weapons")
            .and_then(|group| group.rows.into_iter().next())
            .map(|row| row.key)
            .expect("turrets-galore should provide a weapon");

        apply_content_ui_actions(
            &mut game,
            vec![egui_content::ContentUiAction::SelectObject(
                weapon_key.clone(),
            )],
        );
        let weapon_detail = content_browser_view(&game)
            .object_detail
            .expect("selected weapon should have details");
        assert!(weapon_detail
            .artwork
            .as_deref()
            .is_some_and(|path| path.ends_with(".png")));
        let install_item = weapon_detail
            .references
            .iter()
            .find(|reference| reference.label == "Install item")
            .expect("weapon should link its install item")
            .target
            .clone();

        apply_content_ui_actions(
            &mut game,
            vec![egui_content::ContentUiAction::SelectObject(
                install_item.clone(),
            )],
        );
        assert_eq!(game.content_browser.selected_object, Some(install_item));
        assert_eq!(
            game.content_browser.object_history,
            vec![weapon_key.clone()]
        );

        apply_content_ui_actions(&mut game, vec![egui_content::ContentUiAction::BackObject]);
        assert_eq!(game.content_browser.selected_object, Some(weapon_key));
        assert!(game.content_browser.object_history.is_empty());
    }

    #[test]
    fn pinned_content_browser_is_visible_after_c_closes_its_parent_state() {
        let mut game = test_game_with_systems(content::ContentRegistry::default(), Vec::new());
        game.content_open = false;

        assert!(egui_content_visible(
            &game,
            EguiPinSnapshot {
                content: true,
                ..EguiPinSnapshot::default()
            }
        ));
        assert!(!egui_content_visible(&game, EguiPinSnapshot::default()));
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
        game.completed_research = vec!["core:advanced_scanner_core".to_string()];
        game.active_research = Some(ActiveResearch {
            research: "core:fusion_drive_core".to_string(),
            remaining_seconds: 12.5,
        });
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

        assert!(!serialized.contains("purchased_recipe_unlocks"));
        assert_eq!(restored.content_pack_options, game.content_pack_options);
        assert_eq!(restored.ship_id, game.ship_id);
        assert_eq!(restored.completed_research, game.completed_research);
        assert_eq!(restored.active_research.len(), 1);
        assert_eq!(
            restored.active_research[0].research,
            "core:fusion_drive_core"
        );
        assert_eq!(restored.active_research[0].remaining_seconds, 12.5);
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
            restored.weapon_slot_loadout,
            vec![Some("core:point_defense_turret".to_string())]
        );
    }

    #[test]
    fn legacy_purchased_recipe_unlocks_migrate_to_completed_research() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let game = test_game_with_systems(registry, Vec::new());
        let serialized = toml::to_string(&game.to_save()).expect("save should serialize");
        let legacy_serialized = serialized.replace(
            "completed_research = []",
            "purchased_recipe_unlocks = [\"core:advanced_scanner_core\"]",
        );
        let save =
            toml::from_str::<SaveData>(&legacy_serialized).expect("legacy save should deserialize");

        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut restored = test_game_with_systems(registry, Vec::new());
        restored.stations = make_test_recipe_unlock_station();
        restored.recipe_vendor_locked_recipes =
            research_locked_recipes(&restored.content_registry, &restored.stations);
        restored.apply_save(save);

        assert_eq!(
            restored.completed_research,
            vec!["core:advanced_scanner_core".to_string()]
        );
        assert!(recipe_is_unlocked(&restored, "core:advanced_scanner_core"));
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
        assert_eq!(ship.attributes.energy_recharge, 8.0);
        assert_eq!(ship_energy_recharge(&ship, &power_modules), 22.0);

        let weapons = equipped_weapons_from_ids(&registry, &ship_def.weapon_slots);
        assert_eq!(weapons.len(), 1);
        assert_eq!(weapons[0].name, "Point Defense Turret");
        assert_eq!(weapons[0].install_item, "core:point_defense_turret");
        assert_eq!(weapons[0].range, 460.0);
        assert_eq!(weapons[0].energy_cost, 7.0);

        let starter_inventory = Inventory::starter(&registry);
        let reactor_pellet = required_item(&registry, "core:reactor_pellet");
        let ember_lance = required_item(&registry, "turrets-galore:ember_lance_turret");
        let sentinel_flak = required_item(&registry, "turrets-galore:sentinel_flak_turret");
        let storm_chain = required_item(&registry, "turrets-galore:storm_chain_turret");
        let super_nuke = required_item(&registry, "turrets-galore:super_nuke_turret");
        assert_eq!(starter_inventory.count(&reactor_pellet), 3);
        assert_eq!(starter_inventory.count(&ember_lance), 1);
        assert_eq!(starter_inventory.count(&sentinel_flak), 1);
        assert_eq!(starter_inventory.count(&storm_chain), 1);
        assert_eq!(starter_inventory.count(&super_nuke), 1);
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
                ammo_item: None,
                ammo_per_shot: 1,
                tracking_degrees: 360.0,
                targeting: content::WeaponTargeting::AllHostiles,
                effect: content::WeaponEffect::Arc,
                beam_color: [61, 178, 255, 255],
                core_color: [184, 245, 255, 255],
                impact_color: [143, 235, 255, 255],
                fire_duration_seconds: WEAPON_FIRE_EVENT_SECONDS,
                path_curve_strength: 0.18,
                path_wobble: 8.0,
                path_cycles: 3.0,
                trail_length: 0.4,
                burst_count: 3,
                travel_speed: None,
                projectile_texture: None,
                projectile_size: 28.0,
                impact: content::WeaponImpact::Single,
                splash_radius: 0.0,
                splash_falloff: content::DamageFalloff::Linear,
                splash_min_multiplier: 0.2,
                chain_targets: 3,
                chain_range: 240.0,
                chain_damage_multiplier: 0.75,
                friendly_fire: content::FriendlyFire::HostilesOnly,
                fire_audio: None,
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
        assert_eq!(
            game.to_save().weapon_slot_loadout,
            vec![Some("core:test_turret".to_string())]
        );
    }

    #[test]
    fn loading_a_save_does_not_reapply_pack_starter_inventory() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let ember_lance = required_item(&registry, "turrets-galore:ember_lance_turret");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.inventory = Inventory::starter(&game.content_registry);
        assert_eq!(game.inventory.count(&ember_lance), 1);

        let mut save = game.to_save();
        save.inventory.clear();
        save.owned_ships[0].inventory.clear();
        game.apply_save(save);

        assert_eq!(game.inventory.count(&ember_lance), 0);
    }

    #[test]
    fn configured_weapon_slots_can_install_multiple_crafted_turrets() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let twinspire_id = "turrets-galore:twinspire_gunship";
        let twinspire = registry
            .ships
            .get(twinspire_id)
            .expect("Turrets Galore two-bank ship should exist")
            .clone();
        let mut game = test_game_with_systems(registry, Vec::new());
        game.ship_id = twinspire_id.to_string();
        game.ship = Ship::from_content(&twinspire);
        game.equipped_weapons =
            equipped_weapons_from_ids(&game.content_registry, &twinspire.weapon_slots);

        assert_eq!(weapon_slot_capacity(&game), 2);
        let storm_item = required_item(&game.content_registry, "turrets-galore:storm_chain_turret");
        let nuke_item = required_item(&game.content_registry, "turrets-galore:super_nuke_turret");
        game.inventory.add_item(storm_item, 1);
        game.inventory.add_item(nuke_item, 1);

        install_weapon_in_slot(&mut game, 0, "turrets-galore:storm_chain_turret")
            .expect("first bank should accept a configured turret");
        install_weapon_in_slot(&mut game, 1, "turrets-galore:super_nuke_turret")
            .expect("second bank should accept an independent configured turret");

        assert_eq!(game.equipped_weapons.len(), 2);
        assert_eq!(
            game.to_save().weapon_slot_loadout,
            vec![
                Some("turrets-galore:storm_chain_turret".to_string()),
                Some("turrets-galore:super_nuke_turret".to_string())
            ]
        );
    }

    #[test]
    fn saved_ship_identity_restores_two_bank_capacity() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.ship_id = "turrets-galore:twinspire_gunship".to_string();
        let save = game.to_save();

        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut restored = test_game_with_systems(registry, Vec::new());
        restored.apply_save(save);

        assert_eq!(restored.ship_id, "turrets-galore:twinspire_gunship");
        assert_eq!(weapon_slot_capacity(&restored), 2);
        assert_eq!(restored.equipped_weapons.len(), 2);
    }

    #[test]
    fn garage_purchase_creates_an_independent_owned_ship_without_starter_cargo() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.ship.position = vec2(120.0, 0.0);
        game.credits = 20_000;
        let price = game.content_registry.ships["turrets-galore:twinspire_gunship"]
            .purchase_price
            .expect("Twinspire should be purchasable");

        assert!(acquire_owned_ship(
            &mut game,
            0,
            "turrets-galore:twinspire_gunship"
        ));

        assert_eq!(game.owned_ships.len(), 2);
        assert_eq!(game.credits, 20_000 - price);
        assert_eq!(
            game.owned_ships[1].ship_id,
            "turrets-galore:twinspire_gunship"
        );
        assert_eq!(game.owned_ships[1].inventory.total_mass(), 0.0);
        assert_eq!(game.owned_ships[1].equipped_weapons.len(), 2);
    }

    #[test]
    fn switching_owned_ships_preserves_cargo_upgrades_damage_and_position() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let iron = required_item(&registry, "core:iron_ore");
        let copper = required_item(&registry, "core:copper_ore");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.ship.position = vec2(120.0, 0.0);
        game.credits = 20_000;
        acquire_owned_ship(&mut game, 0, "turrets-galore:twinspire_gunship");
        game.inventory.add_item(iron.clone(), 3);
        game.ship_upgrades[0].level = 2;
        game.ship.systems.hull.current = 71.0;
        game.ship.position = vec2(222.0, 44.0);

        assert!(switch_active_owned_ship(&mut game, 1));
        assert_eq!(game.inventory.count(&iron), 0);
        game.inventory.add_item(copper.clone(), 5);
        game.ship_upgrades[1].level = 1;

        assert!(switch_active_owned_ship(&mut game, 0));
        assert_eq!(game.inventory.count(&iron), 3);
        assert_eq!(game.inventory.count(&copper), 0);
        assert_eq!(game.ship_upgrades[0].level, 2);
        assert_eq!(game.ship.systems.hull.current, 71.0);
        assert_eq!(game.ship.position, vec2(222.0, 44.0));

        assert!(switch_active_owned_ship(&mut game, 1));
        assert_eq!(game.inventory.count(&copper), 5);
        assert_eq!(game.ship_upgrades[1].level, 1);
    }

    #[test]
    fn owned_ship_cargo_transfer_requires_range_and_preserves_stack_totals() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let iron = required_item(&registry, "core:iron_ore");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.ship.position = vec2(120.0, 0.0);
        game.credits = 20_000;
        acquire_owned_ship(&mut game, 0, "turrets-galore:twinspire_gunship");
        game.inventory.add_item(iron.clone(), 10);
        game.owned_ships[1].ship.position = vec2(1_000.0, 0.0);

        assert_eq!(transfer_owned_ship_item(&mut game, 1, &iron.id, true, 4), 0);
        game.owned_ships[1].ship.position = vec2(180.0, 0.0);
        assert_eq!(transfer_owned_ship_item(&mut game, 1, &iron.id, true, 4), 4);
        assert_eq!(game.inventory.count(&iron), 6);
        assert_eq!(game.owned_ships[1].inventory.count(&iron), 4);
        assert_eq!(
            transfer_owned_ship_item(&mut game, 1, &iron.id, false, 2),
            2
        );
        assert_eq!(game.inventory.count(&iron), 8);
        assert_eq!(game.owned_ships[1].inventory.count(&iron), 2);
    }

    #[test]
    fn fleet_egui_view_and_actions_cover_transfer_switching_and_shipyard_purchase() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let iron = required_item(&registry, "core:iron_ore");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.ship.position = vec2(120.0, 0.0);
        game.credits = 100_000;
        assert!(acquire_owned_ship(
            &mut game,
            0,
            "turrets-galore:twinspire_gunship"
        ));
        game.inventory.add_item(iron.clone(), 10);

        let view = fleet_pane_view(&game, 10);
        assert_eq!(view.ships.len(), 2);
        assert_eq!(view.selected, Some(1));
        assert!(view
            .selected_ship
            .as_ref()
            .is_some_and(|selected| selected.transfer_available));
        assert!(view
            .active_cargo
            .iter()
            .any(|cargo| cargo.item_id == iron.id && cargo.count == 10));
        let offer = view
            .shipyard
            .as_ref()
            .and_then(|shipyard| shipyard.offers.first())
            .expect("nearby garage should expose purchasable hulls")
            .ship_id
            .clone();

        apply_fleet_ui_actions(
            &mut game,
            vec![
                egui_fleet::FleetUiAction::AdjustTransferQuantity(1),
                egui_fleet::FleetUiAction::TransferToSelected(iron.id.clone()),
            ],
            10,
        );
        assert_eq!(game.fleet_transfer_quantity, 11);
        assert_eq!(game.inventory.count(&iron), 0);
        assert_eq!(game.owned_ships[1].inventory.count(&iron), 10);

        apply_fleet_ui_actions(&mut game, vec![egui_fleet::FleetUiAction::SwitchControl], 1);
        assert_eq!(game.active_owned_ship_id, "ship-2");

        let owned_before_purchase = game.owned_ships.len();
        apply_fleet_ui_actions(
            &mut game,
            vec![egui_fleet::FleetUiAction::BuyShip(offer)],
            1,
        );
        assert_eq!(game.owned_ships.len(), owned_before_purchase + 1);
        assert!(game.save_dirty);
    }

    #[test]
    fn inactive_owned_ship_recharges_energy_and_shields() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.ship.position = vec2(120.0, 0.0);
        game.credits = 20_000;
        acquire_owned_ship(&mut game, 0, "turrets-galore:twinspire_gunship");
        game.owned_ships[1].ship.systems.energy.current = 0.0;
        game.owned_ships[1].ship.systems.shields.current = 20.0;
        game.owned_ships[1].shield_recharge_delay_remaining = 0.0;

        update_inactive_owned_ship_systems(&mut game, 1.0);

        assert!(game.owned_ships[1].ship.systems.energy.current > 0.0);
        assert!(game.owned_ships[1].ship.systems.shields.current > 20.0);
    }

    #[test]
    fn inactive_owned_ship_fires_using_its_own_energy_and_ammunition() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.ship.position = vec2(120.0, 0.0);
        game.credits = 20_000;
        acquire_owned_ship(&mut game, 0, "turrets-galore:twinspire_gunship");
        let fleet_position = game.owned_ships[1].ship.position;
        game.owned_ships[1].ship.systems.energy.current = 100.0;
        game.npc_ships = vec![test_npc_ship(
            NpcBehaviorMode::HostileIntercept,
            fleet_position + vec2(100.0, 0.0),
        )];
        let starting_energy = game.owned_ships[1].ship.systems.energy.current;

        update_inactive_owned_ship_weapon_systems(&mut game, 0.1);

        assert_eq!(
            game.owned_ships[1].equipped_weapons[0].status,
            WeaponStatus::Fired
        );
        assert_eq!(
            game.owned_ships[1].equipped_weapons[1].status,
            WeaponStatus::OutOfAmmo
        );
        assert!(game.owned_ships[1].ship.systems.energy.current < starting_energy);
        assert_eq!(game.weapon_fire_events.len(), 1);
        finish_weapon_fire_events(&mut game);
        assert!(game.npc_ships[0].shields.current < game.npc_ships[0].shields.max);
    }

    #[test]
    fn hostile_npc_can_target_and_damage_an_inactive_owned_ship() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.ship.position = vec2(2_000.0, 0.0);
        game.credits = 20_000;
        game.stations[0].position = game.ship.position;
        acquire_owned_ship(&mut game, 0, "turrets-galore:twinspire_gunship");
        game.owned_ships[1].ship.position = Vec2::ZERO;
        game.owned_ships[1].ship.systems.shields.current =
            game.owned_ships[1].ship.systems.shields.max;
        let starting_shields = game.owned_ships[1].ship.systems.shields.current;
        let mut hostile = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(100.0, 0.0));
        hostile.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        stock_test_npc_ammo(
            &mut hostile,
            &game.content_registry,
            "core:interceptor_round",
            1,
        );
        hostile.energy.current = hostile.energy.max;
        game.npc_ships = vec![hostile];

        update_npc_weapon_systems(&mut game, 0.1);
        assert_eq!(game.weapon_fire_events.len(), 1);
        finish_weapon_fire_events(&mut game);

        assert!(game.owned_ships[1].ship.systems.shields.current < starting_shields);
        assert!(game.owned_ships[1].shield_recharge_delay_remaining > 0.0);
        assert_eq!(
            game.ship.systems.shields.current,
            game.ship.systems.shields.max
        );
    }

    #[test]
    fn fleet_save_round_trip_restores_all_owned_ships_and_active_identity() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.ship.position = vec2(120.0, 0.0);
        game.credits = 20_000;
        acquire_owned_ship(&mut game, 0, "turrets-galore:twinspire_gunship");
        switch_active_owned_ship(&mut game, 1);
        game.ship.systems.hull.current = 64.0;
        let save = game.to_save();

        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut restored = test_game_with_systems(registry, Vec::new());
        restored.apply_save(save);

        assert_eq!(restored.owned_ships.len(), 2);
        assert_eq!(restored.active_owned_ship_id, "ship-2");
        assert_eq!(restored.ship_id, "turrets-galore:twinspire_gunship");
        assert_eq!(restored.ship.systems.hull.current, 64.0);
    }

    #[test]
    fn version_one_save_migrates_into_a_single_owned_ship() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.ship.position = vec2(345.0, -90.0);
        let mut save = game.to_save();
        save.version = 1;
        save.owned_ships.clear();
        save.active_owned_ship_id.clear();

        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut restored = test_game_with_systems(registry, Vec::new());
        restored.apply_save(save);

        assert_eq!(restored.owned_ships.len(), 1);
        assert_eq!(restored.active_owned_ship_id, "ship-1");
        assert_eq!(restored.ship.position, vec2(345.0, -90.0));
    }

    #[test]
    fn unavailable_pack_hull_record_survives_with_a_resolvable_fallback_active() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let twinspire = registry.ships["turrets-galore:twinspire_gunship"].clone();
        let mut game = test_game_with_systems(registry, Vec::new());
        game.ship_id = twinspire.id.clone();
        game.ship = Ship::from_content(&twinspire);
        game.equipped_weapons =
            equipped_weapons_from_ids(&game.content_registry, &twinspire.weapon_slots);
        let save = game.to_save();

        let mut registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        registry.ships.remove("turrets-galore:twinspire_gunship");
        let mut restored = test_game_with_systems(registry, Vec::new());
        restored.apply_save(save);

        assert_eq!(restored.ship_id, STARTER_SHIP_ID);
        assert_eq!(restored.owned_ships.len(), 2);
        assert!(restored
            .owned_ships
            .iter()
            .any(|owned| owned.ship_id == "turrets-galore:twinspire_gunship"));
        let unavailable_index = restored
            .owned_ships
            .iter()
            .position(|owned| owned.ship_id == "turrets-galore:twinspire_gunship")
            .unwrap();
        assert!(!switch_active_owned_ship(&mut restored, unavailable_index));
    }

    #[test]
    fn legacy_save_without_ship_identity_keeps_the_core_hull() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let game = test_game_with_systems(registry, Vec::new());
        let serialized = toml::to_string(&game.to_save()).expect("save should serialize");
        let legacy_serialized = serialized
            .lines()
            .filter(|line| !line.starts_with("ship_id = "))
            .collect::<Vec<_>>()
            .join("\n");
        let save =
            toml::from_str::<SaveData>(&legacy_serialized).expect("legacy save should deserialize");

        assert_eq!(save.ship_id, STARTER_SHIP_ID);
    }

    #[test]
    fn weapon_slot_save_preserves_empty_and_missing_pack_entries() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut save = game.to_save();
        save.weapon_slots.clear();
        save.weapon_slot_loadout = vec![
            None,
            Some("turrets-galore:ember_lance_turret".to_string()),
            Some("missing-pack:lost_turret".to_string()),
        ];
        save.owned_ships[0].weapon_slot_loadout = save.weapon_slot_loadout.clone();

        game.apply_save(save);

        assert_eq!(game.equipped_weapons.len(), 3);
        assert!(!game.equipped_weapons[0].resolved);
        assert!(game.equipped_weapons[0].id.is_empty());
        assert!(game.equipped_weapons[1].resolved);
        assert_eq!(
            game.equipped_weapons[1].id,
            "turrets-galore:ember_lance_turret"
        );
        assert!(!game.equipped_weapons[2].resolved);
        assert_eq!(game.equipped_weapons[2].id, "missing-pack:lost_turret");
        assert_eq!(
            game.to_save().weapon_slot_loadout,
            vec![
                None,
                Some("turrets-galore:ember_lance_turret".to_string()),
                Some("missing-pack:lost_turret".to_string()),
            ]
        );
    }

    #[test]
    fn pack_targeting_policy_limits_candidate_classes() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let game = test_game_with_systems(registry, Vec::new());
        let ship_only = WeaponSystem::from_def(
            game.content_registry
                .weapons
                .get("turrets-galore:ember_lance_turret")
                .expect("ship-only proof turret should load"),
        );
        let all_hostiles = WeaponSystem::from_def(
            game.content_registry
                .weapons
                .get("turrets-galore:sentinel_flak_turret")
                .expect("all-hostiles proof turret should load"),
        );

        let threats = vec![test_defense_threat(
            "test:hostile",
            ThreatDisposition::Hostile,
            game.ship.position + vec2(20.0, 0.0),
            10.0,
        )];
        assert!(defense_turret_target_index(
            &game.ship,
            &ship_only,
            &threats,
            &game.current_system_id,
        )
        .is_none());
        assert!(defense_turret_target_index(
            &game.ship,
            &all_hostiles,
            &threats,
            &game.current_system_id,
        )
        .is_some());
        let mut npc = test_npc_ship(
            NpcBehaviorMode::HostileIntercept,
            game.ship.position + vec2(20.0, 0.0),
        );
        npc.angle = std::f32::consts::PI;
        assert!(npc_weapon_can_target_ship(
            &npc,
            &ship_only,
            &game.ship,
            &game.current_system_id,
        ));
        assert!(npc_weapon_can_target_ship(
            &npc,
            &all_hostiles,
            &game.ship,
            &game.current_system_id,
        ));
    }

    #[test]
    fn turrets_galore_weapons_fire_at_the_starter_hostile_npc_range() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut probe = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(420.0, 0.0));
        probe.id = "core:redwake_probe".to_string();
        probe.role = "hostile".to_string();
        probe.faction = Some("core:redwake_raiders".to_string());
        probe.behavior_tags = vec!["hostile".to_string()];
        game.npc_ships = vec![probe];
        game.ship.angle = std::f32::consts::PI;

        for weapon_id in [
            "turrets-galore:ember_lance_turret",
            "turrets-galore:sentinel_flak_turret",
            "turrets-galore:storm_chain_turret",
            "turrets-galore:super_nuke_turret",
        ] {
            game.equipped_weapons =
                equipped_weapons_from_ids(&game.content_registry, &[weapon_id.to_string()]);
            game.ship.systems.energy.current = game.ship.systems.energy.max;
            game.npc_ships[0].shields = ShipResource::full(20.0);
            game.npc_ships[0].hull = ShipResource::full(36.0);
            game.weapon_fire_events.clear();

            update_player_weapon_systems(&mut game, 0.1);

            assert_eq!(
                game.equipped_weapons[0].status,
                WeaponStatus::Fired,
                "{weapon_id} should fire at the starter hostile NPC"
            );
            assert_eq!(game.weapon_fire_events.len(), 1);
            finish_weapon_fire_events(&mut game);
            assert!(game.npc_ships[0].shields.current < 20.0);
        }
    }

    #[test]
    fn twinspire_fires_both_turret_banks_in_the_same_update() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let twinspire = registry
            .ships
            .get("turrets-galore:twinspire_gunship")
            .expect("two-bank ship should load")
            .clone();
        let mut game = test_game_with_systems(registry, Vec::new());
        game.ship_id = twinspire.id.clone();
        game.ship = Ship::from_content(&twinspire);
        game.equipped_weapons =
            equipped_weapons_from_ids(&game.content_registry, &twinspire.weapon_slots);
        game.npc_ships = vec![test_npc_ship(
            NpcBehaviorMode::HostileIntercept,
            vec2(420.0, 0.0),
        )];

        update_player_weapon_systems(&mut game, 0.1);

        assert_eq!(game.equipped_weapons.len(), 2);
        assert!(game
            .equipped_weapons
            .iter()
            .all(|weapon| weapon.status == WeaponStatus::Fired));
        assert_eq!(game.weapon_fire_events.len(), 2);
    }

    #[test]
    fn chain_impact_jumps_between_hostile_ships_and_decays_damage() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["turrets-galore:storm_chain_turret".to_string()],
        );
        game.ship.position = Vec2::ZERO;
        game.ship.systems.energy.current = 100.0;
        game.npc_ships = [120.0, 300.0, 480.0]
            .into_iter()
            .enumerate()
            .map(|(index, x)| {
                let mut npc = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(x, 0.0));
                npc.id = format!("core:chain-target-{index}");
                npc.shields = ShipResource::full(100.0);
                npc
            })
            .collect();

        update_weapon_systems(&mut game, 0.1);
        assert!(game
            .npc_ships
            .iter()
            .all(|npc| npc.shields.current == 100.0));
        finish_weapon_fire_events(&mut game);

        assert!((game.npc_ships[0].shields.current - 78.0).abs() < 0.01);
        assert!((game.npc_ships[1].shields.current - 84.16).abs() < 0.01);
        assert!((game.npc_ships[2].shields.current - 88.5952).abs() < 0.01);
        assert_eq!(game.weapon_fire_events.len(), 2);
    }

    #[test]
    fn splash_impact_uses_falloff_and_default_friendly_fire_safety() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["turrets-galore:super_nuke_turret".to_string()],
        );
        game.ship.position = Vec2::ZERO;
        game.ship.systems.energy.current = 100.0;
        let mut primary = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(200.0, 0.0));
        primary.id = "core:nuke-primary".to_string();
        primary.shields = ShipResource::full(200.0);
        let mut nearby = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(500.0, 0.0));
        nearby.id = "core:nuke-nearby".to_string();
        nearby.shields = ShipResource::full(200.0);
        let mut friendly = test_npc_ship(NpcBehaviorMode::Patrol, vec2(250.0, 0.0));
        friendly.id = "core:nuke-friendly".to_string();
        friendly.shields = ShipResource::full(200.0);
        game.npc_ships = vec![primary, nearby, friendly];

        update_weapon_systems(&mut game, 0.1);
        finish_weapon_fire_events(&mut game);

        assert!((game.npc_ships[0].shields.current - 60.0).abs() < 0.01);
        assert!(game.npc_ships[1].shields.current > 60.0);
        assert!(game.npc_ships[1].shields.current < 200.0);
        assert_eq!(game.npc_ships[2].shields.current, 200.0);
        assert_eq!(game.weapon_area_effect_events.len(), 1);
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
        assert_eq!(game.defense_threats[1].hull.current, 36.0);
        assert_eq!(game.weapon_fire_events.len(), 1);
        finish_weapon_fire_events(&mut game);
        assert_eq!(game.defense_threats[1].hull.current, 18.0);
        assert!(game.save_dirty);
    }

    #[test]
    fn ammunition_turrets_consume_inventory_only_when_they_fire() {
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
            36.0,
        )];
        let ammo = required_item(&game.content_registry, "core:interceptor_round");
        let before = game.inventory.count(&ammo);

        update_weapon_systems(&mut game, 0.1);

        assert_eq!(game.equipped_weapons[0].status, WeaponStatus::Fired);
        assert_eq!(game.inventory.count(&ammo), before - 1);
        assert_eq!(game.weapon_fire_events.len(), 1);
    }

    #[test]
    fn ammunition_turrets_report_out_of_ammo_without_spending_energy() {
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
            36.0,
        )];
        let ammo = required_item(&game.content_registry, "core:interceptor_round");
        let ammo_count = game.inventory.count(&ammo);
        game.inventory.remove_item(&ammo, ammo_count);

        update_weapon_systems(&mut game, 0.1);

        assert_eq!(game.equipped_weapons[0].status, WeaponStatus::OutOfAmmo);
        assert_eq!(game.ship.systems.energy.current, 100.0);
        assert!(game.weapon_fire_events.is_empty());
    }

    #[test]
    fn energy_turrets_fire_without_ammunition() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["turrets-galore:ember_lance_turret".to_string()],
        );
        game.ship.systems.energy.current = 100.0;
        game.npc_ships = vec![test_npc_ship(
            NpcBehaviorMode::HostileIntercept,
            vec2(120.0, 0.0),
        )];
        game.inventory = Inventory {
            slots: std::array::from_fn(|_| None),
        };

        update_weapon_systems(&mut game, 0.1);

        assert_eq!(game.equipped_weapons[0].status, WeaponStatus::Fired);
        assert_eq!(game.ship.systems.energy.current, 85.0);
        assert_eq!(game.weapon_fire_events.len(), 1);
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
    fn defensive_turrets_fire_at_hostile_npc_ships() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        game.ship.systems.energy.current = 100.0;
        let mut hostile = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(120.0, 0.0));
        hostile.role = "hostile".to_string();
        hostile.behavior_tags = vec!["hostile".to_string()];
        hostile.shields = ShipResource::full(25.0);
        hostile.hull = ShipResource::full(50.0);
        game.npc_ships = vec![hostile];

        update_weapon_systems(&mut game, 0.1);

        assert_eq!(game.equipped_weapons[0].status, WeaponStatus::Fired);
        assert_eq!(game.ship.systems.energy.current, 93.0);
        assert_eq!(game.npc_ships[0].shields.current, 25.0);
        assert_eq!(game.weapon_fire_events.len(), 1);
        finish_weapon_fire_events(&mut game);
        assert_eq!(game.npc_ships[0].shields.current, 7.0);
        assert_eq!(game.npc_ships[0].hull.current, 50.0);
        assert!(game.save_dirty);
    }

    #[test]
    fn destroyed_npc_ships_are_removed_after_turret_fire() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        game.ship.systems.energy.current = 100.0;
        let mut hostile = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(120.0, 0.0));
        hostile.role = "hostile".to_string();
        hostile.behavior_tags = vec!["hostile".to_string()];
        hostile.shields = ShipResource::full(0.1);
        hostile.hull = ShipResource::full(10.0);
        game.npc_ships = vec![hostile];
        game.selected_npc_ship = Some(0);

        update_weapon_systems(&mut game, 0.1);
        assert_eq!(game.equipped_weapons[0].status, WeaponStatus::Fired);
        finish_weapon_fire_events(&mut game);
        remove_destroyed_npc_ships(&mut game);

        assert!(game.npc_ships.is_empty());
        assert_eq!(game.selected_npc_ship, None);
    }

    #[test]
    fn destroyed_npc_cargo_is_added_to_player_inventory_when_space_allows() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let circuit = required_item(&registry, "core:circuit");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut destroyed = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(120.0, 0.0));
        destroyed.credit_reward_min = 45;
        destroyed.credit_reward_max = 45;
        destroyed.hull.current = 0.0;
        destroyed.cargo_defaults = vec![ItemStack {
            item: circuit.clone(),
            count: 2,
        }];
        game.npc_ships = vec![destroyed];

        remove_destroyed_npc_ships(&mut game);

        assert!(game.npc_ships.is_empty());
        assert_eq!(game.inventory.count(&circuit), 2);
        assert_eq!(game.credits, default_credits() + 45);
        assert!(operation_feedback_contains(
            &game,
            "Loot",
            "Test NPC: 2 cargo, 45 cr"
        ));
    }

    #[test]
    fn destroyed_npc_cargo_is_skipped_when_cargo_capacity_is_full() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let iron_ore = required_item(&registry, "core:iron_ore");
        let circuit = required_item(&registry, "core:circuit");
        let mut game = test_game_with_systems(registry, Vec::new());
        let existing_count =
            (cargo_rating_kg(&game.ship_upgrades) / iron_ore.unit_mass).ceil() as u32 + 1;
        game.inventory.add_item(iron_ore, existing_count);
        let mut destroyed = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(120.0, 0.0));
        destroyed.credit_reward_min = 30;
        destroyed.credit_reward_max = 30;
        destroyed.hull.current = 0.0;
        destroyed.cargo_defaults = vec![ItemStack {
            item: circuit.clone(),
            count: 1,
        }];
        game.npc_ships = vec![destroyed];

        remove_destroyed_npc_ships(&mut game);

        assert!(game.npc_ships.is_empty());
        assert_eq!(game.inventory.count(&circuit), 0);
        assert_eq!(game.credits, default_credits() + 30);
        assert!(operation_feedback_contains(
            &game,
            "Loot",
            "Test NPC: 30 cr"
        ));
    }

    #[test]
    fn destroyed_non_hostile_npc_does_not_award_credits() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut destroyed = test_npc_ship(NpcBehaviorMode::Patrol, vec2(120.0, 0.0));
        destroyed.credit_reward_min = 99;
        destroyed.credit_reward_max = 99;
        destroyed.hull.current = 0.0;
        game.npc_ships = vec![destroyed];

        remove_destroyed_npc_ships(&mut game);

        assert!(game.npc_ships.is_empty());
        assert_eq!(game.credits, default_credits());
    }

    #[test]
    fn removing_destroyed_npc_ships_remaps_surviving_selection() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut destroyed = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(120.0, 0.0));
        destroyed.id = "core:destroyed".to_string();
        destroyed.hull.current = 0.0;
        let mut selected = test_npc_ship(NpcBehaviorMode::Patrol, vec2(220.0, 0.0));
        selected.id = "core:selected".to_string();
        game.npc_ships = vec![destroyed, selected];
        game.selected_npc_ship = Some(1);

        remove_destroyed_npc_ships(&mut game);

        assert_eq!(game.npc_ships.len(), 1);
        assert_eq!(game.npc_ships[0].id, "core:selected");
        assert_eq!(game.selected_npc_ship, Some(0));
    }

    #[test]
    fn defensive_turrets_ignore_non_hostile_npc_ships() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        game.ship.systems.energy.current = 100.0;
        let mut patrol = test_npc_ship(NpcBehaviorMode::Patrol, vec2(120.0, 0.0));
        patrol.role = "patrol".to_string();
        patrol.behavior_tags = vec!["patrol".to_string(), "non-hostile".to_string()];
        game.npc_ships = vec![patrol];

        update_weapon_systems(&mut game, 0.1);

        assert_eq!(game.equipped_weapons[0].status, WeaponStatus::NoThreat);
        assert_eq!(game.ship.systems.energy.current, 100.0);
        assert_eq!(game.npc_ships[0].shields.current, 25.0);
        assert_eq!(game.npc_ships[0].hull.current, 50.0);
        assert!(game.weapon_fire_events.is_empty());
    }

    #[test]
    fn friendly_npc_turrets_fire_at_hostile_threats() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut patrol = test_npc_ship(NpcBehaviorMode::Patrol, Vec2::ZERO);
        patrol.role = "patrol".to_string();
        patrol.weapon_slots = vec!["core:point_defense_turret".to_string()];
        patrol.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        stock_test_npc_ammo(
            &mut patrol,
            &game.content_registry,
            "core:interceptor_round",
            2,
        );
        patrol.energy.current = 40.0;
        game.npc_ships = vec![patrol];
        game.defense_threats = vec![test_defense_threat(
            "core:hostile",
            ThreatDisposition::Hostile,
            vec2(120.0, 0.0),
            36.0,
        )];

        update_weapon_systems(&mut game, 0.1);

        assert_eq!(
            game.npc_ships[0].equipped_weapons[0].status,
            WeaponStatus::Fired
        );
        assert_eq!(game.npc_ships[0].energy.current, 33.0);
        assert_eq!(
            npc_weapon_ammo_count(&game.npc_ships[0], &game.npc_ships[0].equipped_weapons[0]),
            Some(1)
        );
        assert_eq!(game.defense_threats[0].hull.current, 36.0);
        assert_eq!(game.weapon_fire_events.len(), 1);
        finish_weapon_fire_events(&mut game);
        assert_eq!(game.defense_threats[0].hull.current, 18.0);
    }

    #[test]
    fn hostile_npc_turrets_fire_at_player_ship() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut probe = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(120.0, 0.0));
        probe.role = "hostile".to_string();
        probe.behavior_tags = vec!["hostile".to_string()];
        probe.weapon_slots = vec!["core:point_defense_turret".to_string()];
        probe.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        stock_test_npc_ammo(
            &mut probe,
            &game.content_registry,
            "core:interceptor_round",
            2,
        );
        probe.energy.current = 40.0;
        game.npc_ships = vec![probe];
        game.ship.position = Vec2::ZERO;
        game.ship.systems.shields.current = 100.0;
        game.ship.systems.hull.current = 100.0;

        update_weapon_systems(&mut game, 0.1);

        assert_eq!(
            game.npc_ships[0].equipped_weapons[0].status,
            WeaponStatus::Fired
        );
        assert_eq!(game.npc_ships[0].energy.current, 33.0);
        assert_eq!(game.ship.systems.shields.current, 100.0);
        assert_eq!(game.weapon_fire_events.len(), 1);
        finish_weapon_fire_events(&mut game);
        assert_eq!(game.ship.systems.shields.current, 82.0);
        assert_eq!(game.ship.systems.hull.current, 100.0);
        assert!(game.save_dirty);
    }

    #[test]
    fn hostile_intercept_behavior_counts_as_hostile() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut npc_ship = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(120.0, 0.0));
        npc_ship.role = "probe".to_string();
        npc_ship.behavior_tags = Vec::new();

        assert!(npc_ship_is_hostile(&registry, &npc_ship));
    }

    #[test]
    fn hostile_intercept_turrets_fire_without_hostile_tag() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut probe = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(120.0, 0.0));
        probe.role = "probe".to_string();
        probe.behavior_tags = Vec::new();
        probe.weapon_slots = vec!["core:point_defense_turret".to_string()];
        probe.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        stock_test_npc_ammo(
            &mut probe,
            &game.content_registry,
            "core:interceptor_round",
            2,
        );
        probe.energy.current = 40.0;
        game.npc_ships = vec![probe];
        game.ship.position = Vec2::ZERO;
        game.ship.systems.shields.current = 100.0;

        update_weapon_systems(&mut game, 0.1);

        assert_eq!(
            game.npc_ships[0].equipped_weapons[0].status,
            WeaponStatus::Fired
        );
        finish_weapon_fire_events(&mut game);
        assert!(game.ship.systems.shields.current < 100.0);
    }

    #[test]
    fn incoming_weapon_fire_counts_events_targeting_player() {
        let mut ship = Ship::starter();
        ship.position = vec2(10.0, -4.0);
        let events = vec![
            WeaponFireEvent {
                from: vec2(120.0, 0.0),
                to: ship.position,
                timer: WEAPON_FIRE_EVENT_SECONDS,
                duration: WEAPON_FIRE_EVENT_SECONDS,
                effect: content::WeaponEffect::Arc,
                beam_color: [61, 178, 255, 255],
                core_color: [184, 245, 255, 255],
                impact_color: [143, 235, 255, 255],
                path_curve_strength: 0.18,
                path_wobble: 8.0,
                path_cycles: 3.0,
                trail_length: 0.4,
                burst_count: 3,
                projectile_texture: None,
                projectile_size: 28.0,
                origin: WeaponFireOrigin::Npc,
                pending_impact: None,
            },
            WeaponFireEvent {
                from: Vec2::ZERO,
                to: vec2(400.0, 0.0),
                timer: WEAPON_FIRE_EVENT_SECONDS,
                duration: WEAPON_FIRE_EVENT_SECONDS,
                effect: content::WeaponEffect::Arc,
                beam_color: [61, 178, 255, 255],
                core_color: [184, 245, 255, 255],
                impact_color: [143, 235, 255, 255],
                path_curve_strength: 0.18,
                path_wobble: 8.0,
                path_cycles: 3.0,
                trail_length: 0.4,
                burst_count: 3,
                projectile_texture: None,
                projectile_size: 28.0,
                origin: WeaponFireOrigin::Player,
                pending_impact: None,
            },
        ];

        assert_eq!(incoming_weapon_fire_count(&ship, &events), 1);
    }

    #[test]
    fn curated_weapon_paths_preserve_shot_endpoints() {
        let from = vec2(0.0, 0.0);
        let to = vec2(200.0, 0.0);
        let mut event = WeaponFireEvent {
            from,
            to,
            timer: 0.5,
            duration: 1.0,
            effect: content::WeaponEffect::Straight,
            beam_color: [255; 4],
            core_color: [255; 4],
            impact_color: [255; 4],
            path_curve_strength: 0.18,
            path_wobble: 18.0,
            path_cycles: 3.0,
            trail_length: 0.4,
            burst_count: 5,
            projectile_texture: None,
            projectile_size: 28.0,
            origin: WeaponFireOrigin::Player,
            pending_impact: None,
        };

        for effect in [
            content::WeaponEffect::Straight,
            content::WeaponEffect::Arc,
            content::WeaponEffect::Spiral,
            content::WeaponEffect::Zigzag,
            content::WeaponEffect::Homing,
            content::WeaponEffect::Burst,
        ] {
            event.effect = effect;
            assert_vec2_near(
                weapon_effect_path_point(from, to, &event, 0.0, 0.0, 1.0, 1.0),
                from,
            );
            assert_vec2_near(
                weapon_effect_path_point(from, to, &event, 1.0, 0.0, 1.0, 1.0),
                to,
            );
        }

        event.effect = content::WeaponEffect::Straight;
        let straight_mid = weapon_effect_path_point(from, to, &event, 0.5, 0.0, 1.0, 0.0);
        event.effect = content::WeaponEffect::Arc;
        let arc_mid = weapon_effect_path_point(from, to, &event, 0.5, 0.0, 1.0, 0.0);
        assert_eq!(straight_mid.y, 0.0);
        assert!(arc_mid.y.abs() > 1.0);
    }

    #[test]
    fn pack_travel_speed_controls_visual_shot_duration() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let flak = WeaponSystem::from_def(
            registry
                .weapons
                .get("turrets-galore:sentinel_flak_turret")
                .expect("proof flak turret should load"),
        );

        let event = weapon_fire_event(
            Vec2::ZERO,
            vec2(450.0, 0.0),
            &flak,
            WeaponFireOrigin::Player,
            WeaponImpactOwner::Player,
            WeaponTargetRef::Npc("test-target".to_string()),
            "core:sol".to_string(),
        );

        assert!((event.duration - 0.5).abs() < 0.001);
        assert_eq!(event.effect, content::WeaponEffect::Burst);
        assert_eq!(event.burst_count, 5);
        assert_eq!(event.projectile_size, 25.0);
        assert!(event
            .projectile_texture
            .as_deref()
            .is_some_and(|path| path.ends_with("assets/projectiles/sentinel-flak.png")));
    }

    #[test]
    fn starter_redwake_probe_auto_attacks_player() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let probe_def = registry
            .npc_ships
            .get("core:redwake_probe")
            .expect("starter Redwake probe should load");
        let probe_position = vec2(probe_def.position[0], probe_def.position[1]);
        let probe_id = probe_def.id.clone();
        let probe_name = probe_def.name.clone();
        let probe_role = probe_def.role.clone();
        let probe_faction = probe_def.faction.clone();
        let probe_behavior_tags = probe_def.behavior_tags.clone();
        let probe_weapon_slots = probe_def.weapon_slots.clone();
        let probe_cargo = probe_def.cargo_defaults.clone();
        let probe_energy_capacity = probe_def.energy_capacity;
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut probe = test_npc_ship(NpcBehaviorMode::HostileIntercept, probe_position);
        probe.id = probe_id;
        probe.name = probe_name;
        probe.role = probe_role;
        probe.faction = probe_faction;
        probe.behavior_tags = probe_behavior_tags;
        probe.weapon_slots = probe_weapon_slots.clone();
        probe.equipped_weapons =
            equipped_weapons_from_ids(&game.content_registry, &probe_weapon_slots);
        probe.cargo_defaults = probe_cargo
            .into_iter()
            .map(|stack| ItemStack {
                item: required_item(&game.content_registry, &stack.item),
                count: stack.count,
            })
            .collect();
        probe.energy.current = probe_energy_capacity;
        game.npc_ships = vec![probe];
        game.equipped_weapons = equipped_weapons_from_ids(
            &game.content_registry,
            &["core:point_defense_turret".to_string()],
        );
        game.ship.systems.energy.current = 100.0;
        game.ship.position = Vec2::ZERO;
        game.ship.systems.shields.current = 100.0;
        game.ship.systems.hull.current = 100.0;

        update_weapon_systems(&mut game, 0.1);

        assert_eq!(game.equipped_weapons[0].status, WeaponStatus::Fired);
        assert_eq!(
            game.npc_ships[0].equipped_weapons[0].status,
            WeaponStatus::Fired
        );
        assert_eq!(game.weapon_fire_events.len(), 2);
        assert_eq!(
            incoming_weapon_fire_count(&game.ship, &game.weapon_fire_events),
            1
        );
        assert_eq!(game.ship.systems.shields.current, 100.0);
        finish_weapon_fire_events(&mut game);
        assert!(game.npc_ships[0].shields.current < game.npc_ships[0].shields.max);
        assert!(game.ship.systems.shields.current < 100.0);
    }

    #[test]
    fn hostile_pressure_probe_drains_shields_in_range() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.current_system_id = "remote-duskfall:duskfall_reach".to_string();
        let mut probe = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(100.0, 0.0));
        probe.system = game.current_system_id.clone();
        probe.role = "hostile".to_string();
        probe.behavior_tags = vec!["hostile".to_string(), "pressure".to_string()];
        game.npc_ships = vec![probe];
        game.ship.position = Vec2::ZERO;
        game.ship.systems.shields.current = 50.0;
        game.save_dirty = false;

        update_hostile_npc_pressure(&mut game, 1.0);

        assert_eq!(
            active_hostile_pressure_count(
                &game.content_registry,
                &game.ship,
                &game.npc_ships,
                &game.current_system_id,
            ),
            1
        );
        assert!((game.ship.systems.shields.current - 47.6).abs() < 0.01);
        assert_eq!(game.ship.systems.hull.current, game.ship.systems.hull.max);
        assert!(game.save_dirty);
    }

    #[test]
    fn hostile_pressure_respects_shield_damage_resistance() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.current_system_id = "remote-duskfall:duskfall_reach".to_string();
        game.equipped_shields = equipped_shields_from_ids(
            &game.content_registry,
            &["core:balanced_shield_matrix".to_string()],
        );
        game.rebuild_ship_from_upgrades();
        let mut probe = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(100.0, 0.0));
        probe.system = game.current_system_id.clone();
        probe.role = "hostile".to_string();
        probe.behavior_tags = vec!["hostile".to_string(), "pressure".to_string()];
        game.npc_ships = vec![probe];
        game.ship.position = Vec2::ZERO;
        game.ship.systems.shields.current = 50.0;

        update_hostile_npc_pressure(&mut game, 1.0);

        assert_eq!(game.equipped_shields[0].damage_resistance, 0.10);
        assert!((game.ship.systems.shields.current - 47.84).abs() < 0.01);
    }

    #[test]
    fn hostile_pressure_spills_to_hull_only_after_shields_drop() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.current_system_id = "remote-duskfall:duskfall_reach".to_string();
        let mut probe = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(100.0, 0.0));
        probe.system = game.current_system_id.clone();
        probe.role = "hostile".to_string();
        probe.behavior_tags = vec!["hostile".to_string(), "pressure".to_string()];
        game.npc_ships = vec![probe];
        game.ship.position = Vec2::ZERO;
        game.ship.systems.shields.current = 1.0;
        game.ship.systems.hull.current = 80.0;

        update_hostile_npc_pressure(&mut game, 1.0);

        assert_eq!(game.ship.systems.shields.current, 0.0);
        assert!((game.ship.systems.hull.current - 79.51).abs() < 0.01);
    }

    #[test]
    fn collision_requires_contact_and_meaningful_closing_speed() {
        assert_eq!(
            collision_closing_speed(
                Vec2::ZERO,
                vec2(10.0, 0.0),
                20.0,
                vec2(30.0, 0.0),
                Vec2::ZERO,
                20.0,
            ),
            None
        );
        assert_eq!(
            collision_closing_speed(
                Vec2::ZERO,
                vec2(10.0, 0.0),
                20.0,
                vec2(35.0, 0.0),
                Vec2::ZERO,
                20.0,
            ),
            None
        );
        assert_eq!(
            collision_closing_speed(
                Vec2::ZERO,
                vec2(50.0, 0.0),
                20.0,
                vec2(35.0, 0.0),
                Vec2::ZERO,
                20.0,
            ),
            Some(50.0)
        );
    }

    #[test]
    fn collision_damage_scales_with_speed_and_mass_and_rejects_invalid_values() {
        let light = collision_impact_damage(50.0, 50_000.0, 50_000.0);
        let heavy = collision_impact_damage(50.0, 100_000.0, 100_000.0);

        assert!(heavy > light);
        assert_eq!(collision_impact_damage(f32::NAN, 100_000.0, 100_000.0), 0.0);
        assert_eq!(collision_impact_damage(50.0, f32::INFINITY, 100_000.0), 0.0);
    }

    #[test]
    fn collision_damage_absorbs_shields_then_spills_to_hull() {
        let mut shields = ShipResource::full(10.0);
        let mut hull = ShipResource::full(100.0);

        let application = apply_damage_to_resources(&mut shields, &mut hull, 25.0, 1.0);

        assert_eq!(application.shield_absorbed, 10.0);
        assert_eq!(application.hull_damage, 15.0);
        assert_eq!(shields.current, 0.0);
        assert_eq!(hull.current, 85.0);
    }

    #[test]
    fn npc_collision_damage_respects_equipped_shield_resistance_and_delay() {
        let mut npc = test_npc_ship(NpcBehaviorMode::Patrol, Vec2::ZERO);
        npc.equipped_shields = vec![ShieldSystem {
            id: "test:shield".to_string(),
            name: "Test shield".to_string(),
            install_item: "test:shield".to_string(),
            capacity: 25.0,
            recharge_delay: 3.0,
            recharge_rate: 5.0,
            damage_resistance: 0.2,
            hazard_resistance: 0.0,
        }];

        let application = apply_npc_collision_damage(&mut npc, 10.0);

        assert_eq!(application.shield_absorbed, 8.0);
        assert_eq!(application.hull_damage, 0.0);
        assert_eq!(npc.shields.current, 17.0);
        assert_eq!(npc.shield_recharge_delay_remaining, 3.0);
    }

    #[test]
    fn ship_collision_applies_once_until_pair_cooldown_expires() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut npc = test_npc_ship(NpcBehaviorMode::Patrol, vec2(40.0, 0.0));
        npc.velocity = Vec2::ZERO;
        game.npc_ships = vec![npc];
        game.ship.position = Vec2::ZERO;
        game.ship.velocity = vec2(50.0, 0.0);

        update_ship_collisions(&mut game, 0.1);
        let first_player_shields = game.ship.systems.shields.current;
        let first_npc_shields = game.npc_ships[0].shields.current;
        assert_eq!(game.collision_impact_events.len(), 1);
        assert!(operation_feedback_contains(
            &game,
            "Collision",
            "Impact with"
        ));

        update_ship_collisions(&mut game, 0.1);
        assert_eq!(game.ship.systems.shields.current, first_player_shields);
        assert_eq!(game.npc_ships[0].shields.current, first_npc_shields);

        update_ship_collisions(&mut game, 0.6);
        assert!(game.ship.systems.shields.current < first_player_shields);
        assert!(game.npc_ships[0].shields.current < first_npc_shields);
        assert!(game.save_dirty);
    }

    #[test]
    fn collision_feedback_classifies_shield_hull_and_destroyed_impacts() {
        let minor = collision_impact_severity(
            DamageApplication {
                shield_absorbed: 5.0,
                hull_damage: 0.0,
            },
            false,
        );
        let critical = collision_impact_severity(
            DamageApplication {
                shield_absorbed: 0.0,
                hull_damage: 1.0,
            },
            false,
        );
        let destructive = collision_impact_severity(
            DamageApplication {
                shield_absorbed: 0.0,
                hull_damage: 50.0,
            },
            true,
        );

        assert_eq!(minor, CollisionImpactSeverity::Minor);
        assert_eq!(critical, CollisionImpactSeverity::Critical);
        assert_eq!(destructive, CollisionImpactSeverity::Destructive);
    }

    #[test]
    fn collision_impact_events_expire_without_affecting_operations() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        record_collision_impact(
            &mut game,
            Vec2::ZERO,
            "Impact with Test NPC",
            DamageApplication {
                shield_absorbed: 8.0,
                hull_damage: 0.0,
            },
            false,
        );
        assert_eq!(game.collision_impact_events.len(), 1);
        assert_eq!(game.operation_feedback.len(), 1);

        update_collision_impact_events(&mut game, COLLISION_EVENT_SECONDS + 0.1);

        assert!(game.collision_impact_events.is_empty());
        assert_eq!(game.operation_feedback.len(), 1);
    }

    #[test]
    fn npc_pair_collision_applies_to_both_ships_in_same_system_only() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut left = test_npc_ship(NpcBehaviorMode::Patrol, Vec2::ZERO);
        left.id = "core:left".to_string();
        left.velocity = vec2(50.0, 0.0);
        let mut right = test_npc_ship(NpcBehaviorMode::Patrol, vec2(40.0, 0.0));
        right.id = "core:right".to_string();
        right.velocity = Vec2::ZERO;
        game.npc_ships = vec![left, right];

        update_ship_collisions(&mut game, 0.1);

        assert!(game.npc_ships[0].shields.current < game.npc_ships[0].shields.max);
        assert!(game.npc_ships[1].shields.current < game.npc_ships[1].shields.max);
    }

    #[test]
    fn pressure_requires_hostile_pressure_tag_active_system_and_range() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.current_system_id = "remote-duskfall:duskfall_reach".to_string();
        game.ship.position = Vec2::ZERO;
        game.ship.systems.shields.current = 50.0;
        let mut neutral_pressure = test_npc_ship(NpcBehaviorMode::Patrol, vec2(100.0, 0.0));
        neutral_pressure.system = game.current_system_id.clone();
        neutral_pressure.behavior_tags = vec!["pressure".to_string()];
        let mut hostile_without_pressure =
            test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(120.0, 0.0));
        hostile_without_pressure.system = game.current_system_id.clone();
        hostile_without_pressure.role = "hostile".to_string();
        hostile_without_pressure.behavior_tags = vec!["hostile".to_string()];
        let mut hostile_other_system =
            test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(100.0, 0.0));
        hostile_other_system.system = STARTER_SYSTEM_ID.to_string();
        hostile_other_system.role = "hostile".to_string();
        hostile_other_system.behavior_tags = vec!["hostile".to_string(), "pressure".to_string()];
        let mut hostile_out_of_range =
            test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(900.0, 0.0));
        hostile_out_of_range.system = game.current_system_id.clone();
        hostile_out_of_range.role = "hostile".to_string();
        hostile_out_of_range.behavior_tags = vec!["hostile".to_string(), "pressure".to_string()];
        game.npc_ships = vec![
            neutral_pressure,
            hostile_without_pressure,
            hostile_other_system,
            hostile_out_of_range,
        ];
        game.save_dirty = false;

        update_hostile_npc_pressure(&mut game, 1.0);

        assert_eq!(game.ship.systems.shields.current, 50.0);
        assert!(!game.save_dirty);
    }

    #[test]
    fn duskfall_content_adds_redwake_pressure_probe() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let probe = registry
            .npc_ships
            .get("remote-duskfall:redwake_remote_probe")
            .expect("remote Duskfall pack should define a Redwake pressure probe");

        assert_eq!(probe.system, "remote-duskfall:duskfall_reach");
        assert_eq!(probe.role, "hostile");
        assert_eq!(probe.faction.as_deref(), Some("core:redwake_raiders"));
        assert!(probe.behavior_tags.iter().any(|tag| tag == "pressure"));
    }

    #[test]
    fn inventory_panel_height_is_valid_at_the_minimum_supported_window() {
        let height = inventory_panel_height_for(420.0);

        assert_eq!(height, 336.0);
        assert!(height <= 420.0 - 56.0);
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
    fn starter_redwake_probe_spawns_in_auto_attack_range() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let probe = registry
            .npc_ships
            .get("core:redwake_probe")
            .expect("starter Redwake probe should load");
        let turret = registry
            .weapons
            .get("core:point_defense_turret")
            .expect("point defense turret should load");
        let spawn = vec2(probe.position[0], probe.position[1]);

        assert_eq!(probe.role, "hostile");
        assert!(probe.behavior_tags.iter().any(|tag| tag == "hostile"));
        assert!(probe.weapon_slots.iter().any(|slot| slot == &turret.id));
        assert!(spawn.distance(Vec2::ZERO) <= turret.range + SHIP_RADIUS);
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
            Some("No stock")
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
    fn npc_trade_quotes_are_stable_within_a_market_day() {
        let item = ItemRef {
            id: "core:iron_ore".to_string(),
            name: "Iron ore".to_string(),
            unit_mass: 10.0,
        };
        let npc = test_npc_ship(NpcBehaviorMode::TradeRoute, Vec2::ZERO);
        let first = npc_trade_quote(&npc, &item, 42, 3);
        let second = npc_trade_quote(&npc, &item, 42, 3);

        assert_eq!(first.buy_price, second.buy_price);
        assert_eq!(first.sell_price, second.sell_price);
        assert_eq!(first.rumor, second.rumor);
        assert!(first.buy_price > first.sell_price);
    }

    #[test]
    fn npc_trade_quantity_is_limited_by_stock_credits_and_cargo() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let item = required_item(&registry, "core:iron_ore");
        let npc = test_npc_ship(NpcBehaviorMode::TradeRoute, Vec2::ZERO);
        let stack = ItemStack {
            item: item.clone(),
            count: 8,
        };
        let quote = npc_trade_quote(&npc, &item, 1, 0);
        let mut game = test_game_with_systems(registry, Vec::new());
        game.credits = quote.buy_price * 3;

        assert_eq!(
            npc_trade_buy_amount(
                game.credits,
                &game.inventory,
                &game.ship_upgrades,
                &stack,
                &quote,
                99,
            ),
            3
        );

        game.inventory.add_item(item, 5);
        assert_eq!(npc_trade_sell_amount(&npc, &stack, 5, 99), 5);
    }

    #[test]
    fn identified_nearby_hauler_with_cargo_exposes_trade_action() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let ship = Ship::starter();
        let mut hauler = test_npc_ship(NpcBehaviorMode::TradeRoute, vec2(120.0, 0.0));
        hauler.role = "hauler".to_string();
        hauler.identified = true;
        hauler.cargo_defaults = vec![ItemStack {
            item: required_item(&registry, "core:iron_ore"),
            count: 2,
        }];

        let trade = npc_interaction_rows(&registry, &ship, &hauler)
            .into_iter()
            .find(|row| row.action == NpcInteractionAction::Trade)
            .expect("trade row should exist");
        assert_eq!(trade.state, NpcInteractionState::Available);
        assert_eq!(trade.status, "Open market");
    }

    #[test]
    fn npc_context_menu_starts_with_follow_and_switches_to_stop() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.npc_ships = vec![test_npc_ship(NpcBehaviorMode::Patrol, vec2(120.0, 0.0))];

        let entries = context_action_entries(&game, ContextTarget::NpcShip(0));
        assert_eq!(entries[0].action, ContextAction::Follow);
        assert!(entries[0].enabled);

        game.follow_target = Some(FollowTarget::NpcShip(0));
        let entries = context_action_entries(&game, ContextTarget::NpcShip(0));
        assert_eq!(entries[0].action, ContextAction::StopFollowing);
    }

    #[test]
    fn autopilot_braking_and_speed_limits_are_distance_aware() {
        assert!((autopilot_braking_distance(100.0, 50.0) - 100.0).abs() < 0.01);
        assert_eq!(autopilot_braking_distance(100.0, 0.0), 0.0);
        assert!((autopilot_speed_for_distance(100.0, 50.0, 240.0) - 100.0).abs() < 0.01);
        assert_eq!(autopilot_speed_for_distance(100.0, 50.0, 80.0), 80.0);
    }

    #[test]
    fn follow_keeps_a_moving_npc_outside_collision_clearance() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut npc = test_npc_ship(NpcBehaviorMode::Patrol, vec2(240.0, 0.0));
        npc.velocity = vec2(36.0, 0.0);
        game.npc_ships = vec![npc];
        game.follow_target = Some(FollowTarget::NpcShip(0));

        for _ in 0..240 {
            let npc_velocity = game.npc_ships[0].velocity;
            game.npc_ships[0].position += npc_velocity * 0.1;
            update_ship_follow(&mut game, 0.1);
        }

        let distance = game.ship.position.distance(game.npc_ships[0].position);
        let minimum_distance = game.npc_ships[0].radius + SHIP_RADIUS + AUTOPILOT_CLEARANCE_MARGIN;
        assert!(distance >= minimum_distance - 0.01);
        assert!((distance - PLAYER_FOLLOW_DISTANCE).abs() < 12.0);
    }

    #[test]
    fn npc_context_click_radius_matches_ship_footprint() {
        let npc = test_npc_ship(NpcBehaviorMode::Patrol, Vec2::ZERO);
        let click_radius = npc_ship_click_radius(&npc, 1.0);
        let rendered_size = (npc.radius * 2.0_f32).clamp(22.0, 72.0);

        assert!(click_radius >= rendered_size * 0.5);
        assert!(click_radius <= rendered_size * 0.5 + 6.0);
    }

    #[test]
    fn contextual_action_entries_cover_planets_stations_and_hostile_contacts() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let planet = test_planet("core:context_planet", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        let mut game = test_game_with_systems(registry, vec![planet]);

        let planet_actions = context_action_entries(&game, ContextTarget::Planet(0));
        assert_eq!(planet_actions[0].action, ContextAction::Inspect);
        assert!(planet_actions
            .iter()
            .any(|entry| entry.action == ContextAction::Approach));
        assert!(planet_actions
            .iter()
            .any(|entry| entry.action == ContextAction::Orbit));
        assert!(planet_actions
            .iter()
            .any(|entry| entry.action == ContextAction::Scan));
        assert!(planet_actions
            .iter()
            .any(|entry| entry.action == ContextAction::Mine));
        game.ship.position = vec2(5_000.0, 0.0);
        assert!(
            context_action_entries(&game, ContextTarget::Planet(0))
                .into_iter()
                .find(|entry| entry.action == ContextAction::Inspect)
                .expect("planet Inspect entry should exist")
                .enabled
        );
        execute_context_action(&mut game, ContextTarget::Planet(0), ContextAction::Inspect);
        assert!(game.inventory_open);
        assert_eq!(game.selected_planet, Some(0));
        game.inventory_open = false;
        game.ship.position = Vec2::ZERO;

        game.stations = vec![test_station_destination(
            "core:context_station",
            STARTER_SYSTEM_ID,
            Vec2::ZERO,
        )];
        let station_actions = context_action_entries(&game, ContextTarget::Station(0));
        assert!(station_actions
            .iter()
            .any(|entry| entry.action == ContextAction::Inspect));
        assert!(station_actions
            .iter()
            .any(|entry| entry.action == ContextAction::Dock));
        assert!(station_actions
            .iter()
            .any(|entry| entry.action == ContextAction::Approach));
        assert!(station_actions
            .iter()
            .any(|entry| entry.action == ContextAction::Repair));
        game.ship.position = vec2(5_000.0, 0.0);
        assert!(
            !context_action_entries(&game, ContextTarget::Station(0))
                .into_iter()
                .find(|entry| entry.action == ContextAction::Inspect)
                .expect("station Inspect entry should exist")
                .enabled
        );
        game.ship.position = Vec2::ZERO;

        let mut hostile = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(120.0, 0.0));
        hostile.role = "hostile".to_string();
        hostile.behavior_tags = vec!["hostile".to_string()];
        hostile.identified = true;
        game.npc_ships = vec![hostile];
        let hostile_actions = context_action_entries(&game, ContextTarget::NpcShip(0));
        assert!(hostile_actions
            .iter()
            .any(|entry| entry.action == ContextAction::Inspect));
        assert!(hostile_actions
            .iter()
            .any(|entry| entry.action == ContextAction::Approach));
        game.ship.position = vec2(-5_000.0, 0.0);
        assert!(
            !context_action_entries(&game, ContextTarget::NpcShip(0))
                .into_iter()
                .find(|entry| entry.action == ContextAction::Inspect)
                .expect("contact Inspect entry should exist")
                .enabled
        );
        assert!(
            !hostile_actions
                .iter()
                .find(|entry| entry.action == ContextAction::Hail)
                .expect("hostile contacts retain a readable Hail entry")
                .enabled
        );
        assert!(
            !hostile_actions
                .iter()
                .find(|entry| entry.action == ContextAction::Trade)
                .expect("hostile contacts retain a readable Trade entry")
                .enabled
        );
    }

    #[test]
    fn approach_action_flies_to_planet_and_stops_in_interaction_range() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let planet = test_planet(
            "core:approach_planet",
            STARTER_SYSTEM_ID,
            vec2(600.0, 0.0),
            true,
        );
        let mut game = test_game_with_systems(registry, vec![planet]);

        execute_context_action(&mut game, ContextTarget::Planet(0), ContextAction::Approach);
        assert_eq!(game.approach_target, Some(ApproachTarget::Planet(0)));

        for _ in 0..100 {
            if game.approach_target.is_none() {
                break;
            }
            update_ship_approach(&mut game, 0.1);
        }

        assert!(game.approach_target.is_none());
        let center_distance = game.ship.position.distance(game.planets[0].position);
        assert!(center_distance <= planet_interaction_radius(&game.planets[0]));
        assert!(
            center_distance
                <= planet_interaction_radius(&game.planets[0]) - AUTOPILOT_INTERACTION_INSET
        );
        assert!(
            (planet_surface_distance(&game.ship, &game.planets[0]) - AUTOPILOT_CLEARANCE_MARGIN)
                .abs()
                < 0.1
        );
        assert!((game.ship.angle - std::f32::consts::PI).abs() < 0.01);
    }

    #[test]
    fn mine_context_action_opens_the_selected_planet_pane_only_when_usable() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut planet = test_planet_with_mineables(
            &registry,
            "core:mine_context_planet",
            STARTER_SYSTEM_ID,
            Vec2::ZERO,
            &["core:iron_ore"],
        );
        planet.scan_level = 2;
        let mut game = test_game_with_systems(registry, vec![planet]);

        execute_context_action(&mut game, ContextTarget::Planet(0), ContextAction::Mine);

        assert_eq!(game.selected_planet, Some(0));
        assert!(game.inventory_open);
        assert_eq!(game.selected_station, None);
        assert_eq!(game.selected_npc_ship, None);

        game.inventory_open = false;
        game.ship.position = vec2(5_000.0, 0.0);
        execute_context_action(&mut game, ContextTarget::Planet(0), ContextAction::Mine);
        assert!(!game.inventory_open);
    }

    #[test]
    fn inventory_shortcut_does_not_convert_object_menu_or_pane_to_ship_pane() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let planet = test_planet("core:shortcut_planet", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        let mut game = test_game_with_systems(registry, vec![planet]);

        game.selected_planet = Some(0);
        game.context_action_menu = Some(ContextActionMenu {
            content: ContextMenuContent::Actions(ContextTarget::Planet(0)),
            position: Vec2::ZERO,
        });
        handle_inventory_shortcut(&mut game);
        assert!(game.context_action_menu.is_none());
        assert!(!game.inventory_open);
        assert_eq!(game.selected_planet, Some(0));

        game.inventory_open = true;
        handle_inventory_shortcut(&mut game);
        assert!(!game.inventory_open);
        assert_eq!(game.selected_planet, None);

        handle_inventory_shortcut(&mut game);
        assert!(game.inventory_open);
        assert_eq!(game.selected_planet, None);
    }

    #[test]
    fn egui_context_menu_choice_executes_the_existing_world_action() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let planet = test_planet("core:context_planet", STARTER_SYSTEM_ID, Vec2::ZERO, true);
        let mut game = test_game_with_systems(registry, vec![planet]);
        game.context_action_menu = Some(ContextActionMenu {
            content: ContextMenuContent::Actions(ContextTarget::Planet(0)),
            position: Vec2::ZERO,
        });

        apply_context_menu_outcome(
            &mut game,
            Some(egui_context_menu::ContextMenuOutcome::Choose(0)),
        );

        assert!(game.context_action_menu.is_none());
        assert!(game.inventory_open);
        assert_eq!(game.selected_planet, Some(0));
    }

    #[test]
    fn context_target_ranking_chooses_the_nearest_object_across_types() {
        let content = context_menu_content_for_candidates(vec![
            ContextTargetCandidate {
                target: ContextTarget::Planet(0),
                cursor_distance: 82.0,
            },
            ContextTargetCandidate {
                target: ContextTarget::Station(0),
                cursor_distance: 18.0,
            },
        ]);

        assert_eq!(
            content,
            Some(ContextMenuContent::Actions(ContextTarget::Station(0)))
        );
    }

    #[test]
    fn context_target_ranking_requests_a_choice_for_nearly_tied_objects() {
        let content = context_menu_content_for_candidates(vec![
            ContextTargetCandidate {
                target: ContextTarget::Planet(0),
                cursor_distance: 42.0,
            },
            ContextTargetCandidate {
                target: ContextTarget::Station(0),
                cursor_distance: 29.0,
            },
            ContextTargetCandidate {
                target: ContextTarget::NpcShip(0),
                cursor_distance: 90.0,
            },
        ]);

        assert_eq!(
            content,
            Some(ContextMenuContent::ChooseTarget(vec![
                ContextTarget::Station(0),
                ContextTarget::Planet(0),
            ]))
        );
    }

    #[test]
    fn station_approach_leaves_inspect_enabled_and_opens_station_pane() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:approach_station",
            STARTER_SYSTEM_ID,
            vec2(900.0, 0.0),
        )];

        begin_approach(&mut game, ApproachTarget::Station(0));
        for _ in 0..120 {
            if game.approach_target.is_none() {
                break;
            }
            update_ship_approach(&mut game, 0.1);
        }

        let center_distance = game.ship.position.distance(game.stations[0].position);
        assert!(
            center_distance
                <= station_interaction_radius(&game.stations[0]) - AUTOPILOT_INTERACTION_INSET
        );
        assert!(
            (station_surface_distance(&game.ship, &game.stations[0]) - AUTOPILOT_CLEARANCE_MARGIN)
                .abs()
                < 0.1
        );

        let inspect = context_action_entries(&game, ContextTarget::Station(0))
            .into_iter()
            .find(|entry| entry.action == ContextAction::Inspect)
            .expect("station Inspect entry should exist");
        assert!(inspect.enabled);
        execute_context_action(&mut game, ContextTarget::Station(0), ContextAction::Inspect);
        assert!(game.inventory_open);
        assert_eq!(game.selected_station, Some(0));
        assert_eq!(game.selected_planet, None);
        assert_eq!(game.selected_npc_ship, None);
    }

    #[test]
    fn station_content_only_marks_true_repair_yards_as_garages() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let frontier = registry
            .stations
            .values()
            .find(|station| station.id.ends_with("frontier_exchange"))
            .expect("Frontier Exchange should load");
        let cinder = registry
            .stations
            .values()
            .find(|station| station.id.ends_with("cinder_repair_yard"))
            .expect("Cinder Repair Yard should load");

        assert!(frontier
            .services
            .iter()
            .any(|service| !service.trade.is_empty()));
        assert!(!frontier
            .services
            .iter()
            .any(|service| service.kind == "garage"));
        assert!(cinder
            .services
            .iter()
            .any(|service| service.kind == "garage"));
    }

    #[test]
    fn npc_cargo_save_round_trip_restores_valid_runtime_items() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let item = required_item(&registry, "core:iron_ore");
        let mut npc = test_npc_ship(NpcBehaviorMode::TradeRoute, Vec2::ZERO);
        npc.id = "core:trade_test".to_string();
        npc.cargo_capacity = 1_000.0;
        npc.cargo_defaults = vec![ItemStack { item, count: 4 }];
        let saved = save_npc_cargo(&[npc]);

        let mut restored = test_npc_ship(NpcBehaviorMode::TradeRoute, Vec2::ZERO);
        restored.id = "core:trade_test".to_string();
        restored.cargo_capacity = 1_000.0;
        apply_npc_cargo_save(std::slice::from_mut(&mut restored), &saved, &registry);

        assert_eq!(restored.cargo_defaults[0].item.id, "core:iron_ore");
        assert_eq!(restored.cargo_defaults[0].count, 4);

        let mut depleted = test_npc_ship(NpcBehaviorMode::HostileIntercept, Vec2::ZERO);
        depleted.id = "core:depleted_ammo".to_string();
        let depleted_save = save_npc_cargo(&[depleted]);
        let mut restored_depleted = test_npc_ship(NpcBehaviorMode::HostileIntercept, Vec2::ZERO);
        restored_depleted.id = "core:depleted_ammo".to_string();
        restored_depleted.cargo_defaults = vec![ItemStack {
            item: required_item(&registry, "core:interceptor_round"),
            count: 60,
        }];
        apply_npc_cargo_save(
            std::slice::from_mut(&mut restored_depleted),
            &depleted_save,
            &registry,
        );
        assert!(restored_depleted.cargo_defaults.is_empty());
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
            max_stock: Some(2),
            restock_days: Some(3.0),
            next_restock_day: Some(3.0),
            catalog_rotation: None,
            unavailable: false,
        }];
        game.ship.position = vec2(120.0, 0.0);

        assert!(buy_station_trade_offer(&mut game, 0, 0, 0));
        assert_eq!(game.credits, 75);
        assert_eq!(game.inventory.count(&iron), 1);
        assert_eq!(game.stations[0].services[0].trade[0].stock, Some(1));
        assert!(operation_feedback_contains(
            &game,
            "Trade",
            "Bought Iron ore"
        ));

        assert!(sell_station_trade_offer(&mut game, 0, 0, 0));
        assert_eq!(game.credits, 85);
        assert_eq!(game.inventory.count(&iron), 0);
        assert_eq!(game.stations[0].services[0].trade[0].stock, Some(2));
        assert_eq!(
            latest_operation_feedback(&game),
            Some(("Trade", "Sold Iron ore to Test Station for 10 cr"))
        );
    }

    #[test]
    fn station_ui_actions_dispatch_through_existing_market_rules() {
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
            max_stock: Some(2),
            restock_days: None,
            next_restock_day: None,
            catalog_rotation: None,
            unavailable: false,
        }];
        game.ship.position = vec2(120.0, 0.0);
        game.selected_station = Some(0);
        game.selected_station_service = Some(0);

        apply_station_ui_actions(&mut game, vec![egui_station::StationUiAction::Buy(0)]);

        assert_eq!(game.credits, 75);
        assert_eq!(game.inventory.count(&iron), 1);
        assert_eq!(game.stations[0].services[0].trade[0].stock, Some(1));
        assert!(game.save_dirty);
    }

    #[test]
    fn garage_repairs_hull_and_shields_for_credits() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.stations[0].faction = Some("core:helioforge_yard_union".to_string());
        game.selected_station_service = Some(1);
        game.credits = 1_000;
        game.ship.position = vec2(120.0, 0.0);
        game.ship.systems.hull.current = 80.0;
        game.ship.systems.shields.current = 90.0;
        let expected_cost = (game.ship.systems.hull.max - 80.0).ceil() as u32 * 3
            + (game.ship.systems.shields.max - 90.0).ceil() as u32;

        assert!(repair_ship_at_station(&mut game, 0));
        assert_eq!(game.ship.systems.hull.current, game.ship.systems.hull.max);
        assert_eq!(
            game.ship.systems.shields.current,
            game.ship.systems.shields.max
        );
        assert_eq!(game.credits, 1_000 - expected_cost);
        assert_eq!(game.faction_reputation["core:helioforge_yard_union"], 1);
    }

    #[test]
    fn survey_contract_accepts_tracks_scan_and_pays_at_origin() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(
            registry,
            vec![test_planet(
                "core:test_target",
                STARTER_SYSTEM_ID,
                vec2(400.0, 0.0),
                true,
            )],
        );
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.selected_station = Some(0);
        game.ship.position = vec2(120.0, 0.0);
        game.stations[0].services[0].contracts = vec![ContractOffer {
            id: "core:test_survey".to_string(),
            name: "Test Survey".to_string(),
            kind: "survey".to_string(),
            description: None,
            origin_station: "core:test_station".to_string(),
            origin_service: "core:test_market".to_string(),
            target_station: None,
            target_planet: Some("core:test_target".to_string()),
            item: None,
            amount: 1,
            reward: 240,
            duration_days: 10.0,
            reputation_faction: Some("core:frontier_cartographers".to_string()),
            reputation_required: 0,
            reputation_reward: 5,
        }];

        assert!(accept_or_complete_contract(&mut game, 0, 0, 0));
        assert_eq!(game.active_contracts.len(), 1);
        let serialized = toml::to_string(&game.to_save()).expect("contract save should serialize");
        let restored_save =
            toml::from_str::<SaveData>(&serialized).expect("contract save should load");
        assert_eq!(restored_save.active_contracts.len(), 1);
        assert_eq!(restored_save.active_contracts[0].id, "core:test_survey");
        game.planets[0].scan_level = 1;
        update_contract_progress(&mut game);
        assert!(game.active_contracts[0].target_reached);
        assert!(accept_or_complete_contract(&mut game, 0, 0, 0));
        assert!(game.active_contracts.is_empty());
        assert_eq!(game.credits, default_credits() + 240);
        assert_eq!(game.faction_reputation["core:frontier_cartographers"], 5);
    }

    #[test]
    fn active_contract_menu_resolves_progress_and_destination() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(
            registry,
            vec![test_planet(
                "core:test_target",
                STARTER_SYSTEM_ID,
                vec2(400.0, 0.0),
                true,
            )],
        );
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.stations[0].services[0].contracts = vec![ContractOffer {
            id: "core:test_survey".to_string(),
            name: "Test Survey".to_string(),
            kind: "survey".to_string(),
            description: Some("Survey the target.".to_string()),
            origin_station: "core:test_station".to_string(),
            origin_service: "core:test_market".to_string(),
            target_station: None,
            target_planet: Some("core:test_target".to_string()),
            item: None,
            amount: 2,
            reward: 240,
            duration_days: 10.0,
            reputation_faction: None,
            reputation_required: 0,
            reputation_reward: 5,
        }];
        game.active_contracts.push(ActiveContract {
            id: "core:test_survey".to_string(),
            origin_station: "core:test_station".to_string(),
            origin_service: "core:test_market".to_string(),
            expires_day: 10.0,
            target_reached: false,
        });

        let entries = active_contract_menu_entries(&game);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].origin_name, "Test Station");
        assert_eq!(entries[0].target_name, "core:test_target");
        assert_eq!(entries[0].progress, "Survey level 0/2");
        assert_eq!(entries[0].status, "Active");

        let initial_view = contracts_view(&game);
        assert_eq!(initial_view.rows.len(), 1);
        assert_eq!(initial_view.rows[0].name, "Test Survey");
        assert!(initial_view.detail.is_none());
        apply_contracts_ui_actions(
            &mut game,
            vec![egui_contracts::ContractsUiAction::Select(0)],
        );
        let selected_view = contracts_view(&game);
        assert_eq!(selected_view.selected, Some(0));
        assert_eq!(
            selected_view
                .detail
                .as_ref()
                .map(|detail| detail.progress.as_str()),
            Some("Survey level 0/2")
        );
        assert!(selected_view
            .detail
            .as_ref()
            .is_some_and(|detail| detail.focus_enabled));

        game.active_contracts[0].target_reached = true;
        let completed_entries = active_contract_menu_entries(&game);
        assert_eq!(completed_entries[0].progress, "Survey level 2/2");
        assert_eq!(completed_entries[0].status, "Ready to complete");

        game.contracts_open = true;
        apply_contracts_ui_actions(
            &mut game,
            vec![egui_contracts::ContractsUiAction::FocusOrigin(0)],
        );
        assert!(!game.contracts_open);
        assert!(game.inventory_open);
        assert_eq!(game.selected_station, Some(0));
        assert_eq!(game.selected_station_service, Some(0));
    }

    #[test]
    fn fit_debug_text_truncates_unicode_without_splitting_utf8() {
        let fitted = append_debug_ellipsis("Freight Lock · Ready → ×".to_string(), 30);
        assert!(fitted.is_char_boundary(fitted.len()));
        assert_eq!(fitted, "Freight Lock · Ready...");
    }

    #[test]
    fn vendor_catalogs_are_deterministic_and_rotate_by_world_day() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let vendor = registry
            .vendors
            .get("core:frontier_exchange_juno")
            .expect("core vendor should load");
        let reputation = faction_reputation_from_save(&registry, None);
        let (_, first) = runtime_vendor_from_def(&registry, vendor, 42, 0.0, &reputation);
        let (_, repeated) = runtime_vendor_from_def(&registry, vendor, 42, 0.0, &reputation);
        assert_eq!(first.len(), 4);
        assert_eq!(
            first
                .iter()
                .map(|offer| (&offer.item.id, offer.buy_price, offer.stock))
                .collect::<Vec<_>>(),
            repeated
                .iter()
                .map(|offer| (&offer.item.id, offer.buy_price, offer.stock))
                .collect::<Vec<_>>()
        );

        let (rotated_vendor, rotated) =
            runtime_vendor_from_def(&registry, vendor, 42, 5.0, &reputation);
        assert_eq!(rotated_vendor.rotation, 1);
        assert!(rotated
            .iter()
            .all(|offer| offer.catalog_rotation == Some(1)));
        assert!(first
            .iter()
            .zip(rotated.iter())
            .any(|(before, after)| before.buy_price != after.buy_price
                || before.stock != after.stock
                || before.item.id != after.item.id));
    }

    #[test]
    fn reputation_changes_vendor_access_and_prices() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let vendor = registry
            .vendors
            .get("core:cinder_yard_mara")
            .expect("repair vendor should exist");
        let neutral = faction_reputation_from_save(&registry, None);
        let (_, locked) = runtime_vendor_from_def(&registry, vendor, 7, 0.0, &neutral);
        assert!(locked.iter().all(|offer| offer.unavailable));

        let mut trusted = neutral.clone();
        trusted.insert("core:helioforge_yard_union".to_string(), 100);
        let (_, open) = runtime_vendor_from_def(&registry, vendor, 7, 0.0, &trusted);
        assert!(open.iter().all(|offer| !offer.unavailable));
        assert!(open
            .iter()
            .zip(locked.iter())
            .all(|(trusted, locked)| trusted.buy_price <= locked.buy_price));
    }

    #[test]
    fn faction_reputation_round_trips_and_clamps_to_content_bounds() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.faction_reputation
            .insert("core:cinder_cooperative".to_string(), 100);
        let serialized = toml::to_string(&game.to_save()).expect("save should serialize");
        let mut restored_save = toml::from_str::<SaveData>(&serialized).expect("save should load");
        restored_save
            .faction_reputation
            .iter_mut()
            .find(|entry| entry.faction == "core:cinder_cooperative")
            .expect("cinder reputation should be saved")
            .value = 999;
        let restored = faction_reputation_from_save(
            &game.content_registry,
            Some(restored_save.faction_reputation.as_slice()),
        );
        assert_eq!(restored["core:cinder_cooperative"], 100);
    }

    #[test]
    fn station_market_restock_refills_after_world_time_advances() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let iron = required_item(&registry, "core:iron_ore");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.stations[0].services[0].trade = vec![TradeOffer {
            item: iron,
            buy_price: 25,
            sell_price: 10,
            stock: Some(0),
            max_stock: Some(4),
            restock_days: Some(1.0),
            next_restock_day: Some(1.0),
            catalog_rotation: None,
            unavailable: false,
        }];

        advance_world_time_and_planets(&mut game, GAME_DAY_SECONDS * 1.1);

        let offer = &game.stations[0].services[0].trade[0];
        assert_eq!(offer.stock, Some(4));
        assert_eq!(offer.next_restock_day, Some(2.0));
        assert!(game.save_dirty);
    }

    #[test]
    fn station_market_state_round_trips_through_save_data() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let iron = required_item(&registry, "core:iron_ore");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.stations[0].services[0].trade = vec![TradeOffer {
            item: iron,
            buy_price: 25,
            sell_price: 10,
            stock: Some(1),
            max_stock: Some(4),
            restock_days: Some(3.0),
            next_restock_day: Some(7.5),
            catalog_rotation: None,
            unavailable: false,
        }];

        let serialized = toml::to_string(&game.to_save()).expect("save should serialize");
        let restored_save =
            toml::from_str::<SaveData>(&serialized).expect("save should deserialize");
        let mut restored = test_game_with_systems(
            content::load_content_packs(Path::new("content/packs"))
                .expect("content packs should load and validate"),
            Vec::new(),
        );
        let restored_iron = required_item(&restored.content_registry, "core:iron_ore");
        restored.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        restored.stations[0].services[0].trade = vec![TradeOffer {
            item: restored_iron,
            buy_price: 25,
            sell_price: 10,
            stock: Some(4),
            max_stock: Some(4),
            restock_days: Some(3.0),
            next_restock_day: Some(3.0),
            catalog_rotation: None,
            unavailable: false,
        }];
        restored.apply_save(restored_save);

        let offer = &restored.stations[0].services[0].trade[0];
        assert_eq!(offer.stock, Some(1));
        assert_eq!(offer.next_restock_day, Some(7.5));
    }

    #[test]
    fn debug_console_give_adds_items_by_content_id() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let iron = required_item(&registry, "core:iron_ore");
        let mut game = test_game_with_systems(registry, Vec::new());

        assert_eq!(
            execute_debug_console_command(&mut game, "give core:iron_ore 7"),
            "Gave Iron ore x7"
        );
        assert_eq!(game.inventory.count(&iron), 7);
        assert!(game.save_dirty);

        assert_eq!(
            execute_debug_console_command(&mut game, "give core.iron_ore"),
            "Gave Iron ore x1"
        );
        assert_eq!(game.inventory.count(&iron), 8);
    }

    #[test]
    fn debug_console_updates_credits() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());

        assert_eq!(
            execute_debug_console_command(&mut game, "credits 250"),
            "Added 250 credits"
        );
        assert_eq!(game.credits, default_credits() + 250);

        assert_eq!(
            execute_debug_console_command(&mut game, "credits set 42"),
            "Credits set to 42"
        );
        assert_eq!(game.credits, 42);
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
        game.recipe_vendor_locked_recipes =
            research_locked_recipes(&game.content_registry, &game.stations);
        game.ship.position = vec2(120.0, 0.0);

        assert!(!recipe_is_unlocked(&game, "core:advanced_scanner_core"));
        assert!(purchase_recipe_unlock(&mut game, 0, 0, 0));
        assert_eq!(game.credits, 750);
        assert!(recipe_is_unlocked(&game, "core:advanced_scanner_core"));
        assert_eq!(
            game.completed_research,
            vec!["core:advanced_scanner_core".to_string()]
        );
        assert!(operation_feedback_contains(
            &game,
            "Unlock",
            "Advanced scanner core"
        ));
    }

    #[test]
    fn research_node_state_tracks_locked_available_affordable_and_completed() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let starter = registry
            .research
            .get("core:frontier_survey_methods")
            .expect("starter research should exist");
        let scanner = registry
            .research
            .get("core:advanced_scanner_core")
            .expect("scanner research should exist");

        assert_eq!(
            research_node_state(starter, None, &[], starter.price),
            ResearchNodeState::Affordable
        );
        assert_eq!(
            research_node_state(starter, None, &[], starter.price.saturating_sub(1)),
            ResearchNodeState::Available
        );
        assert_eq!(
            research_node_state(scanner, None, &[], 10_000),
            ResearchNodeState::Locked
        );
        assert_eq!(
            research_node_state(
                scanner,
                None,
                &[
                    "core:frontier_survey_methods".to_string(),
                    "core:mining_calibration_i".to_string(),
                ],
                scanner.price,
            ),
            ResearchNodeState::Affordable
        );
        assert_eq!(
            research_node_state(
                starter,
                Some(&ActiveResearch {
                    research: "core:frontier_survey_methods".to_string(),
                    remaining_seconds: 3.0,
                }),
                &[],
                starter.price,
            ),
            ResearchNodeState::Researching
        );
        assert_eq!(
            research_node_state(
                starter,
                None,
                &["core:frontier_survey_methods".to_string()],
                0,
            ),
            ResearchNodeState::Completed
        );
    }

    #[test]
    fn research_egui_browser_projects_authored_tiers_and_uses_existing_start_rules() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.credits = 1_500;
        let starter_id = "core:frontier_survey_methods".to_string();

        let initial = research_view(&game);
        assert_eq!(initial.nodes.len(), game.content_registry.research.len());
        assert!(!initial.edges.is_empty());
        assert!(initial.nodes.iter().any(|node| node.tier == 4));
        assert!(initial.nodes.iter().any(|node| {
            node.id == starter_id && node.state == egui_research::ResearchState::Affordable
        }));

        apply_research_ui_actions(
            &mut game,
            vec![egui_research::ResearchUiAction::Select(starter_id.clone())],
        );
        let selected = research_view(&game);
        assert_eq!(selected.selected.as_deref(), Some(starter_id.as_str()));
        assert!(selected
            .detail
            .as_ref()
            .is_some_and(|detail| detail.start_enabled));

        let price = game.content_registry.research[&starter_id].price;
        apply_research_ui_actions(
            &mut game,
            vec![egui_research::ResearchUiAction::Start(starter_id.clone())],
        );
        assert_eq!(game.credits, 1_500 - price);
        assert_eq!(
            game.active_research
                .as_ref()
                .map(|active| active.research.as_str()),
            Some(starter_id.as_str())
        );
        assert_eq!(
            research_view(&game)
                .detail
                .as_ref()
                .map(|detail| detail.state),
            Some(egui_research::ResearchState::Researching)
        );
    }

    #[test]
    fn debug_console_completes_research_nodes() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());

        let result =
            execute_debug_console_command(&mut game, "research complete core:mining_calibration_i");

        assert!(result.contains("Completed research"));
        assert!(game
            .completed_research
            .contains(&"core:mining_calibration_i".to_string()));

        let result = execute_debug_console_command(&mut game, "research complete all");

        assert!(result.contains("Completed"));
        assert_eq!(
            game.completed_research.len(),
            game.content_registry.research_order.len()
        );
    }

    #[test]
    fn debug_console_unlocks_recipe_research() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());

        let result = execute_debug_console_command(&mut game, "recipes unlock all");

        assert!(result.contains("Unlocked recipes"));
        assert!(recipe_is_unlocked(&game, "core:advanced_scanner_core"));
    }

    #[test]
    fn research_starts_with_timer_then_completes_rewarded_recipe() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.credits = 1_500;
        game.recipe_vendor_locked_recipes =
            research_locked_recipes(&game.content_registry, &game.stations);
        game.completed_research = vec![
            "core:frontier_survey_methods".to_string(),
            "core:mining_calibration_i".to_string(),
        ];

        assert!(!recipe_is_unlocked(&game, "core:advanced_scanner_core"));
        assert!(start_research(&mut game, "core:advanced_scanner_core"));

        assert_eq!(game.credits, 650);
        assert!(!game
            .completed_research
            .contains(&"core:advanced_scanner_core".to_string()));
        assert!(game.active_research.as_ref().is_some_and(|active| {
            active.research == "core:advanced_scanner_core" && active.remaining_seconds == 15.0
        }));
        assert!(!recipe_is_unlocked(&game, "core:advanced_scanner_core"));
        assert!(operation_feedback_contains(
            &game,
            "Research",
            "Started Advanced Scanner Core"
        ));
        assert_eq!(
            research_node_state(
                game.content_registry
                    .research
                    .get("core:yield_optimization_i")
                    .expect("yield research should exist"),
                game.active_research.as_ref(),
                &[
                    "core:frontier_survey_methods".to_string(),
                    "core:mining_calibration_i".to_string(),
                    "core:refinery_throughput_i".to_string(),
                    "core:fabrication_templates_i".to_string(),
                ],
                10_000,
            ),
            ResearchNodeState::Locked
        );
        assert!(start_research(&mut game, "core:yield_optimization_i"));
        assert_eq!(game.credits, 650);
        assert_eq!(
            game.active_research
                .as_ref()
                .map(|active| active.research.as_str()),
            Some("core:advanced_scanner_core")
        );

        update_active_research(&mut game, 14.0);
        assert!(!recipe_is_unlocked(&game, "core:advanced_scanner_core"));
        update_active_research(&mut game, 1.0);

        assert!(game
            .completed_research
            .contains(&"core:advanced_scanner_core".to_string()));
        assert!(game.active_research.is_none());
        assert!(recipe_is_unlocked(&game, "core:advanced_scanner_core"));
        assert!(operation_feedback_contains(
            &game,
            "Research",
            "Completed Advanced Scanner Core"
        ));
    }

    #[test]
    fn completed_research_rewards_drive_operation_effects() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let completed = vec![
            "core:frontier_survey_methods".to_string(),
            "core:mining_calibration_i".to_string(),
            "core:refinery_throughput_i".to_string(),
            "core:fabrication_templates_i".to_string(),
            "core:yield_optimization_i".to_string(),
        ];
        let smelting_recipes = make_recipes_for_station(&registry, "core:smelting");
        let smelting_recipe = smelting_recipes
            .iter()
            .find(|recipe| recipe.id == "core:iron_plate")
            .expect("iron plate recipe should load");
        let fabrication_recipes = make_recipes_for_station(&registry, "core:crafting");
        let fabrication_recipe = fabrication_recipes
            .iter()
            .find(|recipe| recipe.id == "core:gear")
            .expect("gear recipe should load");

        assert_eq!(
            completed_research_reward_amount(&registry, &completed, "mining_speed_percent"),
            13.0
        );
        assert_eq!(
            completed_research_reward_amount(&registry, &completed, "bonus_output_chance"),
            3.0
        );
        assert!(mining_operation_seconds(&registry, &completed) < BASE_MINING_SECONDS);
        assert!(
            recipe_operation_seconds(&registry, &completed, WorkKind::Smelting, smelting_recipe)
                < smelting_recipe.base_seconds
        );
        assert!(
            recipe_operation_seconds(
                &registry,
                &completed,
                WorkKind::Fabrication,
                fabrication_recipe
            ) < fabrication_recipe.base_seconds
        );
    }

    #[test]
    fn operation_feedback_is_bounded_and_deduped() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());

        push_operation_feedback(&mut game, "Trade", "Bought Iron ore");
        push_operation_feedback(&mut game, "Trade", "Bought Iron ore");
        assert_eq!(game.operation_feedback.len(), 1);

        for index in 0..(OPERATION_FEEDBACK_LIMIT + 2) {
            push_operation_feedback(&mut game, "Test", format!("Message {index}"));
        }

        assert_eq!(game.operation_feedback.len(), OPERATION_FEEDBACK_LIMIT);
        assert_eq!(
            latest_operation_feedback(&game),
            Some(("Test", "Message 7"))
        );
        assert!(!operation_feedback_contains(
            &game,
            "Trade",
            "Bought Iron ore"
        ));
    }

    #[test]
    fn operation_feedback_aggregates_repeated_output() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let iron = required_item(&registry, "core:iron_ore");
        let mut game = test_game_with_systems(registry, Vec::new());

        push_aggregate_operation_feedback(
            &mut game,
            "Mining",
            format!("mine:{}", iron.id),
            1,
            |count| format!("Recovered Iron ore x{count}"),
        );
        push_aggregate_operation_feedback(
            &mut game,
            "Mining",
            format!("mine:{}", iron.id),
            2,
            |count| format!("Recovered Iron ore x{count}"),
        );

        assert_eq!(
            latest_operation_feedback(&game),
            Some(("Mining", "Recovered Iron ore x3"))
        );
        assert_eq!(game.operation_feedback[0].count, 3);
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
    fn transition_asset_id_comes_from_file_stem() {
        assert_eq!(
            transition_asset_id_from_path("assets/transitions/frontier-station-approach.png"),
            STATION_APPROACH_TRANSITION_ID
        );
        assert_eq!(
            transition_asset_id_from_path("assets/transitions/frontier-transition-01.jpeg"),
            "frontier-transition-01"
        );
    }

    #[test]
    fn station_system_switch_prefers_station_approach_transition() {
        let station_system = "core:station_system";
        let stations = vec![test_station_destination(
            "core:test_station",
            station_system,
            vec2(100.0, 0.0),
        )];

        assert_eq!(
            preferred_transition_asset_id_for_action(
                &stations,
                &TransitionAction::SwitchSystem(station_system.to_string()),
            ),
            Some(STATION_APPROACH_TRANSITION_ID)
        );
        assert_eq!(
            preferred_transition_asset_id_for_action(
                &stations,
                &TransitionAction::SwitchSystem("core:empty_system".to_string()),
            ),
            None
        );
    }

    #[test]
    fn startup_transition_prefers_station_approach_for_station_system() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");

        assert_eq!(
            preferred_transition_asset_id_for_system(&registry, STARTER_SYSTEM_ID),
            Some(STATION_APPROACH_TRANSITION_ID)
        );
        assert_eq!(
            preferred_transition_asset_id_for_system(&registry, "core:missing_system"),
            None
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
    fn remote_route_readiness_points_to_local_fuel_stock_when_missing() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let fuel_canister =
            core_item(&registry, "fuel_canister").expect("core fuel canister should exist");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        game.stations[0].name = "Fuel Stop".to_string();
        game.stations[0].services[0].trade = vec![TradeOffer {
            item: fuel_canister,
            buy_price: 190,
            sell_price: 64,
            stock: Some(4),
            max_stock: Some(4),
            restock_days: Some(5.0),
            next_restock_day: Some(5.0),
            catalog_rotation: None,
            unavailable: false,
        }];

        assert_eq!(
            route_readiness_summary(&game, "remote-duskfall:duskfall_reach"),
            "Need Fuel canister x1; Fuel Stop stocks it"
        );
    }

    #[test]
    fn remote_route_readiness_recommends_scanner_array_after_fuel_is_ready() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let fuel_canister =
            core_item(&registry, "fuel_canister").expect("core fuel canister should exist");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.inventory.add_item(fuel_canister, 1);

        assert_eq!(
            route_readiness_summary(&game, "remote-duskfall:duskfall_reach"),
            "Route ready; Scanner array 2 recommended"
        );
    }

    #[test]
    fn remote_route_readiness_reports_ready_after_scanner_prep() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let fuel_canister =
            core_item(&registry, "fuel_canister").expect("core fuel canister should exist");
        let mut game = test_game_with_systems(registry, Vec::new());
        game.inventory.add_item(fuel_canister, 1);
        game.ship_upgrades
            .iter_mut()
            .find(|upgrade| upgrade.kind == ShipUpgradeKind::ScannerArray)
            .expect("scanner array upgrade should exist")
            .level = 2;

        assert_eq!(
            route_readiness_summary(&game, "remote-duskfall:duskfall_reach"),
            "Remote prep ready"
        );
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
        assert!(operation_feedback_contains(
            &game,
            "Travel",
            "Warp charging"
        ));

        update_pending_warp(&mut game, WARP_CHARGE_SECONDS + 0.1);
        assert_eq!(game.inventory.count(&fuel_canister), 0);
        assert!(game.scene_transition.is_some());
        assert!(operation_feedback_contains(
            &game,
            "Travel",
            "Warp committed"
        ));

        apply_transition_action(
            &mut game,
            TransitionAction::SwitchSystem(remote_system.clone()),
        );
        assert_eq!(game.current_system_id, remote_system);
        assert!(operation_feedback_contains(
            &game,
            "Travel",
            "Arrived in Duskfall Reach"
        ));
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
        assert!(operation_feedback_contains(
            &game,
            "Survey",
            "surface record updated"
        ));

        game.inventory.add_item(improved_survey_drone.clone(), 1);
        assert!(launch_planet_scan(&mut game, 0));
        assert_eq!(game.planets[0].scan_level, MAX_SCAN_LEVEL);
        assert_eq!(game.inventory.count(&improved_survey_drone), 0);
        assert!(planet_has_surface_scan(&game.planets[0]));
        assert!(planet_has_composition_scan(&game.planets[0]));
        assert!(planet_has_richness_scan(&game.planets[0]));
        assert!(operation_feedback_contains(
            &game,
            "Survey",
            "survey complete"
        ));
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
    fn ship_upgrades_egui_view_and_actions_use_existing_purchase_rules() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(registry, Vec::new());
        let first_cost = game.ship_upgrades[0].next_cost(&game.content_registry);
        for stack in &first_cost {
            game.inventory.add_item(stack.item.clone(), 100);
        }

        let view = upgrades_view(&game, 2);
        assert_eq!(view.rows.len(), SHIP_UPGRADE_COUNT);
        assert_eq!(view.purchase_count, 2);
        assert!(view.rows[0].affordable);
        assert_eq!(view.rows[0].level, 0);
        assert!(view.rows[0]
            .cost
            .iter()
            .all(|cost| cost.available >= cost.required));

        apply_upgrades_ui_actions(
            &mut game,
            vec![egui_upgrades::UpgradesUiAction::Purchase(0)],
            2,
        );

        assert_eq!(game.ship_upgrades[0].level, 2);
        assert!(operation_feedback_contains(&game, "Upgrade", "level 2"));
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
        game.research_open = true;
        game.upgrades_open = true;
        game.content_open = true;
        game.contracts_open = true;
        game.open_turret_bank = Some(0);

        handle_escape_pressed(&mut game);
        assert_eq!(game.open_turret_bank, None);
        assert!(game.content_open);

        handle_escape_pressed(&mut game);
        assert!(!game.content_open);
        assert!(game.contracts_open);
        assert!(game.upgrades_open);

        handle_escape_pressed(&mut game);
        assert!(!game.contracts_open);
        assert!(game.upgrades_open);
        assert!(!game.escape_dialog_open);

        handle_escape_pressed(&mut game);
        assert!(!game.upgrades_open);
        assert!(game.research_open);
        assert!(!game.escape_dialog_open);

        handle_escape_pressed(&mut game);
        assert!(!game.research_open);
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
        let entry_position = game.ship.position;

        assert!(enter_planet_orbit(&mut game, 0));
        assert_eq!(game.ship.position, entry_position);
        assert_eq!(game.orbiting_planet, None);
        assert_eq!(game.approach_target, Some(ApproachTarget::PlanetOrbit(0)));
        advance_test_ship_to_orbit(&mut game, 0);
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
        advance_test_ship_to_orbit(&mut game, 0);
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
        assert_eq!(game.approach_target, None);
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
        advance_test_ship_to_orbit(&mut game, 0);
        update_orbital_hazards(&mut game, 1.0);
        assert_eq!(game.ship.systems.shields.current, max_shields);
    }

    #[test]
    fn navigation_route_uses_stable_planet_ids_in_save_data() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(
            registry,
            vec![test_planet(
                "turrets-galore:route_proof",
                STARTER_SYSTEM_ID,
                vec2(400.0, 0.0),
                true,
            )],
        );
        game.navigation_route = vec![NavigationStop {
            system: STARTER_SYSTEM_ID.to_string(),
            planet: "turrets-galore:route_proof".to_string(),
        }];
        game.navigation_route_current = Some(0);
        game.navigation_route_active = true;

        let serialized = toml::to_string(&game.to_save()).expect("route save should serialize");
        let restored: SaveData = toml::from_str(&serialized).expect("route save should load");

        assert_eq!(restored.navigation_route.len(), 1);
        assert_eq!(
            restored.navigation_route[0].planet,
            "turrets-galore:route_proof"
        );
        assert_eq!(restored.navigation_route_current, Some(0));
        assert!(restored.navigation_route_active);
    }

    #[test]
    fn starmap_panel_size_respects_dragged_dimensions_and_screen_bounds() {
        assert_eq!(
            starmap_panel_size_for(vec2(1600.0, 900.0), Some(vec2(900.0, 600.0))),
            vec2(900.0, 600.0)
        );
        assert_eq!(
            starmap_panel_size_for(vec2(1600.0, 900.0), Some(vec2(5000.0, 5000.0))),
            vec2(1552.0, 852.0)
        );
        assert_eq!(
            starmap_panel_size_for(vec2(480.0, 320.0), Some(vec2(100.0, 100.0))),
            vec2(432.0, 280.0)
        );
    }

    #[test]
    fn starmap_panel_position_stays_on_screen() {
        let screen = vec2(1600.0, 900.0);
        let size = vec2(900.0, 600.0);
        assert_eq!(
            starmap_panel_position_for(screen, size, Some(vec2(300.0, 200.0))),
            vec2(300.0, 200.0)
        );
        assert_eq!(
            starmap_panel_position_for(screen, size, Some(vec2(2000.0, 2000.0))),
            vec2(700.0, 300.0)
        );
        assert_eq!(
            starmap_panel_position_for(screen, size, Some(vec2(-50.0, -20.0))),
            Vec2::ZERO
        );
    }

    #[test]
    fn starmap_geometry_parser_accepts_saved_and_legacy_sizes() {
        assert_eq!(
            parse_starmap_panel_geometry("120 80 900 600\n"),
            Some((Some(vec2(120.0, 80.0)), vec2(900.0, 600.0)))
        );
        assert_eq!(
            parse_starmap_panel_geometry("900 600\n"),
            Some((None, vec2(900.0, 600.0)))
        );
        assert_eq!(parse_starmap_panel_geometry("120 80 900\n"), None);
    }

    #[test]
    fn startup_loading_composition_stays_centered_and_visible() {
        let (logo, panel_center_y) = startup_loading_layout(vec2(1100.0, 760.0));
        assert_eq!(logo.w, 720.0);
        assert!(logo.x >= 0.0 && logo.y >= 0.0);
        assert!(logo.x + logo.w <= 1100.0);
        assert!(logo.y + logo.h < panel_center_y);
        assert!(panel_center_y <= 760.0 - 72.0);

        let (compact_logo, compact_panel_center_y) = startup_loading_layout(vec2(640.0, 420.0));
        assert!(compact_logo.x >= 0.0 && compact_logo.y >= 0.0);
        assert!(compact_logo.x + compact_logo.w <= 640.0);
        assert!(compact_logo.y + compact_logo.h < compact_panel_center_y);
        assert!(compact_panel_center_y <= 420.0 - 72.0);
    }

    #[test]
    fn title_submenu_actions_queue_the_selected_new_game() {
        let ship = TitleShip {
            id: "test:ship".to_string(),
            name: "Test Ship".to_string(),
            texture: None,
            hull_capacity: 100.0,
            shield_capacity: 100.0,
            weapon_banks: 1,
        };
        let mut menu = TitleMenu {
            view: TitleView::NewGame,
            pending_action: None,
            branding_egui_image: None,
            branding_egui_texture: None,
            new_game_seed_text: "1".to_string(),
            ships: vec![ship],
            selected_ship_index: 0,
            ship_textures: HashMap::new(),
            save_slots: Vec::new(),
            selected_save_index: 0,
            pending_delete_save_index: None,
            delete_save_error: None,
            content_packs: Vec::new(),
            selected_pack_index: 0,
            settings: AppSettings::default(),
            selected_settings_category: SettingsCategory::Display,
        };
        let selected_ship = menu.ships.len().saturating_sub(1);

        apply_title_submenu_actions(
            &mut menu,
            vec![
                egui_title::TitleSubmenuAction::SetSeed("4242".to_string()),
                egui_title::TitleSubmenuAction::SelectShip(selected_ship),
                egui_title::TitleSubmenuAction::StartGame,
            ],
        );

        match menu.pending_action.as_ref() {
            Some(TitleAction::NewGame { seed, ship_id, .. }) => {
                assert_eq!(*seed, 4242);
                assert_eq!(ship_id, &menu.ships[selected_ship].id);
            }
            _ => panic!("new-game submenu should queue a start action"),
        }
    }

    #[test]
    fn title_ship_preview_rotation_swaps_image_dimensions() {
        let image = Image {
            bytes: vec![
                255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255,
                255, 0, 255, 255, 255,
            ],
            width: 2,
            height: 3,
        };

        let rotated = quarter_turned_cropped_color_image(&image, Rect::new(0.0, 0.0, 2.0, 3.0));

        assert_eq!(rotated.size, [3, 2]);
        assert_eq!(rotated.pixels[0], egui::Color32::from_rgb(255, 0, 255));
        assert_eq!(rotated.pixels[2], egui::Color32::from_rgb(255, 0, 0));
    }

    #[test]
    fn navigation_route_load_discards_missing_content_and_resumes_valid_stop() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(
            registry,
            vec![test_planet(
                "turrets-galore:route_proof",
                STARTER_SYSTEM_ID,
                vec2(400.0, 0.0),
                true,
            )],
        );
        let mut save = game.to_save();
        save.navigation_route = vec![
            SaveNavigationStop {
                system: STARTER_SYSTEM_ID.to_string(),
                planet: "turrets-galore:route_proof".to_string(),
            },
            SaveNavigationStop {
                system: STARTER_SYSTEM_ID.to_string(),
                planet: "missing-pack:missing-planet".to_string(),
            },
        ];
        save.navigation_route_current = Some(0);
        save.navigation_route_active = true;

        game.apply_save(save);

        assert_eq!(game.navigation_route.len(), 1);
        assert_eq!(
            game.navigation_route[0].planet,
            "turrets-galore:route_proof"
        );
        assert!(game.navigation_route_active);
        assert_eq!(game.approach_target, Some(ApproachTarget::Planet(0)));
    }

    #[test]
    fn active_navigation_route_advances_through_each_planet() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load and validate");
        let mut game = test_game_with_systems(
            registry,
            vec![
                test_planet("core:route_one", STARTER_SYSTEM_ID, vec2(360.0, 0.0), true),
                test_planet(
                    "core:route_two",
                    STARTER_SYSTEM_ID,
                    vec2(780.0, 120.0),
                    true,
                ),
            ],
        );
        game.ship.position = Vec2::ZERO;
        game.navigation_route = vec![
            NavigationStop {
                system: STARTER_SYSTEM_ID.to_string(),
                planet: "core:route_one".to_string(),
            },
            NavigationStop {
                system: STARTER_SYSTEM_ID.to_string(),
                planet: "core:route_two".to_string(),
            },
        ];

        assert!(activate_navigation_route(&mut game));
        assert_eq!(game.approach_target, Some(ApproachTarget::Planet(0)));
        for _ in 0..600 {
            if !game.navigation_route_active {
                break;
            }
            update_ship_approach(&mut game, 0.1);
        }

        assert!(!game.navigation_route_active);
        assert_eq!(game.navigation_route_current, Some(2));
        assert_eq!(game.approach_target, None);
        assert!(planet_in_interaction_range(&game.ship, &game.planets[1]));
        assert!(operation_feedback_contains(
            &game,
            "Route",
            "Itinerary complete"
        ));
    }

    #[test]
    fn manual_route_pause_preserves_the_remaining_itinerary() {
        let mut game = test_game_with_systems(
            content::ContentRegistry::default(),
            vec![test_planet(
                "core:route_pause",
                STARTER_SYSTEM_ID,
                vec2(360.0, 0.0),
                true,
            )],
        );
        game.navigation_route = vec![NavigationStop {
            system: STARTER_SYSTEM_ID.to_string(),
            planet: "core:route_pause".to_string(),
        }];
        game.navigation_route_current = Some(0);
        game.navigation_route_active = true;
        game.approach_target = Some(ApproachTarget::Planet(0));

        pause_navigation_route(&mut game, Some("Manual flight input paused itinerary"));

        assert!(!game.navigation_route_active);
        assert_eq!(game.navigation_route_current, Some(0));
        assert_eq!(game.navigation_route.len(), 1);
        assert_eq!(game.approach_target, None);
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
                    vendor: None,
                    trade: Vec::new(),
                    research: Vec::new(),
                    recipe_unlocks: Vec::new(),
                    contracts: Vec::new(),
                    reputation_required: None,
                },
                StationService {
                    id: "core:test_garage".to_string(),
                    name: "Test Garage".to_string(),
                    kind: "garage".to_string(),
                    description: Some("A test garage service.".to_string()),
                    vendor: None,
                    trade: Vec::new(),
                    research: Vec::new(),
                    recipe_unlocks: Vec::new(),
                    contracts: Vec::new(),
                    reputation_required: None,
                },
            ],
        }
    }

    fn make_test_recipe_unlock_station() -> Vec<StationDestination> {
        let mut stations = vec![test_station_destination(
            "core:test_station",
            STARTER_SYSTEM_ID,
            vec2(100.0, 0.0),
        )];
        stations[0].services[0].recipe_unlocks = vec![RecipeUnlockOffer {
            recipe: "core:advanced_scanner_core".to_string(),
            price: 250,
            unavailable: false,
        }];
        stations
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

    fn finish_weapon_fire_events(game: &mut GameState) {
        for event in &mut game.weapon_fire_events {
            event.timer = 0.0;
        }
        update_weapon_systems(game, 0.0);
    }

    fn test_npc_ship(behavior: NpcBehaviorMode, position: Vec2) -> NpcShip {
        NpcShip {
            id: "core:test_npc".to_string(),
            name: "Test NPC".to_string(),
            system: STARTER_SYSTEM_ID.to_string(),
            position,
            velocity: Vec2::ZERO,
            mass: 60_000.0,
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
            trade_open: false,
            credit_reward_min: 0,
            credit_reward_max: 0,
            hull: ShipResource::full(50.0),
            shields: ShipResource::full(25.0),
            equipped_shields: Vec::new(),
            shield_recharge_delay_remaining: 0.0,
            energy: ShipResource::full(20.0),
            shield_slots: Vec::new(),
            weapon_slots: Vec::new(),
            equipped_weapons: Vec::new(),
            summary: "Test NPC ship.".to_string(),
        }
    }

    fn stock_test_npc_ammo(
        npc_ship: &mut NpcShip,
        registry: &content::ContentRegistry,
        item_id: &str,
        count: u32,
    ) {
        npc_ship.cargo_defaults.push(ItemStack {
            item: required_item(registry, item_id),
            count,
        });
    }

    #[test]
    fn inventory_ui_actions_update_production_without_owning_game_rules() {
        let registry = content::ContentRegistry::default();
        let mut game = test_game_with_systems(registry, Vec::new());
        let output = ItemRef {
            id: "test:plate".to_string(),
            name: "Test plate".to_string(),
            unit_mass: 1.0,
        };
        game.smelt_recipes.push(Recipe {
            id: "test:smelt_plate".to_string(),
            output: ItemStack {
                item: output,
                count: 1,
            },
            ingredients: Vec::new(),
            base_seconds: 1.0,
        });
        game.smelt_settings.push(CraftSetting::starter());

        apply_inventory_ui_actions(
            &mut game,
            vec![
                egui_inventory::InventoryUiAction::AdjustKeep {
                    row: 0,
                    direction: 1,
                },
                egui_inventory::InventoryUiAction::QueueRecipe { row: 0 },
            ],
            1,
        );

        assert_eq!(game.smelt_settings[0].keep, 1);
        assert_eq!(game.smelt_settings[0].queued, 1);
        assert!(
            game.save_dirty,
            "post-update UI actions must dirty the save"
        );
    }

    #[test]
    fn inventory_ui_mode_action_selects_the_matching_recipe_table() {
        let mut game = test_game_with_systems(content::ContentRegistry::default(), Vec::new());
        game.work_scroll = 90.0;

        apply_inventory_ui_actions(
            &mut game,
            vec![egui_inventory::InventoryUiAction::SetProductionTab(
                egui_inventory::ProductionTab::Process,
            )],
            1,
        );

        assert!(matches!(game.production_mode, ProductionMode::Processing));
        assert_eq!(game.work_scroll, 0.0);
        assert!(game.save_dirty);
    }

    #[test]
    fn production_keep_click_uses_macroquad_key_steps() {
        assert_eq!(production_keep_click_step_for_keys(false, false), 1);
        assert_eq!(production_keep_click_step_for_keys(true, false), 10);
        assert_eq!(production_keep_click_step_for_keys(false, true), 20);
        assert_eq!(production_keep_click_step_for_keys(true, true), 20);
    }

    #[test]
    fn planet_ui_actions_update_mining_through_gameplay_state() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load");
        let mut planet = test_planet_with_mineables(
            &registry,
            "core:test_mining_planet",
            STARTER_SYSTEM_ID,
            Vec2::ZERO,
            &["core:iron_ore"],
        );
        planet.scan_level = 2;
        let mut game = test_game_with_systems(registry, vec![planet]);
        game.selected_planet = Some(0);

        apply_planet_ui_actions(
            &mut game,
            vec![
                egui_planet::PlanetUiAction::AdjustMiningKeep {
                    row: 0,
                    direction: 1,
                },
                egui_planet::PlanetUiAction::QueueMining { row: 0 },
            ],
            10,
        );

        assert_eq!(game.planets[0].mining[0].keep, 10);
        assert_eq!(game.planets[0].mining[0].queued, 1);
        assert!(game.save_dirty);
    }

    #[test]
    fn npc_contact_ui_dispatches_identify_hail_trade_and_quantity_actions() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load");
        let iron = required_item(&registry, "core:iron_ore");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut trader = test_npc_ship(NpcBehaviorMode::TradeRoute, vec2(120.0, 0.0));
        trader.role = "trader".to_string();
        trader.cargo_capacity = 10_000.0;
        trader.cargo_defaults = vec![ItemStack {
            item: iron.clone(),
            count: 5,
        }];
        game.npc_ships = vec![trader];
        game.selected_npc_ship = Some(0);
        game.ship.position = Vec2::ZERO;
        game.credits = 10_000;

        apply_npc_ui_actions(&mut game, vec![egui_npc::NpcUiAction::Interaction(0)], 10);
        assert!(game.npc_ships[0].identified);

        apply_npc_ui_actions(&mut game, vec![egui_npc::NpcUiAction::Interaction(1)], 10);
        assert!(operation_feedback_contains(&game, "Rumor", "Iron ore"));

        apply_npc_ui_actions(&mut game, vec![egui_npc::NpcUiAction::Interaction(3)], 10);
        assert!(game.npc_ships[0].trade_open);
        assert_eq!(game.npc_trade_quantity, 1);

        apply_npc_ui_actions(
            &mut game,
            vec![
                egui_npc::NpcUiAction::AdjustTradeQuantity(1),
                egui_npc::NpcUiAction::Buy(0),
            ],
            10,
        );
        assert_eq!(game.npc_trade_quantity, 11);
        assert_eq!(game.inventory.count(&iron), 5);
        assert_eq!(game.npc_ships[0].cargo_defaults[0].count, 0);
        assert!(operation_feedback_contains(
            &game,
            "Trade",
            "Bought Iron ore x5"
        ));
        assert!(game.save_dirty);
    }

    #[test]
    fn npc_contact_view_projects_hostile_status_and_blocks_friendly_actions() {
        let registry = content::load_content_packs(Path::new("content/packs"))
            .expect("content packs should load");
        let mut game = test_game_with_systems(registry, Vec::new());
        let mut hostile = test_npc_ship(NpcBehaviorMode::HostileIntercept, vec2(120.0, 0.0));
        hostile.identified = true;
        hostile.behavior_tags = vec!["hostile".to_string()];
        game.npc_ships = vec![hostile];
        game.selected_npc_ship = Some(0);
        game.ship.position = Vec2::ZERO;

        let view = npc_pane_view(&game, 20);

        assert!(view.hostile);
        assert!(view.identified);
        assert_eq!(view.quantity_step, 20);
        assert_eq!(view.interactions[1].status, "Hostile");
        assert!(!view.interactions[1].enabled);
        assert_eq!(view.interactions[3].status, "Hostile");
        assert!(!view.interactions[3].enabled);
    }

    fn assert_vec2_near(actual: Vec2, expected: Vec2) {
        assert!(
            actual.distance(expected) < 0.01,
            "expected {expected:?}, got {actual:?}"
        );
    }

    fn advance_test_ship_to_orbit(game: &mut GameState, planet_index: usize) {
        for _ in 0..120 {
            if game.orbiting_planet == Some(planet_index) {
                return;
            }
            update_ship_approach(game, 0.1);
        }
        panic!("ship did not reach the requested orbit");
    }

    fn test_game_with_systems(
        content_registry: content::ContentRegistry,
        planets: Vec<Planet>,
    ) -> GameState {
        let mut inventory = Inventory {
            slots: std::array::from_fn(|_| None),
        };
        for stack in &content_registry.starter_inventory {
            if content_registry
                .items
                .get(&stack.item)
                .is_some_and(|item| item.tier == "ammunition")
            {
                inventory.add_item(required_item(&content_registry, &stack.item), stack.count);
            }
        }
        let mut game = GameState {
            runtime_flags: RuntimeFlags::default(),
            audio: audio::AudioManager::empty(1.0),
            content_registry,
            content_pack_options: Vec::new(),
            transition_assets: Vec::new(),
            scene_transition: None,
            current_system_id: STARTER_SYSTEM_ID.to_string(),
            save_path: save_state_path(),
            world_seed: 1,
            world_elapsed_days: 0.0,
            credits: default_credits(),
            owned_ships: Vec::new(),
            active_owned_ship_id: "ship-1".to_string(),
            next_owned_ship_sequence: 2,
            ship_id: default_starter_ship_id(),
            ship: Ship::starter(),
            installed_power_modules: Vec::new(),
            equipped_shields: Vec::new(),
            equipped_weapons: Vec::new(),
            npc_ships: Vec::new(),
            collision_cooldowns: HashMap::new(),
            collision_impact_events: Vec::new(),
            defense_threats: Vec::new(),
            weapon_fire_events: Vec::new(),
            weapon_area_effect_events: Vec::new(),
            weapon_projectile_textures: HashMap::new(),
            ship_textures: HashMap::new(),
            system_light_haze_texture: None,
            system_stars: Vec::new(),
            planets,
            stations: Vec::new(),
            recipe_vendor_locked_recipes: Vec::new(),
            active_research: None,
            completed_research: Vec::new(),
            selected_planet: None,
            selected_station: None,
            selected_npc_ship: None,
            selected_station_service: None,
            active_contracts: Vec::new(),
            faction_reputation: HashMap::new(),
            selected_research: None,
            destination_planet: None,
            orbiting_planet: None,
            system_destinations: HashMap::new(),
            starmap_selected_planet: None,
            navigation_route: Vec::new(),
            navigation_route_current: None,
            navigation_route_active: false,
            pending_warp: None,
            camera_zoom: 1.0,
            starmap_zoom: 1.0,
            starmap_pan: Vec2::ZERO,
            starmap_drag_previous_mouse: None,
            starmap_panel_position: None,
            starmap_panel_size: None,
            starmap_move_previous_mouse: None,
            starmap_resize_previous_mouse: None,
            open_turret_bank: None,
            inventory,
            smelt_recipes: Vec::new(),
            smelt_settings: Vec::new(),
            craft_recipes: Vec::new(),
            craft_settings: Vec::new(),
            processing_recipes: Vec::new(),
            processing_settings: Vec::new(),
            production_mode: ProductionMode::Smelting,
            ship_upgrades: make_ship_upgrades(),
            inventory_open: false,
            map_open: false,
            research_open: false,
            upgrades_open: false,
            content_open: false,
            contracts_open: false,
            fleet_open: false,
            selected_owned_ship_index: 0,
            fleet_transfer_quantity: 1,
            content_browser: ContentBrowserState::default(),
            escape_dialog_open: false,
            quit_to_title_requested: false,
            starmap_filter: StarmapFilter::All,
            starmap_resource_filter_index: 0,
            work_scroll: 0.0,
            contract_menu_scroll: 0.0,
            selected_contract_index: None,
            inventory_scroll: 0.0,
            upgrades_scroll: 0.0,
            shield_recharge_delay_remaining: 0.0,
            last_window_size: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
            window_save_delay: None,
            save_delay: None,
            save_dirty: false,
            save_status_timer: 0.0,
            save_status_manual: false,
            operation_feedback: Vec::new(),
            debug_console: DebugConsole::default(),
            context_action_menu: None,
            follow_target: None,
            approach_target: None,
            npc_trade_quantity: 1,
        };
        game.owned_ships.push(game.capture_active_owned_ship());
        game
    }
}
