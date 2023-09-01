#!/bin/bash

#
# bash <(curl -s https://raw.githubusercontent.com/kamiyaa/joshuto/master/utils/install.sh)
# RELEASE_VER='v0.9.5' bash <(curl -s https://raw.githubusercontent.com/kamiyaa/joshuto/master/utils/install.sh)
#
# testing from fork:
# REPO="kennycallado/joshuto" bash <(curl -s https://raw.githubusercontent.com/kennycallado/joshuto/master/utils/install.sh)
#

set -eo pipefail

# Hardcoded version is used for now
declare -r HARDCODED_VERSION="v0.9.5"

declare -r REPO="${REPO:-"kamiyaa/joshuto"}"
declare -r OS="$(uname -s)"
declare -r ARCH="$(uname -m)"
declare -r INSTALL_PREFIX="${INSTALL_PREFIX:-"$HOME/.local/bin"}"
declare -r RELEASE_VER="${RELEASE_VER:-"$HARDCODED_VERSION"}"

declare ARCHIVE_NAME

if [ "$OS" == "Linux" ]; then
  # TODO: add support for musl
  ARCHIVE_NAME="joshuto-$RELEASE_VER-$ARCH-unknown-linux-gnu"
elif [ "$OS" == "Darwin" ]; then
  # ARCHIVE_NAME="joshuto-$RELEASE_VER-$ARCH-apple-darwin"
  ARCHIVE_NAME="joshuto-v0.9.5-x86_64-apple-darwin"
else
  echo "$OS platform is not supported currently"
  exit 1
fi

readonly ARCHIVE_NAME
if [[ -z "$ARCHIVE_NAME" ]]; then
  echo "RELEASE_VER is not set"
  exit 1
fi

declare -r ARCHIVE_URL="https://github.com/$REPO/releases/download/$RELEASE_VER/$ARCHIVE_NAME.tar.gz"
declare -r CHECKSUM_URL="$ARCHIVE_URL.sha256sum"
echo $ARCHIVE_URL

declare -r DOWNLOAD_DIR="$(mktemp -d)"
declare -r RELEASE_SHA="$(curl -Ls "$CHECKSUM_URL" | awk '{print $1}')"
echo $RELEASE_SHA

function main() {
  download_archive
  # verify_archive # verification: disabled for now
  install_joshuto
}

function download_archive() {
  echo "Downloading joshuto's binary from $RELEASE_VER release.."
  if ! curl --progress-bar --fail -L "$ARCHIVE_URL" -o "$DOWNLOAD_DIR/$ARCHIVE_NAME.tar.gz"; then
    echo "Download failed.  Check that the release/filename are correct."
    exit 1
  fi
  echo "Download complete!"
}

# Verification is disabled for now
#
# Action automated-build should generate a checksum
# archive on release to enable verifications
function verify_archive() {
  echo "Verifying the installation..."
  DOWNLOADED_SHA="$(openssl dgst -sha256 "$DOWNLOAD_DIR/$ARCHIVE_NAME.tar.gz" | awk '{print $2}')"
  echo $DOWNLOADED_SHA

  if [ "$RELEASE_SHA" != "$DOWNLOADED_SHA" ]; then
    echo "Error! checksum mismatch."
    echo "Expected: $RELEASE_SHA but got: $DOWNLOADED_SHA"
    exit 1
  fi
  echo "Verification complete!"
}

function install_joshuto() {
  echo "Installing joshuto..."

  pushd "$DOWNLOAD_DIR"
  tar -xzf "$DOWNLOAD_DIR/$ARCHIVE_NAME.tar.gz"
  popd

  if [ ! -d "$DOWNLOAD_DIR/$ARCHIVE_NAME" ]; then
    echo "Error! Archive does not contain a $ARCHIVE_NAME directory."
    exit 1
  fi

  cp -r "$DOWNLOAD_DIR/$ARCHIVE_NAME/." "$INSTALL_PREFIX"
  echo "Installation complete!"
  echo "======================"
  echo "Now you can run $INSTALL_PREFIX/joshuto"
}

main "$@"
