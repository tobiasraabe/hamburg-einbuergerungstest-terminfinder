#!/usr/bin/env bash

# This script is an adapted version of
# https://github.com/prefix-dev/pixi/blob/main/install/install.sh.

set -euo pipefail
# Version: v0.0.4

__wrap__() {

VERSION=${FINDER_VERSION:-latest}
FINDER_HOME=${FINDER_HOME:-"$PWD"}
BIN_DIR="$FINDER_HOME"

REPO=tobiasraabe/hamburg-einbuergerungstest-terminfinder
PLATFORM=$(uname -s)
ARCH=${FINDER_ARCH:-$(uname -m)}

if [[ $PLATFORM == "Darwin" ]]; then
  PLATFORM="apple-darwin"
elif [[ $PLATFORM == "Linux" ]]; then
  PLATFORM="unknown-linux-musl"
elif [[ $(uname -o) == "Msys" ]]; then
  PLATFORM="pc-windows-msvc"
fi

if [[ $ARCH == "arm64" ]] || [[ $ARCH == "aarch64" ]]; then
  ARCH="aarch64"
fi



BINARY="finder-${ARCH}-${PLATFORM}"
EXTENSION="tar.gz"
if [[ $(uname -o) == "Msys" ]]; then
  EXTENSION="zip"
fi

if [[ $VERSION == "latest" ]]; then
  DOWNLOAD_URL=https://github.com/${REPO}/releases/latest/download/${BINARY}.${EXTENSION}
else
  DOWNLOAD_URL=https://github.com/${REPO}/releases/download/${VERSION}/${BINARY}.${EXTENSION}
fi

printf "This script will automatically download and install finder (${VERSION}) for you.\nGetting it from this url: $DOWNLOAD_URL\n"

if ! hash curl 2> /dev/null && ! hash wget 2> /dev/null; then
  echo "error: you need either 'curl' or 'wget' installed for this script."
  exit 1
fi

if ! hash tar 2> /dev/null; then
  echo "error: you do not have 'tar' installed which is required for this script."
  exit 1
fi

TEMP_FILE=$(mktemp "${TMPDIR:-/tmp}/.finder_install.XXXXXXXX")

cleanup() {
  rm -f "$TEMP_FILE"
}

trap cleanup EXIT

if hash curl 2> /dev/null; then
  HTTP_CODE=$(curl -SL --progress-bar "$DOWNLOAD_URL" --output "$TEMP_FILE" --write-out "%{http_code}")
  if [[ ${HTTP_CODE} -lt 200 || ${HTTP_CODE} -gt 299 ]]; then
    echo "error: '${DOWNLOAD_URL}' is not available"
    exit 1
  fi
elif hash wget 2> /dev/null; then
  if ! wget -q --show-progress --output-document="$TEMP_FILE" "$DOWNLOAD_URL"; then
    echo "error: '${DOWNLOAD_URL}' is not available"
    exit 1
  fi
fi

# Check that file was correctly created (https://github.com/prefix-dev/pixi/issues/446)
if [[ ! -s $TEMP_FILE ]]; then
  echo "error: temporary file ${TEMP_FILE} not correctly created."
  echo "       As a workaround, you can try set TMPDIR env variable to directory with write permissions."
  exit 1
fi

# Extract finder from the downloaded file
mkdir -p "$BIN_DIR"
if [[ $(uname -o) == "Msys" ]]; then
  unzip "$TEMP_FILE" -d "$BIN_DIR"
else
  tar -xzf "$TEMP_FILE" -C "$BIN_DIR"
  chmod +x "$BIN_DIR/finder"
fi

echo "The 'finder' binary is installed into '${BIN_DIR}'"

}; __wrap__
