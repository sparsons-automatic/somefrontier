# Plugin System

Some Frontier plugins should start as local data packs. A pack can add items,
recipes, ships, NPC ships, factions, power modules, weapons, shields, planets,
stations, systems, upgrades, assets, and eventually events without compiling
new Rust code.

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
- Skill XP rules, bonus calculations, and progression math.
- Scan/drone mechanics and reveal behavior.
- UI layout and interaction patterns.
- Save/load format and migration rules.
- Pack discovery, dependency sorting, and validation.

Core pack responsibilities:

- Default items.
- Default recipes.
- Default ship, NPC ship, faction, power-module, weapon, and shield definitions.
- Default planets and planet assets.
- Default starting inventory.
- Default station list.
- Default station destinations, service groups, vendor stock, and recipe unlocks.
- Default upgrade cost definitions.
- Default scan requirements, such as the starter survey drone.

Plugin pack responsibilities:

- Add new items, recipes, ships, NPC ships, factions, power modules, weapons,
  shields, planets, stations, systems, and assets.
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
  universe.toml
  systems.toml
  planets.toml
  stations.toml
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
  universe.toml
  systems.toml
  planets.toml
  stations.toml
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
  planets.toml
```

On startup, the loader discovers `remote-duskfall`, loads it after `core`
because `pack.toml` declares `depends_on = ["core"]`, validates all references,
and makes `remote-duskfall:duskfall_reach` available as a remote system.

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
xp_value = 1.5
unit_mass = 10.0

[[items]]
id = "hydrogen_fuel"
name = "Hydrogen fuel"
tier = "fuel"
xp_value = 2.5
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

## `stations.toml`

Stations define both world content categories used by recipes and physical
station destinations that appear in local space. The base game still owns the
execution mechanics and UI behavior for each station or service kind.

```toml
[[stations]]
id = "smelting"
name = "Smelting"
skill = "smelting"
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

[[stations.services.recipe_unlocks]]
recipe = "advanced_scanner_core"
price = 850
```

Rules:

- `id` resolves to a namespaced station ID.
- `name` is the player-facing label.
- `skill` is optional and should match a base-game skill hook when present.
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
- Trade `item` values must reference existing items. `buy_price` and
  `sell_price` must be positive. `stock` and `restock_days` are optional, and
  `restock_days` must be positive when present.
- Recipe unlock `recipe` values must reference existing recipes, and `price`
  must be positive.
- `unavailable = true` can mark trade stock or recipe unlocks as known but not
  currently purchasable.
- Defining a station does not automatically create new UI or behavior. The base
  game must support the station or service mechanic.

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
damage application, and future NPC/faction integration. Each weapon also points
at an inventory install item, allowing crafted turret objects to be swapped into
ship weapon slots as equipment UI is added.

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
tracking_degrees = 360.0
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
- `tracking_degrees` defaults to 360.0 and must not be negative. Values at or
  above 359 degrees behave as full-coverage defensive turrets.
- Turret defense weapons only engage valid hostile threats. Neutral, owned, and
  environmental entities are ignored by the base targeting rules.
- `summary` is optional player-facing metadata.

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

Ship definitions provide data-driven hull and handling stats. The current
starter ship uses this content metadata, while the base game still owns flight,
damage, energy, save/load, shield and weapon slot behavior, and upgrade
behavior.

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
  textures.
- `mass`, acceleration, energy, drag, hull, and shield values must be positive.
- `power_modules` defaults to an empty list. Entries resolve to namespaced power
  module IDs and must reference loaded power modules.
- `shield_slots` defaults to an empty list. Entries resolve to namespaced shield
  IDs and must reference loaded shields.
- `weapon_slots` defaults to an empty list. Entries resolve to namespaced weapon
  IDs and must reference loaded weapons.

## `npc_ships.toml`

NPC ship definitions provide data-driven non-player ship archetypes that can
appear in local space independently of the player. The runtime derives a
behavior mode from each ship's role, faction, and behavior tags, then moves the
ship with lightweight steering and spacing rules. Players can inspect nearby
NPC ships and identify contacts to reveal faction, disposition, systems, loadout,
and action hooks. Full hailing, docking, trade exchanges, and autonomous combat
are owned by later base-game systems.

```toml
[[npc_ships]]
id = "frontier_patrol_cutter"
name = "Frontier Patrol Cutter"
texture = "./assets/ships/npc-scout-01.png"
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
  can also select hostile interception.
- `spawn_weight` defaults to 1.0 and must be positive.
- `spawn_count` defaults to 1 and must be greater than zero.
- `mass`, `cargo_capacity`, hull, shield, and energy capacities must be
  positive.
- `cargo_defaults` defaults to an empty list. Entries resolve to namespaced item
  IDs and must reference loaded items; counts must be greater than zero.
- `shield_slots` and `weapon_slots` default to empty lists and must reference
  loaded shields and weapons when present.
- `summary` is optional player-facing metadata.

## `factions.toml`

Faction files define player-facing societies, cultures, authorities, crews, and
hostile groups that can own or influence world content. Faction records are
data hooks for ownership and disposition; behavior systems such as diplomacy,
regional spawning, contracts, and combat rules remain owned by the base game.

```toml
[[factions]]
id = "cinder_cooperative"
name = "Cinder Cooperative"
kind = "cooperative"
default_disposition = "friendly"
color = [150, 221, 226]
tags = ["industrial", "security", "starter"]
summary = "Frontier industrial cooperative that coordinates starter-system refining, patrol, and station logistics."
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
- Systems, planets, stations, and NPC ships can reference factions with a
  `faction` field.
- Stations can also use `culture` to reference a faction or society record that
  describes local dock culture separately from formal ownership.

## `starter.toml`

Starter files define default world startup content. The runtime resolves starter
stacks through the content item registry and stores registry-backed item IDs in
inventory.

```toml
inventory = [
  { item = "core:iron_ore", count = 18 },
  { item = "core:copper_ore", count = 14 },
  { item = "core:iron_plate", count = 4 },
  { item = "core:copper_plate", count = 2 },
  { item = "core:survey_drone", count = 25 },
]
```

Rules:

- Starter item IDs must exist.
- Counts must be positive.
- Multiple packs can contribute starter inventory while the framework is in
  development. Later scenario support may choose one start profile explicitly.

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
14. Load faction definitions.
15. Load universe, galaxy-group, galaxy-cluster, galaxy, region, system, and
    star metadata.
16. Load planet definitions.
17. Load station definitions and station services.
18. Load upgrade cost definitions.
19. Load starter inventory definitions.
20. Validate cross-references.
21. Record duplicate recipe-output warnings.
22. Build runtime registries.
23. Start the game only if validation succeeds.

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
- A faction has an empty name or kind, or an unsupported default disposition.
- An NPC ship references a missing system, faction, cargo item, shield, or
  weapon.
- An NPC ship has an empty name, archetype, or role; non-positive radius,
  spawn weight, mass, cargo capacity, hull, shield, or energy capacity; zero
  spawn count; or a zero-count cargo default.
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
  invalid trade item, invalid recipe unlock, zero prices, or non-positive
  restock days.
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
UI labels. Item names and XP values come from loaded content definitions.

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
    xp_value: f32,
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
    skill: Option<String>,
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
    xp_value: f32,
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
xp_value = 1.5
unit_mass = 13.0

[[items]]
id = "nickel_plate"
name = "Nickel plate"
tier = "refined_material"
xp_value = 2.5
unit_mass = 19.0

[[items]]
id = "structural_alloy"
name = "Structural alloy"
tier = "refined_material"
xp_value = 4.0
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
