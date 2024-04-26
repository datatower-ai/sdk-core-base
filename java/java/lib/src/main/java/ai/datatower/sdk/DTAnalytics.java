package ai.datatower.sdk;

import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class DTAnalytics {
    private static final String SDK_TYPE = "dt_server_sdk_java";

    private static volatile DTAnalytics instance = null;

    private DTAnalytics(Consumer consumer, boolean isDebug) {
        Map<String, Object> config = consumer.getConfigMap();
        config.put("_debug", isDebug);
        DTBase.init(config);
    }

    /**
     * Initialize the DTAnalytics with given consumer.
     *
     * @param consumer DTConsumer. e.g. DTLogConsumer.
     * @param isDebug If set to true, the data will not be inserted to production environment.
     */
    public static DTAnalytics init(Consumer consumer, boolean isDebug) {
        try {
            if (instance == null) {
                synchronized (DTAnalytics.class) {
                    if (instance == null) {
                        instance = new DTAnalytics(consumer, isDebug);
                    }
                }
            }
            return instance;
        } catch (Throwable t) {
            System.out.println("[DT Java] Failed to init DTAnalytics!");
            t.printStackTrace();
            return null;
        }
    }

    /**
     * Preload the dynamic library.
     */
    public static void preload() throws IOException {
        DTBase.load();
    }

    /**
     * Track an event.
     *
     * @param dtId The device-scoped id.
     * @param acId The account-scoped id.
     * @param eventName Event name, can be custom event or preset event.
     * @param properties properties of this event. (preset properties are scoped by event name, and has type constraints)
     * @return True if given event and properties is valid or False if there are invalid and will not be processed.
     */
    public boolean track(String dtId, String acId, String eventName, Map<String, Object> properties) {
        return add(dtId, acId, eventName, "track", properties);
    }

    /**
     * Set user properties for the user with given dtId and acId.
     *
     * @param dtId The device-scoped id.
     * @param acId The account-scoped id.
     * @param properties properties of this event. (preset properties are scoped by event name, and has type constraints)
     * @return True if given event and properties is valid or False if there are invalid and will not be processed.
     */
    public boolean userSet(String dtId, String acId, Map<String, Object> properties) {
        return add(dtId, acId, "#user_set", "user", properties);
    }

    /**
     * Set user properties for the user with given dtId and acId.
     * the value will not override existed property.
     *
     * @param dtId The device-scoped id.
     * @param acId The account-scoped id.
     * @param properties properties of this event. (preset properties are scoped by event name, and has type constraints)
     * @return True if given event and properties is valid or False if there are invalid and will not be processed.
     */
    public boolean userSetOnce(String dtId, String acId, Map<String, Object> properties) {
        return add(dtId, acId, "#user_set_once", "user", properties);
    }

    /**
     * Arithmetic add the value of property by given number for user with given dtId and acId.
     * Hence, the type of value for 'custom properties' should be a number.
     *
     * @param dtId The device-scoped id.
     * @param acId The account-scoped id.
     * @param properties properties of this event. (preset properties are scoped by event name, and has type constraints)
     * @return True if given event and properties is valid or False if there are invalid and will not be processed.
     */
    public boolean userAdd(String dtId, String acId, Map<String, Object> properties) {
        return add(dtId, acId, "#user_add", "user", properties);
    }

    /**
     * Unset properties for user with given dtId and acId.
     * Only the key of 'custom properties' will be used and its value is meaningless here.
     *
     * @param dtId The device-scoped id.
     * @param acId The account-scoped id.
     * @param properties properties of this event. (preset properties are scoped by event name, and has type constraints)
     * @return True if given event and properties is valid or False if there are invalid and will not be processed.
     */
    public boolean userUnset(String dtId, String acId, Map<String, Object> properties) {
        Map<String, Object> props = new HashMap<>();
        for (Map.Entry<String, Object> entry : properties.entrySet()) {
            String key = entry.getKey();
            if (key.startsWith("#")) {
                props.put(key, entry.getValue());
            } else {
                props.put(key, 0);
            }
        }
        return add(dtId, acId, "#user_unset", "user", props);
    }

    /**
     * Delete the user with given dtId and acId.
     *
     * @param dtId The device-scoped id.
     * @param acId The account-scoped id.
     * @param properties preset properties of this event.
     * @return True if given event and properties is valid or False if there are invalid and will not be processed.
     */
    public boolean userDelete(String dtId, String acId, Map<String, Object> properties) {
        return add(dtId, acId, "#user_delete", "user", properties);
    }

    /**
     * Append values to property for the user with given dtId and acId.
     * Hence, the type of value for 'custom properties' should be an array.
     *
     * @param dtId The device-scoped id.
     * @param acId The account-scoped id.
     * @param properties properties of this event. (preset properties are scoped by event name, and has type constraints)
     * @return True if given event and properties is valid or False if there are invalid and will not be processed.
     */
    public boolean userAppend(String dtId, String acId, Map<String, Object> properties) {
        return add(dtId, acId, "#user_append", "user", properties);
    }

    /**
     * Append values to property without duplications for the user with given dtId and acId.
     * Hence, the type of value for 'custom properties' should be an array.
     *
     * @param dtId The device-scoped id.
     * @param acId The account-scoped id.
     * @param properties properties of this event. (preset properties are scoped by event name, and has type constraints)
     * @return True if given event and properties is valid or False if there are invalid and will not be processed.
     */
    public boolean userUniqAppend(String dtId, String acId, Map<String, Object> properties) {
        return add(dtId, acId, "#user_uniq_append", "user", properties);
    }

    /**
     * Flush the data buffer manually.
     */
    public void flush() {
        DTBase.flush();
    }

    /**
     * Close the DTAnalytics, remember to call this before the program finishes to preventing data loss!
     */
    public void close() {
        DTBase.close();
    }

    /**
     * To enable and disable the logging.
     */
    public static void toggleLogger(boolean enable) {
        DTBase.toggleLogger(enable);
    }
    
    private boolean add(String dtId, String acId, String eventName, String eventType, Map<String, Object> properties) {
        Map<String, Object> event = new HashMap<>(properties);
        event.put("#dt_id", dtId);
        event.put("#acid", acId);
        event.put("#event_name", eventName);
        event.put("#event_type", eventType);
        event.put("#sdk_type", DTAnalytics.SDK_TYPE);
        
        return DTBase.addEvent(event);
    }
}