package ai.datatower.sdk;

import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

public class DTLogConsumer extends Consumer {
    private final Map<String, Object> configMap;

    /**
     * The Consumer that will put events to log files.
     * This Consumer is designed to run with FileScout.
     *
     * @param path The folder to store log files.
     * @param maxBatchLen Number of events to flush into log file at once.
     * @param namePrefix [Nullable] The prefix of log file name.
     * @param maxFileSizeBytes [Nullable] The ideal maximum size for each log file. (will be larger if size of a single event is over such limit)
     */
    public DTLogConsumer(
            String path, int maxBatchLen, String namePrefix, long maxFileSizeBytes
    ) {
        configMap = new HashMap<>();
        configMap.put("consumer", "log");
        configMap.put("path", path);
        configMap.put("max_batch_len", maxBatchLen);
        configMap.put("name_prefix", namePrefix);
        configMap.put("max_file_size_bytes", maxFileSizeBytes);
    }

    @Override
    Map<String, Object> getConfigMap() {
        return Collections.unmodifiableMap(configMap);
    }
}
