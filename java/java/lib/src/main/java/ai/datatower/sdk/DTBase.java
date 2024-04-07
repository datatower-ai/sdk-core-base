package ai.datatower.sdk;

import java.io.*;
import java.nio.file.Files;
import java.util.Map;
import java.util.concurrent.atomic.AtomicBoolean;

class DTBase {
    static native boolean init(Map<String, Object> config);
    static native boolean addEvent(Map<String, Object> event);
    static native boolean addEventStr(String event);
    static native void flush();
    static native void close();
    static native void toggleLogger(boolean enable);
    static native void setStaticCommonProperties(Map<String, Object> properties);
    static native void clearStaticCommonProperties();

    static AtomicBoolean loaded = new AtomicBoolean(false);

    static synchronized void load() throws IOException {
        if (loaded.get()) return;
        loadDynamicLib();
        loaded.set(true);
    }

    static {
        try {
            DTBase.load();
        } catch (Throwable t) {
            t.printStackTrace();
        }
    }

    private static void loadDynamicLib() throws IOException {
        String os = getOs();
        String arch = getArch();
        String soName = System.mapLibraryName(String.format("dt_core_java-%s-%s", os, arch));
        try (InputStream inputStream = DTBase.class.getResourceAsStream("/ai/datatower/sdk/" + soName)) {
            int i = soName.lastIndexOf(".");
            String ext = soName.substring(i + 1);
            File file = File.createTempFile("lib", ext);
            file.deleteOnExit();        // register for auto deletion.
            OutputStream outputStream = Files.newOutputStream(file.toPath());
            byte[] buffer = new byte[1024];
            int length;
            while((length = inputStream.read(buffer)) != -1) {
                outputStream.write(buffer, 0, length);
            }
            outputStream.close();
            System.load(file.getAbsolutePath());
        } catch (Throwable t) {
            System.out.printf("[DT Java] Failed to load dynamic library \"%s\".%n", soName);
            throw t;
        }
    }

    private static String getOs() {
        // linux, windows, macos
        String os = System.getProperty("os.name").toLowerCase();
        if (os.contains("win")) {
            return "windows";
        } else if (os.contains("mac")) {
            return "macos";
        } else if (os.contains("linux")) {
            return "linux";
        }
        return os;
    }

    private static String getArch() {
        // arm, amd
        String arch = System.getProperty("os.arch").toLowerCase();
        if (arch.contains("amd64") || arch.contains("x86_64")) {
            return "amd64";
        } else if (arch.contains("arm64") || arch.contains("aarch")) {
            return "arm64";
        }
        return arch;
    }
}
