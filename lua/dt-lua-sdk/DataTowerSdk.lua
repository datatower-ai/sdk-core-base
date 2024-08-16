-- DT LuaSDK
function loadBaseLib()
--     local divider = package.config:sub(1,1)
--     local suffix = divider == "\\" and "dll" or "so"
--     local pathStr = debug.getinfo(2, "S").source:sub(2)
--     local path = pathStr:match("(.*" .. divider .. ")")
--     if path == nil then
--         return nil
--     end
--     package.cpath = package.cpath .. ";" .. path .. "?." .. suffix .. ";"
    return require("dt_core_lua")
end
local dt_base = loadBaseLib()

local Util = {}
local DTLog = {}

local function class(base, _ctor)
    local c = {}
    if not _ctor and type(base) == 'function' then
        _ctor = base
        base = nil
    elseif type(base) == 'table' then
        for i, v in pairs(base) do
            c[i] = v
        end
        c._base = base
    end
    c.__index = c
    local mt = {}
    mt.__call = function(_, ...)
        local obj = {}
        setmetatable(obj, c)
        if _ctor then
            _ctor(obj, ...)
        end
        return obj
    end
    c._ctor = _ctor
    c.is_a = function(self, klass)
        local m = getmetatable(self)
        while m do
            if m == klass then
                return true
            end
            m = m._base
        end
        return false
    end
    setmetatable(c, mt)
    return c
end

---
---@param dtId string
---@param acId string
---@param eventType string
---@param eventName string
---@param properties table
---@param superProperties table
---@param dynamicSuperPropertiesTracker function
local function upload(dtId, acId, eventType, eventName, properties, superProperties, dynamicSuperPropertiesTracker, debug)
    local dynamicSuperProperties = {}
    if dynamicSuperPropertiesTracker ~= nil and type(dynamicSuperPropertiesTracker) == "function" then
        dynamicSuperProperties = dynamicSuperPropertiesTracker()
    end

    if acId ~= nil and string.len(acId) ~= 0 then
        properties["#acid"] = tostring(acId)
    end

    if dtId ~= nil and string.len(dtId) ~= 0 then
        properties["#dt_id"] = tostring(dtId)
    end

    properties["#event_type"] = eventType
    properties["#event_name"] = tostring(eventName)

    if debug ~= nil and type(debug) == type(true) then
        properties["#debug"] = debug
    end
    if eventType == "track" then
        properties = Util.mergeTables(properties, superProperties)
        properties = Util.mergeTables(properties, dynamicSuperProperties)
    end
    properties["#sdk_type"] = DTAnalytics.platform

    result = dt_base.add_event(properties)
    return result
end

---
--- Init analytics instance
---@param self any
---@param consumer any consumer
DTAnalytics = class(function(self, consumer, debug)
    if consumer == nil or type(consumer) ~= "table" or consumer.consumerProps == nil then
        DTLog.error("consumer params is invalidate.")
        return
    end
    self.superProperties = {}
    self.dynamicSuperPropertiesTracker = nil
    self.debug = debug
    dt_base.init(consumer.consumerProps)

    DTLog.info("SDK init success")
end)

--- Enable log or not
---@param enable boolean
function DTAnalytics.enableLog(enable)
    DTLog.enable = enable
    dt_base.enable_log(enable)
end

--- Set common properties
---@param params table
function DTAnalytics:setSuperProperties(params)
    if (type(params) == "table") then
        self.superProperties = Util.mergeTables(self.superProperties, params)
    end
end

--- Set common property
---@param key string
---@param value any
function DTAnalytics:setSuperProperty(key, value)
    if (key ~= nil) then
        local params = {}
        params[key] = value
        DTLog.info(params[key])
        self:setSuperProperties(params)
    end
end

--- Remove common properties with key
---@param key any
function DTAnalytics:removeSuperProperty(key)
    if key == nil then
        return nil
    end
    self.superProperties[key] = nil
end

--- Find common properties with key
---@param key string
function DTAnalytics:getSuperProperty(key)
    if key == nil then
        return nil
    end
    return self.superProperties[key]
end

--- Get all properties
---@return table
function DTAnalytics:getSuperProperties()
    return self.superProperties
end

--- Clear common properties
function DTAnalytics:clearSuperProperties()
    self.superProperties = {}
end

--- Set user properties. Would overwrite existing names
---@param acId string
---@param dtId string
---@param properties table
function DTAnalytics:userSet(acId, dtId, properties)
    local ok, ret = pcall(upload, dtId, acId, "user", "#user_set", properties, self.debug)
    if ok then
        return ret
    end
end

--- Set user properties, if such property had been set before, this message would be neglected.
---@param acId string
---@param dtId string
---@param properties table
function DTAnalytics:userSetOnce(acId, dtId, properties)
    local ok, ret = pcall(upload, dtId, acId, "user", "#user_set_once", properties, self.debug)
    if ok then
        return ret
    end
end

--- To accumulate operations against the property
---@param acId string
---@param dtId string
---@param properties table
function DTAnalytics:userAdd(acId, dtId, properties)
    local ok, ret = pcall(upload, dtId, acId, "user", "#user_add", properties, self.debug)
    if ok then
        return ret
    end
end

--- To add user properties of array type
---@param acId string
---@param dtId string
---@param properties table
function DTAnalytics:userAppend(acId, dtId, properties)
    local ok, ret = pcall(upload, dtId, acId, "user", "#user_append", properties, self.debug)
    if ok then
        return ret
    end
end

--- Append user properties to array type by unique.
---@param acId string
---@param dtId string
---@param properties table
function DTAnalytics:userUniqAppend(acId, dtId, properties)
    local ok, ret = pcall(upload, dtId, acId, "user", "#user_uniq_append", properties, self.debug)
    if ok then
        return ret
    end
end

--- Clear the user properties of users
---@param acId string
---@param dtId string
---@param properties table
function DTAnalytics:userUnset(acId, dtId, properties)
    local unSetProperties = {}
    for key, value in pairs(properties) do
        if Util.startWith(key, '#')then
            unSetProperties[key] = value
        else
            unSetProperties[key] = 0
        end
    end
    local ok, ret = pcall(upload, dtId, acId, "user", "#user_unset", unSetProperties, self.debug)
    if ok then
        return ret
    end
end

--- Delete a user, This operation cannot be undone
---@param acId string
---@param dtId string
function DTAnalytics:userDelete(acId, dtId, properties)
    local ok, ret = pcall(upload, dtId, acId, "user", "#user_delete", properties, self.debug)
    if ok then
        return ret
    end
end

--- Report ordinary event
---@param acId string
---@param dtId string
---@param eventName string
---@param properties table
function DTAnalytics:track(acId, dtId, eventName, properties)
    local ok, ret = pcall(upload, dtId, acId, "track", eventName, properties, self.superProperties, self.dynamicSuperPropertiesTracker, self.debug)
    if ok then
        return ret
    end
end

--- Flush data
function DTAnalytics:flush()
    dt_base.flush()
end

--- Close SDK
function DTAnalytics:close()
    dt_base.close()
    DTLog.info("SDK closed!")
end


--- Construct LogConsumer
---@param self any
---@param logPath string, The path/directory to store event logs, will be created if not exist.
---@param batchNum number, maximum number of events to be written to log once.
---@param fileSize number, approximated maximum file size in bytes. (will be exceeded if the actual event size is larger)
---@param fileNamePrefix string, prefix of log file.
DTAnalytics.DTLogConsumer = class(function(self, logPath, batchNum, fileSize, fileNamePrefix)
    self.consumerProps = {
        ["consumer"] = "log",
        ["path"] = logPath,
        ["max_batch_len"] = batchNum,
        ["name_prefix"] = fileNamePrefix,
        ["max_file_size_bytes"] = fileSize
    }
end)


--- Construct LogConsumer (MMAP)
---@param self any
---@param logPath string, The path/directory to store event logs, will be created if not exist.
---@param batchNum number, prefix of log file.
---@param fileSize number, Maximum size of log file in Byte. File is guarantee to not exceed this size, thus single event will be rejected if its size is over file_size. Default value is 2 MB when 'None' or '0' is provided.
---@param flushSize number, Flush will be triggered automatically when un-flushed size is equals to or over flush_size in Byte. Default behaviour is flush once per file is full when 'None' or '0' is provided.
DTAnalytics.DTMmapLogConsumer = class(function(self, logPath, fileNamePrefix, fileSize, flushSize)
    self.consumerProps = {
        ["consumer"] = "mlog",
        ["path"] = logPath,
        ["name_prefix"] = fileNamePrefix,
        ["file_size"] = fileSize,
        ["flush_size"] = flushSize
    }
end)

--- Set dynamic common properties
---@param callback function
function DTAnalytics:setDynamicSuperProperties(callback)
    if callback ~= nil then
        self.dynamicSuperPropertiesTracker = callback
    end
end


DTAnalytics.platform = "dt_lua_sdk"

function Util.mergeTables(...)
    local tabs = { ... }
    if not tabs then
        return {}
    end
    local origin = tabs[1]
    for i = 2, #tabs do
        if origin then
            if tabs[i] then
                for k, v in pairs(tabs[i]) do
                    if (v ~= nil) then
                        origin[k] = v
                    end
                end
            end
        else
            origin = tabs[i]
        end
    end
    return origin
end

function Util.startWith(str, substr)
    if str == nil or substr == nil then
        return nil, "the string or the substring parameter is nil"
    end
    if string.find(str, substr) ~= 1 then
        return false
    else
        return true
    end
end

Util.enableLog = false
DTLog.enable = false
function DTLog.info(...)
    if DTLog.enable then
        io.write("[DT Lua][" .. os.date("%Y-%m-%d %H:%M:%S") .. "][Info] ")
        print(...)
    end
end

function DTLog.error(...)
    if DTLog.enable then
        io.write("[DT Lua][" .. os.date("%Y-%m-%d %H:%M:%S") .. "][Error] ")
        print(...)
    end
end

return DTAnalytics
