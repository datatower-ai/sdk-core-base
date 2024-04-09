BASEDIR=$(dirname "$0")
cd "$BASEDIR" || (echo "Cannot cd to script's path" && exit)

# Naming of artifacts:
#   dt_core_{package}[-{package_specific}]-{platform}-{architecture}.so
# E.g.
#   dt_core_lua-lua54-macos-aarch64.so

####################################
# Build Python
####################################
function find_name_4_whl() {
  find ../target/wheels/ -name "dt_core_python-*-*$1*$2*.whl" -type f -exec basename {} \; | head -1
}

# target, platform, architecture
function build_python_aux() {
  if [ "$2" = "win" ]; then
    maturin build --release --zig --interpreter python3.9 --target "$1"
  else
    maturin build --release --zig --target "$1"
  fi;
  name=$(find_name_4_whl "$2" "$3")
  cp -f "../target/wheels/$name" "../output/python/$name"
}

function build_python() {
  version_check
  mkdir -p "$BASEDIR/output/python/"
  cd python || (echo "Failed to \`cd python\`" & exit)
  source .env/bin/activate

  build_python_aux x86_64-apple-darwin macos x86_64

  build_python_aux aarch64-apple-darwin macos arm64

  build_python_aux x86_64-unknown-linux-gnu manylinux x86_64

  build_python_aux aarch64-unknown-linux-gnu manylinux aarch64

  build_python_aux x86_64-pc-windows-msvc win amd64

  build_python_aux aarch64-pc-windows-msvc win arm64

  deactivate
  cd "../"
}

function version_check() {
    version=$(grep -oE "^version = .*$" "./python/Cargo.toml" | sed -ne "s/^version = \"\(.*\)\" *$/\1/p")
    if [ -z "$version" ]; then
      echo "\033[0;31mCannot found version in Cargo.toml\033[0m"
      exit
    fi
    echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "┃ version: \033[1;35m$version\033[0m"
    echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}


####################################
# Build
####################################
build_python