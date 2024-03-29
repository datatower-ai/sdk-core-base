package ai.datatower.sdk;

import java.util.*;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.function.Supplier;

class DTBase {
    static native boolean init(Map<String, Object> config);
    static native boolean addEvent(Map<String, Object> event);
    static native void flush();
    static native void close();
    static native void toggleLogger(boolean enable);
    static native void setStaticCommonProperties(Map<String, Object> properties);
    static native void clearStaticCommonProperties();

    static AtomicBoolean loaded = new AtomicBoolean(false);

    static void load(String filename) {
        if (loaded.get()) return;
        System.load(filename);
        loaded.set(true);
    }
}

class DTAnalytics {
    private static final String SDK_TYPE = "dt_server_sdk_java";
    private static final String SDK_VERSION = "1.0.0";

    private Supplier<Map<String, Object>> dynamicCommonPropertiesSupplier;

    public DTAnalytics(Consumer consumer, String soFileName, boolean isDebug) {
        DTBase.load(soFileName);
        DTBase.toggleLogger(isDebug);
        DTBase.init(consumer.getConfigMap());
    }
    
    public boolean track(String dtId, String acId, String eventName, Map<String, Object> properties) {
        return add(dtId, acId, eventName, "track", properties);
    }

    public boolean userSet(String dtId, String acId, Map<String, Object> properties) {
        return add(dtId, acId, "#user_set", "user", properties);
    }

    public boolean userSetOnce(String dtId, String acId, Map<String, Object> properties) {
        return add(dtId, acId, "#user_set_once", "user", properties);
    }

    public boolean userAdd(String dtId, String acId, Map<String, Object> properties) {
        return add(dtId, acId, "#user_add", "user", properties);
    }

    public boolean userUnset(String dtId, String acId, List<String> properties) {
        Map<String, Object> props = new HashMap<>();
        for (String prop : properties) {
            props.put(prop, 0);
        }
        return add(dtId, acId, "#user_unset", "user", props);
    }

    public boolean userDelete(String dtId, String acId) {
        return add(dtId, acId, "#user_delete", "user", new HashMap<>());
    }

    public boolean userAppend(String dtId, String acId, Map<String, Object> properties) {
        return add(dtId, acId, "#user_append", "user", properties);
    }

    public boolean userUniqAppend(String dtId, String acId, Map<String, Object> properties) {
        return add(dtId, acId, "#user_uniq_append", "user", properties);
    }

    public void setDynamicCommonProperties(Supplier<Map<String, Object>> dynamicCommonPropertiesSupplier) {
        this.dynamicCommonPropertiesSupplier = dynamicCommonPropertiesSupplier;
    }

    public void clearDynamicCommonProperties() {
        this.dynamicCommonPropertiesSupplier = null;
    }

    public void setStaticCommonProperties(Map<String, Object> properties) {
        DTBase.setStaticCommonProperties(properties);
    }

    public void clearStaticCommonProperties() {
        DTBase.clearStaticCommonProperties();
    }
    
    public void flush() {
        DTBase.flush();
    }

    public void close() {
        DTBase.close();
    }
    
    private boolean add(String dtId, String acId, String eventName, String eventType, Map<String, Object> properties) {
        Map<String, Object> event = new HashMap<>();

        if (dynamicCommonPropertiesSupplier != null) {
            try {
                event.putAll(dynamicCommonPropertiesSupplier.get());
            } catch (Throwable t) {
                t.printStackTrace();
            }
        }

        event.putAll(properties);
        event.put("#dt_id", dtId);
        event.put("#acid", acId);
        event.put("#event_name", eventName);
        event.put("#event_type", eventType);
        event.put("#sdk_type", DTAnalytics.SDK_TYPE);
        event.put("#sdk_version_name", DTAnalytics.SDK_VERSION);
        
        return DTBase.addEvent(event);
    }
}

abstract class Consumer {
    abstract Map<String, Object> getConfigMap();
}

class DTLogConsumer extends Consumer {
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

class MainDemo {
    public static void main(String[] args) {
        System.out.println("xxxxx");
        DTLogConsumer consumer = new DTLogConsumer("log", 200, null, 10*1024*1024);
        DTAnalytics dt = new DTAnalytics(consumer, "/Users/linkailong/RustroverProjects/sdk-core-base/target/release/libdt_core_java.dylib", true);

        HashMap<String, Object> properties = new HashMap<>();
        properties.put("#app_id", "appidid");
        properties.put("#bundle_id", "com.example");
        properties.put("prooo", "hool");
        dt.track("xxx", null, "event_test_java_sdk", properties);
        dt.flush();
        dt.close();
    }
}