#include <string>
#include <cstdint>
#include "log_consumer.h"

DTLogConsumer::DTLogConsumer(const std::string& path, uint32_t maxBatchLen, const std::string& namePrefix, uint64_t maxFileSizeBytes)
    : path(path), maxBatchLen(maxBatchLen), namePrefix(namePrefix), maxFileSizeBytes(maxFileSizeBytes) {}

JsonConfig DTLogConsumer::getConfig() const {
    return JsonConfig {
        consumer: "log",
        path: path,
        max_batch_len: maxBatchLen,
        name_prefix: namePrefix,
        max_file_size_bytes: maxFileSizeBytes,
    };
}