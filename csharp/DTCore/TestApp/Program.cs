// See https://aka.ms/new-console-template for more information

using System.Collections;
using System.Diagnostics;
using DTCore;

const string dir = "test_log";
var directoryInfo = new DirectoryInfo(dir);
if (directoryInfo.Exists)
{
    directoryInfo.Delete(true);
}

DtAnalytics.ToggleLogger(true);
// var consumer = new LogConsumer(dir, "test", 2000, 10 * 1024 * 1024);
var consumer = new MmapLogConsumer(dir, "test", 2 * 1024 * 1024, flushSize: 2 * 1024 * 1024);
DtAnalytics.Init(consumer);
DtAnalytics.ToggleLogger(false);

const string dtId = "1234567890987654321";
string? acId = null;
    
var properties = new Dictionary<string, object>
{
    { "productNames", new List<string> {"Lua", "hello"} },
    { "productType", "Lua book" },
    { "producePrice", 80 },
    { "shop", "xx-shop" },
    { "#os", "1.1.1.1" },
    { "sex", "female" },
    { "#app_id", "appid_1234567890" },
    { "#bundle_id", "com.example" },
};

for (var i = 0; i < 50; i++)
{
    properties["a" + i] = string.Concat(Enumerable.Repeat("asd", i));
}

const int n = 1000000 * 3;

var sw = new Stopwatch();
var total = 0L;
// var lst = new ArrayList();

for (var i = 0; i < n; i++)
{
    // var now = DateTimeOffset.Now;
    // properties["$_event_call_time"] = now.ToUnixTimeMilliseconds() * 1000 + now.Microsecond;
    sw.Start();
    DtAnalytics.Track(dtId, acId, "" +
                                  "eventName" +
                                  "", properties);
    sw.Stop();
    var elapsed = sw.ElapsedTicks / (Stopwatch.Frequency / (1000L * 1000L));
    total += elapsed;
    // lst.Add(elapsed);
    sw.Reset();
}

DtAnalytics.Flush();

Console.WriteLine("time elapsed: " + total + "μs");
Console.WriteLine("time elapsed avg: " + total/n + "μs");
Console.WriteLine("Approximate QPS: " + (n/((double)total/1_000_000)).ToString("#.##"));
// lst.Sort();
// Console.WriteLine("min: " + lst[0] + "μs");
// Console.WriteLine("max: " + lst[^1] + "μs");
// Console.WriteLine("50': " + lst[(lst.Count-1)/2] + "μs");
// Console.WriteLine("80': " + lst[(lst.Count-1)*4/5] + "μs");
// Console.WriteLine("90': " + lst[(lst.Count-1)*9/10] + "μs");
// Console.WriteLine("95': " + lst[(lst.Count-1)*95/100] + "μs");
// Console.WriteLine("99': " + lst[(lst.Count-1)*99/100] + "μs");

DtAnalytics.Close();