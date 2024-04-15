package ai.datatower.sdk;

import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

public class DTLogConsumer extends Consumer {
    private final String path;
    private final int maxBatchLen;
    private final String namePrefix;
    private final long maxFileSizeBytes;

    /**
     * The Consumer that will put events to log files.
     * This Consumer is designed to run with FileScout.
     *
     * @param path The path/directory to store log files.
     * @param maxBatchLen Number of events to be written into log file at once.
     * @param namePrefix [Nullable] The prefix of log file name.
     * @param maxFileSizeBytes [Nullable] The ideal maximum size for each log file. null for unlimited. (will be larger if size of a single event is over such limit)
     */
    public DTLogConsumer(
            String path, int maxBatchLen, String namePrefix, long maxFileSizeBytes
    ) {
        this.path = path;
        this.maxBatchLen = maxBatchLen;
        this.namePrefix = namePrefix;
        this.maxFileSizeBytes = maxFileSizeBytes;
    }

    @Override
    Map<String, Object> getConfigMap() {
        Map<String, Object> configMap = new HashMap<>();
        configMap.put("consumer", "log");
        configMap.put("path", path);
        configMap.put("max_batch_len", maxBatchLen);
        configMap.put("name_prefix", namePrefix);
        configMap.put("max_file_size_bytes", maxFileSizeBytes);
        return configMap;
    }
}
