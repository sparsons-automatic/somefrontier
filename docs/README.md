# Some Frontier Docs

This folder holds lightweight design references for gameplay systems, material
chains, balance guidelines, and future feature planning.

The project uses a mechanics/content split: base Rust code owns stable gameplay
rules and validation, while the `core` pack and future plugin packs provide
world content that stacks on top.

## Documents

- [Game Guidelines](GAME_GUIDELINES.md): core design principles for navigation,
  progression, UI, and player safety.
- [Recipes and Materials](RECIPES_AND_MATERIALS.md): current item chains,
  recipes, planet sources, and planned fuel/material directions.
- [Plugin System](PLUGIN_SYSTEM.md): implementation and authoring guide for
  file-based content packs.

## Working Rules

- Keep docs close to the playable build.
- Prefer small, explicit tables over large speculative systems.
- Mark planned content clearly so current behavior stays easy to identify.
- Update these docs when adding recipes, resources, upgrades, weapons, planets,
  stations, starter inventory, or progression gates.
