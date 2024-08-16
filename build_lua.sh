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
build_lua() {
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

build_macos() {
  if [ "$f_benchmark" = true ]; then
    cargo rustc --release --package lua --no-default-features --features "$1,benchmark" --target x86_64-apple-darwin -- -C link-arg=-undefined -C link-arg=dynamic_lookup
    cargo rustc --release --package lua --no-default-features --features "$1,benchmark" --target aarch64-apple-darwin -- -C link-arg=-undefined -C link-arg=dynamic_lookup
  else
    cargo rustc --release --package lua --no-default-features --features "$1" --target x86_64-apple-darwin -- -C link-arg=-undefined -C link-arg=dynamic_lookup
    cargo rustc --release --package lua --no-default-features --features "$1" --target aarch64-apple-darwin -- -C link-arg=-undefined -C link-arg=dynamic_lookup
  fi

  build_rock "$1" macosx x86_64 "$2" x86_64-apple-darwin libdt_core_lua.dylib dt_core_lua.so
#  build_rock "$1" macosx aarch64 "$2" aarch64-apple-darwin libdt_core_lua.dylib dt_core_lua.so
}

build_linux() {
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

build_windows() {
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


# $1: Lua version
# $2: OS
# $3: Arch
# $4: Lua version number
# $5: Target
# $6: Artifact name
# $7: Target Artifact name
# Copy -> Compress -> Build -> Move -> Clear
build_rock() {
  mkdir -p "$target_path/tmp"
  version=$(grep -oE "^version = \".*\"$" "./lua/Cargo.toml" | sed -ne "s/version = \"\(.*\)\"$/\1/p")
  version=$(echo "$version" | sed -nE "s/^(v?([0-9]+)\.([0-9]+)\.([0-9]+)([-._]?)(([a|A])lpha|([b|B])eta|SNAPSHOT)?([0-9]*))$/\2.\3.\4\5\6\9/p")
  cp -f "./lua/gen_rockspec.sh" "$target_path/tmp/"
  cp -f "./lua/Cargo.toml" "$target_path/tmp/"
  archive_folder="dt-lua-sdk-$version"
  cp -r "./lua/dt-lua-sdk/" "$target_path/tmp/$archive_folder"
  cp -f "./target/$5/release/$6" "$target_path/tmp/$archive_folder/$7"
  cd "$target_path/tmp"
  tar czpf "$archive_folder.tar.gz" "./$archive_folder/"
  name=$(sh ./gen_rockspec.sh "$4" "$2")
  luarocks pack "$name"
  rock_file=$(basename $(find . -maxdepth 1 -mindepth 1 -type f -name "*.rock" | head -1))
  prefix="$1-$2-$3"
  mv "$rock_file" "../$prefix-$rock_file"
  cd ../        # ./xxx/lua
#  rm -rf ./tmp/
  cd ../../        # .
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
version_check
build_lua lua54 5.4
#build_lua lua53 5.3
#build_lua lua52 5.2
#build_lua lua51 5.1
#build_lua luajit
#build_lua luau
