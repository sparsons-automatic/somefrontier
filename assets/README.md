# Assets

Shared game art, audio, and engine/UI assets live here.

- `ships/`: generated/source ship previews only; runtime content-pack ship
  textures live under `content/packs/<pack>/assets/`.
- `planets/`: generated/source planet previews only; runtime content-pack planet
  textures live under `content/packs/<pack>/assets/`.
- `backgrounds/`: starfields, distant objects, and skybox-style assets.
- `transitions/`: random full-screen space images used by scene transitions.
- `ui/`: HUD icons, panels, fonts, cursors, and menu art.
- `effects/`: particles, projectiles, engine flames, shields, impacts, and explosions.
- `audio/`: music, ambient loops, engine sounds, alerts, and UI sounds.

Current image assets are created through Codex image generation, then post-processed to transparent PNGs when needed. Keep shared engine assets at the top level of their asset folder and keep source chroma-key renders or previews under that folder's `generated/` directory.

Content-pack runtime assets:

- `content/packs/core/assets/ships/frontier-cargo-ship-01.png`
- `content/packs/core/assets/planets/frontier-planet-01.png` through
  `frontier-planet-20.png`
- `content/packs/core/assets/stations/frontier-exchange.png`
- `content/packs/remote-duskfall/assets/planets/` contains the remote pack's
  planet textures.

Shared runtime assets:

- `assets/transitions/` supports `.png`, `.jpg`, and `.jpeg` transition images
  loaded at startup.
- `assets/transitions/frontier-transition-01.png`
- `assets/transitions/frontier-transition-02.png`

The old local image-generation pipeline has been removed.
