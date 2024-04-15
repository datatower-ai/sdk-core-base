package dt_analytics

type DTLogConsumer struct {
	path             string
	maxBatchLen      uint32
	namePrefix       string
	maxFileSizeTypes uint64
}

// NewDTLogConsumer creates an DTConsumer to log the events.
// Event logs will be stored in the path with name_prefix.
// The inputted event will be buffered and be written to log per every max_batch_len length.
// Also, the event log is sharding by either of these circumstances:
//   - Every hour,
//   - File size is over approximated maximum file size max_file_size_bytes in bytes (0 for unlimited).
func NewDTLogConsumer(path string, maxBatchLen uint32, namePrefix string, maxFileSizeBytes uint64) DTConsumer {
	return DTLogConsumer{
		path,
		maxBatchLen,
		namePrefix,
		maxFileSizeBytes,
	}
}

func (c DTLogConsumer) getConfig() map[string]interface{} {
	return map[string]interface{}{
		"consumer":            "log",
		"path":                c.path,
		"max_batch_len":       c.maxBatchLen,
		"name_prefix":         c.namePrefix,
		"max_file_size_bytes": c.maxFileSizeTypes,
	}
}
