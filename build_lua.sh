BASEDIR=$(dirname "$0")
cd $BASEDIR

#
# Build Lua
#
mkdir -p "$BASEDIR/output/lua/"

cargo rustc --release --package lua --features lua54
cp -f "$BASEDIR/target/release/libdt_core_lua.dylib" "$BASEDIR/output/lua/dt_core_lua-lua54-x86_64-macos.so"

cargo rustc --release --package lua --features lua54 --target aarch64-apple-darwin
cp -f "$BASEDIR/target/aarch64-apple-darwin/release/libdt_core_lua.dylib" "$BASEDIR/output/lua/dt_core_lua-lua54-aarch64-macos.so"

cargo rustc --release --package lua --features lua54 --target x86_64-unknown-linux-gnu
cp -f "$BASEDIR/target/x86_64-unknown-linux-gnu/release/libdt_core_lua.so" "$BASEDIR/output/lua/dt_core_lua-lua54-x86_64-linux-gnu.so"

#
# Build xxx
#
