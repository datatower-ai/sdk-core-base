using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;
using DTCore.Base;
using DTCore.Native;

namespace DTCore;

using Properties = Dictionary<string, object>;

public static class DtAnalytics
{
    private const string SdkName = "dt_server_sdk_csharp";
    
    public static void Init(IConsumer consumer, bool debug = false)
    {
        try
        {
            var configMap = consumer.Config();
            configMap["_debug"] = debug;
            unsafe
            {
                var ba = AnyDict2ByteArray(configMap);
                fixed (byte* bp = ba)
                {
                    NativeMethods.dt_init(bp); 
                }
            }
        }
        catch (Exception e)
        {
            Console.Error.WriteLine($"[DT C#] Failed to init DT: {e}");
        }
    }

    public static void ToggleLogger(bool enable)
    {
        NativeMethods.dt_toggle_logger(enable? (byte) 1 : (byte) 0);
    }

    public static void Track(string dtId, string? acId, string eventName, Properties properties)
    {
        Add(dtId, acId, eventName, "track", properties);
    }

    public static void UserSet(string dtId, string? acId, Properties properties)
    {
        Add(dtId, acId, "#user_set", "user", properties);
    }

    public static void UserSetOnce(string dtId, string? acId, Properties properties)
    {
        Add(dtId, acId, "#user_set_once", "user", properties);
    }

    public static void UserAdd(string dtId, string? acId, Properties properties)
    {
        Add(dtId, acId, "#user_add", "user", properties);
    }

    public static void UserUnset(string dtId, string? acId, Properties properties)
    {
        Add(dtId, acId, "#user_unset", "user", properties);
    }

    public static void UserDelete(string dtId, string? acId, Properties properties)
    {
        Add(dtId, acId, "#user_delete", "user", properties);
    }

    public static void UserAppend(string dtId, string? acId, Properties properties)
    {
        Add(dtId, acId, "#user_append", "user", properties);
    }

    public static void UserUniqAppend(string dtId, string? acId, Properties properties)
    {
        Add(dtId, acId, "#user_uniq_append", "user", properties);
    }

    public static void Flush()
    {
        NativeMethods.dt_flush();
    }

    public static void Close()
    {
        NativeMethods.dt_close();
    }
    
    private static byte[] AnyDict2ByteArray(Dictionary<string, object?> dict)
    {
        var config = JsonSerializer.Serialize(dict);
        var bytes = Encoding.UTF8.GetBytes(config);
        return bytes;
    }

    private static byte[] Dict2ByteArray(Properties dict)
    {
        var config = JsonSerializer.Serialize(dict);
        var bytes = Encoding.UTF8.GetBytes(config);
        return bytes;
    }
    
    private static void Add(string dtId, string? acId, 
        string eventName, string eventType, Properties properties)
    {
        try
        {
            properties["#dt_id"] = dtId;
            if (acId != null)
            {
                properties["#acid"] = acId;
            }
            properties["#event_name"] = eventName;
            properties["#event_type"] = eventType;
            properties["#sdk_type"] = SdkName;
            
            unsafe
            {
                var ba = Dict2ByteArray(properties);
                fixed (byte* bp = ba)
                {
                    NativeMethods.dt_add_event_bytes(bp, ba.Length);
                }
            }
        }
        catch (Exception e)
        {
            Console.Error.WriteLine($"[DT C#] Failed to Track({eventName}): {e}");
        }
    }
}
