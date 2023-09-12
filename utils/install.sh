#!/usr/bin/env sh

# testing from fork:
# REPO="kennycallado/joshuto" bash <(curl -s https://raw.githubusercontent.com/kennycallado/joshuto/master/utils/install.sh)

set -eo pipefail

# Check before starting the installation
readonly INSTALL_PREFIX="${INSTALL_PREFIX:-"$HOME/.local/bin"}"
if [ ! -d "$INSTALL_PREFIX" ];then echo "Error! $INSTALL_PREFIX does not exist"; exit 1 ;fi

readonly GITHUB_API_URL="https://api.github.com/repos/kamiyaa/joshuto/releases"

function get_latest_version() {
  curl -Lfs "$GITHUB_API_URL" | \
    grep '"tag_name":' | \
    sed -E 's/.*"([^"]+)".*/\1/' | \
    head -n 1
}

function get_release_body() {
  curl -Lfs "$GITHUB_API_URL" | \
    grep '"body":' | \
    sed -E 's/.*"([^"]+)".*/\1/' | \
    head -n 1
}

readonly OS="$(uname -s)"
readonly ARCH="$(uname -m)"
readonly REPO="${REPO:-"kamiyaa/joshuto"}"
readonly LIBR="$(if [ -f  /lib/libc.musl-x86_64.so.1 ];then echo "musl" ; else echo "gnu" ;fi)"

readonly RELEASE_VER="${RELEASE_VER:-"$(get_latest_version)"}"
readonly RELEASE_BODY="$(get_release_body)"

if [ "$OS" == "Linux" ]; then
  readonly ARCHIVE_NAME="joshuto-$RELEASE_VER-$ARCH-unknown-linux-$LIBR"
elif [ "$OS" == "Darwin" ]; then
  readonly ARCHIVE_NAME="joshuto-$RELEASE_VER-$ARCH-apple-darwin"
else
  echo
  echo "$OS platform is not supported currently"
  exit 1
fi

readonly ARCHIVE_URL="https://github.com/$REPO/releases/download/$RELEASE_VER/$ARCHIVE_NAME.tar.gz"
readonly CHECKSUM_URL="$ARCHIVE_URL.sha256sum"

readonly DOWNLOAD_DIR="$(mktemp -d)"
readonly RELEASE_SHA="$(curl -Lfs "$CHECKSUM_URL" | awk '{print $1}')"

function main() {
  printf "$RELEASE_BODY\n\n"

  printf '\e[1;32m%-6s\e[m' "===================="
  printf '\e[1;34m%-6s\e[m' " Release notes  "
  printf '\e[1;32m%-6s\e[m' "===================="

  printf "\n\n";

  download_archive
  verify_archive
  install_joshuto
}

function download_archive() {
  echo "Downloading joshuto's binary from $RELEASE_VER release.."
  if ! curl --progress-bar -Lf "$ARCHIVE_URL" -o "$DOWNLOAD_DIR/$ARCHIVE_NAME.tar.gz"; then
    echo
    echo "Download failed.  Check that the release/filename are correct."
    exit 1
  fi
  echo "Download complete!"
}

# Some releases don't have checksum file
function checksum_file() {
  local _return=$1

  if [ -z "$RELEASE_SHA" ]; then
    printf "\n"
    printf '\e[1;33m%-6s\e[m' "Warning! "
    printf '\e[1;0m%-6s\e[m' "release checksum file is not found."
    printf "\n"

    echo "Would you like to continue? [y/N]"
    read -r answer
    if [ "$answer" == "${answer#[Yy]}" ]; then
      echo "Installation aborted."
      exit 1
    fi
  fi

  eval $_return="'$DOWNLOADED_SHA'"
}

function verify_archive() {
  echo "Verifying the installation..."

  DOWNLOADED_SHA="$(openssl dgst -sha256 "$DOWNLOAD_DIR/$ARCHIVE_NAME.tar.gz" | awk '{print $2}')"
  checksum_file VERIFIED

  if [ "$VERIFIED" != "$DOWNLOADED_SHA" ]; then
    printf "\n"
    printf '\e[1;31m%-6s\e[m' "Error! "
    printf '\e[1;0m%-6s\e[m' "checksum mismatch."
    printf "\n"

    exit 1
  fi
  echo "Verification complete!"
}

function install_joshuto() {
  echo "Installing joshuto..."

  PWD="$(pwd)"
  cd $DOWNLOAD_DIR
  tar -xzf "$DOWNLOAD_DIR/$ARCHIVE_NAME.tar.gz"
  cd $PWD

  if [ ! -d "$DOWNLOAD_DIR/$ARCHIVE_NAME" ]; then
    printf "\n"
    printf '\e[1;31m%-6s\e[m' "Error! "
    printf '\e[1;0m%-6s\e[m' "Archive does not contain a $ARCHIVE_NAME directory."
    printf "\n"

    exit 1
  fi

  cp -r "$DOWNLOAD_DIR/$ARCHIVE_NAME/joshuto" "$INSTALL_PREFIX/"
  echo
  echo "Installation complete!"
  echo "======================"
  echo "Now you can run $INSTALL_PREFIX/joshuto"
}

main "$@"
