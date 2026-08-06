# Remote Assets

Some Frontier can consume optional runtime assets published by the project's
asset host. The first delivery phase is limited to audio; remote content packs
are a separate future feature.

This document defines the delivery contract, local cache behavior, and startup
status display. The game can prepare optional remote audio during startup;
audio playback covers interface and combat cues. Local committed assets remain
the fallback source whenever remote delivery is unavailable.

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

Downloaded files belong in the per-user Some Frontier cache/config area, under
`remote-assets/<release_id>/`. They must be written to a temporary `.part`
file, checked for the declared size and digest, and atomically renamed only
after validation. A verified file can be reused when its size and SHA-256
digest still match the manifest. Incomplete temporary files can be cleaned up
without removing verified releases.
The repository's `assets/audio/` directory remains the place for committed
shared fallback audio, not a mutable download cache.

## Client behavior

The remote client uses the public HTTPS manifest by default. For local testing,
set `SOME_FRONTIER_ASSET_MANIFEST_URL` to another endpoint. Plain HTTP is
rejected unless `SOME_FRONTIER_ALLOW_INSECURE_ASSET_HTTP=1` (or `true`) is also
set; production configuration should continue to use HTTPS.

The client fetches the manifest, selects the requested audio entries, skips
files that already match the cached size and digest, and downloads only missing
or invalid files. Network and server failures that may be temporary receive a
small bounded retry with backoff. Manifest validation, unsupported responses,
unsafe paths, and checksum failures are reported without accepting the file.
Downloads run on a background worker so they do not block the game frame loop.
During startup, the game shows checking, downloading, verifying, ready, failed,
or offline status with the current file and overall asset progress. The game
does not wait indefinitely: if remote preparation takes too long, startup
continues with local assets while the worker is allowed to finish separately.
Available cues include interface actions, rotating weapon-fire variants, shield
impacts, hull impacts, and destruction explosions. Missing optional cues are
skipped without preventing startup.

For local tests, use the deterministic fake-transport tests rather than the
public service:

```text
cargo test --all-targets --all-features
```

These tests cover manifest parsing, endpoint security, retries, missing-file
downloads, cache reuse, and the existing size/checksum safeguards.

## Release verification checklist

Before treating a remote audio release as ready, verify the following:

- The active manifest uses the intended release ID and points only to immutable
  files below that release namespace.
- The webpage release validator passes every file's path, content type, byte
  count, and SHA-256 checksum.
- `scripts/verify-remote-audio-release.sh` passes against the public manifest
  URL, including the manifest's no-cache header and each asset's immutable
  cache header.
- A clean game cache reaches startup ready after downloading and verifying the
  release, while a warm cache reuses the verified files.
- An unavailable endpoint, interrupted download, or modified cached file does
  not prevent offline startup and does not replace a verified file.
- The selected Kenney sound files and their CC0 license and attribution record
  remain present in the companion release repository.

The current public release can be checked with:

```text
./scripts/verify-remote-audio-release.sh
```

## Current audio release

The current release contains optional interface cues—click, select, confirm,
back, error, open, and close—plus small, large, and retro laser effects,
force-field shield impacts, metal hull impacts, and destruction explosions.
These sounds come from Kenney's `Interface Sounds` and `Sci-Fi Sounds` packs.
They are distributed under the Creative Commons Zero (CC0) license; see the
release attribution and license files for details.

## Failure behavior

During startup, remote audio is optional: network errors, incompatible
manifests, unsafe paths, and checksum failures show a useful status and allow
offline startup using whatever local or previously verified assets are
available. A warm cache skips verified downloads and reaches the ready state
quickly.
