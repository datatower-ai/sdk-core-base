<p align="center">
    <a href="https://datatower.ai/" target="_blank">
        <picture>
            <source srcset="https://dash.datatower.ai/logo_v2.png" media="(prefers-color-scheme: dark)">
            <source srcset="https://dash.datatower.ai/logoWhite_v2.png" media="(prefers-color-scheme: light)" >
            <img src="https://dash.datatower.ai/logoWhite_v2.png" alt="DataTower.ai">
        </picture>
    </a>
</p>

# DataTower.ai Lua SDK | Server
This repository is used for publication purpose of Lua packages, and refers to [this repo](https://github.com/datatower-ai/sdk-core-base).

> ⚠ This SDK is designed to work with [FileScout](https://docs.datatower.ai/docs/FileScout-shi-yong-zhi-nan).

## Getting Started
**With Luarocks**
1. Get the [latest release](https://github.com/datatower-ai/sdk-core-lua/releases/latest).
2. Download a `*.src.rock` file base on your:
    - Lua interpreter version (lua51, lua52, lua53, lua54, ...),
    - Operating System (Linux, macOS, Windows, ...),
    - CPU architecture (x86_64, aarch64, ...).
3. Rename downloaded `*.src.rock` file to `dt-lua-sdk-{version}.src.rock`
   - Removes the prefix part: `{luaXx}-{OS}-{Arch}-`
4. Install the rock with Luarocks
   ```bash
   luarocks install dt-lua-sdk-{version}.src.rock
   #example:
   luarocks install dt-lua-sdk-1.0.0-1.src.rock
   ```
5. Finally, `dt = require("DataTowerSdk")`.

> Feel free to contact us, if no release file is met your requirements.
