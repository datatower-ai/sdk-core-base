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
  maturin build --release --zig --interpreter python3.9 --target "$1"
  name=$(find_name_4_whl "$2" "$3")
  cp -f "../target/wheels/$name" "../output/python/$name"
}

function build_python() {
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


####################################
# Build
####################################
build_python &
wait
