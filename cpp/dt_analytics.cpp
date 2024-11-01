#include <string>
#include <map>
#include <stdexcept>
#include <cstdlib>
#include <json/json.h>
#include "./dt_analytics.h"

extern "C" {
    #include "include/dt_core_clib.h"
}

DTAnalytics DTAnalytics::New(DTConsumer* consumer, bool isDebug) {
    auto config = consumer->getConfig();
    Json::Value jsonConfig;

    if (isDebug) {
        jsonConfig["_debug"] = 1;
    }

    jsonConfig["consumer"] = config.consumer;
    jsonConfig["path"] = config.path;
    jsonConfig["max_batch_len"] = config.max_batch_len;
    jsonConfig["name_prefix"] = config.name_prefix;
    jsonConfig["max_file_size_bytes"] = config.max_file_size_bytes;

    Json::FastWriter writer;
    std::string eventJson = writer.write(jsonConfig);
    const char* cEventJson = eventJson.c_str();
    int ret = dt_init(cEventJson);

    if (ret != 0) {
        return DTAnalytics();
    }
    throw std::runtime_error("failed to init DTAnalytics");
}

void DTAnalytics::Track(const std::string& dtId, const std::string& acId, const std::string& eventName, Json::Value& properties) {
    add(dtId, acId, eventName, "track", properties);
}

void DTAnalytics::Flush() {
    dt_flush();
}

void DTAnalytics::Close() {
    dt_close();
}

void DTAnalytics::ToggleLogger(bool enable) {
    uint8_t enabled = enable ? 1 : 0;
    dt_toggle_logger(enabled);
}

void DTAnalytics::add(const std::string& dtId, const std::string& acId, const std::string& eventName, const std::string& eventType, Json::Value& properties) {
    Json::Value event = properties;
    event["#dt_id"] = dtId;
    event["#acid"] = acId;
    event["#event_name"] = eventName;
    event["#event_type"] = eventType;
    event["#sdk_type"] = "dt_server_sdk_cpp";

    Json::FastWriter writer;
    std::string eventJson = writer.write(event);
    const char* cEventJson = eventJson.c_str();
    int ret = dt_add_event(cEventJson);

    if (ret == 0) {
        throw std::invalid_argument("given event is not valid");
    }
}