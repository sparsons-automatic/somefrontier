# Assets

Shared game art, audio, and engine/UI assets live here.

- `ships/`: generated/source ship previews only; runtime content-pack ship
  textures live under `content/packs/<pack>/assets/`.
- `planets/`: generated/source planet previews only; runtime content-pack planet
  textures live under `content/packs/<pack>/assets/`.
- `backgrounds/`: starfields, distant objects, and skybox-style assets.
- `branding/`: official logos, derived app/window icons, and other project
  identity assets.
- `transitions/`: random full-screen space images used by scene transitions.
- `ui/`: HUD icons, panels, fonts, cursors, and menu art.
- `effects/`: particles, projectiles, engine flames, shields, impacts, and explosions.
- `audio/`: music, ambient loops, engine sounds, alerts, and UI sounds.

Current image assets are created through Codex image generation, then
post-processed to transparent PNGs when needed. Keep shared engine assets at the
top level of their asset folder and keep source chroma-key renders or previews
under that folder's `generated/` directory.

Runtime object sprites under `content/packs/<pack>/assets/ships/`,
`content/packs/<pack>/assets/planets/`, and
`content/packs/<pack>/assets/stations/` should be transparent PNGs so they
composite cleanly over the game scene. Full-screen transition images under
`assets/transitions/` and generated/source files under `assets/*/generated/` may
remain opaque when they are backgrounds or source material.

Check committed runtime object sprites with:

```sh
scripts/audit-runtime-asset-alpha.sh
```

Content-pack runtime assets:

- `content/packs/core/assets/ships/frontier-cargo-ship-01.png`
- `content/packs/core/assets/planets/frontier-planet-01.png` through
  `frontier-planet-20.png`
- `content/packs/core/assets/stations/frontier-exchange.png`
- `content/packs/remote-duskfall/assets/planets/` contains the remote pack's
  planet textures.

Shared runtime assets:

- `assets/branding/some-frontier-logo.png` is the official game logo used by
  title, pause, and documentation surfaces. Keep this as the canonical logo
  asset; individual UI surfaces can crop or scale it at render time for their
  available space.
- `assets/branding/some-frontier-icon-16.png`,
  `assets/branding/some-frontier-icon-32.png`, and
  `assets/branding/some-frontier-icon-64.png` are derived square app/window
  icons. Matching `.rgba` files hold raw startup icon bytes included by
  `src/branding_icon.rs` for Macroquad window configuration.
- `assets/transitions/` supports `.png`, `.jpg`, and `.jpeg` transition images
  loaded at startup.
- `assets/transitions/frontier-transition-01.png`
- `assets/transitions/frontier-transition-02.png`
- `assets/transitions/frontier-station-approach.png`

The old local image-generation pipeline has been removed.
