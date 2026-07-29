# Some Frontier

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

## Controls

- `W`: forward thrust
- `S`: reverse thrust
- `A` / `D` or left / right arrows: turn the ship
- `Tab` or `E`: toggle inventory and crafting
- `M`: toggle map view
- `K`: toggle skills view
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
stores the world seed, ship state, inventory, skills, upgrades, active
destination, equipped shield and weapon slots, production settings, scanned
planets, and mining quotas.

New games generate a world seed that lightly rotates and offsets content-defined
planet positions. Loading a save reuses its seed so the same world layout comes
back every time.

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

Items track `unit_mass` in kilograms. The inventory panel shows per-stack mass
and total cargo mass; cargo mass is currently informational and does not affect
flight handling yet.

The starter ship has an 85 tonne dry mass. Thruster force is scaled around that
mass so flight feel remains tuned while the displayed mass is more believable.

## Skills

Mining, smelting, and crafting actions award XP to their related skills. As a
skill level rises, low-value actions grant less XP. Open the skills view with
`K`, then click `+` to spend XP on skill levels. Unaffordable upgrades are
greyed out.

Skill tiers:

- Levels 1-10: +1% to +10% speed for the related activity.
- Levels 11-20: +0.05% to +0.50% chance for free extra output.

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
by inventory install items, so future equipment screens can swap crafted turret
objects in and out of ship weapon slots while saves preserve the equipped weapon
IDs.

## Content Debugging

Press `C` to open a compact content browser showing loaded packs, items,
recipes, NPC ships, shields, weapons, systems, stars, planets, and upgrades.
The pack list is selectable: choose a pack to filter the item, recipe, NPC ship,
and planet columns to that pack, and use the mouse wheel over a column to scroll
longer lists.

## Startup Transition

The game shows a startup transition while content and runtime assets load. If
`assets/transitions/` contains supported images, one loaded transition image is
picked for the startup sequence behind labels such as
`Loading planet asset ... {asset name}`.

Press `T` in-game to trigger a temporary debug transition between the starter
system and a remote system discovered from loaded content packs. Runtime
transitions pick a random loaded transition image, fade in, hold, apply their
midpoint action hook, then fade back to gameplay.

## Assets

Runtime world-content assets live under `content/packs/<pack>/assets/`:

- `content/packs/core/assets/ships/frontier-cargo-ship-01.png`
- `content/packs/core/assets/ships/npc-scout-01.png`
- `content/packs/core/assets/planets/frontier-planet-01.png` through
  `frontier-planet-20.png`
- `content/packs/core/assets/stations/`

Shared engine/runtime assets live under `assets/`:

- `assets/transitions/` for random space transition images loaded at startup
  (`.png`, `.jpg`, and `.jpeg`)
- `assets/transitions/frontier-transition-01.png`
- `assets/transitions/frontier-transition-02.png`

Additional asset organization notes are in [assets/README.md](assets/README.md).

## Design Docs

Gameplay planning docs live under [docs/](docs/), including recipe/material
chains, broader game guidelines, and the file-based plugin/content-pack model.
The intended pattern is that base game code owns mechanics and validation while
`core` and plugin packs add world content such as items, recipes, planets, and
systems and assets.

Built-in world content lives under `content/packs/core/`, and optional
first-party/plugin packs live beside it under `content/packs/`. The game
currently loads items, recipes, ships, NPC ships, shields, weapons, planets,
stations, systems, stars, and starter inventory from discovered packs.
Inventory, recipes, mining, and UI labels use registry-backed item IDs.

## Development

Useful Cargo commands:

```sh
cargo check
cargo fmt
cargo clippy
```

Build artifacts are generated under `target/`.
