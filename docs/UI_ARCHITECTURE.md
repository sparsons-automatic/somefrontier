# UI Architecture

Some Frontier uses Macroquad for world rendering, ships, effects, HUD elements,
and decorative panel art. The general Ship / Production / Inventory workspace
uses `egui-macroquad` for widget layout and interaction. Planet survey, orbit,
and mining interactions, station services, and NPC contacts also use stock egui
windows.

World-space right-click actions share one stock egui context-menu renderer for
planets, stations, and NPC ships. The game supplies ordered labels, availability,
and blocker details; the renderer owns popup placement, hover, disabled styling,
and click-away dismissal before returning the selected index to existing gameplay
action dispatch. Hit testing runs only when the right button is pressed and ranks
all matching object types by cursor distance from their visible center. The
nearest object opens directly; candidates within 24 screen pixels of the nearest
distance first produce a stock-egui target chooser. This prevents content order
or object type from owning overlapping interaction rings.

## Inventory Overlay Boundary

The egui overlay receives an owned, presentation-only view model. Content-pack
item, recipe, shield, weapon, and ship names enter the view model without being
shortened. The renderer owns clipping, scrolling, tooltips, focus, and responsive
placement; it does not own crafting, installation, inventory, save, or combat
rules.

User input produces typed actions. The gameplay layer validates and applies those
actions with the existing recipe and equipment functions. This keeps domain and
save behavior independent of egui and leaves the presentation replaceable.

Ship stats, cargo inventory, and Production are pages of the Fleet control
workspace. The former standalone Ship and Inventory windows and their `Tab`/`E`
shortcut have been retired. Players can move, resize, collapse, overlap, and
scroll Fleet alongside inspection windows while the world remains visible.
Opening Fleet is additive: it does not close existing parent-owned or inspection windows.
The game serializes egui memory to the global configuration directory, so layout
survives launches without becoming part of a world save. Restored windows are
checked against the current display, and any unreachable geometry is discarded.
The movable stock-egui pause menu provides a `Reset Menus` recovery action that
clears all persisted egui areas. Its position participates in the shared layout
persistence, while its dimensions remain bounded and non-resizable. Each
persistent workspace window also exposes a stock pin toggle.
Pinned visibility is stored globally beside layout memory—not in world saves—and
allows independently rendered windows to remain interactive when their parent
workspace closes. All visible pinned and parent-owned windows are composed in one
egui pass so multiple pins can coexist. Reset Menus clears both geometry and pin
state. Production and Fleet inventory use stock striped grids whose row heights follow their contents instead of fixed game
geometry. Full clipped text is available on hover where clipping is used.

Fleet Inventory projects the selected hull's directly linked cargo graph into a
single aggregate manifest. Rows retain per-hull provenance while displaying
combined quantity and mass. Search and unavailable-hull explanations remain
presentation state; physical ownership, range, sharing, capacity, production
consumption order, and consolidation stay in the gameplay layer.

Inspecting a planet opens a movable Planet window independently of Fleet. The planet view model owns
only display data; typed actions return to the gameplay layer for scan, orbit,
mining queue, and Keep validation.

Inspecting a station follows the same workspace model: a movable Station window
appears independently beside any open Fleet or inspection windows. Its presentation-only
view model includes services, market quotes and stock, maintenance state,
contracts, recipe unlocks, and research leads. Typed actions flow back through
the existing range, reputation, credit, cargo, contract, progression, and save
rules rather than changing those rules in the renderer. Service selection uses
full-width clickable cards: optional content-pack artwork is projected through
the shared egui artwork cache beside the name, description, and access context.
With no selection, cards span the Station window. Selecting any point on a card
opens a right-hand detail rail; selected-service status, offers, actions, and
operation feedback render there for every station.

Inspecting an NPC ship opens a movable Contact window with identification,
disposition, systems, defenses, and cargo details. Hail and market controls emit
typed actions back to the established range, hostility, credit, cargo-capacity,
inventory, and save rules. Hostile contacts remain visible while friendly-only
actions use egui's disabled state.

Fleet management is a wide movable egui control workspace. A horizontally
scrollable hull selector keeps one controllable asset in focus while a persistent
left rail switches the content area among Stats, Upgrades, Inventory, and
Production. The
Stats page owns condition readout and asset-specific operations, the Upgrades
page uses a selectable module-card/detail layout derived from Research, and the
Inventory page groups cargo by authored item tier. The Production page reuses the
existing recipe projection and typed queue/Keep actions for the active hull, and
projects installed autonomous smelting for inactive mining drones. All four are built from one
presentation-only snapshot. Typed actions return ship selection, page and module
selection, control transfer, upgrade purchases, and cargo item IDs to the
existing range, capacity, cost, and save validation. A per-ship inventory-
sharing checkbox also returns a typed action; the gameplay layer resolves the
same-system 180-unit link, local-first atomic input consumption, and producer-
local output capacity for production, upgrades, and Auto Repair. Ammunition,
mining jobs, and cargo limits remain outside that shared-input resolver. Shipyard offers belong
to the selected station service, where typed hull-purchase actions flow through
station range, service, content, and credit validation. Mining-drone hulls add
typed destination, resource, quota, recipient-ship, completion behavior, pause,
ordered-route editing, recall, and cancel actions to that same boundary. Each
persisted route leg owns a planet, resource, quota, and mined progress; legacy
single-destination jobs migrate to one-leg routes. The gameplay layer owns job
validation, leg transitions, recipient rendezvous, capacity-limited cargo
delivery, and the persisted state machine. The Fleet window participates in the same global
layout persistence and `Reset Menus` recovery path as the inspection windows.
The former standalone Ship Upgrades entry routes to the selected active hull's
Fleet Upgrades page, and `P` routes to the active hull's Fleet Production page,
so neither system maintains a second primary presentation path.

The Content Browser uses the same presentation/action boundary. Its view model
projects registry summary counts, loaded packs, categorized object rows, and a
typed detail record. Egui owns the movable three-pane pack/object/detail layout;
the gameplay layer owns pack selection, object selection, and inspection
history. References are typed by content kind and ID, so following one can
select the owning pack and resolve the target through the registry without UI
code reaching into gameplay state. The browser can be pinned and composed
alongside any other pinned egui workspace.

Contracts use a movable egui master/detail workspace backed by the existing
active-contract projection. Selecting a row changes presentation state only;
the typed focus action returns to gameplay code, which validates the contract
and current system before opening its origin station. Contract window geometry
and pin state participate in the shared persistence and `Reset Menus` recovery
paths. Source and destination flight actions return stable contract identifiers
to gameplay code, which resolves their current station, planet, or NPC indices
before engaging the normal approach controller. Remote-system destinations are
projected as disabled controls with explanatory system text.

The same workspace lists active authored contract chains above repeatable
contracts. Contract-chain rows project the content definition plus persistent
runtime stage, objective progress, status, and blocker text.
`AbandonContractChain` is a typed UI action; gameplay code owns the state change
and origin restart rule. Station service contract-chain buttons likewise return
typed accept/deliver actions to gameplay code, which validates range and
consumes delivery cargo through nearby shared-fleet inventory. The UI never
advances objectives or grants rewards directly.

Durable target interactions are written to a bounded save-backed contract fact
ledger before they are projected into an active chain. Gameplay reconciliation
replays the ledger after acceptance, restart, and save restoration, so the UI
never needs target-specific repair logic. Station rows expose `Recover cargo`
only when gameplay reports that unique contract cargo is globally absent and its
source is irrecoverable; the typed station action still performs capacity,
origin, inventory, progression, and reward validation in the gameplay layer.

Research uses a data-driven graph canvas. Pure layout code normalizes authored
column and row values into stable screen lanes, resolves duplicate positions by
name and content ID, and derives scrollable content bounds without depending on
registry hash order. Prerequisite/reveal edges are routed through column gutters
behind opaque module cards and end in arrowheads. Selecting a module emphasizes
its incoming and outgoing edges, while state-specific text markers, fills, and
borders keep completed, active, affordable, unfunded, and locked cards distinct
without relying on color alone. The graph scrolls in both directions; the fixed
details pane presents prerequisites, dependents, rewards, cost, duration, and
progress on the right at ordinary widths and stacks below the graph in narrow
windows. Selection and start actions return research IDs to
the gameplay layer, which retains prerequisite, credit, timer, save, and reward
authority. The window participates in the shared pin, geometry persistence, and
`Reset Menus` recovery paths without an additional graph or layout dependency.

Combat scanning is a gameplay-owned staged state machine rather than an egui
timer. The selected live NPC supplies per-contact range, difficulty, and current
telemetry; a save-backed intelligence ledger records only the highest completed
stage by namespaced NPC archetype. Partial progress, current hull/shield/energy,
weapon readiness, and ammunition are never persisted as learned snapshots.
Scanner Array levels and validated Research rewards determine maximum depth and
speed, while combat resolution remains authoritative regardless of what the
player can see.

Player and owned-ship runtime systems keep drive energy and weapon-capacitor
energy as separate resources. Flight control and passive scanning spend drive
energy; automatic weapons consult and spend only the capacitor. Both resources
serialize independently, with legacy saves receiving a full capacitor derived
from their resolved hull. Ship and Fleet panes label both pools explicitly.
The save-version migration also restores authored ammunition for armed NPCs
whose older persistent cargo record was depleted; current-version saves retain
normal NPC ammunition consumption without repeated refills.

Resolved weapon damage emits transient impact-render events after shield and
hull application. Each event records the actual target edge, incoming direction,
absorbed damage type, magnitude, and weapon impact color. The renderer turns
shield events into localized deflection flares and hull events into procedural
debris, keeping visual feedback generic across player, fleet, NPC, and
content-pack weapons without persisting cosmetic state in saves.

Hostile-intercept contacts entering scanner range are acquired by the gameplay
layer before this update runs: entry-edge tracking emits one proximity alarm,
selects the nearest current threat, and starts the same scan state machine used
by manual selection. The entry set is runtime-only and is cleared naturally
when contacts leave range, change systems, or are destroyed.

One reveal projection drives the Macroquad contact label and hostile health
plate, compact HUD status, context/interaction availability, and egui Contact
facts. This prevents an unknown surface from leaking a faction color, name,
capacity, or loadout that another surface hides. The Contact renderer receives
only prepared labels, optional scan progress, blockers, and permitted facts;
typed actions still return to gameplay code. Visual testing should cover every
intel stage. Hostile interceptors draw a masked `?` plate before hull telemetry
is known, without using the live health fraction; the normal hull and shield
fills appear only at their corresponding reveal stages. Weapon targeting never
reads the intelligence ledger, so unknown contacts remain valid combat targets.
Visual testing should also cover range interruption and reacquisition,
insufficient power, scanner
depth limits, save/reload, hostile hull-only plates, shield-enabled plates, and
unknown contacts at representative zooms and window sizes. Include a moving
hostile at its authored standoff distance: both `Space` acquisition and the
padded pointer hit target should select it before the starter turret kill window.

The Operations Log is a separate movable, pinnable egui window over the shared
gameplay feedback stream. Gameplay systems continue to publish through one
event path, so the compact Ship readout, contextual Planet, Station, Contact,
and Research feedback, warning audio, and full history do not need duplicate
messages. Each runtime event records elapsed world time, its original source,
one broad category, one severity, and its message. The model deduplicates exact
repeats, aggregates routine production and mining output, and discards the
oldest entries beyond 200.

The renderer receives a chronological presentation-only snapshot and emits
typed actions for category tabs, severity filters, reset, clear, and pinning.
The `All` tab and eight category tabs project the shared history without
deleting events; clear is the only UI action that removes history. Each tab has
independent bottom-stick scroll state, following new events only while the
player remains at that tab's bottom and preserving position when older rows are
being reviewed. The explicit tab model also leaves room for future data sources
such as network chat. Operations history and view state are intentionally
runtime-only and never enter world saves. Window geometry and pin state still
use the shared global egui persistence and `Reset Menus` recovery path.

The starmap deliberately remains a Macroquad 3D renderer rather than becoming an
egui drawing surface. `M` composes the smaller left-biased map with an independent
stock-egui Destinations window. Map hit-testing returns a planet index only to
update selection; the Destinations presentation model exposes scanned facts and
an ordered itinerary, and typed actions return Fly, Add, reorder, remove,
activate, pause, and clear intent to gameplay code. Runtime route stops and save
records use stable `(system ID, planet ID)` pairs, never vector indexes. The
gameplay layer resolves those IDs through loaded content, drives the existing
physical approach controller, advances after interaction-range arrival, and
pauses on manual flight or an unavailable target. Pin and geometry state remain
global UI preferences, while itinerary progress belongs to the world save.
The Macroquad panel uses its header as a drag surface, has its own bottom-right
resize handle, and persists both position and size globally because it is outside
egui geometry memory. Loaded and dragged geometry is clamped to the current
display, and the shared `Reset Menus` action restores its defaults. Its `Camera3D`
receives the panel viewport's explicit aspect ratio; relying on the full-window
default would stretch spheres whenever the panel and display had different
proportions.

Startup loading keeps transition artwork and the cropped game logo in the
Macroquad render layer, then composes a centered, non-interactive stock-egui
status frame beneath the logo. The frame deliberately has no window title bar or
persisted geometry: it presents the current loading stage, activity spinner, and
optional remote-asset detail without behaving like a movable gameplay workspace.
Its determinate bar and step count cover the complete local startup chain,
including preview preparation, transition images, ships, projectiles, planets,
stations, contacts, save restoration, and audio finalization. Remote download
status remains supplemental detail and owns the bar only before the local startup
plan is available.

The branded title landing screen and its New Game, Load Game, Content Packs, and
Settings child flows use stock-egui renderers. The landing screen keeps the
cropped game logo in an egui texture and emits typed navigation actions to the
title controller. Each submenu likewise emits typed actions for starting and
loading games, deleting saves, changing pack options, and persisting settings.
Ship preview pixels are cropped to their alpha bounds, rotated once on load, and
registered as egui textures so the New Game grid retains content-pack art
without stretching it.

Gameplay object windows keep stock egui chrome and render authored artwork
inside their content area. Ship, upgrade, fleet, planet, station, contact,
destination, and Content Browser detail views receive resolved content texture
paths through their view models. A shared lazy egui cache crops transparent
padding, caps thumbnails at 320 pixels on the long edge, preserves aspect ratio,
and places previews in non-interactive grouped frames. Artwork therefore cannot
change window geometry, intercept controls, or bypass content-pack path
resolution.

The optional debug console is also a stock-egui window with native text editing,
scrollable command history, movement, resizing, collapsing, and shared layout
persistence. Its typed execute actions return to the existing command parser;
the UI does not directly mutate gameplay state.

## Dependency Decision

The integration pins `egui-macroquad` 0.17.3 and uses its stock egui 0.31 font,
dark theme, spacing, window frames, and controls. Game identity remains in the
world and content rather than custom widget geometry.

Taffy was considered as the layout-only alternative. It would provide Flexbox
and Grid pane geometry with a smaller conceptual migration, but the project would
still need to maintain its custom tables, clipping, scrolling, pointer capture,
focus, keyboard navigation, and duplicate input/render geometry. Egui addresses
those failure modes together. Taffy remains a reasonable option for future
non-interactive world-space layouts, but is not the selected overlay backend.

## Visual Validation Matrix

Review the inventory overlay in each configuration below with empty and crowded
inventories, long content-pack names, locked recipes, and multi-turret ships.

| Window | Expected presentation |
| --- | --- |
| 640 x 420 | Three movable windows with usable minimum sizes and scrolling |
| 960 x 500 | Windows can be rearranged without losing controls |
| 1100 x 760 | Three useful default windows with the world visible behind them |
| 1280 x 720 | Windows resize cleanly around long content names |
| 1600 x 900 | Windows remain content-sized instead of stretching full-height |

Also check normal and high-DPI desktop scaling. Verify mouse scrolling remains in
the hovered table, tooltips reveal clipped values, `Up`/`Down` changes the active
production row, `Enter` queues it, production tabs remain reachable, and ship
equipment dropdowns contain compatible cargo items.

For the Operations Log, verify long messages wrap without hiding timestamps or
text markers, category tabs and severity toggles work at each matrix size, and a
new event follows the bottom only when the active tab was already there. Scroll
upward, switch tabs, generate another event, and confirm each tab retains its
own reviewed position. Clear history, reset filters, pin the window, and use
`Reset Menus` to verify each action retains its documented scope.

For Research, test locked, unfunded, affordable, active, and completed cards at
each window size. Confirm every state retains its marker and border, long pack
names remain inside cards, arrows stay behind controls, selected prerequisites
and dependents brighten together, and duplicate pack-authored grid positions do
not overlap. Scroll both axes through a dense tree, use keyboard focus and
pointer selection/start actions, resize with a module selected, and confirm the
fixed details pane remains readable.

## Native And Web Builds

Native rendering requires no integration files beyond the Rust dependencies.
`egui-macroquad` web builds additionally require its
`quad-url.js` and `quad-storage.js` plugin scripts beside the Macroquad web shell.

CI checks the Rust game target with `wasm32-unknown-unknown`. Optional remote
asset downloads are disabled for that target so the native TLS client is not
linked into the browser binary. This repository still does not contain the web
shell needed to launch or publish that binary; a future deployment must provide
the Macroquad loader and the two egui integration scripts described above.

## Migration Rule

Move other menus incrementally. Each migration should introduce a view model and
typed action boundary, prefer stock egui interaction patterns, add responsive tests,
and remove only the legacy geometry and input code that the accepted surface
supersedes.
