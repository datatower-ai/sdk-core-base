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

target_path="./output/lua/"
if [ "$f_benchmark" = true ]; then
  target_path="./output-benchmark/lua/"
fi
mkdir -p "$target_path"

# Naming of artifacts:
#   dt_core_{package}[-{package_specific}]-{platform}-{architecture}.so
# E.g.
#   dt_core_lua-lua54-macos-aarch64.so

####################################
# Build Lua
# $1: features
####################################
function build_lua() {
  if [ "$f_has_macos" = true ]; then
    build_macos "$1"
  fi

  if [ "$f_has_linux" = true ]; then
    build_linux "$1"
  fi

  if [ "$f_has_windows" = true ]; then
    build_windows "$1"
  fi
}

function build_macos() {
  if [ "$f_benchmark" = true ]; then
    cargo rustc --release --package lua --no-default-features --features "$1,benchmark" --target x86_64-apple-darwin -- -C link-arg=-undefined -C link-arg=dynamic_lookup
    cargo rustc --release --package lua --no-default-features --features "$1,benchmark" --target aarch64-apple-darwin -- -C link-arg=-undefined -C link-arg=dynamic_lookup
  else
    cargo rustc --release --package lua --no-default-features --features "$1" --target x86_64-apple-darwin -- -C link-arg=-undefined -C link-arg=dynamic_lookup
    cargo rustc --release --package lua --no-default-features --features "$1" --target aarch64-apple-darwin -- -C link-arg=-undefined -C link-arg=dynamic_lookup
  fi

  cp -f "./target/x86_64-apple-darwin/release/libdt_core_lua.dylib" "$target_path/dt_core_lua-$1-macos-x86_64.so"
  cp -f "./target/aarch64-apple-darwin/release/libdt_core_lua.dylib" "$target_path/dt_core_lua-$1-macos-aarch64.so"
}

function build_linux() {
  if [ "$f_benchmark" = true ]; then
    cargo rustc --release --package lua --no-default-features --features "$1,benchmark" --target x86_64-unknown-linux-gnu
    cargo rustc --release --package lua --no-default-features --features "$1,benchmark" --target aarch64-unknown-linux-gnu
  else
    cargo rustc --release --package lua --no-default-features --features "$1" --target x86_64-unknown-linux-gnu
    cargo rustc --release --package lua --no-default-features --features "$1" --target aarch64-unknown-linux-gnu
  fi

  cp -f "./target/x86_64-unknown-linux-gnu/release/libdt_core_lua.so" "$target_path/dt_core_lua-$1-linux-x86_64.so"
  cp -f "./target/aarch64-unknown-linux-gnu/release/libdt_core_lua.so" "$target_path/dt_core_lua-$1-linux-aarch64.so"
}

function build_windows() {
  mv "./.cargo/config.toml" "./.cargo/blocked.config.toml"
  colima start

  if [ "$f_benchmark" = true ]; then
    cross rustc --release --package lua --no-default-features --features "$1,benchmark" --target x86_64-pc-windows-msvc
    cross rustc --release --package lua --no-default-features --features "$1,benchmark" --target aarch64-pc-windows-msvc
  else
    cross rustc --release --package lua --no-default-features --features "$1" --target x86_64-pc-windows-msvc
    cross rustc --release --package lua --no-default-features --features "$1" --target aarch64-pc-windows-msvc
  fi

  cp -f "./target/x86_64-pc-windows-msvc/release/dt_core_lua.dll" "$target_path/dt_core_lua-$1-windows-x86_64.dll"
  cp -f "./target/aarch64-pc-windows-msvc/release/dt_core_lua.dll" "$target_path/dt_core_lua-$1-windows-aarch64.dll"

  mv "./.cargo/blocked.config.toml" "./.cargo/config.toml"
}

function version_check() {
    common_version=$(grep -oE "^version = \".*\"$" "./common/Cargo.toml" | sed -ne "s/version = \"\(.*\)\"$/\1/p")
    echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    printf "┃ version: \t\033[1;35m%s\033[0m\n" "$common_version"
    echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
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
