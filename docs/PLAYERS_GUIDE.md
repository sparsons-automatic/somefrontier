# The Player's Guide to Some Frontier

<img src="../assets/branding/some-frontier-logo.png" alt="Some Frontier logo" width="420">

This guide describes the current playable build of Some Frontier. It avoids
future plans unless a feature is already visible in-game.

## Getting Started

Run the game from the project root:

```sh
cargo run
```

For local debugging, `cargo run -- --debug` enables an in-game console. Press
`` ` `` or `F12` during play to open it. Useful commands include
`give <item_id> [count]`, `credits <amount>`, `credits set <amount>`,
`research complete <id|all>`, `recipes unlock all`, and `warp <system_id>`.

The title screen displays the Some Frontier logo and lets you start a new game,
load an existing save, review content packs, adjust settings, or quit. New games
use a world seed; choose one manually or randomize it before starting. Loading a
save restores the seed, ship state, inventory, research, upgrades, selected
destination, equipped modules, production settings, scanned planets, mining
quotas, and content-pack options. The Load Game list can be scrolled when many
saves exist, supports double-click loading, and includes a two-step delete
confirmation for removing the selected save without leaving the menu.

## Controls

| Action | Control |
|---|---|
| Forward thrust | `W` |
| Reverse thrust | `S` |
| Turn | `A` / `D` or left / right arrows |
| Open inventory and production | `Tab` or `E` |
| Open map | `M` |
| Open research | `K` |
| Open content browser | `C` |
| Zoom flight camera | Mouse wheel, `PageUp`, or `PageDown` |
| Inspect nearby planet or station | `Space` |
| Select a visible planet | Left click the planet |
| Set a starmap destination | Left click a planet in the map |
| Open ship upgrades | Click the ship preview in inventory |
| Close menu / pause actions | `Esc` |

On the title screen, use the visible buttons or keyboard shortcuts shown beside
them. `Esc` backs out of submenus or leaves the title flow when available.

## Flight And Navigation

You pilot the Frontier Cargo Ship in local space. Thrust and turning are manual.
The camera follows the ship, and zoom helps inspect nearby planets, stations,
NPC ships, orbit guides, and route context.

The map shows the current system and known destinations. Systems tagged as
starter, surveyed, known, or remote can appear in the known systems panel. Warp
travel charges before switching systems, and non-starter routes can require fuel
canisters. The starter system is always free to return to.

Known system rows include route readiness guidance. If a remote route is not
ready, the row calls out the missing fuel and can point at a local station that
stocks it. If the ship can warp but the route leads to harsher remote space, the
row can recommend scanner preparation before departure.

Planets can be selected from local space or the map. When you are close enough,
the planet panel shows available actions. You can enter orbit from the planet
panel while in range; applying manual thrust or starting warp breaks orbit.

## Surveying

Planets begin unscanned. Before survey, the detail panel hides classification,
resources, hazards, and ownership. Build survey drones from the crafting tab,
fly within interaction range, and launch a drone from the planet panel.

Survey levels reveal more information:

- Surface scan: classification, summary, hazards, and ownership.
- Composition scan: mineable resources.
- Richness scan: resource richness and bonus-yield chances.

Basic survey drones are consumed when launched. Improved survey drones and ship
upgrades can increase scan depth. The drone bay upgrade can add a chance to
recover survey drones.

## Mining

After a planet has composition data, selecting that planet changes the right
side work table into mining rows. Click a mineable resource row to queue one
mining bill. Mining completes one bill at a time from top to bottom.

The work table shows `Item`, `Keep`, `Status`, `%`, and `Active` columns.
`Status` shows current stock against the keep target, `%` shows scanned resource
richness when available, and `Active` shows the currently running mining bill.
The `Keep` column sets an auto-maintained inventory target. Increase or decrease
keep values with left click, right click, or mouse wheel over the keep cell. Hold
`Shift` to adjust by 5 or `Control` to adjust by 10.

Hazard effects are lightweight in the current build. Some planets can drain
shields while the ship is close. Stable orbit mitigates configured shield drain.

## Production

Open inventory with `Tab` or `E`. The production table has three tabs:

- `Smelt`: turn raw ore into plates and other refined materials.
- `Craft`: build components, survey drones, ship modules, and upgrade parts.
- `Process`: make fuel, coolant, reactor components, and related materials.

Click a recipe row to queue one bill. Production consumes ingredients from
inventory and completes one bill at a time. The `Keep` column works like mining:
it keeps producing until the target stock level is reached, as long as inputs
are available. Production rows use the same `Item`, `Keep`, `Status`, `%`, and
`Active` layout as mining; `%` shows progress for the active bill.

Completed research can improve mining, smelting, fabrication, and bonus output
effects.

## Inventory And Cargo

Inventory stores item stacks from mining, production, starter cargo, trading,
and module swaps. Starter cargo includes survey drones and reactor pellets for
the armed starter ship. Items have unit mass in kilograms. The inventory table
shows `Item`, `Qty`, and `Mass` columns for each stack. Total cargo load and
capacity are shown in the ship detail panel below the ship image; cargo mass is
currently informational and does not change flight handling.

Some crafted items are installable modules. Shield and weapon install items can
be swapped through ship detail surfaces when a matching slot exists.

The ship detail panel includes an `Operations` readout for recent meaningful
changes. It records compact feedback for survey results, mined resources,
produced materials, station trades, research progress, installed modules, upgrades,
warp preparation, and remote-system arrival. Repeated messages collapse into
the newest row so routine mining or production stays readable. Planet, station,
and contact panes also mirror the latest relevant operation while selected.
Disabled trade and research actions use short reason labels such as approach
requirements, missing cargo, missing credits, completed research, and out-of-stock
offers.

## Research

Open research with `K`. Research is arranged left to right by tier. Nodes show
whether they are locked, available, affordable, researching, or completed with
module colors, connection traces, tier lanes, and reward details. Click a node
to inspect its cost, duration, requirements, rewards, and summary in the bottom
console, then use the research button to spend credits and start it. Only one
research project can run at a time. The active project completes in the
background while you keep flying, mining, trading, or producing.

Research can unlock production recipes and other progression hooks. Locked
production rows become available once the required research node is completed.

## Ship Upgrades

Open inventory, then click the ship preview to open ship upgrades. Upgrades
consume crafted components and immediately improve ship systems such as thrust,
turning, energy, shields, survey drone behavior, warp charge time, scan depth,
and cargo rating.

For the first remote route, fuel canisters make the jump possible and Scanner
Array level 2 is recommended so improved survey work reaches deeper resource
data after arrival.

Upgrade costs scale by level. If you cannot afford an upgrade, its action is
disabled until the required items are in inventory.

## Shields And Weapons

Ships can mount shield modules through shield slots. Shield modules define
capacity, recharge delay, recharge rate, damage resistance, and hazard
resistance. The ship detail panel shows installed shields, current shield
strength, recharge state, and resistance values.

Ships can also mount weapon modules through weapon slots. The current weapon
type is automatic turret defense. Turrets scan for valid hostile threats in
range, including hostile NPC ships, fire when ready, and spend ship energy.
They ignore neutral, owned, and environmental threats. There is no manual
targeting or fire button in the current build. The Defense rail beside the ship
pane lists configured turret slots, installed turrets, range, damage, energy
cost, cooldown state, and available crafted turrets that can be swapped into a
slot. Ships can support multiple turret slots when their ship content
configuration declares them.
NPC ships can also mount active turrets from their content configuration:
non-hostile patrol ships engage hostile threats, while hostile ships can fire on
the player when in range. The starter Frontier system includes a Redwake probe
that begins PvE pressure with automatic turret fire. Destroyed NPC ships are
removed from local space, and their cargo is automatically moved into your
inventory when the full cargo stack fits within your cargo rating.

Hostile probes in remote space can create light pressure when they get close,
draining shields and causing limited hull spillover if shields are down. The HUD
shows `Redwake probe pressure` while this is happening. Damage resistance
reduces this pressure, and existing shield recharge resumes after the pressure
delay clears.

## Stations And Services

Stations appear as local-space destinations. Fly within dock range and inspect
or select them with `Space` or left click where supported. The station panel
shows name, range, summary, ownership, culture, disposition, and service groups.

Current service groups can include trade stock and research leads. Trade rows
let you buy one unit with left click or sell one unit with right click when the
ship is in range and the offer is available. Research leads point to progression
tree nodes that can expose additional production recipes after completion.

Core stations have distinct practical roles:

- Frontier Exchange: broad starter commerce and common early cargo.
- Ore Lattice Depot: bulk ore stock, freight staging, and future contracts.
- Cinder Repair Yard: repairs, refits, shield and turret parts, and upgrade support.
- Pale Orbit Archive: scan data, route knowledge, and research leads.
- Freebelt Commissary: drones, fuel, coolant, and independent-hauler supplies.
- Ember Watch Array: navigation, route intel, signal logs, and remote warnings.

Some service groups are present as player-facing station context before their
full mechanics exist.

## NPC Ships And Factions

NPC ships appear as moving contacts in local space. They have names, roles,
archetypes, cargo defaults, stats, loadouts, faction ownership, and behavior
modes such as patrol, traffic, trade-route travel, follow, flee, or hostile
intercept. These behaviors make local space feel active while keeping ship
interaction lightweight.

Fly near an NPC ship and press `Space`, or click a visible NPC ship, to inspect
the contact. Nearby contacts can be identified from the interaction panel,
revealing faction, disposition, systems, cargo summary, and supported hooks such
as hail, dock, trade, or conflict. Some hooks are visible before their full
mechanics exist, so unavailable rows describe the current blocker.

Factions and societies own systems, planets, stations, and NPC ships. The UI can
show faction names and default disposition such as friendly, neutral, hostile,
or unknown. Hostile disposition can drive intercept behavior, turret targeting,
and pressure effects when an NPC is tagged for pressure.

## Content Packs

Content packs live under `content/packs/`. The built-in `core` pack provides
the default playable content. Optional first-party packs can add systems,
planets, recipes, resources, and related data when enabled.

Open the title screen Content Packs view to inspect available packs and options
before starting a new game. In gameplay, press `C` to open the content browser.
The content browser lists loaded packs, items, recipes, NPC ships, planets, and
summary counts for other loaded content such as factions. Select a pack to
narrow the visible rows and use the mouse wheel over a column to scroll.

Removing a content pack can leave old saves pointing at missing content. For
stable saves, keep the same pack set enabled.

## Saving And Configuration

The game autosaves progress and also saves when using pause-menu save actions.
`Esc` closes the topmost open overlay first. When no overlay is open, `Esc`
opens the pause dialog with actions such as resume, save now, title menu, and
quit to desktop.

Save files are stored under:

```text
$XDG_CONFIG_HOME/some-frontier/
```

If `XDG_CONFIG_HOME` is not set, the fallback is:

```text
~/.config/some-frontier/
```

Window size is saved in the same configuration area.

## Troubleshooting

- If the game does not start, run it from the repository root so `content/` and
  `assets/` paths resolve correctly.
- If startup reports content validation errors, check recently edited pack files
  for missing IDs, missing textures, invalid references, or unsupported values.
- If a save behaves strangely after pack changes, restore the previous pack set
  or start a new game with the current pack configuration.
- If production rows are unavailable, check that the required ingredients are in
  inventory and that any needed research has been completed.
- If a planet cannot be mined, survey it until composition data is visible and
  keep the planet selected or stay in range.
