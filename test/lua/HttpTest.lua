--[[

Run this:
    $ lua HttpTest.lua 8015

Run Client:
    1. From command line:
        $ curl -X POST http://127.0.0.1:8015/track -d '{"dt_id": "test_dt_id", "event_name": "curl_event", "props": {"ppp": "vvv"}}'

    2. From browser terminal:
        fetch('http://127.0.0.1:8015/track', {
          method: 'POST',
          body: JSON.stringify({
            "dt_id": 'test_dt_id',
            "acid": 'acid123',
            "event_name": "curl_event",
            "props": {"ppp": "vvv"}
          })
        }).then(console.log)

]]--
local dtAnalytics = require("DataTowerSdk")

dtAnalytics.enableLog(true)

local sdk

---

local json = require("JSON")
local http_server = require("http.server")
local http_headers = require "http.headers"

local port = arg[1] or 0 -- 0 means pick one at random

function split (inputstr, sep)
    if sep == nil then
        sep = "%s"
    end
    local t={}
    for str in string.gmatch(inputstr, "([^"..sep.."]+)") do
        table.insert(t, str)
    end
    return t
end

local function reply(myserver, stream) -- luacheck: ignore 212
    -- Read in headers
    local req_headers = assert(stream:get_headers())
    local req_method = req_headers:get ":method"

    -- Log request to stdout
    assert(io.stdout:write(string.format('[DT ServerTest] [%s] "%s %s HTTP/%g"  "%s" "%s"\n',
            os.date("%d/%b/%Y:%H:%M:%S %z"),
            req_method or "",
            req_headers:get(":path") or "",
            stream.connection.version,
            req_headers:get("referer") or "-",
            req_headers:get("user-agent") or "-"
    )))

    local additional_text = ""

    if (req_method or "") == "POST" then
        local path = req_headers:get ":path"

        if path == "/init" then
            local body_str = stream:get_body_as_string()
            print("[DT ServerTest] Received /init " .. body_str)
            local consumer
            if body_str == nil or body_str == '' then
                consumer = dtAnalytics.DTLogConsumer("log_test", 200, nil, nil)
            else
                local body = json:decode(body_str)
                local path = body["path"] or "log_test"
                local max_batch_length = body["max_batch_length"] or 200
                local log_prefix = body["log_prefix"] or nil
                local max_log_size = body["max_log_size"] or nil
                local debug = body["debug"] or false
                consumer = dtAnalytics.DTLogConsumer(path, max_batch_length, max_log_size, log_prefix)
            end
            sdk = dtAnalytics(consumer, debug)
            additional_text = "init"
        elseif path == "/track" then
            local body_str = stream:get_body_as_string()
            print("[DT ServerTest] Received /track " .. body_str)
            local body = json:decode(body_str)
            local dt_id = body["dt_id"] or nil
            local acid = body["acid"] or nil
            local event_name = body["event_name"] or nil
            local props = body["props"] or nil
            sdk:track(acid, dt_id, event_name, props)
            additional_text = "track"
        elseif path == "/user_set" then
            local body_str = stream:get_body_as_string()
            print("[DT ServerTest] Received /userSet " .. body_str)
            local body = json:decode(body_str)
            local dt_id = body["dt_id"] or nil
            local acid = body["acid"] or nil
            local props = body["props"] or nil
            sdk:userSet(acid, dt_id, props)
            additional_text = "user_set"
        elseif path == "/user_set_once" then
            local body_str = stream:get_body_as_string()
            print("[DT ServerTest] Received /userSetOnce " .. body_str)
            local body = json:decode(body_str)
            local dt_id = body["dt_id"] or nil
            local acid = body["acid"] or nil
            local props = body["props"] or nil
            sdk:userSetOnce(acid, dt_id, props)
            additional_text = "user_set_once"
        elseif path == "/user_unset" then
            local body_str = stream:get_body_as_string()
            print("[DT ServerTest] Received /userUnset " .. body_str)
            local body = json:decode(body_str)
            local dt_id = body["dt_id"] or nil
            local acid = body["acid"] or nil
            local props = body["props"] or nil
            sdk:userUnset(acid, dt_id, props)
            additional_text = "user_unset"
        elseif path == "/user_append" then
            local body_str = stream:get_body_as_string()
            print("[DT ServerTest] Received /userAppend " .. body_str)
            local body = json:decode(body_str)
            local dt_id = body["dt_id"] or nil
            local acid = body["acid"] or nil
            local props = body["props"] or nil
            sdk:userAppend(acid, dt_id, props)
            additional_text = "user_append"
        elseif path == "/user_uniq_append" then
            local body_str = stream:get_body_as_string()
            print("[DT ServerTest] Received /userUniqAppend " .. body_str)
            local body = json:decode(body_str)
            local dt_id = body["dt_id"] or nil
            local acid = body["acid"] or nil
            local props = body["props"] or nil
            sdk:userUniqAppend(acid, dt_id, props)
            additional_text = "user_uniq_append"
        elseif path == "/user_delete" then
            local body_str = stream:get_body_as_string()
            print("[DT ServerTest] Received /userDelete " .. body_str)
            local body = json:decode(body_str)
            local dt_id = body["dt_id"] or nil
            local acid = body["acid"] or nil
            local props = body["props"] or nil
            sdk:userDelete(acid, dt_id, props)
            additional_text = "user_delete"
        elseif path == "/user_add" then
            local body_str = stream:get_body_as_string()
            print("[DT ServerTest] Received /userAdd " .. body_str)
            local body = json:decode(body_str)
            local dt_id = body["dt_id"] or nil
            local acid = body["acid"] or nil
            local props = body["props"] or nil
            sdk:userAdd(acid, dt_id, props)
            additional_text = "user_add"
        elseif path == "/flush" then
            print("[DT ServerTest] Received /flush")
            sdk:flush()
            additional_text = "flush"
        elseif path == "/close" then
            print("[DT ServerTest] Received /close")
            sdk:flush()
            sdk:close()
            print("[DT ServerTest] Closing...")
            additional_text = "close"
            myserver:close()
        elseif path == "/log/enable" then
            dtAnalytics.enableLog(true)
            additional_text = "/log/enable"
        elseif path == "/log/disable" then
            dtAnalytics.enableLog(false)
            additional_text = "/log/disable"
        end
    end

    -- Build response headers
    local res_headers = http_headers.new()
    res_headers:append(":status", "200")
    res_headers:append("content-type", "text/plain")
    -- Send headers to client; end the stream immediately if this was a HEAD request
    assert(stream:write_headers(res_headers, req_method == "HEAD"))
    if req_method ~= "HEAD" then
        -- Send body, ending the stream
        assert(stream:write_chunk("DT Core Lua ServerTest\n" .. additional_text .. "\n", true))
    end
end

local myserver = assert(http_server.listen {
    host = "localhost";
    port = port;
    onstream = reply;
    onerror = function(myserver, context, op, err, errno) -- luacheck: ignore 212
        local msg = op .. " on " .. tostring(context) .. " failed"
        if err then
            msg = msg .. ": " .. tostring(err)
        end
        assert(io.stderr:write(msg, "\n"))
    end;
})

-- Manually call :listen() so that we are bound before calling :localname()
assert(myserver:listen())
do
    local bound_port = select(3, myserver:localname())
    assert(io.stderr:write(string.format("[DT ServerTest] Now listening on port %d\n", bound_port)))
end
-- Start the main server loop
assert(myserver:loop())