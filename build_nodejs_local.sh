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

target_path="$BASEDIR/output/nodejs"
if [ "$f_benchmark" = true ]; then
  target_path="$BASEDIR/output-benchmark/nodejs"
fi
tmp_path="$target_path/dt_core_nodejs"
mkdir -p "$tmp_path"

# Naming of artifacts:
#   dt_core_{package}[-{package_specific}]-{platform}-{architecture}.so
# E.g.
#   dt_core_lua-lua54-macos-aarch64.so

####################################
# Build Node.js
####################################
function build_nodejs() {
  version_check

  cd "$BASEDIR/nodejs" || (echo "Cannot cd to project path" && exit)

  if [ "$f_has_macos" = true ]; then
    build_macos
  fi

  if [ "$f_has_linux" = true ]; then
    build_linux
  fi

  if [ "$f_has_windows" = true ]; then
    build_windows
  fi

  cd "../$target_path" || (echo "Cannot cd to output path" && exit)
  echo "Zipping..."
  zip -r -q dt_core_nodejs.zip dt_core_nodejs
  echo "Done"
}

function build_macos() {
  if [ "$f_benchmark" = true ]; then
    yarn build --target x86_64-apple-darwin "../$tmp_path" --features "benchmark"
    yarn build --target aarch64-apple-darwin "../$tmp_path" --features "benchmark"
  else
    yarn build --target x86_64-apple-darwin "../$tmp_path"
    yarn build --target aarch64-apple-darwin "../$tmp_path"
  fi
}

function build_linux() {
  if [ "$f_benchmark" = true ]; then
    yarn build --target x86_64-unknown-linux-gnu "../$tmp_path" --features "benchmark"
    yarn build --target aarch64-unknown-linux-gnu "../$tmp_path" --features "benchmark"
  else
    yarn build --target x86_64-unknown-linux-gnu "../$tmp_path"
    yarn build --target aarch64-unknown-linux-gnu "../$tmp_path"
  fi
}

function build_windows() {
  if [ "$f_benchmark" = true ]; then
    yarn build --target x86_64-pc-windows-msvc "../$tmp_path" --features "benchmark"
    yarn build --target aarch64-pc-windows-msvc "../$tmp_path" --features "benchmark"
  else
    yarn build --target x86_64-pc-windows-msvc "../$tmp_path"
    yarn build --target aarch64-pc-windows-msvc "../$tmp_path"
  fi
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
