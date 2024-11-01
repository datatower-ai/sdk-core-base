#ifndef LOG_CONSUMER_H
#define LOG_CONSUMER_H

#include <string>
#include <cstdint>

struct JsonConfig {
    std::string consumer;
    std::string path;
    uint32_t max_batch_len;
    std::string name_prefix;
    uint64_t max_file_size_bytes;
};

// DTConsumer 基类
class DTConsumer {
public:
    virtual JsonConfig getConfig() const = 0; // 纯虚函数
};

class DTLogConsumer : public DTConsumer {
public:
    DTLogConsumer(const std::string& path, uint32_t maxBatchLen, const std::string& namePrefix, uint64_t maxFileSizeBytes);
    JsonConfig getConfig() const override;

private:
    std::string path;
    uint32_t maxBatchLen;
    std::string namePrefix;
    uint64_t maxFileSizeBytes;
};

#endif