base_path=$(dirname "$0")
raw_version=$(echo "$1" | sed -nE "s/^(v?([0-9]+)\.([0-9]+)\.([0-9]+)[-._]?(([a|A])lpha|([b|B])eta)?([0-9]*))$/\2.\3.\4 \5 \8/p")

arr=($raw_version)
version=${arr[0]}
tailing=${arr[1]}
tailing_version=${arr[2]}

if [[ -z "$version" ]]; then
  echo "Failed to parse version for $1!"
  exit 1
fi

if [[ -z "$tailing" ]] && [[ -n "$tailing_version" ]]; then
  echo "Given version is invalid: $1"
  exit 1
fi

if [[ -n "$tailing" ]] && [[ -z "$tailing_version" ]]; then
  echo "Given version is invalid: $1"
  exit 1
fi


if [[ -z "$tailing" ]]; then
  std_ver="$version"
  var_ver="$version"
else
  std_ver="$version-$tailing$tailing_version"
  var_ver="$version-$tailing.$tailing_version"
fi

echo "New version: $std_ver, $var_ver"

# $1: From regex; $2: to; $3: Target file
update() {
  sed -i '' -e "s/$1/$2/g" "$3" || echo "! Failed for $3"
}

# Common
update "^version = \".*\"$" "version = \"$std_ver\"" "$base_path/common/Cargo.toml"
# Clib
update "^version = \".*\"$" "version = \"$std_ver\"" "$base_path/clib/Cargo.toml"
# Golang
# ...
# Java
update "^version = \".*\"$" "version = \"$std_ver\"" "$base_path/java/Cargo.toml"
update "^version = \".*\"$" "version = \"$std_ver\"" "./java/java/lib/build.gradle"
# Lua
update "^version = \".*\"$" "version = \"$std_ver\"" "$base_path/lua/Cargo.toml"
# Node.js
update "^version = \".*\"$" "version = \"$std_ver\"" "$base_path/nodejs/Cargo.toml"
update "^  \"version\": \".*\",$" "  \"version\": \"$var_ver\"," "$base_path/nodejs/package.json"
# Python
update "^version = \".*\"$" "version = \"$std_ver\"" "$base_path/python/Cargo.toml"

echo "Finished!"
