BASEDIR=$(dirname "$0")
cd "$BASEDIR" || (echo "Cannot cd to script's path" && exit)

# Naming of artifacts:
#   dt_core_{package}[-{package_specific}]-{platform}-{architecture}.so
# E.g.
#   dt_core_lua-lua54-macos-aarch64.so

####################################
# Build Golang
####################################
function build_golang() {
  version_check
  mkdir -p "$BASEDIR/output/go/"
  target_path="$BASEDIR/output/go/"

  cargo rustc --release --package clib --target x86_64-apple-darwin
  cp -f "$BASEDIR/target/x86_64-apple-darwin/release/libdt_core_clib.dylib" "$target_path/libdt_core_clib-macos-amd64.dylib"

  cargo rustc --release --package clib --target aarch64-apple-darwin
  cp -f "$BASEDIR/target/aarch64-apple-darwin/release/libdt_core_clib.dylib" "$target_path/libdt_core_clib-macos-arm64.dylib"

  cargo rustc --release --package clib --target x86_64-unknown-linux-gnu
  cp -f "$BASEDIR/target/x86_64-unknown-linux-gnu/release/libdt_core_clib.so" "$target_path/libdt_core_clib-linux-amd64.so"

  cargo rustc --release --package clib --target aarch64-unknown-linux-gnu
  cp -f "$BASEDIR/target/aarch64-unknown-linux-gnu/release/libdt_core_clib.so" "$target_path/libdt_core_clib-linux-arm64.so"

  mv "$BASEDIR/.cargo/config.toml" "$BASEDIR/.cargo/blocked.config.toml"
  colima start

  cross rustc --release --package clib --target x86_64-pc-windows-msvc
  cp -f "$BASEDIR/target/x86_64-pc-windows-msvc/release/dt_core_clib.dll" "$target_path/dt_core_clib-windows-amd64.dll"

  cross rustc --release --package clib --target aarch64-pc-windows-msvc
  cp -f "$BASEDIR/target/aarch64-pc-windows-msvc/release/dt_core_clib.dll" "$target_path/dt_core_clib-windows-arm64.dll"

  mv "$BASEDIR/.cargo/blocked.config.toml" "$BASEDIR/.cargo/config.toml"
}

function version_check() {
    version=$(grep -oE "^\t_sdkVersion = .*$" "./go/dt_sdk_golang/src/dt_analytics/dt_sdk.go" | sed -ne "s/^\t_sdkVersion = \"\(.*\)\" *$/\1/p")
    if [ -z "$version" ]; then
      echo "\033[0;31mCannot found version in dt_sdk.go\033[0m"
      exit
    fi
    echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "┃ version: \033[1;35m$version\033[0m"
    echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

####################################
# Build
####################################
build_golang
