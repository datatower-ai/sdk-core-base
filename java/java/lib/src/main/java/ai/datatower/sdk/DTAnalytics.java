package ai.datatower.sdk;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class DTAnalytics {
    private static final String SDK_TYPE = "dt_server_sdk_java";
    private static final String SDK_VERSION = "1.0.0";

    private DTAnalytics(Consumer consumer, boolean isDebug) {
        Map<String, Object> config = consumer.getConfigMap();
        config.put("_debug", isDebug);
        DTBase.init(config);
    }

    public static DTAnalytics init(Consumer consumer, boolean isDebug) {
        try {
            return new DTAnalytics(consumer, isDebug);
        } catch (Throwable t) {
            System.out.println("[DT Java] Failed to init DTAnalytics!");
            t.printStackTrace();
            return null;
        }
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
    
    public void flush() {
        DTBase.flush();
    }

    public void close() {
        DTBase.close();
    }

    public static void toggleLogger(boolean enable) {
        DTBase.toggleLogger(enable);
    }
    
    private boolean add(String dtId, String acId, String eventName, String eventType, Map<String, Object> properties) {
        Map<String, Object> event = new HashMap<>();

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