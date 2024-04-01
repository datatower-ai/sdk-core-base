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

#  cross rustc --release --package java --target x86_64-pc-windows-msvc
#  cp -f "$BASEDIR/target/x86_64-pc-windows-msvc/release/libdt_core_java.dll" "$target_path/libdt_core_java-windows-amd64.dll"
#
#  cross rustc --release --package java --target aarch64-pc-windows-msvc
#  cp -f "$BASEDIR/target/aarch64-pc-windows-msvc/release/libdt_core_java.dll" "$target_path/libdt_core_java-windows-arm64.dll"

  cd ./output/java/tmp/ || (echo "Cannot cd to java project" && exit)
  ./gradlew lib:build
  version=$(grep -oE "^ *private static final String SDK_VERSION = .*; *$" "./lib/src/main/java/ai/datatower/sdk/DTAnalytics.java" | sed -ne "s/^ *private static final String SDK_VERSION = \"\(.*\)\"; *$/\1/p")
  cp "./lib/build/libs/lib.jar" "../dt_server_sdk_java-$version.jar"
  cd "../" || (echo "Cannot cd back" && exit)
  rm -rf "./tmp/"
}

####################################
# Build
####################################
build_java
