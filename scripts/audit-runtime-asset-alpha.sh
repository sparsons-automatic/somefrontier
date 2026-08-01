#!/usr/bin/env bash
set -euo pipefail

if ! command -v magick >/dev/null 2>&1; then
  echo "ImageMagick 'magick' is required to audit runtime asset alpha." >&2
  exit 2
fi

status=0
while IFS= read -r asset; do
  read -r alpha_min alpha_max < <(
    magick identify -quiet -format '%[fx:minima.a] %[fx:maxima.a]\n' "$asset"
  )
  if [[ "$alpha_min" == "$alpha_max" && "$alpha_max" == "1" ]]; then
    echo "Runtime object asset needs transparency: $asset (alpha_min=$alpha_min alpha_max=$alpha_max)" >&2
    status=1
  fi
done < <(
  find content/packs -type f \
    \( -path '*/assets/planets/*.png' \
    -o -path '*/assets/ships/*.png' \
    -o -path '*/assets/stations/*.png' \) \
    | sort
)

exit "$status"
