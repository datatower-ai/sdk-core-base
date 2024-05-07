[![Golang CI](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_golang.yaml/badge.svg?branch=main&event=push)](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_golang.yaml)
[![Java CI](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_java.yaml/badge.svg?branch=main&event=push)](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_java.yaml)
[![Lua CI](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_lua.yaml/badge.svg?branch=main&event=push)](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_lua.yaml)
[![Node.js CI](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_nodejs.yaml/badge.svg?branch=main&event=push)](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_nodejs.yaml)
[![Python CI](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_python.yaml/badge.svg?branch=main&event=push)](https://github.com/datatower-ai/sdk-core-base/actions/workflows/build_pub_python.yaml)

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
    <span>Available in</span><br />
    <a href="" style="font-weight: bold">Golang</a>
    <span>-</span>
    <a href="" style="font-weight: bold">Java</a>
    <span>-</span>
    <a href="" style="font-weight: bold">Lua</a>
    <span>-</span>
    <a href="" style="font-weight: bold">Node.js</a>
    <span>-</span>
    <a href="" style="font-weight: bold">Python</a>
</p>

# DataTower.ai SDK Core (server-side)

> Check out our [API Docs](https://docs.datatower.ai/docs/wb9UC1) to getting started!

## Universal Working Flow
1. Enabling logger with `toggleLogger(true)` if needed.
2. Creating a `Consumer` (e.g. `DTLogConsumer`). 
3. Initializing the `DTAnalytics` with `Consumer`.
4. Using `track()` to track event. 
5. Using `userXxx()` to track user event. 
6. Manually flushing the buffer by `flush()`. 
7. Gracefully closed by `close()`. 