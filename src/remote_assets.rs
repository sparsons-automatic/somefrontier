#![allow(dead_code)] // The cache API is consumed by the following download/startup tasks.

use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};

const MANIFEST_SCHEMA_VERSION: u32 = 1;
const MAX_ASSET_BYTES: u64 = 16 * 1024 * 1024;
const MAX_RELEASE_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RemoteAssetManifest {
    pub schema_version: u32,
    pub release_id: String,
    pub channel: String,
    pub game_compatibility: GameCompatibility,
    pub asset_root: String,
    pub assets: Vec<RemoteAssetEntry>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct GameCompatibility {
    pub min_version: String,
    pub max_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RemoteAssetEntry {
    pub path: String,
    pub url: String,
    pub bytes: u64,
    pub sha256: String,
    pub content_type: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheWrite {
    Hit,
    Written,
}

#[derive(Debug, Clone)]
pub struct RemoteAssetCache {
    root: PathBuf,
}

impl RemoteAssetCache {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn release_dir(&self, manifest: &RemoteAssetManifest) -> Result<PathBuf, String> {
        validate_manifest(manifest)?;
        Ok(self.root.join(&manifest.release_id))
    }

    pub fn asset_path(
        &self,
        manifest: &RemoteAssetManifest,
        entry: &RemoteAssetEntry,
    ) -> Result<PathBuf, String> {
        validate_manifest(manifest)?;
        validate_entry(manifest, entry)?;
        Ok(self
            .root
            .join(&manifest.release_id)
            .join(Path::new(&entry.path)))
    }

    pub fn has_verified_asset(
        &self,
        manifest: &RemoteAssetManifest,
        entry: &RemoteAssetEntry,
    ) -> Result<bool, String> {
        let path = self.asset_path(manifest, entry)?;
        if !path.is_file() {
            return Ok(false);
        }
        let metadata =
            fs::metadata(&path).map_err(|error| format!("Could not inspect cache: {error}"))?;
        if metadata.len() != entry.bytes {
            return Ok(false);
        }
        let bytes = fs::read(&path).map_err(|error| format!("Could not read cache: {error}"))?;
        Ok(sha256_hex(&bytes) == entry.sha256)
    }

    pub fn write_verified<R: Read>(
        &self,
        manifest: &RemoteAssetManifest,
        entry: &RemoteAssetEntry,
        mut reader: R,
    ) -> Result<CacheWrite, String> {
        if self.has_verified_asset(manifest, entry)? {
            return Ok(CacheWrite::Hit);
        }

        let destination = self.asset_path(manifest, entry)?;
        let parent = destination
            .parent()
            .ok_or_else(|| "Remote asset has no cache parent".to_string())?;
        fs::create_dir_all(parent)
            .map_err(|error| format!("Could not create asset cache: {error}"))?;
        let partial = partial_path(&destination);
        let _ = fs::remove_file(&partial);

        let result = (|| {
            let mut output = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&partial)
                .map_err(|error| format!("Could not create partial asset: {error}"))?;
            let mut hasher = Sha256::new();
            let mut buffer = [0_u8; 16 * 1024];
            let mut total = 0_u64;

            loop {
                let read = reader
                    .read(&mut buffer)
                    .map_err(|error| format!("Could not download asset: {error}"))?;
                if read == 0 {
                    break;
                }
                total = total
                    .checked_add(read as u64)
                    .ok_or_else(|| "Remote asset size overflowed".to_string())?;
                if total > entry.bytes || total > MAX_ASSET_BYTES {
                    return Err("Remote asset exceeded its declared size".to_string());
                }
                output
                    .write_all(&buffer[..read])
                    .map_err(|error| format!("Could not cache asset: {error}"))?;
                hasher.update(&buffer[..read]);
            }

            if total != entry.bytes {
                return Err(format!(
                    "Remote asset size mismatch: expected {}, received {}",
                    entry.bytes, total
                ));
            }
            let digest = format!("{:x}", hasher.finalize());
            if digest != entry.sha256 {
                return Err("Remote asset checksum mismatch".to_string());
            }
            output
                .sync_all()
                .map_err(|error| format!("Could not flush cached asset: {error}"))?;
            fs::rename(&partial, &destination)
                .map_err(|error| format!("Could not finalize cached asset: {error}"))?;
            Ok(CacheWrite::Written)
        })();

        if result.is_err() {
            let _ = fs::remove_file(&partial);
        }
        result
    }

    pub fn cleanup_partials(&self, manifest: &RemoteAssetManifest) -> Result<usize, String> {
        let release_dir = self.release_dir(manifest)?;
        if !release_dir.exists() {
            return Ok(0);
        }
        remove_partials(&release_dir)
            .map_err(|error| format!("Could not clean asset cache: {error}"))
    }
}

pub fn parse_manifest(text: &str) -> Result<RemoteAssetManifest, String> {
    let manifest: RemoteAssetManifest = serde_json::from_str(text)
        .map_err(|error| format!("Could not parse remote asset manifest: {error}"))?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

pub fn validate_manifest(manifest: &RemoteAssetManifest) -> Result<(), String> {
    if manifest.schema_version != MANIFEST_SCHEMA_VERSION {
        return Err(format!(
            "Unsupported remote asset manifest schema: {}",
            manifest.schema_version
        ));
    }
    if !valid_release_id(&manifest.release_id) {
        return Err("Remote asset release ID is unsafe".to_string());
    }
    if manifest.channel.trim().is_empty()
        || manifest.game_compatibility.min_version.trim().is_empty()
    {
        return Err("Remote asset manifest has incomplete compatibility metadata".to_string());
    }
    let expected_root = format!("releases/{}", manifest.release_id);
    if manifest.asset_root != expected_root {
        return Err("Remote asset root does not match release ID".to_string());
    }

    let mut paths = HashSet::new();
    let mut total_bytes = 0_u64;
    for entry in &manifest.assets {
        validate_entry(manifest, entry)?;
        if !paths.insert(entry.path.as_str()) {
            return Err(format!("Remote asset path is duplicated: {}", entry.path));
        }
        total_bytes = total_bytes
            .checked_add(entry.bytes)
            .ok_or_else(|| "Remote asset release size overflowed".to_string())?;
    }
    if total_bytes > MAX_RELEASE_BYTES {
        return Err("Remote asset release is too large".to_string());
    }
    Ok(())
}

fn validate_entry(manifest: &RemoteAssetManifest, entry: &RemoteAssetEntry) -> Result<(), String> {
    if !entry.path.starts_with("audio/")
        || entry.path.contains('\\')
        || entry.path.split('/').any(|part| part.is_empty())
        || !safe_relative_path(&entry.path)
    {
        return Err(format!("Remote asset path is unsafe: {}", entry.path));
    }
    if entry.url != format!("{}/{}", manifest.asset_root, entry.path) {
        return Err(format!(
            "Remote asset URL does not match its path: {}",
            entry.path
        ));
    }
    if entry.bytes == 0 || entry.bytes > MAX_ASSET_BYTES {
        return Err(format!("Remote asset has an invalid size: {}", entry.path));
    }
    if entry.sha256.len() != 64
        || !entry
            .sha256
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(format!(
            "Remote asset has an invalid SHA-256: {}",
            entry.path
        ));
    }
    if entry.content_type != "audio/ogg" {
        return Err(format!(
            "Remote asset has an unsupported type: {}",
            entry.path
        ));
    }
    Ok(())
}

fn safe_relative_path(value: &str) -> bool {
    let path = Path::new(value);
    !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn valid_release_id(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn partial_path(destination: &Path) -> PathBuf {
    PathBuf::from(format!("{}.part", destination.display()))
}

fn remove_partials(root: &Path) -> std::io::Result<usize> {
    let mut removed = 0;
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            removed += remove_partials(&path)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == "part")
        {
            fs::remove_file(path)?;
            removed += 1;
        }
    }
    Ok(removed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io::Cursor, time::SystemTime};

    fn manifest(release_id: &str, bytes: &[u8]) -> RemoteAssetManifest {
        RemoteAssetManifest {
            schema_version: 1,
            release_id: release_id.to_string(),
            channel: "stable".to_string(),
            game_compatibility: GameCompatibility {
                min_version: "0.1.0".to_string(),
                max_version: None,
            },
            asset_root: format!("releases/{release_id}"),
            assets: vec![RemoteAssetEntry {
                path: "audio/ui/click.ogg".to_string(),
                url: format!("releases/{release_id}/audio/ui/click.ogg"),
                bytes: bytes.len() as u64,
                sha256: sha256_hex(bytes),
                content_type: "audio/ogg".to_string(),
            }],
        }
    }

    fn temp_root(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("some-frontier-remote-assets-{label}-{nanos}"));
        fs::create_dir_all(&path).expect("temp cache should be created");
        path
    }

    #[test]
    fn rejects_unsafe_paths_and_duplicate_entries() {
        let mut unsafe_manifest = manifest("safe-release", b"hello");
        unsafe_manifest.assets[0].path = "audio/../secret.ogg".to_string();
        assert!(validate_manifest(&unsafe_manifest).is_err());

        let mut duplicate_manifest = manifest("duplicate-release", b"hello");
        duplicate_manifest
            .assets
            .push(duplicate_manifest.assets[0].clone());
        assert!(validate_manifest(&duplicate_manifest).is_err());
    }

    #[test]
    fn parses_manifest_json() {
        let text = r#"
        {
          "schema_version": 1,
          "release_id": "release-json",
          "channel": "stable",
          "game_compatibility": { "min_version": "0.1.0", "max_version": null },
          "asset_root": "releases/release-json",
          "assets": [{
            "path": "audio/ui/click.ogg",
            "url": "releases/release-json/audio/ui/click.ogg",
            "bytes": 5,
            "sha256": "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
            "content_type": "audio/ogg"
          }]
        }
        "#;
        assert_eq!(parse_manifest(text).unwrap().release_id, "release-json");
    }

    #[test]
    fn writes_verified_asset_and_reuses_cache_hit() {
        let root = temp_root("cache-hit");
        let data = b"hello";
        let manifest = manifest("release-one", data);
        let entry = &manifest.assets[0];
        let cache = RemoteAssetCache::new(&root);

        assert_eq!(
            cache
                .write_verified(&manifest, entry, Cursor::new(data))
                .unwrap(),
            CacheWrite::Written
        );
        assert!(cache.has_verified_asset(&manifest, entry).unwrap());
        assert_eq!(
            cache
                .write_verified(&manifest, entry, Cursor::new(b"wrong"))
                .unwrap(),
            CacheWrite::Hit
        );
        assert!(!partial_path(&cache.asset_path(&manifest, entry).unwrap()).exists());
        fs::remove_dir_all(root).expect("temp cache should be removed");
    }

    #[test]
    fn rejects_checksum_and_size_mismatches_without_leaving_partials() {
        let root = temp_root("invalid-download");
        let data = b"hello";
        let mut bad_manifest = manifest("release-two", data);
        bad_manifest.assets[0].sha256 = sha256_hex(b"wrong");
        let cache = RemoteAssetCache::new(&root);
        assert!(cache
            .write_verified(&bad_manifest, &bad_manifest.assets[0], Cursor::new(data))
            .is_err());
        assert!(!cache
            .release_dir(&bad_manifest)
            .unwrap()
            .join("audio/ui/click.ogg.part")
            .exists());

        let valid_manifest = manifest("release-three", data);
        assert_eq!(
            cache
                .write_verified(
                    &valid_manifest,
                    &valid_manifest.assets[0],
                    Cursor::new(data)
                )
                .unwrap(),
            CacheWrite::Written
        );
        let mut truncated = manifest("release-four", b"longer");
        truncated.assets[0].bytes = 6;
        assert!(cache
            .write_verified(&truncated, &truncated.assets[0], Cursor::new(data))
            .is_err());
        fs::remove_dir_all(root).expect("temp cache should be removed");
    }

    #[test]
    fn cleans_partial_files_and_separates_releases() {
        let root = temp_root("release-separation");
        let cache = RemoteAssetCache::new(&root);
        let first = manifest("release-a", b"first");
        let second = manifest("release-b", b"second");
        cache
            .write_verified(&first, &first.assets[0], Cursor::new(b"first"))
            .unwrap();
        cache
            .write_verified(&second, &second.assets[0], Cursor::new(b"second"))
            .unwrap();
        assert_ne!(
            cache.asset_path(&first, &first.assets[0]).unwrap(),
            cache.asset_path(&second, &second.assets[0]).unwrap()
        );
        let partial = partial_path(&cache.asset_path(&first, &first.assets[0]).unwrap());
        fs::write(&partial, b"interrupted").expect("partial should be written");
        assert_eq!(cache.cleanup_partials(&first).unwrap(), 1);
        assert!(!partial.exists());
        fs::remove_dir_all(root).expect("temp cache should be removed");
    }
}
