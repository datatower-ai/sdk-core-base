BASEDIR=$(dirname "$0")
cd "$BASEDIR" || (echo "Cannot cd to script's path" && exit)

# Naming of artifacts:
#   dt_core_{package}[-{package_specific}]-{platform}-{architecture}.so
# E.g.
#   dt_core_lua-lua54-macos-aarch64.so

####################################
# Build Lua
# $1: features
####################################
function build_lua() {
  mkdir -p "$BASEDIR/output/lua/"

  cargo rustc --release --package lua --no-default-features --features "$1" --target x86_64-apple-darwin
  cp -f "$BASEDIR/target/x86_64-apple-darwin/release/libdt_core_lua.dylib" "$BASEDIR/output/lua/dt_core_lua-$1-macos-x86_64.so"

  cargo rustc --release --package lua --no-default-features --features "$1" --target aarch64-apple-darwin
  cp -f "$BASEDIR/target/aarch64-apple-darwin/release/libdt_core_lua.dylib" "$BASEDIR/output/lua/dt_core_lua-$1-macos-aarch64.so"

  cargo rustc --release --package lua --no-default-features --features "$1" --target x86_64-unknown-linux-gnu
  cp -f "$BASEDIR/target/x86_64-unknown-linux-gnu/release/libdt_core_lua.so" "$BASEDIR/output/lua/dt_core_lua-$1-linux-x86_64.so"

  cargo rustc --release --package lua --no-default-features --features "$1" --target aarch64-unknown-linux-gnu
  cp -f "$BASEDIR/target/aarch64-unknown-linux-gnu/release/libdt_core_lua.so" "$BASEDIR/output/lua/dt_core_lua-$1-linux-aarch64.so"

  mv "$BASEDIR/.cargo/config.toml" "$BASEDIR/.cargo/blocked.config.toml"
  colima start

  cross rustc --release --package lua --no-default-features --features "$1" --target x86_64-pc-windows-msvc
  cp -f "$BASEDIR/target/x86_64-pc-windows-msvc/release/dt_core_lua.dll" "$BASEDIR/output/lua/dt_core_lua-$1-windows-x86_64.dll"

  cross rustc --release --package lua --no-default-features --features "$1" --target aarch64-pc-windows-msvc
  cp -f "$BASEDIR/target/aarch64-pc-windows-msvc/release/dt_core_lua.dll" "$BASEDIR/output/lua/dt_core_lua-$1-windows-aarch64.dll"

  mv "$BASEDIR/.cargo/blocked.config.toml" "$BASEDIR/.cargo/config.toml"
}

function version_check() {
#    version=$(grep -oE "^\t_sdkVersion = .*$" "./go/dt_sdk_golang/src/dt_analytics/dt_sdk.go" | sed -ne "s/^\t_sdkVersion = \"\(.*\)\" *$/\1/p")
#    common_version=$(grep -oE "^version = \".*\"$" "./common/Cargo.toml" | sed -ne "s/version = \"\(.*\)\"$/\1/p")
#    if [ -z "$version" ]; then
#      echo "\033[0;31mCannot found version in dt_sdk.go\033[0m"
#      exit
#    fi
#    echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
#    printf "┃ version: \t\033[1;35m%s\033[0m\n" "$version"
#    printf "┃ common ver: \t\033[1;35m%s\033[0m\n" "$common_version"
#    echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""
}


####################################
# Build
####################################
version_check
build_lua lua54
build_lua lua53
build_lua lua52
build_lua lua51
build_lua luajit
build_lua luau
