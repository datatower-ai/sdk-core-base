using DTCore.Base;

namespace DTCore;

public class LogConsumer(string path, string namePrefix, ulong maxBatchLen, ulong maxFileSizeBytes)
    : IConsumer
{
    private const string Name = "log";


    public Dictionary<string, object?> Config()
    {
        return new Dictionary<string, object?>
        {
            {"consumer", Name},
            {"path", path},
            {"name_prefix", namePrefix},
            {"max_batch_len", maxBatchLen},
            {"max_file_size_bytes", maxFileSizeBytes},
        };
    }
}