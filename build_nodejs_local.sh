BASEDIR=$(dirname "$0")
cd "$BASEDIR" || (echo "Cannot cd to script's path" && exit)

# Naming of artifacts:
#   dt_core_{package}[-{package_specific}]-{platform}-{architecture}.so
# E.g.
#   dt_core_lua-lua54-macos-aarch64.so

####################################
# Build Node.js
####################################
function build_nodejs() {
  version_check
  mkdir -p "$BASEDIR/output/nodejs/"
  target_path="./output/nodejs/dt_core_nodejs"

  cd "$BASEDIR/nodejs" || (echo "Cannot cd to project path" && exit)

  yarn build --target x86_64-apple-darwin "../$target_path"

  yarn build --target aarch64-apple-darwin "../$target_path"

  yarn build --target x86_64-unknown-linux-gnu "../$target_path"

  yarn build --target aarch64-unknown-linux-gnu "../$target_path"

  yarn build --target x86_64-pc-windows-msvc "../$target_path"

  yarn build --target aarch64-pc-windows-msvc "../$target_path"

  cd ../output/nodejs || (echo "Cannot cd to output path" && exit)
  echo "Zipping..."
  zip -r -q dt_core_nodejs.zip dt_core_nodejs
  echo "Done"
}

function version_check() {
    version=$(grep -oE "^static VERSION: &'static str = .*$" "./nodejs/src/lib.rs" | sed -ne "s/^static VERSION: &'static str = \"\(.*\)\"; *$/\1/p")
    common_version=$(grep -oE "^version = \".*\"$" "./common/Cargo.toml" | sed -ne "s/version = \"\(.*\)\"$/\1/p")
    if [ -z "$version" ]; then
      echo "\033[0;31mCannot found version in lib.rs\033[0m"
      exit
    fi
    echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    printf "┃ version: \t\033[1;35m%s\033[0m\n" "$version"
    printf "┃ common ver: \t\033[1;35m%s\033[0m\n" "$common_version"
    echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

####################################
# Build
####################################
build_nodejs
