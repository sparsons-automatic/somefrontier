# Plugin System

Some Frontier plugins should start as local data packs. A pack can add items,
recipes, ships, power modules, planets, stations, systems, upgrades, assets, and
eventually events without compiling new Rust code.

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
- Default ship and power-module definitions.
- Default planets and planet assets.
- Default starting inventory.
- Default station list.
- Default station destinations, service groups, vendor stock, and recipe unlocks.
- Default upgrade cost definitions.
- Default scan requirements, such as the starter survey drone.

Plugin pack responsibilities:

- Add new items, recipes, ships, power modules, planets, stations, systems, and
  assets.
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
  ships.toml
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
  ships.toml
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
culture = "Freebelt dockworkers"
faction = "Cinder Cooperative"
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
- `culture`, `faction`, and `summary` are optional player-facing metadata.
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

## `ships.toml`

Ship definitions provide data-driven hull and handling stats. The current
starter ship uses this content metadata, while the base game still owns flight,
damage, energy, save/load, and upgrade behavior.

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
```

Rules:

- `id` resolves to a namespaced ship ID.
- `name` must not be empty.
- `texture` is optional and uses the same path rules as planet and station
  textures.
- `mass`, acceleration, energy, drag, hull, and shield values must be positive.
- `power_modules` defaults to an empty list. Entries resolve to namespaced power
  module IDs and must reference loaded power modules.

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
9. Load ship definitions.
10. Load recipe definitions.
11. Load universe, galaxy-group, galaxy-cluster, galaxy, region, system, and
    star metadata.
12. Load planet definitions.
13. Load station definitions and station services.
14. Load upgrade cost definitions.
15. Load starter inventory definitions.
16. Validate cross-references.
17. Record duplicate recipe-output warnings.
18. Build runtime registries.
19. Start the game only if validation succeeds.

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
- A ship references a missing power module.
- A ship has an empty name or non-positive mass, acceleration, energy, drag,
  hull, or shield values.
- A system references a missing region, galaxy, or universe.
- A system primary star is missing or belongs to another system.
- A star references a missing system.
- A planet references a missing system after local systems are enabled.
- A planet references a missing mineable item.
- A planet orbit has a missing anchor, an anchor outside the planet's system,
  a non-positive radius, unsupported eccentricity, or invalid period.
- A station references a missing system.
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
