ver_common=$(grep -oE "^version = \".*\"$" "./common/Cargo.toml" | sed -ne "s/version = \"\(.*\)\"$/\1/p")
echo "Common: $ver_common"

failed=false

# $1: version, $2: name
function check() {
    if [ "$1" = "$ver_common" ]; then
      echo "✔ $2: $1"
    else
      echo "✘ $2: $1"
      failed=true
    fi
}

ver_python=$(grep -oE "^version = .*$" "./python/Cargo.toml" | sed -ne "s/^version = \"\(.*\)\" *$/\1/p")
check "$ver_python" "Python"

ver_java=$(grep -oE "^version = .*$" "./java/java/lib/build.gradle" | sed -ne "s/^version = \"\(.*\)\"$/\1/p")
check "$ver_java" "Java"


if [ "$failed" = true ]; then
  exit 1
fi