#include <iostream>
#include <string>
#include <chrono>
#include <algorithm>
#include <numeric>
#include <stdexcept>
#include <json/json.h>
#include "../log_consumer.h"
#include "../dt_analytics.h"

int main() {
    DTAnalytics::ToggleLogger(true);
    DTLogConsumer consumer("log", 1000, "dt_go_demo", 0);
    DTAnalytics dt = DTAnalytics::New(&consumer, true);
    DTAnalytics::ToggleLogger(false);

    Json::Value properties;

    Json::Value list;
    list.append("Lua");
    list.append("hello");

    properties["productNames"] = list;
    properties["productType"] = "Lua book";
    properties["producePrice"] = 80;
    properties["shop"] = "xx-shop";
    properties["shop"] = "xx-shop";
    properties["#os"] = "1.1.1.1";
    properties["#os"] = "1.1.1.1";
    properties["sex"] = "female";
    properties["#app_id"] = "appid_1234567890";
    properties["#bundle_id"] = "com.example";

    for (int i = 0; i < 20; ++i) {
        properties["a" + std::to_string(i)] = std::string(i, 'a');
    }

    const int n = 100000;
    auto start = std::chrono::high_resolution_clock::now();
    long long total_time = 0;
    std::vector<long long> lst;

    for (int i = 0; i < n; ++i) {
        // properties["$_event_call_time"] = std::to_string(std::chrono::duration_cast<std::chrono::microseconds>(
        //     std::chrono::high_resolution_clock::now().time_since_epoch()).count());

        auto st = std::chrono::high_resolution_clock::now();
        try {
            dt.Track("dtiddd", "", "simple_event", properties);
        } catch(const std::invalid_argument& err) {
            std::cout << "Caught an exception: " << err.what() << std::endl;
        }
        auto elapsed = std::chrono::duration_cast<std::chrono::microseconds>(
            std::chrono::high_resolution_clock::now() - st).count();
        
        total_time += elapsed;
        lst.push_back(elapsed);
    }

    auto total_elapsed = std::chrono::duration_cast<std::chrono::microseconds>(
        std::chrono::high_resolution_clock::now() - start).count();
    
    std::cout << "Time elapsed: " << static_cast<double>(total_elapsed) / 1000 << "ms" << std::endl;
    std::cout << "Time elapsed avg: " << static_cast<double>(total_time) / n / 1000 << "ms" << std::endl;

    std::sort(lst.begin(), lst.end());
    std::cout << "min: " << static_cast<double>(lst.front()) / 1000 << "ms" << std::endl;
    std::cout << "max: " << static_cast<double>(lst.back()) / 1000 << "ms" << std::endl;
    std::cout << "50': " << static_cast<double>(lst[(n - 1) / 2]) / 1000 << "ms" << std::endl;
    std::cout << "80': " << static_cast<double>(lst[(n - 1) * 8 / 10]) / 1000 << "ms" << std::endl;
    std::cout << "90': " << static_cast<double>(lst[(n - 1) * 9 / 10]) / 1000 << "ms" << std::endl;
    std::cout << "95': " << static_cast<double>(lst[(n - 1) * 95 / 100]) / 1000 << "ms" << std::endl;
    std::cout << "99': " << static_cast<double>(lst[(n - 1) * 99 / 100]) / 1000 << "ms" << std::endl;

    int numWrite = n / 200;
    std::cout << static_cast<double>((n - numWrite)) / n << "' : " 
              << static_cast<double>(lst[lst.size() - numWrite - 1]) / 1000 << "ms" << std::endl;

    std::vector<long long> allExceptWrite(lst.begin(), lst.end() - numWrite);
    double avgExceptWrite = std::accumulate(allExceptWrite.begin(), allExceptWrite.end(), 0LL) / 
                            static_cast<double>(allExceptWrite.size());
    std::cout << "avg (except write): " << avgExceptWrite / 1000 << "ms" << std::endl;

    std::vector<long long> allOnlyWrite(lst.end() - numWrite, lst.end());
    double avgOnlyWrite = std::accumulate(allOnlyWrite.begin(), allOnlyWrite.end(), 0LL) / 
                          static_cast<double>(allOnlyWrite.size());
    std::cout << "avg (write only): " << avgOnlyWrite / 1000 << "ms" << std::endl;

    dt.Flush();
    dt.Close();

    return 0;
}