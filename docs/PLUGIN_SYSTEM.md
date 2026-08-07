# Plugin System

Some Frontier plugins should start as local data packs. A pack can add items,
recipes, research, ships, NPC ships, factions, power modules, weapons, shields,
planets, stations, systems, upgrades, assets, and eventually events without
compiling new Rust code.

The goal is composition: a player or developer should be able to install one
small thing or a larger themed pack, and packs should be able to reference each
other when dependencies are declared.

## Ownership Boundary

The base game owns mechanics. Content packs provide world content that stacks on
top of those mechanics.

Base game responsibilities:

- Flight physics and camera behavior.
- Inventory rules and item stack behavior.
- Mining, smelting, crafting, and future processing execution.
- Research purchase rules, reward application, and progression math.
- Scan/drone mechanics and reveal behavior.
- UI layout and interaction patterns.
- Save/load format and migration rules.
- Pack discovery, dependency sorting, and validation.

Core pack responsibilities:

- Default items.
- Default recipes and research nodes.
- Default ship, NPC ship, faction, power-module, weapon, and shield definitions.
- Default planets and planet assets.
- Default starting inventory.
- Default station list.
- Default station destinations, service groups, vendor stock, and research leads.
- Default named vendors and rotating vendor catalogs.
- Default research progression definitions.
- Default upgrade cost definitions.
- Default scan requirements, such as the starter survey drone.

Plugin pack responsibilities:

- Add new items, recipes, research nodes, ships, NPC ships, factions, power
  modules, weapons, shields, planets, stations, systems, and assets.
- Add compatibility recipes between packs.
- Add alternate resource branches that use existing mechanics.
- Add upgrade cost branches for base-game upgrade mechanics.
- Add pack configuration options for future scenario or tuning hooks.
- Add alternate starts later, without replacing base game rules.

Plugins should extend the world, not redefine the rules underneath the player.
If a pack needs new behavior, add the mechanic to the base game first and expose
data hooks for packs to use.

## Design Goals

- Packs are file-based and inspectable.
- Content is data-driven before behavior is plugin-driven.
- Internal IDs are namespaced to avoid naming clashes.
- Packs can depend on other packs explicitly.
- Pack validation should fail loudly before the game starts.
- Current core content lives in the built-in `core` pack.
- Game mechanics should remain stable, predictable, and owned by the base game.

## Pack Layout

Recommended folder structure:

```text
content/packs/core/
  pack.toml
  config.toml
  items.toml
  power.toml
  weapons.toml
  shields.toml
  ships.toml
  npc_ships.toml
  factions.toml
  recipes.toml
  research.toml
  universe.toml
  systems.toml
  planets.toml
  stations.toml
  vendors.toml
  upgrades.toml
  starter.toml
  assets/
    planets/
    ships/
    stations/

content/packs/icy-frontier/
  pack.toml
  config.toml
  items.toml
  power.toml
  weapons.toml
  shields.toml
  ships.toml
  npc_ships.toml
  factions.toml
  recipes.toml
  research.toml
  universe.toml
  systems.toml
  planets.toml
  stations.toml
  vendors.toml
  upgrades.toml
  starter.toml
  assets/
    planets/
    ships/
    stations/
```

The repository includes `content/packs/remote-duskfall/` as the first-party
example remote destination pack. It depends on `core` and contributes the
`remote-duskfall:duskfall_reach` system without requiring hardcoded runtime
changes.

Debug warp selection uses loaded system metadata. A plugin system tagged
`remote` can be selected by the temporary in-game transition trigger without
adding the plugin system ID to source code. Player-facing starmap travel lists
systems tagged `starter`, `surveyed-route`, `known`, or `remote`; plugin packs
can expose compatibility routes by adding one of those tags to their systems.

Files are optional if a pack does not need that content type. For example, a
recipe-only compatibility pack can include only `pack.toml` and `recipes.toml`.
The `core` pack should include all default world content, including the default
ship metadata, power modules, starter inventory, station list, upgrade costs,
and starter system metadata.

## Installing and Removing Packs

Content packs are installed by placing a pack directory under `content/packs/`.
Each installed pack must include a valid `pack.toml`.

To install the first remote destination example, keep this directory in place:

```text
content/packs/remote-duskfall/
  pack.toml
  config.toml
  systems.toml
  items.toml
  recipes.toml
  research.toml
  npc_ships.toml
  planets.toml
```

On startup, the loader discovers `remote-duskfall`, loads it after `core`
because `pack.toml` declares `depends_on = ["core"]`, validates all references,
loads plugin research such as `remote-duskfall:duskfall_vanadium_frames`, and
makes `remote-duskfall:duskfall_reach` available as a remote system.

To remove the remote destination pack, move or delete the whole directory:

```sh
mv content/packs/remote-duskfall content/packs.disabled/remote-duskfall
```

The game only scans direct child directories of `content/packs/`, so moving a
pack elsewhere disables it. Do not remove `content/packs/core/`; it contains the
base items, stations, recipes, starter inventory, starter system, and default
planet content.

Save files may still reference content from removed packs. Until save migration
and missing-content recovery screens exist, remove optional packs before relying
on long-lived saves that contain their items, planets, or system IDs.

## Namespaced IDs

Display names are for players. IDs are for data references.

Use this format:

```text
pack-id:local_id
```

Examples:

```text
core:iron_ore
core:copper_plate
icy-frontier:water_ice
scanner-tech:improved_survey_drone
```

Within a pack file, local IDs can be accepted as shorthand for that pack:

```toml
id = "water_ice"
```

At load time, the game resolves it to:

```text
icy-frontier:water_ice
```

References to other packs must use fully qualified IDs:

```toml
ingredients = [
  { item = "core:copper_plate", count = 1 },
  { item = "icy-frontier:water_ice", count = 2 },
]
```

## `pack.toml`

```toml
id = "icy-frontier"
name = "Icy Frontier"
version = "0.1.0"
description = "Adds icy bodies, water ice, coolant, and hydrogen fuel."
depends_on = ["core"]
optional_depends_on = []
```

Rules:

- `id` must be unique.
- `id` should use lowercase letters, numbers, and hyphens.
- `depends_on` packs must load first.
- `depends_on` and `optional_depends_on` accept either string pack IDs or inline
  tables with exact versions:

```toml
depends_on = [
  { id = "core", version = "0.1.0" },
]
optional_depends_on = [
  { id = "scanner-tech", version = "0.1.0" },
]
```

- Required dependencies fail startup when missing or when an exact declared
  version does not match.
- Optional dependencies gate the whole pack. If any optional dependency is
  missing or has the wrong exact version, the pack is skipped and startup
  continues with a content warning.

## `config.toml`

Pack configuration options are loaded as metadata for future setup screens,
scenario tuning, and compatibility switches. They do not change gameplay by
themselves until base-game code reads them.

```toml
[[options]]
id = "resource_density"
label = "Resource density"
description = "Reserved for future world generation resource tuning."
type = "choice"
default = "standard"
choices = ["lean", "standard", "rich"]

[[options]]
id = "starter_supply_bonus"
label = "Starter supply bonus"
type = "bool"
default = false
```

Rules:

- `id` resolves to a namespaced option ID and must be unique within the pack.
- `label` is required and must not be empty.
- `description` is optional.
- Supported `type` values are `bool`, `boolean`, `integer`, `int`, `number`,
  `float`, `text`, `string`, and `choice`.
- `default` must match the declared type. Integer defaults are also accepted for
  `number` options.
- `choice` options must include a non-empty `choices` list, and the default must
  be one of those choices.
- Empty choice entries are ignored.

## `items.toml`

```toml
[[items]]
id = "water_ice"
name = "Water ice"
tier = "raw_resource"
unit_mass = 10.0

[[items]]
id = "hydrogen_fuel"
name = "Hydrogen fuel"
tier = "fuel"
unit_mass = 4.0
```

Suggested item tiers:

```text
raw_resource
refined_material
component
utility
fuel
exotic
```

`unit_mass` is kilograms per inventory unit. Keep masses positive and tuned for
gameplay-scale cargo units rather than literal laboratory samples.

Rules:

- `id` resolves to a namespaced item ID.
- `name` and `tier` must not be empty.
- `unit_mass` must be positive.

## `recipes.toml`

```toml
[[recipes]]
id = "hydrogen_fuel"
station = "processing"
output = { item = "icy-frontier:hydrogen_fuel", count = 1 }
ingredients = [
  { item = "icy-frontier:water_ice", count = 2 },
]
purpose = "Common fuel for longer-range travel."

[[recipes]]
id = "coolant"
station = "processing"
output = { item = "icy-frontier:coolant", count = 1 }
ingredients = [
  { item = "icy-frontier:water_ice", count = 1 },
  { item = "core:copper_plate", count = 1 },
]
purpose = "Reactor and engine support."
```

Suggested stations:

```text
core:smelting
core:crafting
core:processing
core:refining
core:assembly
```

Inside the same pack, local shorthand is allowed:

```toml
station = "smelting"
```

The loader resolves shorthand against the current pack. In `core`, this becomes:

```text
core:smelting
```

Plugin packs should reference core stations explicitly unless they define their
own same-pack station. A recipe in `icy-frontier` that writes `station =
"smelting"` points at `icy-frontier:smelting`, not `core:smelting`.

```toml
station = "core:crafting"
```

Early implementation can map unsupported stations to existing tabs or reject
them until the UI supports those stations.

Rules:

- `id`, `station`, `output.item`, and every ingredient `item` resolve to
  namespaced IDs.
- `output.count` and ingredient counts must be positive.
- `ingredients` must not be empty.
- `station`, output item, and ingredient items must all resolve to loaded
  definitions.
- `allow_duplicate_output = true` suppresses duplicate-output warnings only when
  every recipe for that station/output pair opts in.

## `research.toml`

Research nodes define the progression tree that spends credits on knowledge,
recipe access, visibility, and passive production effects. Runtime purchase UI,
serial research timing, and reward application are base-game behavior, but packs
can contribute nodes to the loaded research registry.

```toml
[[research]]
id = "frontier_survey_methods"
name = "Frontier Survey Methods"
tier = 0
column = 0
row = 0
price = 450
duration_seconds = 5.0
requires = []
revealed_by = []
summary = "Basic archive methods for turning scan records into useful frontier knowledge."

[[research.rewards]]
kind = "mining_speed_percent"
amount = 5.0

[[research]]
id = "advanced_scanner_core"
name = "Advanced Scanner Core"
tier = 1
column = 1
row = 0
price = 850
duration_seconds = 15.0
requires = ["frontier_survey_methods"]
revealed_by = ["frontier_survey_methods"]
summary = "Unlocks the advanced scanner core recipe."

[[research.rewards]]
kind = "recipe_unlock"
target = "advanced_scanner_core"
```

Rules:

- `id`, `requires`, `revealed_by`, and reward `target` values resolve to
  namespaced IDs.
- `name` must not be empty.
- `tier` is a non-negative progression tier used for grouping, labels, and
  author intent.
- `column` and `row` place the node in a horizontal tree layout. Keep `column`
  aligned with `tier` for ordinary tiered progression so the research screen's
  vertical bands and labels stay clear; rows can branch related choices
  vertically.
- `price` must be positive and is paid in credits.
- `duration_seconds` must be positive and controls how long the research takes
  after purchase. Research runs asynchronously during flight, but only one node
  can be active at a time.
- `requires` gates purchase until the referenced research nodes are complete.
- `revealed_by` gates visibility until the referenced research nodes are
  complete. Leave it empty for starting-visible research.
- Every node must define at least one reward.
- `recipe_unlock` rewards require a `target` that references an existing recipe.
- `item_visibility` rewards require a `target` that references an existing item.
- `station_visibility` rewards require a `target` that references an existing
  station.
- `mining_speed_percent`, `smelting_speed_percent`,
  `fabrication_speed_percent`, and `bonus_output_chance` rewards require a
  positive `amount`.

## `stations.toml`

Stations define both world content categories used by recipes and physical
station destinations that appear in local space. The base game still owns the
execution mechanics and UI behavior for each station or service kind.

```toml
[[stations]]
id = "smelting"
name = "Smelting"
base_seconds = 2.0

[[stations]]
id = "frontier_exchange"
name = "Frontier Exchange"
system = "frontier"
position = [760.0, -420.0]
radius = 58.0
icon = "ring"
texture = "./assets/stations/frontier-exchange.png"
culture = "freebelt_compact"
faction = "cinder_cooperative"
summary = "A modular trade station where refinery crews, haulers, and survey pilots exchange cargo and rumors."

[[stations.services]]
id = "market"
name = "Exchange Market"
kind = "shop"
description = "Cargo buying and selling counter."

[[stations.services.trade]]
item = "iron_ore"
buy_price = 18
sell_price = 7
stock = 80
restock_days = 3.0

[[stations.services.research]]
research = "advanced_scanner_core"
```

Rules:

- `id` resolves to a namespaced station ID.
- `name` is the player-facing label.
- `base_seconds` is optional and must be positive when present.
- `system` and `position` must be provided together. A station with both fields
  becomes a local-space destination; a station with neither can still be used as
  a recipe category.
- `radius` defaults to 54.0 and must be positive.
- `texture` is optional and uses the same path rules as planet and ship
  textures.
- `icon` defaults to `station`; currently supported values are interpreted by
  base-game rendering.
- `faction` is optional. When present, it resolves to a namespaced faction ID
  and must reference a loaded faction record.
- `culture` is optional. When present, it also resolves to a namespaced faction
  or society record and must reference a loaded faction record.
- `summary` is optional player-facing metadata.
- Service `id` values resolve to namespaced IDs and must be unique within the
  station.
- Service `name` and `kind` must not be empty.
- Service `kind` is player-facing context unless the base game has specific
  behavior for it. Supported mechanics currently include trade stock, research
  leads, legacy recipe unlock rows, garage repair actions, and hauling/survey
  contract boards. Other concise kinds such as `cargo`, `navigation`, or
  `signals` can label future hooks without adding behavior.
- Trade `item` values must reference existing items. `buy_price` and
  `sell_price` must be positive. `stock` and `restock_days` are optional, and
  `restock_days` must be positive when present. When both are present, `stock`
  is the station's initial and maximum finite stock; the runtime persists the
  current stock and replenishes it after each elapsed restock interval. Offers
  without `stock` remain unlimited and do not restock.
- Research lead `research` values must reference existing research nodes.
- Legacy recipe unlock `recipe` values must reference existing recipes, and
  `price` must be positive. Prefer research leads for new content so the
  research tree remains the progression purchase surface.
- Contract entries are authored as `[[stations.services.contracts]]` records.
  `id`, `name`, `kind`, `amount`, `reward`, and `duration_days` are required.
  Supported kinds are `hauling` and `survey`.
- Hauling contracts must provide `target_station` and an existing `item`; survey
  contracts must provide `target_planet`. A contract has exactly one target,
  and `amount`, `reward`, and `duration_days` must be positive.
- Hauling progress is reached by carrying the requested item to the target
  station in interaction range. Survey progress is reached when the target
  planet has been scanned to the contract amount. Return to the originating
  service to complete the contract and receive its credit reward.
- `reputation_required` defaults to `0` and gates accepting or completing the
  contract against the sponsoring station or vendor faction's player standing.
  `reputation_reward` defaults to `0` and is awarded to that faction when the
  contract is completed.
- Active contracts are limited to three per save, expire at their deadline, and
  are persisted with backward-compatible save defaults.
- `unavailable = true` can mark trade stock, research leads, or legacy recipe
  unlocks as known but not currently usable.
- Defining a station does not automatically create new UI or behavior. The base
  game must support the station or service mechanic.

### Contract example

```toml
[[stations.services]]
id = "freight_lock"
name = "Freight Lock"
kind = "cargo"

[[stations.services.contracts]]
id = "iron_run"
name = "Iron Run"
kind = "hauling"
description = "Deliver iron ore to the exchange."
target_station = "frontier_exchange"
item = "iron_ore"
amount = 6
reward = 180
duration_days = 8.0

[[stations.services.contracts]]
id = "survey_moon"
name = "Survey Moon"
kind = "survey"
target_planet = "kestrel_titanium_moon"
amount = 1
reward = 340
duration_days = 15.0
```

## `vendors.toml`

Vendors provide named, data-driven catalogs for station services. A vendor is
attached to one station and one service, then selects a weighted subset of its
offers for each deterministic world-seed rotation.

```toml
[[vendors]]
id = "frontier_exchange_juno"
name = "Juno Vale"
station = "frontier_exchange"
service = "market"
faction = "cinder_cooperative"
specialties = ["starter ore", "survey supplies"]
rotation_days = 5.0
slots = 4
price_variance = 0.10
reputation_required = 0
price_reputation_scale = -0.05

[[vendors.offers]]
item = "iron_ore"
buy_price = 18
sell_price = 7
min_stock = 45
max_stock = 90
weight = 5.0
```

Rules:

- `id` resolves to a namespaced vendor ID and must be unique.
- `station` and `service` must reference an existing station service.
- `name` must not be empty.
- `faction` is optional and must reference a loaded faction when present.
- `reputation_required` defaults to `0`. A vendor is unavailable while the
  player's standing with its faction is below this value.
- `price_reputation_scale` defaults to `0.0` and must be between `-1.0` and
  `1.0`. It scales the vendor's prices from the player's standing, with the
  runtime clamping the resulting buy/sell multipliers to a safe range.
- `specialties` is player-facing descriptive metadata.
- `rotation_days` and `slots` must be positive.
- `price_variance` must be between `0.0` and `1.0`.
- Each offer's `item` must reference an existing item.
- `buy_price`, `sell_price`, and `weight` must be positive.
- `min_stock` must not exceed `max_stock`.
- The runtime chooses up to `slots` weighted offers, varies their prices from
  the configured base values, and chooses stock between the configured bounds.
- Catalog selection and pricing use the world seed, vendor ID, and rotation
  period, so the same save produces stable results.
- Vendor catalog state uses the same station market save persistence; old saves
  without vendor state fall back to the catalog generated from saved world time.

## `power.toml`

Power modules define installable ship power sources. The base game owns how
power generation is applied; packs provide module stats and item references.

```toml
[[power_modules]]
id = "compact_fission_cell"
name = "Compact Fission Cell"
family = "Nuclear"
install_item = "compact_fission_cell"
generation = 14.0
mass = 3200.0
fuel_item = "reactor_pellet"
fuel_per_minute = 0.01
heat = 0.35
risk = 0.10
summary = "Reliable nuclear ship power for frontier cargo work."
```

Rules:

- `id`, `install_item`, and `fuel_item` resolve to namespaced IDs.
- `name` and `family` must not be empty.
- `generation` and `mass` must be positive.
- `fuel_item` is optional. When present, it must reference an existing item.
- `fuel_per_minute`, `heat`, and `risk` default to 0.0 and must not be
  negative.
- `install_item` must reference an existing item, usually something produced by
  a recipe.
- `summary` is optional player-facing metadata.

## `weapons.toml`

Weapon files define ship-mounted weapon equipment. The first supported weapon
type is automatic turret defense: the player does not manually target or fire
these weapons. The base game owns threat scanning, cooldowns, energy spending,
target resolution, projectile visuals, and damage application. Each weapon also
points at an inventory install item, allowing crafted turret objects to be
swapped into ship weapon slots through the ship Defense rail.

```toml
[[weapons]]
id = "point_defense_turret"
name = "Point Defense Turret"
kind = "turret_defense"
install_item = "point_defense_turret"
range = 460.0
cooldown_seconds = 1.4
damage = 18.0
energy_cost = 7.0
ammo_item = "interceptor_round"
ammo_per_shot = 1
tracking_degrees = 360.0
targeting = "all_hostiles"
effect = "arc"
path_curve_strength = 0.18
path_wobble = 8.0
path_cycles = 3.0
trail_length = 0.4
burst_count = 3
# travel_speed = 900.0
projectile_texture = "./assets/projectiles/point-defense.png"
projectile_size = 28.0
impact = "chain"
chain_targets = 5
chain_range = 260.0
chain_damage_multiplier = 0.72
friendly_fire = "hostiles_only"
beam_color = "#3db2ffff"
core_color = "#b8f5ffff"
impact_color = "#8febffff"
fire_duration_seconds = 0.55
# fire_audio = "./assets/audio/point-defense-fire.wav"
summary = "Automatic defensive turret that engages hostile threats near the ship."
```

Rules:

- `id` resolves to a namespaced weapon ID.
- `name` must not be empty.
- `kind` must be `turret_defense`.
- `install_item` must reference an existing item, usually something produced by
  a crafting recipe. It is the inventory object consumed when this weapon is
  installed and returned when the weapon is swapped out.
- `range`, `cooldown_seconds`, and `damage` must be positive.
- `energy_cost` defaults to 0.0 and must not be negative.
- `ammo_item` is optional and resolves like other pack-local item references.
  Omit it for energy weapons. When present, the player consumes the item from
  inventory and NPC ships consume it from their saved `cargo_defaults`.
- `ammo_per_shot` defaults to 1 and must be positive. Ammunition and energy are
  consumed only after a valid target is acquired and the shot actually fires.
  A weapon without enough ammunition reports `out of ammo` and does not spend
  energy or begin its cooldown.
- `tracking_degrees` defaults to 360.0 and must not be negative. Values at or
  above 359 degrees behave as full-coverage defensive turrets.
- `targeting` defaults to `all_hostiles`. Packs can use `ships_only` for
  anti-ship weapons or `threats_only` for dedicated interceptors. Neutral,
  owned, and environmental entities remain ineligible.
- `effect` selects an engine-owned visual path and defaults to `arc`. Supported
  values are `beam`, `straight`, `arc`, `spiral`, `zigzag`, `homing`, and
  `burst`. These names select curated mechanics; packs do not execute code.
- `path_curve_strength` controls distance-relative bending and defaults to
  0.18. It must not be negative.
- `path_wobble` controls lateral variation in screen pixels and defaults to
  8.0. It must not be negative.
- `path_cycles` controls spiral, zigzag, homing, and burst oscillations. It
  defaults to 3.0 and must be positive.
- `trail_length` is the normalized portion of the path retained behind a
  projectile. It defaults to 0.4 and must be within 0.01 through 1.0.
- `burst_count` controls parallel projectiles for the `burst` path. It defaults
  to 3 and must be between 1 and 8.
- `travel_speed` is optional and must be positive. When supplied, the visual
  duration is calculated from shot distance divided by this speed; otherwise
  `fire_duration_seconds` supplies a fixed duration.
- `projectile_texture` is optional. Relative image paths resolve inside the
  declaring pack and are validated at startup. The sprite should point right;
  the engine rotates it along the selected path and moves it to the impact.
  Without a texture, the engine falls back to its procedural projectile.
- `projectile_size` controls the sprite's length in world units, defaults to
  28.0, and must be positive. The original image aspect ratio is preserved.
- `impact` defaults to `single`. `chain` jumps between nearby targets,
  `splash` damages an area at impact, and `chain_splash` creates an area impact
  at every chain hop. Damage resolves when the visual reaches its target.
- `chain_targets` counts the primary target and defaults to 3 (1 through 16).
  `chain_range` defaults to 240.0 and is the maximum distance between hops.
  `chain_damage_multiplier` defaults to 0.75 and scales each successive hop.
- `splash_radius` is required for `splash` and `chain_splash` and may be up to
  5000.0. `splash_falloff` accepts `none`, `linear`, or `quadratic`.
  `splash_min_multiplier` defaults to 0.2 and controls edge damage.
- `friendly_fire` defaults to `hostiles_only`. Packs may explicitly opt into
  `all_except_owner` or `everyone`; the default never damages neutral, friendly,
  owned, or environmental entities.
- `beam_color`, `core_color`, and `impact_color` accept six- or eight-digit
  hexadecimal RGBA colors.
- `fire_duration_seconds` defaults to 0.55 and must be positive.
- `fire_audio` is optional. Relative paths resolve inside the declaring pack,
  are validated at startup, and fall back to the base weapon cue if decoding
  fails at runtime.
- `summary` is optional player-facing metadata.

`content/packs/turrets-galore/` is the reference implementation. It defines
four inventory items and recipes, contrasting targeting and path policies, a
five-target chain turret, a wide-radius SUPER NUKE, pack-local fire sounds, and
an NPC loadout without adding turret-specific Rust code. It also demonstrates
pack-owned ammunition items and recipes for its physical flak and nuke weapons;
the Ember Lance and Storm Chain remain energy-only.

Weapon-slot saves preserve slot positions and unresolved namespaced IDs. If a
pack is temporarily removed, its weapon is shown as missing rather than being
silently replaced with core equipment; restoring the pack resolves it on the
next load. Legacy compact `weapon_slots` saves remain readable.

## `shields.toml`

Shield files define ship-mounted defensive shield equipment. The base game owns
capacity, recharge timing, resistance math, hazard interaction, save/load, and
slot swapping. Each shield points at an inventory install item so crafted shield
modules can be swapped into ship shield slots.

```toml
[[shields]]
id = "balanced_shield_matrix"
name = "Balanced Shield Matrix"
install_item = "balanced_shield_matrix"
capacity = 100.0
recharge_delay = 4.0
recharge_rate = 7.5
damage_resistance = 0.10
hazard_resistance = 0.15
summary = "Balanced shield matrix with steady recharge and modest all-around resistance."
```

Rules:

- `id` resolves to a namespaced shield ID.
- `name` must not be empty.
- `install_item` must reference an existing item, usually something produced by
  a crafting recipe. It is the inventory object consumed when this shield is
  installed and returned when the shield is swapped out.
- `capacity`, `recharge_delay`, and `recharge_rate` must be positive.
- `damage_resistance` and `hazard_resistance` default to 0.0 and must be
  between 0.0 and 1.0.
- Hazard resistance reduces configured planet hazard shield drain while the ship
  is near a hazardous planet.
- `summary` is optional player-facing metadata.

## `ships.toml`

Ship definitions provide data-driven hull and handling stats. A content pack can
select one as the new-game ship through `starter.toml`, while the base game owns
flight, damage, energy, save/load, shield and weapon slot behavior, and upgrade
behavior. Every loaded ship appears as an independent choice in the New Game
ship grid; it does not need to be nominated in `starter.toml` to be selectable.

```toml
[[ships]]
id = "frontier_cargo_ship_01"
name = "Frontier Cargo Ship"
texture = "./assets/ships/frontier-cargo-ship-01.png"
mass = 85000.0
forward_acceleration = 420.0
reverse_acceleration = 280.0
turn_acceleration = 4.8
energy_capacity = 100.0
energy_recharge = 8.0
linear_drag = 0.985
hull_capacity = 100.0
shield_capacity = 100.0
power_modules = ["compact_fission_cell"]
shield_slots = ["balanced_shield_matrix"]
weapon_slots = ["point_defense_turret"]
```

Rules:

- `id` resolves to a namespaced ship ID.
- `name` must not be empty.
- `texture` is optional and uses the same path rules as planet and station
  textures. The New Game card removes transparent canvas padding, preserves the
  visible image's aspect ratio, and applies a quarter-turn for presentation.
  Gameplay rendering follows ship heading and preserves the source texture's
  aspect ratio without that card-only crop, so authors should still trim large
  transparent margins from the source asset. Do not pre-rotate an asset merely
  to compensate for the New Game card. A missing or unreadable image leaves the
  ship selectable and uses the engine's fallback presentation.
- `mass`, acceleration, energy, drag, hull, and shield values must be positive.
- `power_modules` defaults to an empty list. Entries resolve to namespaced power
  module IDs and must reference loaded power modules.
- `shield_slots` defaults to an empty list. Entries resolve to namespaced shield
  IDs and must reference loaded shields.
- `weapon_slots` defaults to an empty list. Each entry creates one independently
  configurable `Turret Bank` and supplies that bank's initially fitted weapon.
  List order is bank order, repeated weapon IDs are allowed, and every entry
  resolves to a namespaced weapon ID that must reference a loaded weapon.

`content/packs/turrets-galore/ships.toml` is the reference multi-bank player
ship. Its two ordered `weapon_slots` entries produce two independently swappable
banks in the Defense rail, while its `starter.toml` nomination only controls the
picker's initial highlight.

## `npc_ships.toml`

NPC ship definitions provide data-driven non-player ship archetypes that can
appear in local space independently of the player. The runtime derives a
behavior mode from each ship's role, faction, and behavior tags, then moves the
ship with lightweight steering and spacing rules. Players can inspect nearby
NPC ships and identify contacts to reveal faction, disposition, systems, loadout,
and action hooks. Configured turret weapons are active at runtime: non-hostile
NPCs engage hostile threats, and hostile NPCs can fire on the player when in
range. Full hailing, docking, and trade exchanges are owned by later base-game
systems.

```toml
[[npc_ships]]
id = "frontier_patrol_cutter"
name = "Frontier Patrol Cutter"
texture = "./assets/ships/frontier-patrol-cutter.png"
system = "frontier"
position = [820.0, -520.0]
radius = 28.0
archetype = "patrol-cutter"
role = "patrol"
behavior_tags = ["security", "patrol", "non-hostile"]
spawn_weight = 0.75
spawn_count = 1
mass = 42000.0
cargo_capacity = 12000.0
cargo_defaults = [
  { item = "fuel_canister", count = 1 },
]
credit_reward_min = 0
credit_reward_max = 0
hull_capacity = 82.0
shield_capacity = 80.0
energy_capacity = 90.0
shield_slots = ["balanced_shield_matrix"]
weapon_slots = ["point_defense_turret"]
summary = "Local patrol craft that gives the system an early moving security presence."
```

Rules:

- `id` resolves to a namespaced NPC ship ID.
- `name`, `archetype`, and `role` must not be empty.
- `texture` is optional and uses the same path rules as planet, station, and
  ship textures.
- `system` resolves to a namespaced system ID and must reference a loaded
  system.
- `position` is the local-space spawn and route anchor used by lightweight NPC
  movement.
- `radius` defaults to 28.0 and must be positive.
- `faction` is optional. When present, it resolves to a namespaced faction ID
  and must reference a loaded faction record.
- `behavior_tags` defaults to an empty list. The runtime currently recognizes
  `patrol`, `traffic`, `trade-route`, `follow`, `flee`, and `hostile` as
  behavior-selection hints. Faction default disposition and `role = "hostile"`
  can also select hostile interception. Hostile NPCs tagged `pressure` apply
  light shield pressure while close to the player, with limited hull spillover
  after shields are depleted.
- `spawn_weight` defaults to 1.0 and must be positive.
- `spawn_count` defaults to 1 and must be greater than zero.
- `mass`, `cargo_capacity`, hull, shield, and energy capacities must be
  positive.
- `cargo_defaults` defaults to an empty list. Entries resolve to namespaced item
  IDs and must reference loaded items; counts must be greater than zero. At
  runtime, destroyed NPC ships use these entries as automatic loot, adding each
  full stack to the player inventory only when it fits within the player's cargo
  rating.
- `credit_reward_min` and `credit_reward_max` default to 0. Hostile NPC ships
  award a random credit payout in this inclusive range when destroyed; ships
  with `credit_reward_max = 0` do not award credits.
- `shield_slots` and `weapon_slots` default to empty lists and must reference
  loaded shields and weapons when present. NPC `weapon_slots` create live turret
  systems from the same weapon definitions used by player ships.
- `summary` is optional player-facing metadata.

## `factions.toml`

Faction files define player-facing societies, cultures, authorities, crews, and
hostile groups that can own or influence world content. Faction records are
data hooks for ownership, disposition, and player reputation; behavior systems
such as diplomacy, regional spawning, contracts, and combat rules remain owned
by the base game.

```toml
[[factions]]
id = "cinder_cooperative"
name = "Cinder Cooperative"
kind = "cooperative"
default_disposition = "friendly"
color = [150, 221, 226]
tags = ["industrial", "security", "starter"]
summary = "Frontier industrial cooperative that coordinates starter-system refining, patrol, and station logistics."
reputation_start = 0
reputation_min = -100
reputation_max = 100
```

Rules:

- `id` resolves to a namespaced faction ID.
- `name` and `kind` must not be empty. `kind` is descriptive metadata such as
  `cooperative`, `guild`, `authority`, `compact`, `union`, or `raider`.
- `default_disposition` defaults to `neutral` and must be one of `friendly`,
  `neutral`, `hostile`, or `unknown`.
- `color` defaults to `[150, 221, 226]` and is used as display metadata.
- `tags` defaults to an empty list and provides future hooks for spawning,
  encounters, services, and route rules.
- `summary` is optional player-facing metadata.
- `reputation_start` is the initial player standing and must fall within the
  inclusive `reputation_min`/`reputation_max` bounds. The bounds default to
  `-100` and `100`. Optional `reputation_tiers` can name content-defined
  thresholds for standing labels.
- Systems, planets, stations, and NPC ships can reference factions with a
  `faction` field.
- Stations can also use `culture` to reference a faction or society record that
  describes local dock culture separately from formal ownership.

## `starter.toml`

Starter files define default world startup content. The runtime resolves starter
stacks through the content item registry and stores registry-backed item IDs in
inventory.

```toml
ship = "core:frontier_cargo_ship_01"

inventory = [
  { item = "core:iron_ore", count = 18 },
  { item = "core:copper_ore", count = 14 },
  { item = "core:iron_plate", count = 4 },
  { item = "core:copper_plate", count = 2 },
  { item = "core:survey_drone", count = 25 },
]
```

Rules:

- `ship` is optional. When present, it resolves to a namespaced ship ID and must
  reference a loaded ship. It controls the initially highlighted hull in the New
  Game ship picker; the player may select any loaded ship before launch. A
  later-loaded pack declaration replaces an earlier default and emits a loader
  warning.
- Starter item IDs must exist.
- Counts must be positive.
- Multiple packs can contribute starter inventory while the framework is in
  development. Later scenario support may choose one start profile explicitly.
- Starter inventory contributions are additive and independent of the hull the
  player selects. `starter.toml` does not currently define a ship-specific
  loadout or cargo profile; use the ship's module and weapon slot lists for its
  fitted equipment.

The starter ship selection and inventory contributions apply to new games, not
load-time grants. The runtime persists the active namespaced ship ID alongside
its slot loadout. Legacy saves and saves whose ship pack is unavailable fall
back to the core Frontier Cargo Ship. The runtime
builds a new inventory from all loaded packs and then, when loading an existing
save, replaces it with the inventory stored in that save. Removing and restoring
a pack therefore cannot duplicate its starter items in an established game.

For example, `content/packs/turrets-galore/starter.toml` grants one of each
reference turret and selects its two-bank Twinspire Gunship for new games:

```toml
ship = "twinspire_gunship"

inventory = [
  { item = "ember_lance_turret", count = 1 },
  { item = "sentinel_flak_turret", count = 1 },
  { item = "storm_chain_turret", count = 1 },
  { item = "super_nuke_turret", count = 1 },
]
```

Runtime note:

- Starter inventory, recipes, planet mineables, mining, and UI labels use
  registry-backed item IDs.
- Base game mechanics may still reference specific core item IDs for hardcoded
  systems such as survey drone actions.

## `upgrades.toml`

Upgrade files define cost data for base-game upgrade mechanics. The base game
still owns what an upgrade does when purchased.

```toml
[[upgrades]]
id = "engine"
costs = [
  { item = "core:fusion_drive_core", base_count = 0, per_level = 1, per_levels = 4 },
  { item = "core:gear", base_count = 2, per_level = 1 },
  { item = "core:circuit", base_count = 1, per_level = 1, per_levels = 2 },
]
```

Rules:

- `id` resolves to a namespaced upgrade ID.
- `item` must reference an existing item.
- `base_count` is the flat cost added at every level.
- `per_level` defaults to 0 and is multiplied by the next upgrade level.
- `per_levels` defaults to 1 and lets expensive parts appear every few levels.
- Each upgrade must define at least one cost.
- Each cost must have a positive `base_count`, a positive `per_level`, or both.
- `per_levels` must be positive.
- Costs that resolve to zero for an early level are hidden at runtime.

## Universe and System Metadata

Universe-scale metadata lets packs define anything from one extra local system to
a full themed universe. The runtime should only render the active local system,
but the content graph can still describe where that system belongs.

Recommended hierarchy:

```text
universe
  galaxy_group
    galaxy_cluster
      galaxy
        region
          system
            star
              planet
                moon
```

The hierarchy is metadata, not a promise that every level gets a separate UI on
day one. Use stable namespaced IDs so future map views, warp routing, discovery,
and plugin compatibility can reason about the same places.

### `universe.toml`

`universe.toml` defines large-scale places that systems can belong to. Packs can
define only the levels they need.

The runtime content loader supports `universes`, `galaxy_groups`,
`galaxy_clusters`, `galaxies`, and `regions` in this file. Practical warp
destinations and stars belong in `systems.toml`. All IDs are namespaced to the
pack unless they already include an explicit namespace.

```toml
[[universes]]
id = "outer_rim"
name = "Outer Rim Frontier"
description = "A loose frontier setting built around mining guild routes and forgotten survey lanes."

[[galaxies]]
id = "ember_spiral"
name = "Ember Spiral"
universe = "outer_rim"
description = "A mineral-rich spiral galaxy with unstable jump corridors."

[[regions]]
id = "cinder_reaches"
name = "Cinder Reaches"
galaxy = "outer-rim:ember_spiral"
description = "Starter frontier region with industrial-resource systems."
```

Rules:

- Large-scale IDs resolve to namespaced IDs.
- Relationship fields should reference existing IDs when present.
- Plugins may add a single region to `core` or define an entirely separate
  universe namespace.
- Names are player-facing; IDs are for saves, routes, and compatibility packs.

### `systems.toml`

Systems are the practical warp destinations. The game should load and render
only the bodies in the current system. The runtime content loader supports
`systems` and `stars` in this file.

```toml
[[systems]]
id = "cinder_anchor"
name = "Cinder Anchor"
region = "outer-rim:cinder_reaches"
primary_star = "cinder_anchor_primary"
arrival = [0.0, 0.0]
description = "A starter industrial system with iron, copper, water ice, and early fabrication materials."
tags = ["starter", "industrial", "surveyed-route"]

[[stars]]
id = "cinder_anchor_primary"
name = "Cinder Anchor"
system = "outer-rim:cinder_anchor"
classification = "K-type main sequence"
color = [255, 196, 120]
radius = 180.0
position = [-900.0, -700.0]
```

Rules:

- `system` IDs are warp destination IDs.
- `arrival` is the local-space ship position after warping into the system.
- `primary_star` should reference a star in the same system when present.
- `faction` is optional. When present, it resolves to a namespaced faction ID
  and must reference a loaded faction record.
- `tags` are optional and useful for discovery filters, route gating, and plugin
  compatibility.
- Use `starter`, `surveyed-route`, `known`, or `remote` when the system should
  appear in the Known Systems starmap. Untagged systems stay hidden from
  player-facing warp until future discovery rules reveal them.
- Stars are local bodies for display, map context, and future solar mechanics.
- Systems can belong to a region, galaxy, or universe, but they do not have to
  define every hierarchy level.

Universe-pack example:

```toml
# content/packs/space-opera-total-conversion/pack.toml
id = "space-opera-total-conversion"
name = "Space Opera Total Conversion"
version = "0.1.0"
depends_on = ["core"]
```

```toml
# systems.toml
[[systems]]
id = "binary_scrapfield"
name = "Binary Scrapfield"
arrival = [0.0, 0.0]
description = "A two-star salvage system with dense debris belts and illegal refit yards."
tags = ["salvage", "binary-star", "dangerous"]
```

## `planets.toml`

```toml
[[planets]]
id = "fractured_ice_body"
system = "icy-frontier:cinder_anchor"
classification = "Fractured Ice Body"
texture = "assets/planets/frontier-planet-05.png"
position = [40.0, 1520.0]
orbit = { center = [0.0, 0.0], radius = 1520.0, period_days = 240.0, phase = 0.15 }
radius = 70.0
is_poi = true
mineables = [
  "icy-frontier:water_ice",
  "icy-frontier:carbon",
]
hazards = [
  "Unstable cryovolcanic vents",
  "Low-friction landing shelves",
  "Thermal shock fissures",
]
hazard_effects = { shield_drain_per_second = 0.0, mining_speed_multiplier = 1.0 }
summary = "A water-rich frozen body with deep ice fractures that could support fuel and coolant production."
```

Rules:

- `texture` is optional. Paths starting with `./` or `../` resolve relative to
  the pack folder. Paths starting with `assets/` or `content/` resolve from the
  game root. Other relative paths resolve relative to the pack folder.
- Runtime object textures for planets, stations, player ships, and NPC ships
  should be transparent PNGs. Opaque source renders and full-screen transition
  backgrounds can remain outside the content-pack runtime asset folders.
- `system` is required and should reference an existing system.
- `faction` is optional. When present, it resolves to a namespaced faction ID
  and must reference a loaded faction record.
- `mineables` must reference existing item IDs.
- `orbit` is optional. If omitted, `position` remains a static local-space
  coordinate.
- An orbit can use a fixed `center`, `around = "primary_star"`, or `around`
  with a same-system star or planet ID. If neither `center` nor `around` is
  provided, the loader expects the planet's system to define `primary_star`.
- `around = "primary_star"` resolves through the planet's system and is not
  namespaced. Other orbit anchors resolve to namespaced IDs.
- `radius` must be positive.
- `eccentricity` defaults to 0.0 and must be in the supported range
  `0.00..0.85`.
- `axis_phase` and `phase` default to 0.0.
- `period_days` must be positive and at least 30.0 so moving planets stay
  readable during normal play.
- `hazards` are descriptive survey text. They do not cause damage by
  themselves.
- `hazard_effects` is optional and controls actual lightweight hazard behavior:
  `shield_drain_per_second` damages shields when the ship is too close, and
  `mining_speed_multiplier` slows mining while working that planet.
- Negative `shield_drain_per_second` values are clamped to 0.0, and
  `mining_speed_multiplier` values below 1.0 are clamped to 1.0.
- Planets should start unscanned unless a scenario or save file says otherwise.
- `position` should be local coordinates inside the planet's system.

## Compatibility Packs

Compatibility packs connect two or more packs without requiring either pack to
know about the other.

Example:

```toml
# content/packs/icy-scanner-compat/pack.toml
id = "icy-scanner-compat"
name = "Icy Frontier + Scanner Tech Compatibility"
version = "0.1.0"
depends_on = [
  { id = "core", version = "0.1.0" },
]
optional_depends_on = [
  { id = "icy-frontier", version = "0.1.0" },
  { id = "scanner-tech", version = "0.1.0" },
]
```

```toml
# recipes.toml
[[recipes]]
id = "cryo_survey_drone"
station = "core:crafting"
output = { item = "scanner-tech:cryo_survey_drone", count = 1 }
ingredients = [
  { item = "scanner-tech:improved_survey_drone", count = 1 },
  { item = "icy-frontier:coolant", count = 2 },
]
purpose = "Improved survey drone with better icy-body hazard resistance."
```

If either `icy-frontier` or `scanner-tech` is not installed, or if its declared
version does not match exactly, the loader skips `icy-scanner-compat` before
reading its item or recipe files. When both optional dependencies are present,
the compatibility recipes use the same namespaced item, station, and recipe
validation as ordinary pack content.

## Load Order

1. Discover pack folders.
2. Read all `pack.toml` files.
3. Validate unique pack IDs.
4. Skip packs whose `optional_depends_on` entries are missing or version
   mismatched, recording content warnings.
5. Sort remaining packs by `depends_on` and satisfied `optional_depends_on`.
6. For each ordered pack, load `config.toml` options.
7. Load item definitions.
8. Load power module definitions.
9. Load shield definitions.
10. Load weapon definitions.
11. Load ship definitions.
12. Load NPC ship definitions.
13. Load recipe definitions.
14. Load research definitions.
15. Load faction definitions.
16. Load universe, galaxy-group, galaxy-cluster, galaxy, region, system, and
    star metadata.
17. Load planet definitions.
18. Load station definitions and station services.
19. Load upgrade cost definitions.
20. Load starter ship and inventory definitions.
21. Validate cross-references.
22. Record duplicate recipe-output warnings.
23. Build runtime registries.
24. Start the game only if validation succeeds.

## Validation Rules

Reject startup when:

- Two packs have the same pack ID.
- Two definitions resolve to the same namespaced ID.
- A pack ID, dependency ID, or local content ID uses unsupported characters.
- A pack depends on itself.
- A dependency is missing.
- A required dependency declares an exact version that does not match the
  installed pack.
- A dependency declaration has an empty version.
- Dependencies contain a cycle.
- A pack option has an unsupported type, mismatched default, empty choice list,
  or choice default outside its choices.
- An item has an empty name, empty tier, or non-positive unit mass.
- A recipe references a missing item.
- A recipe references a missing station.
- A recipe has no ingredients, zero output count, or a zero-count ingredient.
- A research node has an empty name, zero price, no rewards, missing required or
  revealing research, or a self-reference.
- A research reward has an unsupported kind, is missing a required target or
  amount, or references missing recipe, item, or station content.
- A power module references a missing install item or fuel item.
- A power module has an empty name or family, non-positive generation or mass,
  or negative fuel use, heat, or risk.
- A weapon references a missing install item, has an empty name, unsupported
  kind, non-positive range, cooldown, or damage, or negative energy cost or
  tracking degrees.
- A shield references a missing install item, has an empty name, non-positive
  capacity, recharge delay, or recharge rate, or resistance values outside
  0.0..1.0.
- A ship references a missing power module.
- A ship references a missing shield.
- A ship references a missing weapon.
- A ship has an empty name or non-positive mass, acceleration, energy, drag,
  hull, or shield values.
- Starter configuration references a missing ship.
- A faction has an empty name or kind, or an unsupported default disposition.
- An NPC ship references a missing system, faction, cargo item, shield, or
  weapon.
- An NPC ship has an empty name, archetype, or role; non-positive radius,
  spawn weight, mass, cargo capacity, hull, shield, or energy capacity; zero
  spawn count; a zero-count cargo default; or a credit reward minimum greater
  than its maximum.
- A system references a missing region, galaxy, universe, or faction.
- A system primary star is missing or belongs to another system.
- A star references a missing system.
- A planet references a missing system or faction after local systems are
  enabled.
- A planet references a missing mineable item.
- A planet orbit has a missing anchor, an anchor outside the planet's system,
  a non-positive radius, unsupported eccentricity, or invalid period.
- A station references a missing system, faction, or culture.
- A station defines only one of `system` or `position`.
- A station has an empty name, non-positive base seconds, or non-positive
  radius.
- A station service has an empty name, empty kind, duplicate ID within a station,
  invalid trade item, invalid research lead, invalid legacy recipe unlock, an
  invalid contract target or kind, zero prices/rewards/amounts, or
  non-positive restock days or contract duration.
- An upgrade has no costs, a missing cost item, a zero-count cost, or a
  non-positive `per_levels` interval.
- Starter inventory references a missing item.
- A texture path is missing.
- Counts are zero.
- Planet radius is zero or negative.
- Required strings are empty.

Warnings are acceptable for:

- Unused items.
- Planets placed very far from existing content.
- Systems or regions that are defined but not discoverable yet.
- Optional dependencies that are not installed or whose exact version does not
  match.
- Duplicate recipes for the same station/output pair, unless every duplicate
  recipe opts in with `allow_duplicate_output = true`.

## Runtime Registry

The runtime uses registry-backed item IDs for inventory, recipes, mining, and
UI labels. Item names and unit mass values come from loaded content definitions.

Runtime shape:

```rust
struct PackDef {
    id: String,
    name: String,
    version: String,
    depends_on: Vec<String>,
    optional_depends_on: Vec<String>,
    options: Vec<PackOptionDef>,
}

struct ItemDef {
    id: String,
    name: String,
    tier: String,
    unit_mass: f32,
}

struct RecipeDef {
    id: String,
    station: String,
    output: StackDef,
    ingredients: Vec<StackDef>,
    purpose: Option<String>,
    allow_duplicate_output: bool,
}

struct ResearchDef {
    id: String,
    name: String,
    tier: u32,
    column: i32,
    row: i32,
    price: u32,
    duration_seconds: f32,
    requires: Vec<String>,
    revealed_by: Vec<String>,
    rewards: Vec<ResearchRewardDef>,
    summary: Option<String>,
}

struct ResearchRewardDef {
    kind: String,
    target: Option<String>,
    amount: Option<f32>,
}

struct ShipDef {
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
    power_modules: Vec<String>,
    shield_slots: Vec<String>,
    weapon_slots: Vec<String>,
}

struct NpcShipDef {
    id: String,
    name: String,
    texture: Option<String>,
    system: String,
    position: [f32; 2],
    radius: f32,
    archetype: String,
    role: String,
    faction: Option<String>,
    behavior_tags: Vec<String>,
    spawn_weight: f32,
    spawn_count: u32,
    mass: f32,
    cargo_capacity: f32,
    cargo_defaults: Vec<StackDef>,
    credit_reward_min: u32,
    credit_reward_max: u32,
    hull_capacity: f32,
    shield_capacity: f32,
    energy_capacity: f32,
    shield_slots: Vec<String>,
    weapon_slots: Vec<String>,
    summary: Option<String>,
}

struct FactionDef {
    id: String,
    name: String,
    kind: String,
    default_disposition: FactionDisposition,
    color: [u8; 3],
    tags: Vec<String>,
    summary: Option<String>,
}

enum FactionDisposition {
    Friendly,
    Neutral,
    Hostile,
    Unknown,
}

struct ShieldDef {
    id: String,
    name: String,
    install_item: String,
    capacity: f32,
    recharge_delay: f32,
    recharge_rate: f32,
    damage_resistance: f32,
    hazard_resistance: f32,
    summary: Option<String>,
}

struct WeaponDef {
    id: String,
    name: String,
    kind: WeaponKind,
    install_item: String,
    range: f32,
    cooldown_seconds: f32,
    damage: f32,
    energy_cost: f32,
    tracking_degrees: f32,
    summary: Option<String>,
}

enum WeaponKind {
    TurretDefense,
}

struct PowerModuleDef {
    id: String,
    name: String,
    family: String,
    install_item: String,
    generation: f32,
    mass: f32,
    fuel_item: Option<String>,
    fuel_per_minute: f32,
    heat: f32,
    risk: f32,
    summary: Option<String>,
}

struct PlanetDef {
    id: String,
    system: String,
    faction: Option<String>,
    classification: String,
    texture: Option<String>,
    position: [f32; 2],
    orbit: Option<OrbitDef>,
    radius: f32,
    is_poi: bool,
    mineables: Vec<String>,
    hazards: Vec<String>,
    hazard_effects: HazardEffectsDef,
    summary: String,
}

struct SystemDef {
    id: String,
    name: String,
    region: Option<String>,
    primary_star: Option<String>,
    faction: Option<String>,
    arrival: Vec2,
    description: String,
    tags: Vec<String>,
}

struct StarDef {
    id: String,
    name: String,
    system: String,
    classification: String,
    color: [u8; 3],
    radius: f32,
    position: [f32; 2],
}

struct StationDef {
    id: String,
    name: String,
    base_seconds: Option<f32>,
    system: Option<String>,
    position: Option<[f32; 2]>,
    radius: f32,
    culture: Option<String>,
    faction: Option<String>,
    services: Vec<StationServiceDef>,
}
```

Inventory stores item IDs through runtime item references:

```rust
struct ItemRef {
    id: String,
    name: String,
    unit_mass: f32,
}

struct ItemStack {
    item: ItemRef,
    count: u32,
}
```

## Implementation Plan

1. [x] Add `content/packs/core/` and copy current item, recipe, and planet data into
   TOML files.
2. [x] Add serializable content structs using `serde`.
3. [x] Add a pack discovery and validation module.
4. [x] Load starter inventory, stations, recipes, and planets from content.
5. [x] Add registry-backed item IDs to replace the temporary `ItemKind` bridge.
6. [x] Convert inventory, recipes, mining, and UI labels to read from item IDs.
7. [x] Remove hardcoded core content fallbacks from `main.rs` after parity is
   reached.
8. [x] Add support for optional compatibility packs.
9. Add a simple in-game or startup content error screen.

## Authoring Guidelines

- Start with items, then recipes, then planets.
- Keep recipes connected to existing chains unless the pack is intentionally a
  standalone progression branch.
- Use clear player-facing names even when IDs are technical.
- Avoid adding rare resources without an immediate use.
- Prefer several small packs over one giant pack when features are separable.
- For large themed content, prefer a universe pack that owns its hierarchy and
  systems, then split item/recipe/body expansions into dependent packs when
  useful.
- Use `system` relationships for every local body once systems are enabled so
  total-conversion packs can provide whole alternate universes.
- Include enough starting recipes that the player can actually use the new
  resources after discovering them.
- Do not assume a pack can change flight, inventory, crafting, scanning, or save
  rules. Those mechanics belong to the base game.
- When a content idea needs a new mechanic, document the mechanic first, add it
  to the base game, then expose a pack field for it.

## Minimal Example Pack

```text
content/packs/nickel-expansion/
  pack.toml
  items.toml
  recipes.toml
```

```toml
# pack.toml
id = "nickel-expansion"
name = "Nickel Expansion"
version = "0.1.0"
depends_on = ["core"]
```

```toml
# items.toml
[[items]]
id = "nickel_ore"
name = "Nickel ore"
tier = "raw_resource"
unit_mass = 13.0

[[items]]
id = "nickel_plate"
name = "Nickel plate"
tier = "refined_material"
unit_mass = 19.0

[[items]]
id = "structural_alloy"
name = "Structural alloy"
tier = "refined_material"
unit_mass = 28.0
```

```toml
# recipes.toml
[[recipes]]
id = "nickel_plate"
station = "smelting"
output = { item = "nickel-expansion:nickel_plate", count = 1 }
ingredients = [
  { item = "nickel-expansion:nickel_ore", count = 2 },
]

[[recipes]]
id = "structural_alloy"
station = "smelting"
output = { item = "nickel-expansion:structural_alloy", count = 1 }
ingredients = [
  { item = "core:iron_plate", count = 1 },
  { item = "nickel-expansion:nickel_plate", count = 1 },
]
```
