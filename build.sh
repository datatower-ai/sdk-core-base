BASEDIR=$(dirname "$0")
cd "$BASEDIR" || (echo "Cannot cd to script's path" & exit)

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

  # USE WINDOWS TO BUILD!
  # colima restart
#  cross rustc --release --package lua --no-default-features --features "$1" --target x86_64-pc-windows-msvc
#  cp -f "$BASEDIR/target/x86_64-pc-windows-msvc/release/libdt_core_lua.so" "$BASEDIR/output/lua/dt_core_lua-$1-windows-x86_64.so"
#
#  cross rustc --release --package lua --no-default-features --features "$1" --target aarch64-pc-windows-msvc
#  cp -f "$BASEDIR/target/aarch64-pc-windows-msvc/release/libdt_core_lua.so" "$BASEDIR/output/lua/dt_core_lua-$1-windows-aarch64.so"
}

####################################
# Build Python
####################################
function build_python() {
  mkdir -p "$BASEDIR/output/python/"
  cd python || (echo "Failed to \`cd python\`" & exit)
  source .env/bin/activate

  function find_name_4_whl() {
    find ../target/wheels/ -name "dt_core_python-*$1*$2*.whl" -type f -exec basename {} \; | head -1
  }

  maturin build --release --zig --interpreter python3.9 --target x86_64-apple-darwin
  cp -f "../target/wheels/$(find_name_4_whl macos x86_64)" "../output/python/dt_core_python-macos-x86_64.whl"

  maturin build --release --zig --interpreter python3.9 --target aarch64-apple-darwin
  cp -f "../target/wheels/$(find_name_4_whl macos arm64)" "../output/python/dt_core_python-macos-arm64.whl"

  maturin build --release --zig --interpreter python3.9 --target x86_64-unknown-linux-gnu
  cp -f "../target/wheels/$(find_name_4_whl manylinux x86_64)" "../output/python/dt_core_python-linux-x86_64.whl"

  maturin build --release --zig --interpreter python3.9 --target aarch64-unknown-linux-gnu
  cp -f "../target/wheels/$(find_name_4_whl manylinux aarch64)" "../output/python/dt_core_python-linux-aarch64.whl"

  maturin build --release --zig --interpreter python3.9 --target x86_64-pc-windows-msvc
  cp -f "../target/wheels/$(find_name_4_whl win amd64)" "../output/python/dt_core_python-windows-amd64.whl"

  maturin build --release --zig --interpreter python3.9 --target aarch64-pc-windows-msvc
  cp -f "../target/wheels/$(find_name_4_whl win arm64)" "../output/python/dt_core_python-windows-arm64.whl"

  deactivate
  cd "../"
}


####################################
# Build
####################################
build_lua lua54 &
# build_python &
wait
