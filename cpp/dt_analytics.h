#ifndef DT_ANALYTICS_H
#define DT_ANALYTICS_H

#include "log_consumer.h"
#include <json/json.h>

class DTAnalytics {
public:
    static DTAnalytics New(DTConsumer* consumer, bool isDebug);
    void Track(const std::string& dtId, const std::string& acId, const std::string& eventName, Json::Value& properties);
    void Flush();
    void Close();
    static void ToggleLogger(bool enable);

private:
    void add(const std::string& dtId, const std::string& acId, const std::string& eventName, const std::string& eventType, Json::Value& properties);
};

#endif