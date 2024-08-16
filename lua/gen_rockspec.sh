# $1: Lua version number - 5.1, 5.2, 5.3, 5.4
# $2: OS - linux, macosx, windows

base=$(dirname "$0")

halt() {
  echo "$1"
  exit 1
}

cd "$base" || halt "Failed to cd base: $base"

version=$(grep -oE "^version = \".*\"$" "./Cargo.toml" | sed -ne "s/version = \"\(.*\)\"$/\1/p")
version=$(echo "$version" | sed -nE "s/^(v?([0-9]+)\.([0-9]+)\.([0-9]+)([-._]?)(([a|A])lpha|([b|B])eta|SNAPSHOT)?([0-9]*))$/\2.\3.\4\5\6\9/p")

if [[ -z $version ]]; then
  halt "Cannot get the version"
fi

archive_name=$(basename "$(find . -maxdepth 1 -mindepth 1 -type f -name "dt-lua-sdk-*.tar.gz" | head -1)")

echo "dt-lua-sdk-$version-1.rockspec"
cat << EOT > "dt-lua-sdk-$version-1.rockspec"
package = "dt-lua-sdk"
version = "$version-1"
source = {
    url = "file://$(pwd)/$archive_name",
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
