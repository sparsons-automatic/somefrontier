# Game Guidelines

Some Frontier should feel like practical frontier logistics in deep space:
travel, survey, extract, process, upgrade, and push farther out.

## Core Loop

1. Fly to a point of interest.
2. Survey the planet before trusting its data.
3. Mine resources from known planets.
4. Smelt and fabricate materials into useful components.
5. Spend components on ship upgrades, drones, and future travel systems.
6. Use upgrades to reach better planets and richer material chains.

## Design Principles

- Exploration should create useful decisions, not blind punishment.
- The player should rarely be hard-stuck. Critical resources need a recovery
  path, even if slow.
- Travel should feel spatial and purposeful. Destination markers, scanned state,
  and map interactions should reinforce that planets are real places.
- Production should stay readable. Mining, smelting, crafting, skills, and
  upgrades should continue using familiar table-like UI patterns.
- New resources should earn their place by connecting to at least one meaningful
  recipe, upgrade, hazard, or exploration decision.
- Early systems should be forgiving; deeper systems can become more specialized
  and expensive.

## Mechanics and Content Boundary

The base game should provide stable world guidance: mechanics, validation,
execution rules, and UI behavior. Content packs should add things that operate
inside those rules.

Base game mechanics:

- Flight, camera, map interaction, and destination behavior.
- Inventory, mining, smelting, crafting, skills, scanning, and save/load rules.
- UI layout, disabled states, and action execution.
- Pack validation and compatibility rules.

Core and plugin content:

- Items, recipes, planets, assets, and starting inventory.
- Station definitions and upgrade definitions once those are data-driven.
- Compatibility recipes and alternate resource branches.
- Future alternate starts that still respect base mechanics.

This boundary keeps plugin packs flexible without making each pack responsible
for defining how the game itself works.

## UI Guidelines

- Keep contextual planet actions in the right-side action rail attached to the
  inventory/mining overlay.
- Keep planet details focused on identity, survey data, mineables, hazards, and
  proximity.
- Use disabled states when the player lacks range, items, or prerequisites.
- Avoid hiding why an action is unavailable.
- Prefer compact operational screens over large explanatory panels.

## Fuel Direction

Fuel should not strand the player early. Start with energy and craftable reactor
cells, then expand into fuel-specific resource chains.

Recommended progression:

1. Ship energy remains the basic movement resource.
2. Reactor cells become a crafted way to refill or boost energy.
3. Volatile ice and hydrogen unlock dedicated fuel production.
4. Helium-3 supports advanced engines or long-range travel.
5. Exotic matter supports late-game jump or warp systems.
