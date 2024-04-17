ver_common=$(grep -oE "^version = \".*\"$" "./common/Cargo.toml" | sed -ne "s/version = \"\(.*\)\"$/\1/p")
echo "Common: $ver_common"

failed=0

chmod +x "$(dirname "$0")/check_version_by.sh"

# $1: version, $2: name
function check() {
    if $(dirname "$0")/check_version_by.sh $1 $ver_common; then
      echo "✔ $2: $1"
    else
      echo "✘ $2: $1"
      ((failed++))
    fi
}


# Python
ver_python=$(grep -oE "^version = .*$" "./python/Cargo.toml" | sed -ne "s/^version = \"\(.*\)\" *$/\1/p")
check "$ver_python" "Python"

# Java
ver_java=$(grep -oE "^version = .*$" "./java/java/lib/build.gradle" | sed -ne "s/^version = \"\(.*\)\"$/\1/p")
check "$ver_java" "Java"

# Node.js
ver=$(grep -oE "^ *\"version\": \".*\", *$" "./nodejs/package.json" | sed -ne "s/.*\"version\": \"\(.*\)\",.*/\1/p")
check "$ver" "Node.js"

# Golang: ignored
echo "⊘ Golang: Ignored"

# Lua
# ...


exit $((failed))