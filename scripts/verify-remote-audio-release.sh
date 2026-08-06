#!/usr/bin/env bash
set -euo pipefail

manifest_url="${1:-https://somefrontier.space/game-assets/manifest.json}"

command -v curl >/dev/null || { echo "curl is required" >&2; exit 1; }
command -v jq >/dev/null || { echo "jq is required" >&2; exit 1; }
command -v sha256sum >/dev/null || { echo "sha256sum is required" >&2; exit 1; }

work_dir="$(mktemp -d "${TMPDIR:-/tmp}/some-frontier-remote-audio.XXXXXX")"
trap 'rm -rf "$work_dir"' EXIT

manifest_file="${work_dir}/manifest.json"
manifest_headers="${work_dir}/manifest.headers"
curl --fail --silent --show-error --location \
  --dump-header "${manifest_headers}" \
  "${manifest_url}" \
  --output "${manifest_file}"

manifest_cache_control="$(awk 'BEGIN { IGNORECASE = 1 } /^cache-control:/ { sub(/^[^:]*:[[:space:]]*/, ""); print }' "${manifest_headers}" | tail -n 1)"
[[ "${manifest_cache_control}" == *no-cache* ]] || {
  echo "manifest is not marked no-cache: ${manifest_cache_control}" >&2
  exit 1
}

release_id="$(jq -r '.release_id // empty' "${manifest_file}")"
asset_root="$(jq -r '.asset_root // empty' "${manifest_file}")"
asset_count="$(jq '.assets | length' "${manifest_file}")"
[[ -n "${release_id}" && "${asset_root}" == "releases/${release_id}" ]] || {
  echo "manifest has invalid release metadata" >&2
  exit 1
}
[[ "${asset_count}" -gt 0 ]] || {
  echo "manifest contains no assets" >&2
  exit 1
}

manifest_root="${manifest_url%/*}"
while IFS=$'\t' read -r asset_path asset_url expected_bytes expected_sha256 expected_type; do
  [[ -n "${asset_path}" ]] || continue
  asset_endpoint="${manifest_root}/${asset_url}"
  asset_headers="${work_dir}/asset.headers"
  asset_file="${work_dir}/asset.ogg"

  curl --fail --silent --show-error --location \
    --dump-header "${asset_headers}" \
    "${asset_endpoint}" \
    --output "${asset_file}"

  asset_cache_control="$(awk 'BEGIN { IGNORECASE = 1 } /^cache-control:/ { sub(/^[^:]*:[[:space:]]*/, ""); print }' "${asset_headers}" | tail -n 1)"
  [[ "${asset_cache_control}" == *immutable* ]] || {
    echo "asset is not immutable: ${asset_path}" >&2
    exit 1
  }
  asset_content_type="$(awk 'BEGIN { IGNORECASE = 1 } /^content-type:/ { sub(/^[^:]*:[[:space:]]*/, ""); print }' "${asset_headers}" | tail -n 1 | tr -d '\r')"
  [[ "${asset_content_type}" == "${expected_type}" ]] || {
    echo "content type mismatch for ${asset_path}: ${asset_content_type}" >&2
    exit 1
  }

  actual_bytes="$(wc -c < "${asset_file}")"
  [[ "${actual_bytes}" == "${expected_bytes}" ]] || {
    echo "byte count mismatch for ${asset_path}" >&2
    exit 1
  }
  actual_sha256="$(sha256sum "${asset_file}" | awk '{print $1}')"
  [[ "${actual_sha256}" == "${expected_sha256}" ]] || {
    echo "checksum mismatch for ${asset_path}" >&2
    exit 1
  }
done < <(jq -r '.assets[] | [.path, .url, (.bytes|tostring), .sha256, .content_type] | @tsv' "${manifest_file}")

echo "Verified release ${release_id}: ${asset_count} remote audio asset(s)."
