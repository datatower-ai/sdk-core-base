# $1: Lua version number - 5.1, 5.2, 5.3, 5.4
# $2: OS - linux, macosx, windows

base=$(dirname "$0")

function halt() {
  echo "$1"
  exit 1
}

cd "$base" || halt "Failed to cd base: $base"

version=$(grep -oE "^version = \".*\"$" "../Cargo.toml" | sed -ne "s/version = \"\(.*\)\"$/\1/p")
version=$(echo "$version" | sed -nE "s/^(v?([0-9]+)\.([0-9]+)\.([0-9]+)[-._]?(([a|A])lpha|([b|B])eta)?([0-9]*))$/\2.\3.\4.\5\8/p")

if [[ -z $version ]]; then
  halt "Cannot get the version"
fi

cat << EOT >> "dt-lua-sdk-$version-1.rockspec"
package = "dt-lua-sdk"
version = "$version-1"
source = {
    url = "git://github.com/datatower-ai/sdk-core-lua",
    branch = "dt_test"
}
description = {
    summary = "DataTower.ai Lua SDK",
    homepage = "https://github.com/datatower-ai/sdk-core-lua",
    license = "BSD-3"
}
supported_platforms = { "$2" }
dependencies = {
    "lua $1"
}
build = {
    type = "builtin",
    modules = {
        DataTowerSdk = "DataTowerSdk.lua"
    },
    install = {
        lib = { "dt_core_lua.so" }
    }
}
EOT
