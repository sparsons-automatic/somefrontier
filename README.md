# Some Frontier

<img src="assets/branding/some-frontier-logo.png" alt="Some Frontier logo" width="520">

Some Frontier is a small 2D space game prototype built in Rust with
[Macroquad](https://macroquad.rs/). You pilot a frontier cargo ship through a
starfield, inspect a point-of-interest planet, mine resources, and craft basic
components through the in-game inventory panel.

## Requirements

- Rust toolchain with Cargo

Install Rust from <https://rustup.rs/> if `cargo` is not already available.

## Run the Game

From the project root:

```sh
cargo run
```

Cargo will download dependencies, compile the game, and launch the Macroquad
window. The package sets `some-frontier` as the default binary, so no extra
`--bin` argument is needed.

For an optimized build:

```sh
cargo run --release
```

For local debugging, enable the in-game console:

```sh
cargo run -- --debug
```

Press `` ` `` or `F12` in-game to open it. Current commands include
`give <item_id> [count]`, `credits <amount>`, `credits set <amount>`,
`research complete <id|all>`, `recipes unlock all`, and `warp <system_id>`.

## Controls

For the full current gameplay reference, see
[The Player's Guide to Some Frontier](docs/PLAYERS_GUIDE.md).

- `W`: forward thrust
- `S`: reverse thrust
- `A` / `D` or left / right arrows: turn the ship
- `Tab` or `E`: toggle inventory and crafting
- `M`: toggle map view
- `K`: toggle research
- `C`: toggle content browser
- Mouse wheel in flight view: zoom camera in/out
- `PageUp` / `PageDown`: zoom camera in/out
- `Space`: inspect/select a planet when the ship is over it
- `Esc`: close the topmost menu or open the pause menu for manual save, title menu, or desktop quit
- Left click a visible planet: select it and open the planet panel
- Left click a planet in the starmap: set it as the active destination
- Left click the ship in the inventory panel: open ship upgrades

## Configuration

The game remembers the last window size after resizing. Window size is saved to
`$XDG_CONFIG_HOME/some-frontier/window-size.txt`, or
`~/.config/some-frontier/window-size.txt` when `XDG_CONFIG_HOME` is not set.

Game progress autosaves to `$XDG_CONFIG_HOME/some-frontier/save.toml`, or
`~/.config/some-frontier/save.toml` when `XDG_CONFIG_HOME` is not set. The save
stores the world seed, ship state, inventory, completed and active research,
upgrades, active destination, equipped shield and weapon slots, production
settings, scanned planets, mining quotas, elapsed world time, and finite station
market stock and restock schedules. The active content-defined ship ID is also
saved, so its hull stats, art, and turret-bank capacity return with the loadout.

New games generate a world seed that lightly rotates and offsets content-defined
planet positions. Loading a save reuses its seed so the same world layout comes
back every time. The expanded New Game screen presents every loaded
content-defined player ship in a scrollable three-column grid. Each card loads
the ship's pack-owned image and includes hull, shield, and turret-bank capacity,
so the ship and world seed are selected together.

## Production and Mining

When the inventory panel is open, the right-side work table shows production
recipes. Use the `Smelt`, `Craft`, and `Process` tabs to switch between ore
smelting, component fabrication, and material/fuel processing.
Click an item row to queue one bill. Production completes one bill at a time,
top to bottom. Use the `Keep` column to set auto-maintained stock targets.

When a planet is selected, the same work table switches to mining. Click a
mineable resource row to queue one mining bill. Mining completes one bill at a
time, top to bottom, with a base duration of 3 seconds per mining operation. The
`Keep` column can automatically mine up to the requested stock level while the
planet panel is selected or while the ship is in range of a scanned planet.

Planets begin unscanned. Build `Survey drone` items from the crafting tab, fly
within the planet interaction range, and launch a drone from the planet panel to
reveal its classification, hazards, and mineable resources. Early survey drones
are one-way probes and are consumed when launched.

You can adjust `Keep` values by left clicking to increase, right clicking to
decrease, or hovering the cell and using the mouse wheel. Hold `Shift` to adjust
by 5 or `Control` to adjust by 10.

Items track `unit_mass` in kilograms. The inventory table shows per-stack mass,
and the ship detail panel shows total cargo mass against cargo rating. Cargo
mass is currently informational and does not affect flight handling yet.

The starter ship has an 85 tonne dry mass. Thruster force is scaled around that
mass so flight feel remains tuned while the displayed mass is more believable.

## Research

Research replaces the old skill-level progression. Open research with `K` to
inspect a horizontal tiered tree. Click a node to inspect its cost, duration,
requirements, and rewards, then use the research button to spend credits and
start it. Only one project can run at a time, and the active project completes
in the background while you keep flying, mining, trading, or producing.

Completed research can unlock production recipes and grant passive effects such
as faster mining, faster smelting, faster fabrication, and bonus output chance.

## Station Markets

Station trade offers can have finite stock. The station panel shows the current
stock and the remaining time until the next restock. The game advances one day
every 120 seconds of play, and finite offers refill to their configured stock
capacity when their restock interval expires. Selling an item back to a station
restores one unit of stock up to that capacity. Market stock and restock timing
are preserved when the game saves and loads. Some station services are managed
by named vendors whose catalogs rotate by world day; their specialties, current
catalog, prices, and stock are shown in the station panel.

Garage services restore damaged hull and shields for credits when the ship is
in dock range. Freight Lock and archive services can offer hauling and survey
contracts: accept them at their origin, complete the cargo delivery or scan,
then return to claim the reward. Active contracts and deadlines are preserved
in saves.

## Ship Upgrades

Open the inventory panel, then click the ship preview to open ship upgrades.
Upgrades consume crafted components and immediately improve ship systems such as
engines, thrusters, energy, and shields.

## Shield Modules

Ships can mount shield modules through shield slots. Shield modules define
capacity, recharge delay, recharge rate, damage resistance, and hazard
resistance. The ship detail view shows the installed shield, current strength,
recharge status, and resistance values. Crafted shield modules can be swapped
through the ship detail panel while saves preserve the equipped shield IDs and
recharge state.

## Turret Defense

Ships can mount automatic defensive turrets through weapon slots. Turrets scan
for hostile threats near the ship, fire when they are ready, and spend ship
energy for each shot. They ignore neutral, owned, and environmental objects, and
do not require manual targeting or a fire button. Weapon definitions are backed
by inventory install items. Each `Turret Bank` exposes a `Turret List` dropdown
populated from the player's inventory, allowing an exact turret to be selected
while saves preserve bank positions and equipped weapon IDs. Content packs can
define multi-turret ships by adding one default weapon per `weapon_slots` entry;
the Turrets Galore pack initially highlights its two-bank Twinspire Gunship in
the New Game picker while still allowing any loaded hull to be selected.
Weapons can use beam, projectile, homing, burst, chain, splash, or combined chain-splash
behavior; projectile damage resolves on impact and hostile-only friendly-fire
safety is the default. Packs may also provide transparent projectile sprites;
the engine points each image along its configured flight path. Physical weapons
can consume pack-defined, craftable ammunition from player inventory or NPC
cargo, while energy weapons remain power-only.

## Content Debugging

Press `C` to open a compact content browser showing loaded packs, items,
recipes, factions, NPC ships, shields, weapons, systems, stars, planets, and
upgrades.
The pack list is selectable: choose a pack to filter the item, recipe, NPC ship,
and planet columns to that pack, and use the mouse wheel over a column to scroll
longer lists.

## Startup Transition

The game shows a startup transition while content and runtime assets load. If
`assets/transitions/` contains supported images, one loaded transition image is
picked for the startup sequence behind labels such as
`Loading planet asset ... {asset name}`. Loading into a system with station
content starts with the station-approach image. Once multiple transition images
are loaded, the loading background holds each image for 3 seconds, then
crossfades to the next image over 2 seconds.

Press `T` in-game to trigger a temporary debug transition between the starter
system and a remote system discovered from loaded content packs. Runtime
transitions prefer the station-approach image when the destination system has
station content, otherwise pick a random loaded transition image, fade in, hold,
apply their midpoint action hook, then fade back to gameplay.

## Assets

Runtime world-content assets live under `content/packs/<pack>/assets/`:

- `content/packs/core/assets/ships/frontier-cargo-ship-01.png`
- `content/packs/core/assets/ships/npc-scout-01.png`
- `content/packs/core/assets/planets/frontier-planet-01.png` through
  `frontier-planet-20.png`
- `content/packs/core/assets/stations/`

Shared engine/runtime assets live under `assets/`:

- `assets/branding/some-frontier-logo.png` for title, pause, and documentation
  branding
- `assets/branding/some-frontier-icon-16.png`,
  `assets/branding/some-frontier-icon-32.png`, and
  `assets/branding/some-frontier-icon-64.png` for app/window identity
- `assets/transitions/` for random space transition images loaded at startup
  (`.png`, `.jpg`, and `.jpeg`)
- `assets/transitions/frontier-transition-01.png`
- `assets/transitions/frontier-transition-02.png`
- `assets/transitions/frontier-station-approach.png`

Additional asset organization notes are in [assets/README.md](assets/README.md).

## Design Docs

Gameplay planning docs live under [docs/](docs/), including recipe/material
chains, broader game guidelines, and the file-based plugin/content-pack model.
Player-facing instructions live in
[The Player's Guide to Some Frontier](docs/PLAYERS_GUIDE.md).
The intended pattern is that base game code owns mechanics and validation while
`core` and plugin packs add world content such as items, recipes, factions,
planets, systems, and assets.

Built-in world content lives under `content/packs/core/`, and optional
first-party/plugin packs live beside it under `content/packs/`. The game
currently loads items, recipes, ships, NPC ships, factions, shields, weapons,
planets, stations, systems, stars, and starter inventory from discovered packs.
Inventory, recipes, mining, and UI labels use registry-backed item IDs.

## Development

Useful Cargo commands:

```sh
cargo check
cargo fmt
cargo clippy
```

Build artifacts are generated under `target/`.
