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

target_path="./output/python/"
if [ "$f_benchmark" = true ]; then
  target_path="./output-benchmark/python/"
fi
mkdir -p "$target_path"

# Naming of artifacts:
#   dt_core_{package}[-{package_specific}]-{platform}-{architecture}.so
# E.g.
#   dt_core_lua-lua54-macos-aarch64.so

####################################
# Build Python
####################################
find_name_4_whl() {
  find ../target/wheels/ -name "dt_python_sdk-*-*$1*$2*.whl" -type f -exec basename {} \; | head -1
}

# target, platform, architecture
build_python_aux() {
  if [ "$2" = "win" ]; then
    maturin build --release --zig --interpreter python3.9 --target "$1"
  else
    maturin build --release --zig --target "$1"
  fi;
  name=$(find_name_4_whl "$2" "$3")
  cp -f "../target/wheels/$name" "../$target_path"
}

# target, platform, architecture, features
build_python_aux_featured() {
  if [ "$2" = "win" ]; then
    maturin build --release --zig --interpreter python3.9 --target "$1" --features "$4"
  else
    maturin build --release --zig --target "$1" --features "$4"
  fi;
  name=$(find_name_4_whl "$2" "$3")
  cp -f "../target/wheels/$name" "../$target_path"
}

build_python() {
  version_check
  cd python || (echo "Failed to \`cd python\`" & exit)
  source .env/bin/activate

  if [ "$f_has_macos" = true ]; then
    build_macos
  fi

  if [ "$f_has_linux" = true ]; then
    build_linux
  fi

  if [ "$f_has_windows" = true ]; then
    build_windows
  fi

  deactivate
  cd "../"
}

build_macos() {
  if [ "$f_benchmark" = true ]; then
    build_python_aux_featured x86_64-apple-darwin macos x86_64 "benchmark"
    build_python_aux_featured aarch64-apple-darwin macos arm64 "benchmark"
  else
    build_python_aux x86_64-apple-darwin macos x86_64
    build_python_aux aarch64-apple-darwin macos arm64
  fi
}

build_linux() {
  if [ "$f_benchmark" = true ]; then
    build_python_aux_featured x86_64-unknown-linux-gnu manylinux x86_64 "benchmark"
    build_python_aux_featured aarch64-unknown-linux-gnu manylinux aarch64 "benchmark"
  else
    build_python_aux x86_64-unknown-linux-gnu manylinux x86_64
    build_python_aux aarch64-unknown-linux-gnu manylinux aarch64
  fi
}

build_windows() {
  if [ "$f_benchmark" = true ]; then
    build_python_aux_featured x86_64-pc-windows-msvc win amd64 "benchmark"
    build_python_aux_featured aarch64-pc-windows-msvc win arm64 "benchmark"
  else
    build_python_aux x86_64-pc-windows-msvc win amd64
    build_python_aux aarch64-pc-windows-msvc win arm64
  fi
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
build_python