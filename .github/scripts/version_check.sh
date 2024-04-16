ver_common=$(grep -oE "^version = \".*\"$" "./common/Cargo.toml" | sed -ne "s/version = \"\(.*\)\"$/\1/p")
echo "Common: $ver_common"

failed=0

# $1: version, $2: name
function check() {
    if [ "$1" = "$ver_common" ]; then
      echo "✔ $2: $1"
    else
      echo "✘ $2: $1"
      failed=$((failed+1))
    fi
}

function check_nodejs_version() {
  ver=$(grep -oE "^ *\"version\": \".*\", *$" "./nodejs/package.json" | sed -ne "s/.*\"version\": \"\(.*\)\",.*/\1/p")
  check "$ver" "Node.js"

  find ./nodejs/npm/ -maxdepth 2 -type f -name "package.json" | while read fpath; do
    ver=$(cat $fpath | grep -oE "^ *\"version\": \".*\", *$" | sed -ne "s/.*\"version\": \"\(.*\)\",.*/\1/p")
    check "$ver" "  ◇ $(echo $fpath | sed -n "s/^.*\/\(.*\)\/package.json$/\1/p")"
  done
}


# Python
ver_python=$(grep -oE "^version = .*$" "./python/Cargo.toml" | sed -ne "s/^version = \"\(.*\)\" *$/\1/p")
check "$ver_python" "Python"

# Java
ver_java=$(grep -oE "^version = .*$" "./java/java/lib/build.gradle" | sed -ne "s/^version = \"\(.*\)\"$/\1/p")
check "$ver_java" "Java"

# Node.js
check_nodejs_version

# Golang: ignored

# Lua
# ...


exit $((failed))