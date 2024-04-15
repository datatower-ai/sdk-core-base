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

tmp_path="$BASEDIR/output/java/tmp/"
if [ "$f_benchmark" = true ]; then
  tmp_path="$BASEDIR/output-benchmark/java/tmp/"
fi
target_path="$tmp_path/lib/src/main/resources/ai/datatower/sdk"
mkdir -p "$target_path"

# Naming of artifacts:
#   dt_core_{package}[-{package_specific}]-{platform}-{architecture}.so
# E.g.
#   dt_core_lua-lua54-macos-aarch64.so

####################################
# Build Java
# copy java project -> build native so -> move to java project -> build .jar -> clear
####################################
function build_java() {
  version_check
  mkdir -p "$tmp_path"
  cp -r "$BASEDIR/java/java/" "$tmp_path"

  if [ "$f_has_macos" = true ]; then
    build_macos
  fi

  if [ "$f_has_linux" = true ]; then
    build_linux
  fi

  if [ "$f_has_windows" = true ]; then
    build_windows
  fi

  build_jar
}

function build_macos() {
  if [ "$f_benchmark" = true ]; then
    cargo rustc --release --package java --target x86_64-apple-darwin --features "benchmark"
    cargo rustc --release --package java --target aarch64-apple-darwin --features "benchmark"
  else
    cargo rustc --release --package java --target x86_64-apple-darwin
    cargo rustc --release --package java --target aarch64-apple-darwin
  fi

  cp -f "$BASEDIR/target/x86_64-apple-darwin/release/libdt_core_java.dylib" "$target_path/libdt_core_java-macos-amd64.dylib"
  cp -f "$BASEDIR/target/aarch64-apple-darwin/release/libdt_core_java.dylib" "$target_path/libdt_core_java-macos-arm64.dylib"
}

function build_linux() {
  if [ "$f_benchmark" = true ]; then
    cargo rustc --release --package java --target x86_64-unknown-linux-gnu --features "benchmark"
    cargo rustc --release --package java --target aarch64-unknown-linux-gnu --features "benchmark"
  else
    cargo rustc --release --package java --target x86_64-unknown-linux-gnu
    cargo rustc --release --package java --target aarch64-unknown-linux-gnu
  fi

  cp -f "$BASEDIR/target/x86_64-unknown-linux-gnu/release/libdt_core_java.so" "$target_path/libdt_core_java-linux-amd64.so"
  cp -f "$BASEDIR/target/aarch64-unknown-linux-gnu/release/libdt_core_java.so" "$target_path/libdt_core_java-linux-arm64.so"
}

function build_windows() {
  mv "$BASEDIR/.cargo/config.toml" "$BASEDIR/.cargo/blocked.config.toml"
  colima start

  if [ "$f_benchmark" = true ]; then
    cross rustc --release --package java --target x86_64-pc-windows-msvc --features "benchmark"
    cross rustc --release --package java --target aarch64-pc-windows-msvc --features "benchmark"
  else
    cross rustc --release --package java --target x86_64-pc-windows-msvc
    cross rustc --release --package java --target aarch64-pc-windows-msvc
  fi

  mv "$BASEDIR/.cargo/blocked.config.toml" "$BASEDIR/.cargo/config.toml"

  cp -f "$BASEDIR/target/x86_64-pc-windows-msvc/release/dt_core_java.dll" "$target_path/dt_core_java-windows-amd64.dll"
  cp -f "$BASEDIR/target/aarch64-pc-windows-msvc/release/dt_core_java.dll" "$target_path/dt_core_java-windows-arm64.dll"
}

function build_jar() {
  cd "$tmp_path" || (echo "Cannot cd to java project" && exit)
  ./gradlew lib:build
  artifact=$(ls "./lib/build/libs" | head -1)
  version=$(echo "$artifact" | sed -ne "s/^lib-\(.*\)\.jar$/\1/p")
  cp "./lib/build/libs/$artifact" "../dt_server_sdk_java-$version.jar"
  cd "../" || (echo "Cannot cd back" && exit)
  rm -rf "./tmp/"
}

function version_check() {
    code_version=$(grep -oE "^ *private static final String SDK_VERSION = .*; *$" "./java/java/lib/src/main/java/ai/datatower/sdk/DTAnalytics.java" | sed -ne "s/^ *private static final String SDK_VERSION = \"\(.*\)\"; *$/\1/p")
    gradle_version=$(grep -oE "^version = .*$" "./java/java/lib/build.gradle" | sed -ne "s/^version = \"\(.*\)\"$/\1/p")
    common_version=$(grep -oE "^version = \".*\"$" "./common/Cargo.toml" | sed -ne "s/version = \"\(.*\)\"$/\1/p")
    if [ ! "$code_version" = "$gradle_version" ]; then
      echo "\033[0;31mVersion check failed!\033[0m"
      echo "@code: \033[1m$code_version\033[0m"
      echo "@gradle: \033[1m$gradle_version\033[0m"
      exit
    fi
    echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    printf "┃ version: \t\033[1;35m%s\033[0m\n" "$code_version"
    printf "┃ common ver: \t\033[1;35m%s\033[0m\n" "$common_version"
    echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

####################################
# Build
####################################
build_java
