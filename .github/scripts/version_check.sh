ver_common=$(grep -oE "^version = \".*\"$" "./common/Cargo.toml" | sed -ne "s/version = \"\(.*\)\"$/\1/p")
echo "Common: $ver_common"

failed=0

chmod +x "$(dirname "$0")/check_version_by.sh"

# $1: version, $2: name
check() {
    if "$(dirname "$0")"/check_version_by.sh $1 $ver_common; then
      echo "✔ $2: $1"
    else
      echo "✘ $2: $1"
      ((failed++))
    fi
}


# Golang: ignored
echo "⊘ Golang: Ignored"

# Java
ver_java=$(grep -oE "^version = .*$" "./java/java/lib/build.gradle" | sed -ne "s/^version = \"\(.*\)\"$/\1/p")
check "$ver_java" "Java"

# Lua
ver_lua=$(grep -oE "^version = .*$" "./lua/Cargo.toml" | sed -ne "s/^version = \"\(.*\)\" *$/\1/p")
check "$ver_lua" "Lua"

# Node.js
ver=$(grep -oE "^ *\"version\": \".*\", *$" "./nodejs/package.json" | sed -ne "s/.*\"version\": \"\(.*\)\",.*/\1/p")
check "$ver" "Node.js"

# Python
ver_python=$(grep -oE "^version = .*$" "./python/Cargo.toml" | sed -ne "s/^version = \"\(.*\)\" *$/\1/p")
check "$ver_python" "Python"

# Clib
ver_clib=$(grep -oE "^version = .*$" "./clib/Cargo.toml" | sed -ne "s/^version = \"\(.*\)\" *$/\1/p")
check "$ver_clib" "Clib"


exit $((failed))