BASEDIR=$(dirname "$0")
cd $BASEDIR
mkdir -p "$BASEDIR/output/lua/"

cargo rustc --release --package lua --features lua54 -- -C link-arg=-undefined -C link-arg=dynamic_lookup
mv "$BASEDIR/target/release/libdt_core_lua.dylib" "$BASEDIR/output/lua/dt_core_lua-lua54-x86_64-macos.so"

cargo rustc --release --package lua --features lua54 --target x86_64-unknown-linux-gnu
mv "$BASEDIR/target/x86_64-unknown-linux-gnu/release/libdt_core_lua.so" "$BASEDIR/output/lua/dt_core_lua-lua54-x86_64-linux-gnu.so"
