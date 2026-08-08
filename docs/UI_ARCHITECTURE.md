# UI Architecture

Some Frontier uses Macroquad for world rendering, ships, effects, HUD elements,
and decorative panel art. The general Ship Pane / Production / Inventory overlay
uses `egui-macroquad` for widget layout and interaction. Object-specific planet,
station, and contact panes remain on the established Macroquad presentation path
until they are migrated as separate, reviewable changes.

## Inventory Overlay Boundary

The egui overlay receives an owned, presentation-only view model. Content-pack
item, recipe, shield, weapon, and ship names enter the view model without being
shortened. The renderer owns clipping, scrolling, tooltips, focus, and responsive
placement; it does not own crafting, installation, inventory, save, or combat
rules.

User input produces typed actions. The gameplay layer validates and applies those
actions with the existing recipe and equipment functions. This keeps domain and
save behavior independent of egui and leaves the presentation replaceable.

At wide sizes, the overlay shows Ship Pane, Production, and Inventory together.
When the panel cannot sustain all three panes, it switches to visible pane tabs
instead of shrinking tables past their minimum usable widths. Tables use a
flexible, clipped item-name column and compact numeric columns. Full clipped text
is available on hover.

## Dependency Decision

The integration pins `egui-macroquad` 0.17.3 and `egui_extras` 0.31.1 so both use
the same egui 0.31 API. Junicode is registered as the proportional egui font, and
the color and spacing setup follows the existing terminal palette.

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
| 640 x 420 | Compact pane tabs; no content outside the frame |
| 960 x 500 | Compact pane tabs; tables scroll vertically |
| 1100 x 760 | Three visible panes |
| 1280 x 720 | Three visible panes with wider item columns |
| 1600 x 900 | Three visible panes without excessive numeric-column growth |

Also check normal and high-DPI desktop scaling. Verify mouse scrolling remains in
the hovered table, tooltips reveal clipped values, `Up`/`Down` changes the active
production row, `Enter` queues it, production tabs remain reachable, and ship
equipment dropdowns contain compatible cargo items.

## Native And Web Builds

Native rendering requires no integration files beyond the Rust dependencies and
the existing Junicode asset. `egui-macroquad` web builds additionally require its
`quad-url.js` and `quad-storage.js` plugin scripts beside the Macroquad web shell.

CI checks the Rust game target with `wasm32-unknown-unknown`. Optional remote
asset downloads are disabled for that target so the native TLS client is not
linked into the browser binary. This repository still does not contain the web
shell needed to launch or publish that binary; a future deployment must provide
the Macroquad loader and the two egui integration scripts described above.

## Migration Rule

Move other menus incrementally. Each migration should introduce a view model and
typed action boundary, preserve the current visual language, add responsive tests,
and remove only the legacy geometry and input code that the accepted surface
supersedes.
