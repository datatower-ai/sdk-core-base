-- $ lua dev/Benchmark.lua
socket = require("socket")

function getDivider()
    return package.config:sub(1,1)
end
function script_path()
    local str = debug.getinfo(2, "S").source:sub(2)
    local result = str:match("(.*" .. getDivider() .. ")")
    if result == nil then
        return ""
    end
    return result
end
local sdk_path = script_path() .. ".." .. getDivider() .. "lua" .. getDivider() .. "lua" .. getDivider()
package.path = package.path .. ";" .. sdk_path .. "?.lua"
local dtAnalytics = require("DataTowerSdk")

table.reduce = function (list, fn, init)
    local acc = init
    for k, v in ipairs(list) do
        if 1 == k and not init then
            acc = v
        else
            acc = fn(acc, v)
        end
    end
    return acc
end

table.sum = function(tb)
    return table.reduce(tb, function(a, b) return a + b end, 0)
end

local function getLogConsumer()
return dtAnalytics.DTLogConsumer("log", 200, 10 * 1024 * 1024)
end

dtAnalytics.enableLog(true)

local consumer = getLogConsumer()

--- init SDK with consumer
local sdk = dtAnalytics(consumer, false)

local dtId = "1234567890987654321"
local acId = nil

sdk:userSet("xx", "xxxx", { ["#app_id"] = "aaa", ["#bundle_id"] = "123", ["bb"] = 2, ["cc"] = "ss" })
sdk:flush()
dtAnalytics.enableLog(false)

local properties = {}
properties["productNames"] = { "Lua", "hello" }
properties["productType"] = "Lua book"
properties["producePrice"] = 80
properties["shop"] = "xx-shop"
properties["#os"] = "1.1.1.1"
properties["sex"] = 'female'
properties["#app_id"] = "appid_1234567890"
properties["#bundle_id"] = "com.example"
for i = 0, 5, 1
do
   properties["a" .. tostring(i)] = string.rep("asd", i)
end

n = 100000
tm = 0
local lst = {}
local start_time = socket.gettime()
for i = n, 1, -1
do
    properties["$_event_call_time"] = tostring(math.floor(socket.gettime() * 1000000))
    st = socket.gettime()
    sdk:track(acId, dtId, "eventName", properties)
    tmp = socket.gettime() - st
    tm = tm + tmp
    lst[#lst+1] = tmp
end
local end_time = socket.gettime()
print('time elapsed: ' .. ((end_time - start_time)*1000) .. 'ms')
print('time elapsed avg: ' .. (tm / n * 1000) .. 'ms')
table.sort(lst)
print('min: ' .. tostring(lst[1]*1000) .. "ms")
print("max: " .. tostring(lst[#lst]*1000) .. "ms")
print("50': " .. tostring(lst[math.floor((#lst-1)*0.5)]*1000) .. "ms")
print("80': " .. tostring(lst[math.floor((#lst-1)*0.8)]*1000) .. "ms")
print("90': " .. tostring(lst[math.floor((#lst-1)*0.9)]*1000) .. "ms")
print("95': " .. tostring(lst[math.floor((#lst-1)*0.95)]*1000) .. "ms")
print("99': " .. tostring(lst[math.floor((#lst-1)*0.99)]*1000) .. "ms")
num_write = math.floor(n/200)
print(tostring((n-num_write)/n) .. "': " .. tostring(lst[#lst-num_write]*1000) .. "ms")
all_except_write = {}
local i = 1
while (i <= #lst-num_write) do
    all_except_write[i] = lst[i]
    i = i + 1
end
print("avg (except write): " .. tostring(table.sum(all_except_write) / #all_except_write * 1000) .. "ms")
all_only_write = {}
local i2 = #lst-num_write+1
while (i2 <= #lst) do
    all_only_write[#all_only_write+1] = lst[i2]
    i2 = i2 + 1
end
print("avg (write only): " .. tostring(table.sum(all_only_write) / #all_only_write * 1000) .. "ms")

sdk:flush()
sdk:close()

--[[
Benchmark:
** 2024.04.10 **
QPS: 12000~15000
average: 0.0745ms
average (except write): 0.053 ms
average (write only): 4.256ms
80': 0.068ms
90': 0.083ms
95': 0.099ms
99': 0.160ms
]]--