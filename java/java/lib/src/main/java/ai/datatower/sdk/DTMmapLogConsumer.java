package ai.datatower.sdk;

import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

public class DTMmapLogConsumer extends Consumer {
    private final String path;
    private final String namePrefix;
    private final long fileSize;
    private final long flushSize;

    /**
     * The Consumer that will put events to log files.
     * This Consumer is designed to run with FileScout.
     *
     * @param path The path/directory to store log files.
     * @param namePrefix [Nullable] The prefix of log file name.
     * @param fileSize [Nullable] Maximum size of log file in Byte. File is guarantee to not exceed this size, thus
     * single event will be rejected if its size is over file_size. Default value is 2 MB when 'None' or '0' is provided.
     * @param flushSize [Nullable] Flush will be triggered automatically when un-flushed size is equals to or over
     * flush_size in Byte. Default behaviour is flush once per file is full when 'None' or '0' is provided.
     */
    public DTMmapLogConsumer(
        String path, String namePrefix, long fileSize, long flushSize
    ) {
        this.path = path;
        this.namePrefix = namePrefix;
        this.fileSize = fileSize;
        this.flushSize = flushSize;
    }

    @Override
    Map<String, Object> getConfigMap() {
        Map<String, Object> configMap = new HashMap<>();
        configMap.put("consumer", "mlog");
        configMap.put("path", path);
        configMap.put("name_prefix", namePrefix);
        configMap.put("file_size", fileSize);
        configMap.put("flush_size", flushSize);
        return configMap;
    }
}
