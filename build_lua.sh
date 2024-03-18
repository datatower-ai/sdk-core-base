BASEDIR=$(dirname "$0")
cd "$BASEDIR" || echo "Cannot cd to script's path"

# Naming of artifacts:
#   dt_core_{package}[-{package_specific}]-{platform}-{architecture}.so
# E.g.
#   dt_core_lua-lua54-macos-aarch64.so

####################################
# Build Lua
####################################
mkdir -p "$BASEDIR/output/lua/"

cargo rustc --release --package lua --features lua54
cp -f "$BASEDIR/target/release/libdt_core_lua.dylib" "$BASEDIR/output/lua/dt_core_lua-lua54-macos-x86_64.so"

cargo rustc --release --package lua --features lua54 --target aarch64-apple-darwin
cp -f "$BASEDIR/target/aarch64-apple-darwin/release/libdt_core_lua.dylib" "$BASEDIR/output/lua/dt_core_lua-lua54-macos-aarch64.so"

cargo rustc --release --package lua --features lua54 --target x86_64-unknown-linux-gnu
cp -f "$BASEDIR/target/x86_64-unknown-linux-gnu/release/libdt_core_lua.so" "$BASEDIR/output/lua/dt_core_lua-lua54-linux-x86_64.so"

#
# Build xxx
#
