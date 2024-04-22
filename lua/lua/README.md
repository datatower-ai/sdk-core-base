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
1. Get the [latest release](https://github.com/datatower-ai/sdk-core-lua/releases/latest).
   1. Download `DataTowerSdk.lua`.
   2. Download a .so/.dll file base on your:
       - Lua interpreter version (lua51, lua52, lua53, lua54, ...),
       - Operating System (Linux, macOS, Windows, ...),
       - CPU architecture (x86_64, aarch64, ...).
2. Rename downloaded .so file to `dt_core_lua.so` (`dt_core_lua.dll` for Windows)
3. Place them at the same directory in the project.
4. Finally, `dt = require("DataTowerSdk")`.

> Feel free to contact us, if no .so/.dll file is met your requirements.
