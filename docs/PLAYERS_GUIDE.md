# The Player's Guide to Some Frontier

<img src="../assets/branding/some-frontier-logo.png" alt="Some Frontier logo" width="420">

This guide describes the current playable build of Some Frontier. It avoids
future plans unless a feature is already visible in-game.

## Getting Started

Choose both a ship and world seed from the New Game screen. Click a card in the
scrollable ship grid, or use the arrow keys, to browse every player ship supplied
by loaded content packs. Each card summarizes hull strength, shield strength,
cargo capacity, mass, thrust, turn response, and turret-bank count beneath the
ship's content-pack image before launch. A
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
| Open/close production | `P` |
| Open map | `M` |
| Open research | `K` |
| Open content browser | `C` |
| Open contracts | `J` |
| Open fleet management | `F` (`F` cycles filters while the map is open) |
| Open Operations Log | `L` |
| Zoom flight camera | Mouse wheel, `PageUp`, or `PageDown` |
| Inspect nearby planet or station | `Space` |
| Select a visible planet | Left click the planet; press `Space` to inspect |
| Open object actions | Right click a visible planet, station, or ship |
| Select a starmap destination | Left click a planet or station in the map |
| Fly or build an itinerary | Use the Destinations window after selecting a planet or station |
| Open ship upgrades | Fleet → `Upgrades` |
| Close menu / pause actions | `Esc` |
| Reset movable menu positions | Pause with `Esc`, then choose `Reset Menus` or press `R` |

Ship stats and cargo inventory now live in Fleet rather than separate Ship and
Inventory windows. `P` opens the active hull directly on Fleet → `Production`
without closing an active planet, station, or contact pane. Close inspection
windows through their native window controls or `Esc`.

Every movable egui window has a `Pin` toggle. A pinned window remains visible
and fully interactive when its parent workspace closes.
Pins are remembered across launches and are independent of save slots. Pinning
an inspection window preserves its selected planet, station, or contact when
other parent UI state closes.

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
turns, and brakes more sharply. The large Frontier Longhauler trades still more
speed and turn response for a 90-tonne hold, a durable hull and shields, and two
turret banks. Engine and maneuver-thruster upgrades preserve
those differences. Approach and follow autopilot use each hull's authored
braking authority, while manual thrust or turning remains an immediate override.

The map shows the current system and known destinations in a compact Macroquad
3D panel. Opening it also opens a movable stock-egui Destinations window.
Drag the map panel's header to move it, or its lower-right corner to resize it.
Its 3D camera tracks the panel's aspect ratio so planets remain spherical at
wide, tall, and compact sizes. The chosen position and size are remembered
across launches, kept within the visible display, and restored to their defaults
by `Reset Menus`. The map is anchored to the active system: ordinary ship
acceleration, coasting, and turning move only the ship marker, while planets
remain stable except for their deliberately authored orbital motion.
Stations, live NPC contacts, and inactive owned ships also appear at their
current local-space positions. Squares mark stations, triangles mark NPCs, and
outlined circles mark owned ships. Unscanned NPCs remain `Unknown contact`
until their identity scan completes; hostile coloring likewise does not reveal
disposition early. Station and NPC markers can be clicked to select them, while
the `F` and `R` filters continue to apply specifically to planet markers.
Clicking a planet selects it without changing course; choose `Fly Now` to begin
a physical autopilot approach or `Add Stop` to append it to the itinerary.
Clicking a station also populates Destinations and provides `Fly Now`; stations
cannot currently be added as itinerary stops.
Numbered planet markers show itinerary order on the map. Systems tagged as
starter, surveyed, known, or remote can appear in the known systems panel. Warp
travel requires Warp Drive level 1 on the currently controlled ship and charges
before switching systems; upgrading another owned ship does not unlock the
active one. Non-starter routes can also require fuel canisters. The starter
system has no fuel cost but still requires an installed warp drive.

`Activate Route` approaches every itinerary stop in order and advances after the
ship reaches each planet's interaction range. Manual thrust or turning pauses
the route without discarding its stops; use `Resume Route` when ready. Completed
stops remain marked until the itinerary is cleared. Routes store stable system
and planet content IDs in the world save, so content-pack destinations survive
save/load while their packs remain installed. The current itinerary supports
planets in the active system; a stop from another system pauses with an explicit
message instead of initiating warp automatically.

Research `Automated Extraction Routes` to unlock mining holds in Destinations.
Enable `Mining holds`, then configure ore `Queue` or `Keep` work in a planet's
Mining panel before activating the itinerary. Pending stops snapshot that work
as ore quotas when the route starts. On arrival, the ship remains inside
interaction range, spends ship energy, and mines each quota before approaching
the next stop. Destinations shows traveling, mining, completed, blocked,
skipped, and paused states together with quota progress and any hold reason;
the Operations Log records the same transitions.

A full cargo hold or a ship that cannot supply mining energy blocks the route
without losing its remaining quotas. Free cargo or restore power, then choose
`Resume Route`. Missing destination/resource content, an impossible zero quota,
or leaving interaction range skips that stop so the itinerary cannot wait
silently forever. Manual thrust pauses an active mining hold, and `Clear`
cancels the itinerary and its captured quotas. Save/load preserves stop state,
quota progress, and stable content-pack system, planet, and resource IDs.

Pin the Destinations window before closing the map to keep route progress and
controls visible during flight. Its position, size, collapsed state, and pin
participate in the same persistence and `Reset Menus` behavior as other egui
windows.

To verify starmap stability, open the map while stopped and watch a few planet
markers, then accelerate, coast, and turn with the map still open. Repeat after
closing and reopening the map, and after panning, zooming, resizing the map, and
pinning Destinations. Planet markers should never bob or drift with ship motion;
only the ship marker and slow authored planet orbits should move.

Known system rows include route readiness guidance. If the active ship lacks the
upgrade, the row reports `Warp drive required` before checking fuel. Otherwise,
it calls out missing fuel and can point at a local station that stocks it. If
the ship can warp but the route leads to harsher remote space, the row can
recommend scanner preparation before departure.

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

Inspecting a planet opens a movable Planet window alongside any open Fleet
panel. After the planet has composition data, its Mining section shows one
row per mineable resource. Click a resource to queue one mining bill. Mining
completes one bill at a time from top to bottom.

The work table shows `Item`, `Keep`, `Status`, `%`, and `Active` columns.
`Status` shows current stock against the keep target, `%` shows scanned resource
richness when available, and `Active` shows the currently running mining bill.
The `Keep` column sets an auto-maintained inventory target. Click its stock egui
`-` and `+` buttons to adjust by one, hold `Shift` while clicking to adjust by
10, or hold `Control` while clicking to adjust by 20.

Hazard effects are lightweight in the current build. Some planets can drain
shields while the ship is close. Stars with a configured hazard zone damage
shields first and then the hull; damage increases smoothly as a ship approaches
the star, and equipped hazard-resistant shields reduce it. Stable orbit
mitigates configured planet shield drain, but does not protect against stellar
hazards.

Purchased mining-drone hulls can perform this work without becoming the active
ship. Open Fleet and select an inactive mining drone. Choose a composition-
scanned planet, one of its revealed resources, and a quota, then select `Add
route leg`. Repeat this for each planet-to-planet stop; the numbered Mining Route
preserves that order and lets individual legs be removed or the route cleared.
Each leg owns its planet, ore, quota, and progress. Check `Auto-transfer
inventory to a fleet ship` to deliver after the route, or leave it unchecked to
hold at the final target.

The drone flies to each planet in order and fulfills that leg's quota before
continuing to the next. If its hold fills early, it stops the remaining route
and begins delivery with the cargo already collected. Cargo delivery requires a
same-system fleet ship selected from the `Deliver to` list. The drone rendezvous
with that ship and automatically transfers its entire inventory, including
cargo that was aboard before the job. The recipient accepts as much as its cargo
capacity permits; any remainder stays safely aboard the drone and blocks the
job for attention. The recipient can be active or inactive, and the drone
follows its current position while delivering. Jobs report the active leg and
progress in Fleet and the
Operations Log and can be paused, resumed, recalled, or cancelled.

Autonomous jobs are deliberately local-system operations. A destination that is
missing, out of the hull's operating range, no longer provides the selected
resource, or cannot accept more cargo leaves a visible blocked job instead of
silently losing it. Delivery also blocks with the cargo aboard if its recipient
is destroyed, unavailable, leaves the system, or runs out of capacity. Recall
remains an emergency return to the job's original dispatch point. A damaged
working drone interrupts and recalls; a destroyed drone records the job as
destroyed. Taking direct control pauses an active job, which can be resumed from
Fleet after control is transferred elsewhere.

The optional `Drone smelter` upgrade unlocks an onboard smelting queue in the
drone's Fleet details. Queued recipes run while the drone is autonomous; queue
state and progress persist with the individual hull. Processing pauses while
the drone is under direct control. When fleet inventory sharing is enabled, its
recipes may draw missing inputs from nearby linked ships, but output always
stays in the drone's own cargo. This upgrade is separate from `Ore processor`,
which continues to improve bonus mining yield.

## Production

Open Production directly with `P`, or choose `Production` from the Fleet control
panel's left rail. The active hull's production table has three tabs:

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

Production now lives inside the movable Fleet control panel rather than opening
a second production window. Resize or collapse Fleet normally; the production
table scrolls inside its page. With Fleet → Production or an inspection pane
open, the mouse wheel continues to zoom the flight view while the pointer is over
open space; hovering the recipe table routes the wheel to that table instead.

Completed research can improve mining, smelting, fabrication, and bonus output
effects.

## Inventory And Cargo

Inventory stores item stacks from mining, production, starter cargo, trading,
and module swaps. Starter cargo includes survey drones and reactor pellets for
the armed starter ship. Items have unit mass in kilograms. The inventory table
shows `Item`, `Qty`, and `Mass` columns for each stack. Total cargo load and
capacity are shown in the ship detail panel below the ship image. Each hull has
its own base capacity, and every Cargo Hold upgrade adds 10 tonnes to that base.
Cargo mass is currently informational and does not change flight handling.

Some crafted items are installable modules. Shield and weapon install items can
be swapped through the labeled Ship Pane dropdowns when a matching slot exists.
Each dropdown lists compatible equipment currently installed or held in cargo.

## Fleet And Owned Ships

Press `F` during local flight to open the movable, resizable stock egui Fleet
workspace. Its position, size, and scroll state are remembered with the other
egui menus and can be restored with `Reset Menus`. Each owned ship keeps its own
hull and shield condition, energy, position, cargo, upgrades, power
modules, shields, and turret-bank loadout. Click a fleet row to inspect that
ship's inventory and details, or double-click an available row to switch control
immediately. The `Switch control` button remains available as an explicit
alternative; switching to a vessel in another system
moves the local-space view to its saved location. Destroyed ships and hulls from
currently unavailable content packs cannot be activated.

The Hull Showroom service at Cinder Repair Yard sells hulls whose content
definition includes a purchase price. Select it beside Repair Gantry, Parts
Cage, and Torchcut Bar in the Station window. A purchased ship begins at the yard berth with full default
systems, its configured equipment, no upgrades, and an empty cargo hold. Starter
inventory is granted only to the first new-game ship and is never duplicated by
ship purchases. The shipyard lists cargo, mass, thrust, and turn statistics next
to each offer, plus throughput, operating range, and maximum quota for mining
drones. The 7,000-credit Prospector is quicker and inexpensive but has an
8-tonne hold, ×0.8 mining throughput, 6,500-unit range, and 50-unit job cap. The
19,000-credit Industrial Mining Drone trades agility for a 30-tonne hold, ×1.5
throughput, 16,000-unit range, 200-unit cap, and a defensive turret. The
Frontier Longhauler costs 32,000 credits; its size and slow
handling demand more clearance while approaching, docking, and flying in a
fleet, but its 90-tonne hold and durable systems suit bulk trade.

Mining drones use distinctive circular yellow-and-black industrial hulls in
local space, Fleet previews, and the Hull Showroom. Their radial silhouettes
separate autonomous work platforms from conventional pilotable ships at a
glance; the larger Industrial drone carries a visibly broader extraction frame.
Click an owned drone in local space to open its Fleet inventory and details;
double-click it to switch control directly.

Fleet Inventory presents one shared manifest for the selected hull and every
opted-in hull within 180 units in the same system. Each item appears once with
its combined quantity and mass; hover its carrier count to see where the stacks
physically reside. Search accepts item names, categories, and hull callsigns.
`Consolidate into active hold` moves as much linked cargo as will fit onto the
controlled ship while leaving capacity-limited overflow aboard its carrier.

Press `F` to open the wider Fleet control panel. Owned ships and controllable
drones appear as hull tabs across the top; selecting one keeps that asset in
focus while the left rail switches among `Ship stats`, `Upgrades`, `Inventory`,
and `Production`. Opening Fleet preserves any existing inspection,
research, content, contract, destination, and operations windows; it can be
moved or resized alongside them. Ship stats is the default page and includes control switching,
fleet sharing, installed power details, Auto Repair, shield and weapon loadout
controls, and autonomous mining controls where applicable. Upgrades uses a
selectable module matrix with installed levels, effects, next-level material
requirements, and affordability, following the same browser/detail pattern as
the Research window. Purchases apply to the selected hull, including inactive
ships and drones. Inventory combines linked holds into one searchable manifest,
shows distributed mass and capacity, preserves per-hull provenance, and lists
remote or sharing-disabled hulls under `Unavailable cargo`.
Production exposes the complete Smelt/Craft/Process workspace for the active
hull. Its currently running recipe uses green text, a `▶` marker, and a subtle
row tint. Hover a recipe name or its Status cell for a compact activity line
explaining whether it is active, waiting behind another recipe, missing
materials, blocked by cargo capacity, locked, or simply not queued. Selecting
an inactive mining drone instead exposes its installed onboard smelter with the
same active-work highlight; other inactive ships explain that direct control is
required.

After a destroyed drone's cargo is empty, right-click its wreck in local space
and choose `Salvage`. Dismantling takes 150 seconds while the active ship remains
within the same 180-unit range. Leaving range pauses the operation without
losing progress; return and choose `Resume salvage` to continue. The saved game
retains partial progress. Completion recovers a mass-scaled cache of iron and
copper plates, then removes the wreck from local space and Fleet. If the active
ship cannot hold the complete salvage yield, completion waits without deleting
the wreck or losing materials.

Disabled NPC ships now remain in local space as recoverable wrecks instead of
automatically transferring cargo and disappearing. Any hostile bounty is paid
once when the ship is disabled. Right-click the wreck and choose `Approach
wreck`, then `Loot wreck` to transfer everything the active cargo hold can
accept; capacity-limited cargo remains aboard for a later trip. Once the wreck's
hold is empty, choose `Salvage wreck`. NPC wreck salvage uses the same 150-second,
range-limited, resumable process and mass-scaled iron/copper recovery as owned
drone wrecks. Wreck position, remaining cargo, partial salvage progress, and
completed removal persist through save/load.

Each ship also has a `Share inventory with nearby fleet` checkbox in its Fleet
details, enabled by default. Two opted-in ships in the same system and within
the 180-unit transfer range form a linked cargo pool. Smelting, crafting,
processing, ship upgrades, and Auto Repair consume the acting ship's cargo
first, then draw missing materials from linked ships. Costs are atomic: if the
linked fleet cannot cover the whole bill, nothing is consumed. Produced items
remain on the ship doing the work, and production pauses before consuming inputs
when that ship lacks local cargo capacity. Turret ammunition, mining quotas,
and each hull's cargo-capacity limit remain strictly local. Moving out of range
or disabling either ship's checkbox immediately unlinks their cargo.

Inactive owned ships without an active autonomous job remain at their
saved positions, but their reactors and shields continue to
recharge and their installed turrets automatically engage hostile targets in the
same system. They consume energy and ammunition from that ship's own stores,
and hostile ships can target and damage them. Simultaneous fleet movement and
formation following are separate future behaviors.

The ship detail panel includes an `Operations` readout for recent meaningful
changes. Press `L` to open the full, movable Operations Log. It keeps up to 200
events from the current play session, with an elapsed-world-time marker, a
text-marked and color-coded category, a severity marker, and the original event
source. Use the `All`, mining, production, travel, trade, combat, research,
system, and general tabs to select the kind of history you want to inspect.
Severity filters cover info, success, warning, and error within the active tab.
`Reset filters` returns to `All` with every severity visible. `Clear history`
removes the current session's entries.

Each tab keeps its own scroll position and follows new events automatically
while you are viewing its bottom. If you scroll upward to review older events,
new entries do not pull the view away; scroll back to the bottom to resume
following. Repeated messages collapse into the newest row so routine mining or
production stays readable. Planet, station, and contact panes continue to mirror
the latest relevant operation while selected, so opening the log does not
replace HUD warnings or contextual feedback.

Operations history is deliberately session-only and is not written to save
files. Loading a save starts an empty log while leaving all gameplay state
unchanged. This bounds both runtime memory and save size; the 200-entry runtime
limit discards the oldest entries first.

Disabled trade and research actions use short reason labels such as approach
requirements, missing cargo, missing credits, completed research, and
out-of-stock offers.

## Research

Open the movable stock-egui Research window with `K`. Research modules are
placed on a scrollable tree from their content-pack grid positions. Arrows run
from prerequisites to the modules they unlock. Selecting a module brightens its
incoming and outgoing connections, while the fixed details pane lists both its
requirements and the modules it leads to. Scroll the tree horizontally or
vertically without moving the details pane.

Every card uses text and borders as well as color: `DONE` modules are light
green, `ACTIVE` marks the current research, `READY` means affordable, `FUNDS`
means its prerequisites are complete but more credits are needed, and `LOCKED`
uses a restrained red treatment. Hover a card for the state explanation and
authored grid position. Then use `Start research` to spend credits and begin an
affordable module. Pin the window to keep it visible after closing the Research
workspace. Only one project can run at a time, and it completes in the
background while you keep flying, mining, trading, or producing.

Research can unlock production recipes and other progression hooks. Locked
production rows become available once the required research node is completed.

`Autonomous Hull Patching` branches from Yield Optimization. After completing
it, open the Ship window and enable Auto Repair for the currently controlled
ship. Enabled inactive ships also continue repairing their own hulls in
two-second cycles; the system never restores shields. Iron ore
restores 5 hull per unit and titanium ore restores 20 hull per unit. The system
chooses the largest unit it can use without waste, falling back to the smallest
available unit for minor damage. Any shield or hull damage resets the cycle and
pauses repairs for eight seconds, so sustained combat cannot repair through
incoming fire. Missing materials also pause progress. An opted-in ship may draw
repair ore from nearby linked fleet cargo. Auto Repair consumes no drive energy
or weapon capacitor, and each owned ship remembers its own setting, sharing
preference, and progress through control switching and save/load.

## Ship Upgrades

Open inventory, then choose `Inspect ship upgrades` in the Ship Pane. The
movable stock-egui window lists every upgrade's current level, effect, next
component cost, and the amount currently held in cargo. Upgrades consume those
crafted components and immediately improve ship systems such as thrust, turning,
energy, shields, survey drone behavior, warp capability and charge time, scan
depth, and cargo rating. Pin the window to keep it open while returning to other
workspaces.

For the first remote route, install Warp Drive level 1 on the ship that will make
the jump and carry the required fuel canister. Scanner Array level 2 is
recommended so improved survey work reaches deeper resource data after arrival.

Upgrade costs scale by level. If you cannot afford an upgrade, its action is
disabled until the required items are in inventory. Shift and Control retain
the existing upgrade modifiers, buying up to 5 or 10 levels respectively while
components remain available.

Upgrade catalogs are hull-specific while levels remain stored per owned ship.
Conventional ships use Engine Tuning, Maneuver Thrusters, Energy Core, Shield
Matrix, Drone Bay, Warp Drive, Fuel Systems, Scanner Array, and Cargo Hold.
Autonomous mining drones use Warp Drive alongside Mining Head, Ore Processor,
Cargo Pods, Navigation Core, Job Controller, Delivery Thrusters, Industrial
Plating, and Drone Shield Matrix. These improve mining speed, bonus ore chance,
drone cargo capacity,
operating range, job quota, autonomous travel speed, hull, and shields. Buying an
upgrade affects only the currently controlled hull; switch control to inspect or
improve another fleet ship.

## Shields And Weapons

Ships can mount shield modules through shield slots. Shield modules define
capacity, recharge delay, recharge rate, damage resistance, and hazard
resistance. The ship detail panel shows installed shields, current shield
strength, recharge state, and resistance values.

Ships can also mount weapon modules through weapon slots. The current weapon
type is automatic turret defense. Turrets scan for valid hostile threats in
range, including hostile NPC ships, fire when ready, and spend a dedicated
weapon capacitor.
They ignore neutral, owned, and environmental threats. There is no manual
targeting or fire button in the current build. The Defense rail beside the ship
pane lists drive energy separately from the weapon capacitor, then shows each
`Turret Bank`, its installed turret, range, damage, capacitor cost, and cooldown
state. Open a bank's `Turret List` dropdown to see the turret types
currently in inventory and their owned counts, then select the exact turret to
install. The outgoing turret returns to inventory. Ships support one
independently configurable bank for every weapon slot in their content
definition. With the Turrets Galore pack loaded, new games start aboard its
Twinspire Gunship by default. You can still choose another hull. If selected,
the Twinspire has two banks initially fitted with an Ember Lance and Sentinel
Flak turret.
NPC ships can also mount active turrets from their content configuration:
non-hostile patrol ships engage hostile threats, while hostile ships can fire on
the player when in range. Their weapon capacitors recharge during combat, so a
long engagement no longer permanently silences an NPC after its initial power
reserve is spent. NPC-origin projectiles use a warm orange incoming-fire palette
so simultaneous fire from an identical player turret remains distinguishable.
The starter Frontier system includes a Redwake probe
that begins PvE pressure with automatic turret fire. Destroyed NPC ships are
removed from local space, and their cargo is automatically moved into your
inventory when the full cargo stack fits within your cargo rating.
Saves created before the capacitor/recharge update restore the authored
ammunition reserve for depleted armed NPCs once, preventing an old encounter
from remaining permanently disarmed.

Turret projectiles apply damage when they visually reach their target. Weapons
may strike one target, chain through nearby targets, damage a blast radius, or
combine chaining and splash damage. The Turrets Galore reference pack includes
a Storm Chain turret and a slow, wide-radius SUPER NUKE turret. Both default to
hostile-only damage, so nearby friendly and neutral ships are protected. Each
Turrets Galore weapon has its own visible projectile sprite that turns along
the weapon's flight path before impact.

Physical turrets consume ammunition from ship inventory when they successfully
fire. Their turret-bank status shows the remaining ammo count and changes to
`out of ammo` without spending capacitor power when none remains. Energy weapons such as
the Ember Lance and Storm Chain need no ammunition. Ammunition can be replenished
at the crafting station:

Thrust and turning spend only drive energy. Turrets spend only weapon-capacitor
energy, so exhausting maneuvering power cannot disable automatic defenses.
Both pools recharge from the installed reactor and are persisted independently.

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

Ships, stations, and planets have solid collision boundaries. The engine checks
the complete path traveled during each frame, so a fast ship cannot pass through
a smaller contact between frames. A collision separates the bodies and removes
their unsafe closing motion; stations, planets, and inactive owned ships remain
stationary.

Low-speed contact is separated without damage. Faster impacts scale with closing
speed and ship mass: a light probe suffers more than a cargo hull in the same
ship-to-ship impact, while hitting an immovable station or planet places the
consequence on the moving ship. Shields absorb collision damage first, then any
remainder reaches the hull. Equipped shield resistance and recharge delay apply
normally, and a pair must separate before another impact can deal damage.

Approach, follow, docking, system arrival, ship switching, and save loading use
safe clearances or a brief collision grace period to avoid unavoidable damage.
Approach autopilot also checks the route ahead. When a planet, station, contact,
or inactive fleet ship blocks the direct line, the ship chooses an open side and
flies a visible clearance arc around it before resuming the approach. The chosen
side remains stable for moving contacts. If nearby bodies close both bypasses,
autopilot brakes and waits instead of forcing a collision.
Deliberate high-speed ramming can still destroy a ship quickly.

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
provide explicit Buy and Sell buttons when the ship is in range and the offer is
available. A normal click trades one unit; hold `Shift` for up to 5 units or
`Ctrl` for up to 10. The displayed amount and total price account for stock,
credits, cargo space, and owned cargo. Limited stock is replenished using
the game's elapsed time; the trade row shows the current stock and remaining
time until its next restock. Selling cargo back to a station restores stock up
to that offer's configured capacity. Research leads point to progression-tree
nodes that can expose additional production recipes after completion.

Resize or scroll the Station window when a station has a large catalog or long
contract descriptions. Services appear as full-width clickable cards with an
optional scene thumbnail on the left and their name, description, and access
context beside it. Before a service is selected, the cards use the full Station
window. Selecting a service moves the cards into the left column and opens a
right-hand detail rail containing that service's description, status, offers,
and actions. Cinder Repair Yard supplies distinct artwork for Repair Gantry,
Hull Showroom, Parts Cage, and Torchcut Bar. Freebelt Commissary also illustrates
its Supply Counter, Hauler Tables, and Job Board; content-pack services without
artwork retain the same card layout with text only.

Garage services provide a Maintenance action. Shipyard services list
purchasable ships and mining drones with their price, handling, cargo, and
mining statistics; ship sales no longer appear in Fleet management. When docked and carrying enough
credits, repair restores damaged hull and shields; the price is based on the
amount of damage. This remains the immediate way to restore both systems;
research-gated Auto Repair is slower, hull-only, and consumes carried raw ore.
The Ore Lattice Freight Lock provides authored hauling contract chains. Acquire
the requested ore, then carry it into range of the named destination station;
the destination accepts the shipment and pays automatically, with no return to
the issuing desk. Archive scan desks provide survey contracts: scan
the named planet, then return to the archive. The station panel shows contract
progress, completion, expiry, and rewards. Up to three contracts can be active,
and active contracts survive save/load.

Press `J` at any time to open the movable stock-egui Contracts window. Select an
active contract on the left to inspect its objective, progress, origin,
destination, deadline, reward, and current status. Use `Focus origin station`
or double-click the contract when its origin is in the current system; remote
contracts remain visible with their origin system explained. Pin the window to
keep it visible after closing the Contracts workspace. Opening Contracts does
not close Fleet or other workspace windows, so it can remain alongside ship
controls while you review and fly contract routes.

Contract details provide `Fly to source now` and `Fly to destination now`
controls. Authored contract chains provide the same `Fly now` controls beside
their origin and current-stage destination. A local destination immediately
engages the normal approach autopilot. A destination in another system names
that system and remains disabled until you travel there.

Authored contract chains also appear in station services and in the Contracts
window. They advance through real travel, scanning, mining, production, combat,
research, cargo, and return actions. The window shows the current stage,
objective counts, and explicit blockers. Contract cargo is ordinary physical cargo:
it remains in a ship hold, appears in nearby shared Fleet Inventory, and survives
save/load. You can abandon a chain in Contracts and restart it at its origin.
If a required content pack is removed, the saved contract remains visible as
unavailable instead of silently disappearing.

Persistent contract-relevant discoveries are also reconciled when you accept,
restart, or load a contract. Approaching, identifying, looting, destroying, or
salvaging a target before accepting its contract does not invalidate the chain;
later stages advance from the facts the world already established. If a unique
`contract`-tier cargo item is later lost and its original wreck is missing,
empty, or fully salvaged, return to the issuing service and choose `Recover
cargo`. The issuer restores only the missing amount when your active hold has
space. The original wreck remains salvaged, and rewards still pay only once.

### Testing The Silent Relay

At Frontier Exchange, approach dock range, open `Exchange Rumors`, and accept
`The Silent Relay`. Travel northwest beyond the normal traffic lanes to roughly
`-1840, -1680`. Approach and select the `Pale Relay Derelict`; remain in scanner
range until its identity scan completes. Right-click it, choose `Loot wreck`, and
confirm `Pale Relay Archive` appears in Fleet Inventory. Save and reload here to
verify the recovered-but-not-returned stage. Return to Frontier Exchange, open
`Exchange Rumors`, and choose `Deliver`; completion awards 650 credits and 8
Cinder Cooperative reputation exactly once.

To test out-of-order recovery, loot the archive before accepting `The Silent
Relay`, then accept it at Exchange Rumors; it should open directly at the return
stage. To test lost-cargo recovery, discard or otherwise remove the archive after
the wreck has been emptied, return to Exchange Rumors, and choose `Recover
cargo`. A second action delivers the restored archive.

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

Select an NPC ship with `Space` or a click to begin an automatic passive scan.
`Space` can acquire the nearest contact anywhere inside scanner range, including
hostile ships holding outside the smaller interaction radius, and mouse contact
hit targets include extra padding for moving ships.
Hostile interceptors are acquired automatically when they target the active ship
and enter scanner range. A proximity alarm sounds on entry, the nearest hostile
becomes the selected contact, and scanning starts without requiring a click or
key press. Leaving scanner range arms the alarm again for a later re-entry.
The movable, resizable Contact window shows the current stage, progress, range,
power blocker, and scanner-depth limit. Keep the selected ship within 620 units
of surface clearance: leaving range briefly pauses the scan, while a longer
interruption or selecting another target loses partial progress for the current
stage. Completed stages remain known for that content-pack ship archetype.

Scanning reveals identity and disposition first, followed by hull telemetry,
shields, weapon loadout, and detailed energy, readiness, ammo, and cargo data.
Unknown fields remain labeled instead of exposing exact values. Hostile hull
plates appear as a masked `?` bar while hull telemetry is unknown, then reveal
the live hull fraction after the hull stage. Their shield bar appears only after
the shield stage. Weapons can acquire and damage hostile contacts at any scan
stage; scanning changes available information, never permission to fire.
Weapon impacts show what absorbed the hit. Shield damage produces a brief
localized shield flare and outward-scattering deflection particles on the side
the shot reached. If damage penetrates into the hull, small debris fragments
and hot sparks break away from that contact point; these effects are visual and
do not alter damage, collision, or salvage calculations. Scanning spends a
small amount of ship energy but
yields to automatic weapon fire; insufficient energy pauses progress. The base
scanner can reach shield telemetry. Scanner Array upgrades add one combat scan
stage and 15% speed per level, while Research rewards can add more depth or
speed. Completed intelligence is saved; partial stage progress is not.
Identity and hull telemetry are the fastest stages so an early hostile normally
reveals its health plate before starter automatic turrets destroy it. The
starter Redwake probe also has an easier scan signature than a standard ship.
Starter and shipped content hull and shield capacities are tuned so ordinary
turret combat generally lasts about 10–25 weapon impacts rather than ending in
the first few shots; heavy specialist weapons can still end fights sooner.

Identity unlocks supported hooks such as hail, dock, trade, or conflict. Some
hooks are visible before their full mechanics exist, so unavailable rows describe
the current blocker. Identified non-hostile haulers with cargo expose a mobile
Cargo market: open it from the
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
the default playable content and is always enabled. Optional first-party packs
can add systems, planets, recipes, resources, and related data when enabled.

Open the title screen Content Packs view to inspect available packs and options
before starting a new game. Select a pack and use its Enable/Disable control;
required dependencies turn on automatically, and disabling a dependency also
disables packs that need it. The New Game ship picker and the new world use only
the enabled profile. In gameplay, press `C` to open the content browser.
The movable stock egui Content Browser uses a three-pane layout. Select a loaded
pack from the vertical list on the left, choose any authored object by category
in the middle, and inspect its fields in the right pane. References such as a
weapon's install item or ammunition are clickable, and `Back` returns through
the inspection history. Select `All packs` for the aggregate registry summary.
Pin the browser to keep it visible and interactive after pressing `C` again.

Every new or updated save records the IDs and versions of the packs used to load
it. Loading with the same profile starts immediately. If the title-screen
selection differs, the game asks whether to restore the save profile or try the
current selection. Restoring enables every recorded pack available locally,
disables extras, and clearly lists missing packs or version differences before
continuing. Trying the current selection can remove unavailable items, ships,
systems, or authored progression through existing missing-content fallbacks, so
make a backup when deliberately changing a long-lived save's profile. Older
saves without a recorded profile use the current title-screen selection and
record it on their next save.

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
