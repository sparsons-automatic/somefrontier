# The Player's Guide to Some Frontier

<img src="../assets/branding/some-frontier-logo.png" alt="Some Frontier logo" width="420">

This guide describes the current playable build of Some Frontier. It avoids
future plans unless a feature is already visible in-game.

## Getting Started

Choose both a ship and world seed from the New Game screen. Click a card in the
scrollable ship grid, or use the arrow keys, to browse every player ship supplied
by loaded content packs. Each card summarizes hull strength, shield strength,
and turret-bank count beneath the ship's content-pack image before launch. A
pack may nominate the initially highlighted ship, but your selection is what the
new save records. The chosen hull supplies the ship's flight and durability
stats, art, power and shield modules, turret-bank count, and initially fitted
turrets. Starter inventory is contributed separately by all loaded packs; picking
a hull does not limit starting cargo to that hull's pack. If an optional image
cannot be loaded, its card remains usable and displays a placeholder.

Run the game from the project root:

```sh
cargo run
```

For local debugging, `cargo run -- --debug` enables an in-game console. Press
`` ` `` or `F12` during play to open it. Useful commands include
`give <item_id> [count]`, `credits <amount>`, `credits set <amount>`,
`research complete <id|all>`, `recipes unlock all`, and `warp <system_id>`.
The console is a movable, resizable egui window with scrollable command history;
press `Esc`, use its close button, or press its opening shortcut again to close
it.

The stock-egui title screen displays the Some Frontier logo and lets you start a
new game, load an existing save, review content packs, adjust settings, or quit.
New Game, Load Game, Content Packs, and Settings open as centered stock-egui
windows with native scrolling and controls. New games use a world seed; enter
one in the text field or randomize it before starting. Loading a save restores the seed, ship
state, inventory, research, upgrades, selected destination, equipped modules,
production settings, scanned planets, mining quotas, and content-pack options.
The Load Game list supports double-click loading and includes a two-step delete
confirmation for removing the selected save without leaving the menu.

### Startup and Optional Remote Audio

When a game starts, the transition screen may check for optional remote audio.
It can show the current phase, file name, asset count, and verification
progress. A later start with a verified cache should move through this step
quickly. Remote audio is not required to play: if the network is unavailable or
an asset fails validation, startup continues with local audio and shows an
offline or unavailable status.

Remote audio currently includes menu selection, confirmation, back/close, and
warning sounds, plus rotating laser-fire sounds, shield impacts, hull impacts,
and ship-destruction explosions. The Audio settings panel controls the master
volume; setting it to zero mutes these cues without disabling gameplay.

## Controls

| Action | Control |
|---|---|
| Forward thrust | `W` |
| Reverse thrust | `S` |
| Turn | `A` / `D` or left / right arrows |
| Open/close inventory and production | `Tab` or `E` |
| Open map | `M` |
| Open research | `K` |
| Open content browser | `C` |
| Open contracts | `J` |
| Open fleet management | `F` (`F` cycles filters while the map is open) |
| Zoom flight camera | Mouse wheel, `PageUp`, or `PageDown` |
| Inspect nearby planet or station | `Space` |
| Select a visible planet | Left click the planet; press `Space` to inspect |
| Open object actions | Right click a visible planet, station, or ship |
| Select a starmap destination | Left click a planet in the map |
| Fly or build an itinerary | Use the Destinations window after selecting a planet |
| Open ship upgrades | Choose `Inspect ship upgrades` in the Ship Pane |
| Close menu / pause actions | `Esc` |
| Reset movable menu positions | Pause with `Esc`, then choose `Reset Menus` or press `R` |

`Tab` and `E` close an active planet, station, or contact pane without changing
it into the Ship Pane. If a right-click Actions menu is open, they close that
menu first. With no object pane active, they continue to toggle the general
inventory and production pane.

Every movable egui window has a `Pin` toggle. A pinned window remains visible
and fully interactive when its parent workspace closes: for example, pin
Production and press `Tab` to return to flight with only Production left open.
Pins are remembered across launches and are independent of save slots. Pinning
an inspection window also preserves its selected planet, station, or contact
when `Tab` closes the surrounding inventory workspace.

Movable egui window positions, sizes, collapsed sections, and scroll state are
remembered across launches for the player, independently of save slots. If a
display or resolution change leaves a window unreachable, the game discards the
invalid layout automatically. `Reset Menus` in the pause menu restores every
movable egui window to its default geometry and clears all pins at any time.

On the title screen, use the visible buttons or keyboard shortcuts shown beside
them. `Esc` backs out of submenus or leaves the title flow when available.

## Flight And Navigation

You pilot the Frontier Cargo Ship in local space. Thrust and turning are manual.
The camera follows the ship, and zoom helps inspect nearby planets, stations,
NPC ships, orbit guides, and route context.

Ships build speed gradually and coast down through maneuvering drag rather than
hitting a hard speed cap. The Frontier Cargo Ship favors predictable hauling
control and braking over raw speed; the lighter Twinspire Gunship accelerates,
turns, and brakes more sharply. Engine and maneuver-thruster upgrades preserve
those differences. Approach and follow autopilot use each hull's authored
braking authority, while manual thrust or turning remains an immediate override.

The map shows the current system and known destinations in a compact Macroquad
3D panel. Opening it also opens a movable stock-egui Destinations window.
Drag the map panel's header to move it, or its lower-right corner to resize it.
Its 3D camera tracks the panel's aspect ratio so planets remain spherical at
wide, tall, and compact sizes. The chosen position and size are remembered
across launches, kept within the visible display, and restored to their defaults
by `Reset Menus`.
Clicking a planet selects it without changing course; choose `Fly Now` to begin
a physical autopilot approach or `Add Stop` to append it to the itinerary.
Numbered planet markers show itinerary order on the map. Systems tagged as
starter, surveyed, known, or remote can appear in the known systems panel. Warp
travel charges before switching systems, and non-starter routes can require fuel
canisters. The starter system is always free to return to.

`Activate Route` approaches every itinerary stop in order and advances after the
ship reaches each planet's interaction range. Manual thrust or turning pauses
the route without discarding its stops; use `Resume Route` when ready. Completed
stops remain marked until the itinerary is cleared. Routes store stable system
and planet content IDs in the world save, so content-pack destinations survive
save/load while their packs remain installed. The current itinerary supports
planets in the active system; a stop from another system pauses with an explicit
message instead of initiating warp automatically.

Pin the Destinations window before closing the map to keep route progress and
controls visible during flight. Its position, size, collapsed state, and pin
participate in the same persistence and `Reset Menus` behavior as other egui
windows.

Known system rows include route readiness guidance. If a remote route is not
ready, the row calls out the missing fuel and can point at a local station that
stocks it. If the ship can warp but the route leads to harsher remote space, the
row can recommend scanner preparation before departure.

Planets can be selected from local space or the map without immediately changing
course. The map selection populates Destinations; local-space inspection opens
the planet record.
Inspecting a planet opens its survey record even when it is distant; the record
shows what remains unknown and what scan equipment is required. Launch scans
only when close enough to the planet. Choose Orbit from the planet panel while
in range to autopilot to the nearest safe orbital position; orbit locks only
after the ship flies there. Applying manual thrust, choosing another destination,
or starting warp cancels orbital entry or breaks an established orbit.

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

Inspecting a planet opens a movable Planet window alongside Production and
Inventory. After the planet has composition data, its Mining section shows one
row per mineable resource. Click a resource to queue one mining bill. Mining
completes one bill at a time from top to bottom.

The work table shows `Item`, `Keep`, `Status`, `%`, and `Active` columns.
`Status` shows current stock against the keep target, `%` shows scanned resource
richness when available, and `Active` shows the currently running mining bill.
The `Keep` column sets an auto-maintained inventory target. Click its stock egui
`-` and `+` buttons to adjust by one, hold `Shift` while clicking to adjust by
10, or hold `Control` while clicking to adjust by 20.

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
`Active` layout as mining; `%` shows progress for the active bill. Click the
small `-` and `+` chips to change Keep by one, hold `Shift` while clicking for
steps of 10, or hold `Control` while clicking for steps of 20. The up and down
arrow keys select a production row, and `Enter` queues the selected unlocked
recipe.

Ship, Production, and Inventory open as separate windows over the game world.
Drag their title bars to arrange them, resize from a window edge or corner, and
use the title-bar arrow to collapse a window. Each window scrolls independently,
and hovering clipped text reveals its full content-pack-provided name.

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
be swapped through the labeled Ship Pane dropdowns when a matching slot exists.
Each dropdown lists compatible equipment currently installed or held in cargo.

## Fleet And Owned Ships

Press `F` during local flight to open the movable, resizable stock egui Fleet
workspace. Its position, size, and scroll state are remembered with the other
egui menus and can be restored with `Reset Menus`. Each owned ship keeps its own
hull and shield condition, energy, position, cargo, upgrades, power
modules, shields, and turret-bank loadout. Select a fleet row and choose `Switch
control` to make that vessel active; switching to a vessel in another system
moves the local-space view to its saved location. Destroyed ships and hulls from
currently unavailable content packs cannot be activated.

Station garages act as shipyards for hulls whose content definition includes a
purchase price. A purchased ship begins at the garage berth with full default
systems, its configured equipment, no upgrades, and an empty cargo hold. Starter
inventory is granted only to the first new-game ship and is never duplicated by
ship purchases.

Select a different owned ship in Fleet management to transfer cargo. Both ships
must be in the same system and within 180 units. Transfer amounts respect the
receiving ship's cargo mass and inventory-slot capacity. A normal quantity click
changes the transfer amount by 1, `Shift` changes it by 10, and `Ctrl` changes it
by 20. Inactive owned ships
remain at their saved positions, but their reactors and shields continue to
recharge and their installed turrets automatically engage hostile targets in the
same system. They consume energy and ammunition from that ship's own stores,
and hostile ships can target and damage them. Simultaneous fleet movement and
formation following are separate future behaviors.

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

Open the movable stock-egui Research window with `K`. Research modules are
grouped into one column per authored tier and ordered by their content-pack grid
position. The tier columns are generated from loaded data, so packs can add
higher tiers without an engine UI change; scroll horizontally to reach them.
Modules show whether they are locked, available, affordable, researching, or
completed. Select one to inspect its cost, duration, requirements, rewards,
summary, and live progress in the fixed details column on the right; horizontal
scrolling moves only the research tiers. Then use `Start research` to
spend credits and begin it. Pin the window to keep it visible after closing the
Research workspace. Only one project can run at a time, and it completes in the
background while you keep flying, mining, trading, or producing.

Research can unlock production recipes and other progression hooks. Locked
production rows become available once the required research node is completed.

## Ship Upgrades

Open inventory, then choose `Inspect ship upgrades` in the Ship Pane. The
movable stock-egui window lists every upgrade's current level, effect, next
component cost, and the amount currently held in cargo. Upgrades consume those
crafted components and immediately improve ship systems such as thrust, turning,
energy, shields, survey drone behavior, warp charge time, scan depth, and cargo
rating. Pin the window to keep it open while returning to other workspaces.

For the first remote route, fuel canisters make the jump possible and Scanner
Array level 2 is recommended so improved survey work reaches deeper resource
data after arrival.

Upgrade costs scale by level. If you cannot afford an upgrade, its action is
disabled until the required items are in inventory. Shift and Control retain
the existing upgrade modifiers, buying up to 5 or 10 levels respectively while
components remain available.

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
pane lists each `Turret Bank`, its installed turret, range, damage, energy cost,
and cooldown state. Open a bank's `Turret List` dropdown to see the turret types
currently in inventory and their owned counts, then select the exact turret to
install. The outgoing turret returns to inventory. Ships support one
independently configurable bank for every weapon slot in their content
definition. With the Turrets Galore pack loaded, new games start aboard its
Twinspire Gunship by default. You can still choose another hull. If selected,
the Twinspire has two banks initially fitted with an Ember Lance and Sentinel
Flak turret.
NPC ships can also mount active turrets from their content configuration:
non-hostile patrol ships engage hostile threats, while hostile ships can fire on
the player when in range. The starter Frontier system includes a Redwake probe
that begins PvE pressure with automatic turret fire. Destroyed NPC ships are
removed from local space, and their cargo is automatically moved into your
inventory when the full cargo stack fits within your cargo rating.

Turret projectiles apply damage when they visually reach their target. Weapons
may strike one target, chain through nearby targets, damage a blast radius, or
combine chaining and splash damage. The Turrets Galore reference pack includes
a Storm Chain turret and a slow, wide-radius SUPER NUKE turret. Both default to
hostile-only damage, so nearby friendly and neutral ships are protected. Each
Turrets Galore weapon has its own visible projectile sprite that turns along
the weapon's flight path before impact.

Physical turrets consume ammunition from ship inventory when they successfully
fire. Their turret-bank status shows the remaining ammo count and changes to
`out of ammo` without spending energy when none remains. Energy weapons such as
the Ember Lance and Storm Chain need no ammunition. Ammunition can be replenished
at the crafting station:

- 20 interceptor rounds use an iron plate, two copper wires, and electronics.
- 30 Sentinel flak canisters use an iron plate, a gear, and sulfur.
- Two SUPER NUKE warheads use structural alloy, a guidance circuit, uranium,
  and sulfur.

New ships begin with 120 interceptor rounds, 150 flak canisters, and four nuke
warheads. NPC ships draw ammunition from their own persistent cargo manifests.

Hostile probes in remote space can create light pressure when they get close,
draining shields and causing limited hull spillover if shields are down. The HUD
shows `Redwake probe pressure` while this is happening. Damage resistance
reduces this pressure, and existing shield recharge resumes after the pressure
delay clears.

### Collision And Safe Flight

Ships can damage one another when they overlap while closing at meaningful
speed. Impact damage scales with closing speed and ship mass. Shields absorb
collision damage first, then any remaining damage reaches the hull. Equipped
shield modules provide their normal damage resistance and recharge delay to
collision impacts, and NPCs without shields take the damage directly against
their hull.

Low-speed contact is ignored, and repeated contact while ships remain overlapped
is limited by a short collision cooldown. Approach and follow movement should
therefore be performed at low speed near a contact. High-speed ramming can
destroy a ship quickly.

When a collision causes damage, a short-lived impact ring and flash appear at
the contact point. The Operations feed reports whether shields absorbed the
impact, hull damage was taken, or a ship was destroyed. Critical and destructive
impacts also appear as a warning in the HUD. Minor contact or a collision fully
absorbed by resistance may not create a damage event. These feedback effects do
not change the available contextual actions or object panes.

## Stations And Services

Stations appear as local-space destinations. Fly within dock range and inspect
or select them with `Space` or left click where supported. Inspection opens a
movable, resizable stock egui Station window beside the independent Production
and Inventory windows. It shows name, range, summary, ownership, culture,
disposition, and service groups; egui remembers window placement across launches.
Right-click a station for contextual Inspect, Approach, Dock, Trade, Contracts,
and available service actions such as Repair. Approach engages the ship's
autopilot until you reach the station's interaction range. The ship brakes as
it approaches and stops at a center-relative safe clearance comfortably inside
the interaction ring rather than overlapping the station or balancing on the
outer boundary. Planet approaches use the same rule. Manual thrust or turning
cancels the approach. Actions show their
range, reputation, or availability requirements before they can be used; a
station without a repair service does not advertise Repair.

Current service groups can include trade stock and research leads. Market rows
provide explicit Buy and Sell buttons for one-unit transactions when the ship is
in range and the offer is available. Limited stock is replenished using
the game's elapsed time; the trade row shows the current stock and remaining
time until its next restock. Selling cargo back to a station restores stock up
to that offer's configured capacity. Research leads point to progression-tree
nodes that can expose additional production recipes after completion.

Resize or scroll the Station window when a station has a large catalog or long
contract descriptions. Services remain accessible as native selectable buttons.

Garage services provide a Maintenance action. When docked and carrying enough
credits, repair restores damaged hull and shields; the price is based on the
amount of damage. Freight Lock services provide hauling contracts: accept one,
carry the requested cargo to its target station, and return to the contract
desk to collect the reward. Archive scan desks provide survey contracts: scan
the named planet, then return to the archive. The station panel shows contract
progress, completion, expiry, and rewards. Up to three contracts can be active,
and active contracts survive save/load.

Press `J` at any time to open the movable stock-egui Contracts window. Select an
active contract on the left to inspect its objective, progress, origin,
destination, deadline, reward, and current status. Use `Focus origin station`
or double-click the contract when its origin is in the current system; remote
contracts remain visible with their origin system explained. Pin the window to
keep it visible after closing the Contracts workspace.

Faction reputation is separate from station ownership, culture, and default
disposition. Contract completion increases the reputation of the sponsoring
faction, and repairing your ship can improve the owning faction's standing.
Some vendors, services, and contracts require a minimum reputation; a locked
entry explains the requirement in the station panel. Vendor reputation can also
change buy and sell prices, so better standing may make a faction's goods more
favorable. Reputation is saved with the rest of the game and starts from each
faction's content-defined value.

Some shops have named vendors. Their specialties appear below the service name,
and their available catalog, prices, and stock can change when the vendor's
rotation period advances. Catalogs are deterministic for a save's world seed,
so loading the same save preserves the same rotation until the next period.

Core stations have distinct practical roles:

- Frontier Exchange: broad starter commerce and common early cargo.
- Ore Lattice Depot: bulk ore stock, freight staging, and hauling contracts.
- Cinder Repair Yard: hull and shield repairs, refits, parts, and upgrade support.
- Pale Orbit Archive: scan data, survey contracts, route knowledge, and research leads.
- Freebelt Commissary: drones, fuel, coolant, and independent-hauler supplies.
- Ember Watch Array: navigation, route intel, signal logs, and remote warnings.

Some other service groups remain player-facing station context before their
full mechanics exist.

## NPC Ships And Factions

Planets, stations, and NPC ships can be selected with left click or `Space`,
then right-clicked for an object-specific stock menu. Disabled actions use
egui's muted styling; hover them to see the current range, equipment, survey, or
service requirement. When interaction rings overlap, the object nearest the
cursor is selected regardless of type. If two targets are nearly tied, choose
the intended object from the short target list before its actions appear. Click
elsewhere to dismiss the menu. Planet menus include
Inspect and Approach, plus Orbit, Scan, and Mine when their range and survey
requirements are met. NPC ships appear as moving contacts in local space. They
have names, roles,
archetypes, cargo defaults, stats, loadouts, faction ownership, and behavior
modes such as patrol, traffic, trade-route travel, follow, flee, or hostile
intercept. These behaviors make local space feel active while keeping ship
interaction lightweight.

Fly near an NPC ship and press `Space`, or click a visible NPC ship, to inspect
the contact in a movable, resizable stock egui Contact window. Nearby contacts
can be identified from its interaction table,
revealing faction, disposition, systems, cargo summary, and supported hooks such
as hail, dock, trade, or conflict. Some hooks are visible before their full
mechanics exist, so unavailable rows describe the current blocker. Identified
non-hostile haulers with cargo expose a mobile Cargo market: open it from the
Trade row, choose a quantity with the `−` and `+` controls, then buy or sell
that many units. The quantity controls use the current modifier step: a normal
click changes it by 1, `Shift` changes it by 10, and `Ctrl` changes it by 20.
Hail a hauler first for a short price or supply rumor. Their remaining cargo is
saved with the game, so trades survive a save and reload. Right-click a visible
NPC ship for its contextual
Actions menu. Follow matches your ship's course and pace while keeping you near
the contact while maintaining a safe trailing distance; Approach engages the
autopilot until you reach interaction range and brakes before arrival. Manual
thrust, turning, a system change, or target destruction cancels following or
approach. Hail and Trade are also available there when their range and
identification requirements are met.

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
The movable stock egui Content Browser uses a three-pane layout. Select a loaded
pack from the vertical list on the left, choose any authored object by category
in the middle, and inspect its fields in the right pane. References such as a
weapon's install item or ammunition are clickable, and `Back` returns through
the inspection history. Select `All packs` for the aggregate registry summary.
Pin the browser to keep it visible and interactive after pressing `C` again.

Removing a content pack can leave old saves pointing at missing content. For
stable saves, keep the same pack set enabled.

## Saving And Configuration

The game autosaves progress and also saves when using pause-menu save actions.
`Esc` closes the topmost open overlay first. When no overlay is open, `Esc`
opens the pause dialog with actions such as resume, save now, title menu, and
quit to desktop. The pause dialog can be moved like the other egui windows and
remembers its position; `Reset Menus` restores its centered default.

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
- If startup reports that remote audio is unavailable, check the network if you
  want to refresh it; otherwise continue offline. A checksum or compatibility
  failure is rejected automatically and does not invalidate your save.
- A warm cache should make the remote-audio startup step quick. If the step is
  slow or unavailable, the game can still continue without remote sounds.
- If a save behaves strangely after pack changes, restore the previous pack set
  or start a new game with the current pack configuration.
- If production rows are unavailable, check that the required ingredients are in
  inventory and that any needed research has been completed.
- If a planet cannot be mined, survey it until composition data is visible and
  keep the planet selected or stay in range.
