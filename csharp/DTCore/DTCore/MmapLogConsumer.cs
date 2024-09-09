using DTCore.Base;

namespace DTCore;

public class MmapLogConsumer(string path, string namePrefix, ulong? fileSize = null, ulong? flushSize = null)
    : IConsumer
{
    private const string Name = "mlog";


    public Dictionary<string, object?> Config()
    {
        return new Dictionary<string, object?>
        {
            {"consumer", Name},
            {"path", path},
            {"name_prefix", namePrefix},
            {"file_size", fileSize},
            {"flush_size", flushSize},
        };
    }
}