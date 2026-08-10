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

Ship, Production, and Inventory are independent native egui windows. Players can
move, resize, collapse, overlap, and scroll them while the world remains visible.
The game serializes egui memory to the global configuration directory, so layout
survives launches without becoming part of a world save. Restored windows are
checked against the current display, and any unreachable geometry is discarded.
The movable stock-egui pause menu provides a `Reset Menus` recovery action that
clears all persisted egui areas. Its position participates in the shared layout
persistence, while its dimensions remain bounded and non-resizable. Each
persistent workspace window also exposes a stock pin toggle.
Pinned visibility is stored globally beside layout memory—not in world saves—and
allows independently rendered windows to remain interactive when Inventory or
Fleet closes. All visible pinned and parent-owned windows are composed in one
egui pass so multiple pins can coexist. Reset Menus clears both geometry and pin
state. Production and inventory use
stock striped grids whose row heights follow their contents instead of fixed game
geometry. Full clipped text is available on hover where clipping is used.

Inspecting a planet replaces the Ship window with a movable Planet window while
keeping the shared Production and Inventory windows. The planet view model owns
only display data; typed actions return to the gameplay layer for scan, orbit,
mining queue, and Keep validation.

Inspecting a station follows the same workspace model: a movable Station window
appears beside the shared Production and Inventory windows. Its presentation-only
view model includes services, market quotes and stock, maintenance state,
contracts, recipe unlocks, and research leads. Typed actions flow back through
the existing range, reputation, credit, cargo, contract, progression, and save
rules rather than changing those rules in the renderer.

Inspecting an NPC ship opens a movable Contact window with identification,
disposition, systems, defenses, and cargo details. Hail and market controls emit
typed actions back to the established range, hostility, credit, cargo-capacity,
inventory, and save rules. Hostile contacts remain visible while friendly-only
actions use egui's disabled state.

Fleet management is a separate movable egui workspace. Its owned-ship list,
condition readout, two-way cargo controls, and nearby shipyard offers are built
from a presentation-only snapshot. Typed actions return ship selection, control
transfer, cargo item IDs, and hull IDs to the existing range, capacity, content,
credit, and save validation. The Fleet window participates in the same global
layout persistence and `Reset Menus` recovery path as the inspection windows.

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
paths.

Ship Upgrades project the installed levels, authored effects, next-level costs,
and live inventory counts into a stock egui table. Its typed purchase action
returns only the upgrade row and modifier count; the gameplay layer remains
responsible for recalculating each scaled level cost, consuming components, and
applying ship-stat changes. The window shares pin and geometry persistence with
the other egui workspaces.

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

The Operations Log is a separate movable, pinnable egui window over the shared
gameplay feedback stream. Gameplay systems continue to publish through one
event path, so the compact Ship readout, contextual Planet, Station, Contact,
and Research feedback, warning audio, and full history do not need duplicate
messages. Each runtime event records elapsed world time, its original source,
one broad category, one severity, and its message. The model deduplicates exact
repeats, aggregates routine production and mining output, and discards the
oldest entries beyond 200.

The renderer receives a chronological presentation-only snapshot and emits
typed actions for category filters, severity filters, reset, clear, and pinning.
Filters change visibility rather than deleting events; clear is the only UI
action that removes history. Egui's bottom-stick scroll state follows new events
only while the player remains at the bottom, preserving position when older rows
are being reviewed. Operations history and filter state are intentionally
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
text markers, category and severity toggles work at each matrix size, and a new
event follows the bottom only when the log was already there. Scroll upward,
generate another event, and confirm the reviewed rows remain stationary. Clear
history, reset filters, pin the window, and use `Reset Menus` to verify each
action retains its documented scope.

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
