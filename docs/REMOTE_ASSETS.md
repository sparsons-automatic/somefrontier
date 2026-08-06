# Remote Assets

Some Frontier can consume optional runtime assets published by the project's
asset host. The first delivery phase is limited to audio; remote content packs
are a separate future feature.

This document defines the delivery contract introduced in the current
development work. The game client will begin downloading and presenting these
assets during a later implementation step; until then, local committed assets
remain the only runtime source.

## Release contract

The active manifest is served from:

```text
https://somefrontier.space/game-assets/manifest.json
```

It is a short-lived document that points to immutable, versioned files. A
manifest has this shape:

```json
{
  "schema_version": 1,
  "release_id": "2026-08-05-0001",
  "channel": "stable",
  "game_compatibility": {
    "min_version": "0.1.0",
    "max_version": null
  },
  "asset_root": "releases/2026-08-05-0001",
  "assets": [
    {
      "path": "audio/ui/click.ogg",
      "url": "releases/2026-08-05-0001/audio/ui/click.ogg",
      "bytes": 18432,
      "sha256": "<64 lowercase hexadecimal characters>",
      "content_type": "audio/ogg"
    }
  ]
}
```

The client only accepts relative `audio/` paths, positive byte counts, valid
SHA-256 digests, and URLs that stay below the declared `asset_root`. Release
files are never overwritten in place. A new release is staged completely,
validated, and then made active as one publication step.

The manifest and files are transport data, not content-pack definitions. A
missing or unavailable optional release must not prevent the game from loading
its local content packs or saves. Manifest signing is planned for a later
delivery phase; checksum validation in this phase detects incomplete or
corrupted downloads but does not replace signature verification.

## Local cache

Downloaded files belong in the per-user Some Frontier cache/config area, under a
release-specific directory. They must be written to a temporary file, checked
for the declared size and digest, and atomically renamed only after validation.
The repository's `assets/audio/` directory remains the place for committed
shared fallback audio, not a mutable download cache.

## Failure behavior

The game should show checking, downloading, verifying, ready, and failed states
during startup. Remote audio is optional: network errors, incompatible
manifests, unsafe paths, and checksum failures should produce a useful warning
and allow offline startup using whatever local or previously verified assets are
available.
