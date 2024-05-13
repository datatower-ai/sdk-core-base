[![Golang CI](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_golang.yaml/badge.svg)](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_golang.yaml)
[![Java CI](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_java.yaml/badge.svg)](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_java.yaml)
[![Lua CI](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_lua.yaml/badge.svg)](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_lua.yaml)
[![Node.js CI](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_nodejs.yaml/badge.svg)](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_nodejs.yaml)
[![Python CI](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_python.yaml/badge.svg)](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_python.yaml)


<p align="center">
    <a href="https://datatower.ai/" target="_blank">
        <picture>
            <source srcset="https://dash.datatower.ai/logo_v2.png" media="(prefers-color-scheme: dark)">
            <source srcset="https://dash.datatower.ai/logoWhite_v2.png" media="(prefers-color-scheme: light)" >
            <img src="https://dash.datatower.ai/logoWhite_v2.png" alt="DataTower.ai">
        </picture>
    </a>
</p>

<p align="center">
    <img src="https://img.shields.io/github/v/release/datatower-ai/sdk-core-base?style=for-the-badge&logo=aHR0cHM6Ly9wcml2YXRlLmRhdGF0b3dlci5haS9mYXZpY29uX3YyLmljbw%3D%3D&label=DT%20SDK%20(Server)&labelColor=FFD406&color=4934C9">
</p>

<p align="center">
    <span>Available in</span><br />
    <a href="https://github.com/datatower-ai/dt-golang-sdk/" style="font-weight: bold">Golang</a>
    <span>-</span>
    <a href="https://jitpack.io/#datatower-ai/dt-java-sdk" style="font-weight: bold">Java</a>
    <span>-</span>
    <a href="https://github.com/datatower-ai/sdk-core-lua/releases/latest/" style="font-weight: bold">Lua</a>
    <span>-</span>
    <a href="https://www.npmjs.com/package/@datatower-ai/sdk-core-nodejs" style="font-weight: bold">Node.js</a>
    <span>-</span>
    <a href="https://pypi.org/project/dt-python-sdk/" style="font-weight: bold">Python</a>
</p>

# DataTower.ai SDK (Server-side)

> Check out our [API Docs](https://docs.datatower.ai/docs/wb9UC1) to getting started!

## Universal Working Flow
1. Enabling logger with `toggleLogger(true)` if needed.
2. Creating a `Consumer` (e.g. `DTLogConsumer`). 
3. Initializing the `DTAnalytics` with `Consumer`.
4. Using `track()` to track event. 
5. Using `userXxx()` to track user event. 
6. Manually flushing the buffer by `flush()`. 
7. Gracefully closed by `close()`. 