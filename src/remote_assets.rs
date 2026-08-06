#![allow(dead_code)] // The cache API is consumed by the following download/startup tasks.

use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    env,
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

const MANIFEST_SCHEMA_VERSION: u32 = 1;
const MAX_ASSET_BYTES: u64 = 16 * 1024 * 1024;
const MAX_RELEASE_BYTES: u64 = 64 * 1024 * 1024;
const MAX_MANIFEST_BYTES: u64 = 1024 * 1024;
const DEFAULT_MANIFEST_URL: &str = "https://somefrontier.space/game-assets/manifest.json";
const MANIFEST_URL_ENV: &str = "SOME_FRONTIER_ASSET_MANIFEST_URL";
const ALLOW_INSECURE_HTTP_ENV: &str = "SOME_FRONTIER_ALLOW_INSECURE_ASSET_HTTP";
const DEFAULT_RETRY_COUNT: u32 = 2;
const DEFAULT_RETRY_DELAY: Duration = Duration::from_millis(150);

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

pub struct RemoteResponse {
    pub status: u16,
    pub content_type: Option<String>,
    body: Box<dyn Read + Send>,
}

impl RemoteResponse {
    pub fn new(
        status: u16,
        content_type: Option<String>,
        body: impl Read + Send + 'static,
    ) -> Self {
        Self {
            status,
            content_type,
            body: Box::new(body),
        }
    }
}

pub trait RemoteAssetTransport: Send + Sync + 'static {
    fn get(&self, url: &str) -> Result<RemoteResponse, String>;
}

#[derive(Default)]
pub struct UreqTransport;

impl RemoteAssetTransport for UreqTransport {
    fn get(&self, url: &str) -> Result<RemoteResponse, String> {
        let response = match ureq::get(url).call() {
            Ok(response) => response,
            Err(ureq::Error::Status(_status, response)) => response,
            Err(error) => return Err(format!("Remote asset request failed: {error}")),
        };
        let status = response.status();
        let content_type = response.header("Content-Type").map(str::to_string);
        Ok(RemoteResponse::new(
            status,
            content_type,
            response.into_reader(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct RemoteAssetClient {
    manifest_url: String,
    allow_insecure_http: bool,
    retry_count: u32,
    retry_delay: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadReport {
    pub release_id: String,
    pub total_assets: usize,
    pub cache_hits: usize,
    pub downloaded_assets: usize,
    pub ready_paths: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteAssetPhase {
    Checking,
    Downloading,
    Verifying,
    Ready,
    Failed,
    Offline,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteAssetProgress {
    pub phase: RemoteAssetPhase,
    pub current_path: Option<String>,
    pub completed_assets: usize,
    pub total_assets: usize,
    pub current_bytes: u64,
    pub current_total_bytes: u64,
    pub message: String,
}

pub type RemoteAssetProgressHandle = Arc<Mutex<RemoteAssetProgress>>;

impl RemoteAssetProgress {
    pub fn checking() -> Self {
        Self {
            phase: RemoteAssetPhase::Checking,
            current_path: None,
            completed_assets: 0,
            total_assets: 0,
            current_bytes: 0,
            current_total_bytes: 0,
            message: "Checking remote audio ...".to_string(),
        }
    }
}

impl Default for RemoteAssetClient {
    fn default() -> Self {
        Self {
            manifest_url: DEFAULT_MANIFEST_URL.to_string(),
            allow_insecure_http: false,
            retry_count: DEFAULT_RETRY_COUNT,
            retry_delay: DEFAULT_RETRY_DELAY,
        }
    }
}

impl RemoteAssetClient {
    pub fn from_environment() -> Result<Self, String> {
        let manifest_url =
            env::var(MANIFEST_URL_ENV).unwrap_or_else(|_| DEFAULT_MANIFEST_URL.to_string());
        let allow_insecure_http = env::var(ALLOW_INSECURE_HTTP_ENV)
            .is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
        Self::with_options(
            manifest_url,
            allow_insecure_http,
            DEFAULT_RETRY_COUNT,
            DEFAULT_RETRY_DELAY,
        )
    }

    pub fn new(manifest_url: impl Into<String>) -> Result<Self, String> {
        Self::with_options(
            manifest_url,
            false,
            DEFAULT_RETRY_COUNT,
            DEFAULT_RETRY_DELAY,
        )
    }

    pub fn with_options(
        manifest_url: impl Into<String>,
        allow_insecure_http: bool,
        retry_count: u32,
        retry_delay: Duration,
    ) -> Result<Self, String> {
        let manifest_url = manifest_url.into();
        validate_endpoint(&manifest_url, allow_insecure_http)?;
        Ok(Self {
            manifest_url,
            allow_insecure_http,
            retry_count,
            retry_delay,
        })
    }

    pub fn manifest_url(&self) -> &str {
        &self.manifest_url
    }

    pub fn allow_insecure_http(&self) -> bool {
        self.allow_insecure_http
    }

    pub fn fetch_manifest<T: RemoteAssetTransport>(
        &self,
        transport: &T,
    ) -> Result<RemoteAssetManifest, String> {
        self.fetch_manifest_with_progress(transport, None)
    }

    fn fetch_manifest_with_progress<T: RemoteAssetTransport>(
        &self,
        transport: &T,
        progress: Option<&RemoteAssetProgressHandle>,
    ) -> Result<RemoteAssetManifest, String> {
        update_progress(progress, |state| {
            state.phase = RemoteAssetPhase::Checking;
            state.message = "Checking remote audio manifest ...".to_string();
        });
        let mut response = self.request_with_retry(transport, &self.manifest_url)?;
        if !response
            .content_type
            .as_deref()
            .is_some_and(|value| value.to_ascii_lowercase().starts_with("application/json"))
        {
            return Err("Remote asset manifest did not return JSON".to_string());
        }
        let body = read_bounded(&mut response.body, MAX_MANIFEST_BYTES)?;
        let manifest = parse_manifest(
            std::str::from_utf8(&body)
                .map_err(|error| format!("Remote asset manifest was not UTF-8: {error}"))?,
        )?;
        validate_game_compatibility(&manifest, env!("CARGO_PKG_VERSION"))?;
        Ok(manifest)
    }

    pub fn download_requested<T: RemoteAssetTransport>(
        &self,
        transport: &T,
        cache: &RemoteAssetCache,
        requested_paths: &[String],
    ) -> Result<DownloadReport, String> {
        self.download_requested_with_progress(transport, cache, requested_paths, None)
    }

    pub fn download_requested_with_progress<T: RemoteAssetTransport>(
        &self,
        transport: &T,
        cache: &RemoteAssetCache,
        requested_paths: &[String],
        progress: Option<&RemoteAssetProgressHandle>,
    ) -> Result<DownloadReport, String> {
        let result = self.download_requested_inner(transport, cache, requested_paths, progress);
        if let Err(error) = &result {
            update_progress(progress, |state| {
                state.phase = RemoteAssetPhase::Failed;
                state.message = error.clone();
            });
        }
        result
    }

    fn download_requested_inner<T: RemoteAssetTransport>(
        &self,
        transport: &T,
        cache: &RemoteAssetCache,
        requested_paths: &[String],
        progress: Option<&RemoteAssetProgressHandle>,
    ) -> Result<DownloadReport, String> {
        let manifest = self.fetch_manifest_with_progress(transport, progress)?;
        let requested = requested_paths.iter().collect::<HashSet<_>>();
        let entries = manifest
            .assets
            .iter()
            .filter(|entry| requested_paths.is_empty() || requested.contains(&entry.path))
            .collect::<Vec<_>>();
        if !requested_paths.is_empty() && entries.len() != requested.len() {
            return Err("Remote asset request included an unknown path".to_string());
        }

        update_progress(progress, |state| {
            state.total_assets = entries.len();
            state.message = format!("Preparing {} remote audio asset(s) ...", entries.len());
        });

        let mut cache_hits = 0;
        let mut downloaded_assets = 0;
        let mut ready_paths = Vec::with_capacity(entries.len());
        for entry in entries.iter().copied() {
            update_progress(progress, |state| {
                state.phase = RemoteAssetPhase::Downloading;
                state.current_path = Some(entry.path.clone());
                state.current_bytes = 0;
                state.current_total_bytes = entry.bytes;
                state.message = format!("Downloading {} ...", entry.path);
            });
            let url = resolve_asset_url(&self.manifest_url, &entry.url)?;
            if cache.has_verified_asset(&manifest, entry)? {
                cache_hits += 1;
                ready_paths.push(entry.path.clone());
                update_progress(progress, |state| {
                    state.completed_assets += 1;
                    state.current_bytes = entry.bytes;
                    state.message = format!("Cached {}", entry.path);
                });
                continue;
            }
            let mut response = self.request_with_retry(transport, &url)?;
            if response.status != 200 {
                return Err(format!("Remote asset returned HTTP {}", response.status));
            }
            if response
                .content_type
                .as_deref()
                .is_some_and(|value| !value.to_ascii_lowercase().starts_with(&entry.content_type))
            {
                return Err(format!(
                    "Remote asset returned an unexpected type: {}",
                    entry.path
                ));
            }
            update_progress(progress, |state| {
                state.phase = RemoteAssetPhase::Verifying;
                state.message = format!("Verifying {} ...", entry.path);
            });
            let mut reader = ProgressReader {
                reader: &mut response.body,
                progress,
            };
            cache.write_verified(&manifest, entry, &mut reader)?;
            downloaded_assets += 1;
            ready_paths.push(entry.path.clone());
            update_progress(progress, |state| {
                state.completed_assets += 1;
                state.current_bytes = entry.bytes;
                state.message = format!("Ready {}", entry.path);
            });
        }
        update_progress(progress, |state| {
            state.phase = RemoteAssetPhase::Ready;
            state.message = if entries.is_empty() {
                "No remote audio assets requested".to_string()
            } else {
                format!("Remote audio ready ({} asset(s))", entries.len())
            };
        });
        Ok(DownloadReport {
            release_id: manifest.release_id,
            total_assets: entries.len(),
            cache_hits,
            downloaded_assets,
            ready_paths,
        })
    }

    pub fn spawn_download<T: RemoteAssetTransport>(
        &self,
        transport: T,
        cache: RemoteAssetCache,
        requested_paths: Vec<String>,
    ) -> JoinHandle<Result<DownloadReport, String>> {
        self.spawn_download_with_progress(transport, cache, requested_paths, None)
    }

    pub fn spawn_download_with_progress<T: RemoteAssetTransport>(
        &self,
        transport: T,
        cache: RemoteAssetCache,
        requested_paths: Vec<String>,
        progress: Option<RemoteAssetProgressHandle>,
    ) -> JoinHandle<Result<DownloadReport, String>> {
        let client = self.clone();
        thread::spawn(move || {
            client.download_requested_with_progress(
                &transport,
                &cache,
                &requested_paths,
                progress.as_ref(),
            )
        })
    }

    fn request_with_retry<T: RemoteAssetTransport>(
        &self,
        transport: &T,
        url: &str,
    ) -> Result<RemoteResponse, String> {
        for attempt in 0..=self.retry_count {
            match transport.get(url) {
                Ok(response) if response.status == 200 || !retryable_status(response.status) => {
                    return Ok(response);
                }
                Ok(_) | Err(_) if attempt < self.retry_count => {
                    thread::sleep(self.retry_delay.saturating_mul(attempt + 1));
                }
                Ok(response) => return Ok(response),
                Err(error) => return Err(error),
            }
        }
        Err("Remote asset request exhausted retries".to_string())
    }
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

pub fn validate_game_compatibility(
    manifest: &RemoteAssetManifest,
    game_version: &str,
) -> Result<(), String> {
    let game = version_tuple(game_version)?;
    let minimum = version_tuple(&manifest.game_compatibility.min_version)?;
    if game < minimum {
        return Err(format!(
            "Remote audio requires game version {} or newer",
            manifest.game_compatibility.min_version
        ));
    }
    if let Some(maximum) = manifest.game_compatibility.max_version.as_deref() {
        if game > version_tuple(maximum)? {
            return Err(format!(
                "Remote audio supports game versions through {maximum}"
            ));
        }
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

fn version_tuple(value: &str) -> Result<[u64; 3], String> {
    let mut parts = value.split('.');
    let parsed = [parts.next(), parts.next(), parts.next()]
        .into_iter()
        .map(|part| {
            part.and_then(|part| {
                part.split_once('-')
                    .map(|(number, _)| number)
                    .or(Some(part))
            })
            .ok_or_else(|| format!("Invalid game version: {value}"))
            .and_then(|part| {
                part.parse::<u64>()
                    .map_err(|_| format!("Invalid game version: {value}"))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if parts.next().is_some() || parsed.len() != 3 {
        return Err(format!("Invalid game version: {value}"));
    }
    Ok([parsed[0], parsed[1], parsed[2]])
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

struct ProgressReader<'a> {
    reader: &'a mut dyn Read,
    progress: Option<&'a RemoteAssetProgressHandle>,
}

impl Read for ProgressReader<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let read = self.reader.read(buffer)?;
        if read > 0 {
            update_progress(self.progress, |state| {
                state.current_bytes = state.current_bytes.saturating_add(read as u64);
            });
        }
        Ok(read)
    }
}

fn update_progress(
    progress: Option<&RemoteAssetProgressHandle>,
    update: impl FnOnce(&mut RemoteAssetProgress),
) {
    if let Some(progress) = progress {
        if let Ok(mut state) = progress.lock() {
            update(&mut state);
        }
    }
}

fn validate_endpoint(value: &str, allow_insecure_http: bool) -> Result<(), String> {
    if value.is_empty() || value.chars().any(char::is_whitespace) {
        return Err("Remote asset endpoint is empty or contains whitespace".to_string());
    }
    let https = value.starts_with("https://") && value.len() > "https://".len();
    let http = value.starts_with("http://") && value.len() > "http://".len();
    if !(https || allow_insecure_http && http) {
        return Err("Remote asset endpoint must use HTTPS".to_string());
    }
    if value.contains('#') || value.contains('?') {
        return Err("Remote asset endpoint cannot contain a query or fragment".to_string());
    }
    Ok(())
}

fn resolve_asset_url(manifest_url: &str, relative_url: &str) -> Result<String, String> {
    let base = manifest_url
        .rsplit_once('/')
        .map(|(base, _)| base)
        .ok_or_else(|| "Remote asset endpoint has no base path".to_string())?;
    Ok(format!("{base}/{relative_url}"))
}

fn read_bounded(reader: &mut dyn Read, limit: u64) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    reader
        .take(limit.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| format!("Could not read remote asset response: {error}"))?;
    if bytes.len() as u64 > limit {
        return Err("Remote asset response exceeded its size limit".to_string());
    }
    Ok(bytes)
}

fn retryable_status(status: u16) -> bool {
    status == 408 || status == 429 || (500..=599).contains(&status)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::VecDeque, fs, io::Cursor, sync::Mutex, time::SystemTime};

    struct FakeTransport {
        responses: Mutex<VecDeque<Result<RemoteResponse, String>>>,
    }

    impl FakeTransport {
        fn new(responses: Vec<Result<RemoteResponse, String>>) -> Self {
            Self {
                responses: Mutex::new(responses.into()),
            }
        }
    }

    impl RemoteAssetTransport for FakeTransport {
        fn get(&self, _url: &str) -> Result<RemoteResponse, String> {
            self.responses
                .lock()
                .expect("fake response queue should not be poisoned")
                .pop_front()
                .unwrap_or_else(|| Err("fake transport ran out of responses".to_string()))
        }
    }

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

    #[test]
    fn downloads_missing_assets_and_reuses_verified_cache() {
        let root = temp_root("remote-download");
        let manifest_json = r#"{
          "schema_version": 1,
          "release_id": "download-release",
          "channel": "stable",
          "game_compatibility": { "min_version": "0.1.0", "max_version": null },
          "asset_root": "releases/download-release",
          "assets": [{
            "path": "audio/ui/click.ogg",
            "url": "releases/download-release/audio/ui/click.ogg",
            "bytes": 5,
            "sha256": "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
            "content_type": "audio/ogg"
          }]
        }"#;
        let client = RemoteAssetClient::with_options(
            "https://example.test/game-assets/manifest.json",
            false,
            0,
            Duration::ZERO,
        )
        .unwrap();
        let cache = RemoteAssetCache::new(&root);
        let progress = Arc::new(Mutex::new(RemoteAssetProgress::checking()));
        let transport = FakeTransport::new(vec![
            Ok(RemoteResponse::new(
                200,
                Some("application/json".to_string()),
                Cursor::new(manifest_json.as_bytes()),
            )),
            Ok(RemoteResponse::new(
                200,
                Some("audio/ogg".to_string()),
                Cursor::new(b"hello"),
            )),
        ]);
        let report = client
            .download_requested_with_progress(&transport, &cache, &[], Some(&progress))
            .expect("fake remote download should succeed");
        assert_eq!(
            report,
            DownloadReport {
                release_id: "download-release".to_string(),
                total_assets: 1,
                cache_hits: 0,
                downloaded_assets: 1,
                ready_paths: vec!["audio/ui/click.ogg".to_string()],
            }
        );

        let manifest = parse_manifest(manifest_json).unwrap();
        assert!(cache
            .has_verified_asset(&manifest, &manifest.assets[0])
            .unwrap());
        let progress = progress.lock().unwrap().clone();
        assert_eq!(progress.phase, RemoteAssetPhase::Ready);
        assert_eq!(progress.completed_assets, 1);
        assert_eq!(progress.current_bytes, 5);
        fs::remove_dir_all(root).expect("temp cache should be removed");
    }

    #[test]
    fn retries_transient_http_failures_but_rejects_insecure_defaults() {
        let manifest_json = r#"{
          "schema_version": 1,
          "release_id": "retry-release",
          "channel": "stable",
          "game_compatibility": { "min_version": "0.1.0", "max_version": null },
          "asset_root": "releases/retry-release",
          "assets": []
        }"#;
        assert!(RemoteAssetClient::new("http://localhost/manifest.json").is_err());
        assert!(RemoteAssetClient::with_options(
            "http://localhost/manifest.json",
            true,
            0,
            Duration::ZERO
        )
        .is_ok());

        let client = RemoteAssetClient::with_options(
            "https://example.test/game-assets/manifest.json",
            false,
            1,
            Duration::ZERO,
        )
        .unwrap();
        let transport = FakeTransport::new(vec![
            Ok(RemoteResponse::new(503, None, Cursor::new(Vec::new()))),
            Ok(RemoteResponse::new(
                200,
                Some("application/json".to_string()),
                Cursor::new(manifest_json.as_bytes()),
            )),
        ]);
        assert_eq!(
            client.fetch_manifest(&transport).unwrap().release_id,
            "retry-release"
        );
    }

    #[test]
    fn rejects_incompatible_game_versions() {
        let mut manifest = manifest("compatibility", b"hello");
        manifest.game_compatibility.min_version = "2.0.0".to_string();
        assert!(validate_game_compatibility(&manifest, "1.9.9").is_err());
        manifest.game_compatibility.min_version = "0.1.0".to_string();
        manifest.game_compatibility.max_version = Some("0.9.0".to_string());
        assert!(validate_game_compatibility(&manifest, "1.0.0").is_err());
        assert!(validate_game_compatibility(&manifest, "0.8.0").is_ok());
    }
}
