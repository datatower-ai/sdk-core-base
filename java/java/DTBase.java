package ai.datatower.sdk;

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
