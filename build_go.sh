BASEDIR=$(dirname "$0")
cd "$BASEDIR" || (echo "Cannot cd to script's path" && exit)

f_benchmark=false
f_has_macos=true
f_has_windows=true
f_has_linux=true

while getopts bmwl opt; do
  case "$opt" in
    b) f_benchmark=true;;
    m) f_has_macos=true; f_has_windows=false; f_has_linux=false;;
    w) f_has_macos=false; f_has_windows=true; f_has_linux=false;;
    l) f_has_macos=false; f_has_windows=false; f_has_linux=true;;
    *) ;;
  esac
done

target_path="$BASEDIR/output/go/"
if [ "$f_benchmark" = true ]; then
  target_path="$BASEDIR/output-benchmark/go/"
fi
mkdir -p "$target_path"

# Naming of artifacts:
#   dt_core_{package}[-{package_specific}]-{platform}-{architecture}.so
# E.g.
#   dt_core_lua-lua54-macos-aarch64.so

####################################
# Build Golang
####################################
build_golang() {
  version_check

  if [ "$f_has_macos" = true ]; then
    build_macos
  fi

  if [ "$f_has_linux" = true ]; then
    build_linux
  fi

  if [ "$f_has_windows" = true ]; then
    build_windows
  fi

  cp -f "clib/dt_core_clib.h" "$target_path/dt_core_clib.h"
}

build_macos() {
  if [ "$f_benchmark" = true ]; then
    cargo build --release --package clib --target x86_64-apple-darwin --features "benchmark"
    cargo build --release --package clib --target aarch64-apple-darwin --features "benchmark"
  else
    cargo build --release --package clib --target x86_64-apple-darwin
    cargo build --release --package clib --target aarch64-apple-darwin
  fi

  cp -f "$BASEDIR/target/x86_64-apple-darwin/release/libdt_core_clib.dylib" "$target_path/libdt_core_clib-macos-amd64.dylib"
  cp -f "$BASEDIR/target/aarch64-apple-darwin/release/libdt_core_clib.dylib" "$target_path/libdt_core_clib-macos-arm64.dylib"
}

build_linux() {
  if [ "$f_benchmark" = true ]; then
    cargo build --release --package clib --target x86_64-unknown-linux-gnu --features "benchmark"
    cargo build --release --package clib --target x86_64-unknown-linux-gnu --features "benchmark"
  else
    cargo build --release --package clib --target x86_64-unknown-linux-gnu
    cargo build --release --package clib --target aarch64-unknown-linux-gnu
  fi

  cp -f "$BASEDIR/target/x86_64-unknown-linux-gnu/release/libdt_core_clib.so" "$target_path/libdt_core_clib-linux-amd64.so"
  cp -f "$BASEDIR/target/aarch64-unknown-linux-gnu/release/libdt_core_clib.so" "$target_path/libdt_core_clib-linux-arm64.so"
}

build_windows() {
  mv "$BASEDIR/.cargo/config.toml" "$BASEDIR/.cargo/blocked.config.toml"
  colima start

  if [ "$f_benchmark" = true ]; then
    cross build --release --package clib --target x86_64-pc-windows-msvc --features "benchmark"
    cross build --release --package clib --target aarch64-pc-windows-msvc --features "benchmark"
  else
    cross build --release --package clib --target x86_64-pc-windows-msvc
    cross build --release --package clib --target aarch64-pc-windows-msvc
  fi

  cp -f "$BASEDIR/target/x86_64-pc-windows-msvc/release/dt_core_clib.dll" "$target_path/dt_core_clib-windows-amd64.dll"
  cp -f "$BASEDIR/target/aarch64-pc-windows-msvc/release/dt_core_clib.dll" "$target_path/dt_core_clib-windows-arm64.dll"

  mv "$BASEDIR/.cargo/blocked.config.toml" "$BASEDIR/.cargo/config.toml"
}

version_check() {
    common_version=$(grep -oE "^version = \".*\"$" "./common/Cargo.toml" | sed -ne "s/version = \"\(.*\)\"$/\1/p")
    echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    printf "┃ version: \t\033[1;35m%s\033[0m\n" "$common_version"
    echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

####################################
# Build
####################################
build_golang
