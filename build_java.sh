BASEDIR=$(dirname "$0")
cd "$BASEDIR" || (echo "Cannot cd to script's path" && exit)

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
  mkdir -p "$BASEDIR/output/java/tmp/"
  cp -r "$BASEDIR/java/java/" "$BASEDIR/output/java/tmp/"
  target_path="$BASEDIR/output/java/tmp/lib/src/main/resources/ai/datatower/sdk"

  cargo rustc --release --package java --target x86_64-apple-darwin
  cp -f "$BASEDIR/target/x86_64-apple-darwin/release/libdt_core_java.dylib" "$target_path/libdt_core_java-macos-amd64.dylib"

  cargo rustc --release --package java --target aarch64-apple-darwin
  cp -f "$BASEDIR/target/aarch64-apple-darwin/release/libdt_core_java.dylib" "$target_path/libdt_core_java-macos-arm64.dylib"

  cargo rustc --release --package java --target x86_64-unknown-linux-gnu
  cp -f "$BASEDIR/target/x86_64-unknown-linux-gnu/release/libdt_core_java.so" "$target_path/libdt_core_java-linux-amd64.so"

  cargo rustc --release --package java --target aarch64-unknown-linux-gnu
  cp -f "$BASEDIR/target/aarch64-unknown-linux-gnu/release/libdt_core_java.so" "$target_path/libdt_core_java-linux-arm64.so"

  mv "$BASEDIR/.cargo/config.toml" "$BASEDIR/.cargo/blocked.config.toml"
  colima start

  cross rustc --release --package java --target x86_64-pc-windows-msvc
  cp -f "$BASEDIR/target/x86_64-pc-windows-msvc/release/dt_core_java.dll" "$target_path/dt_core_java-windows-amd64.dll"

  cross rustc --release --package java --target aarch64-pc-windows-msvc
  cp -f "$BASEDIR/target/aarch64-pc-windows-msvc/release/dt_core_java.dll" "$target_path/dt_core_java-windows-arm64.dll"

  mv "$BASEDIR/.cargo/blocked.config.toml" "$BASEDIR/.cargo/config.toml"

  cd ./output/java/tmp/ || (echo "Cannot cd to java project" && exit)
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
