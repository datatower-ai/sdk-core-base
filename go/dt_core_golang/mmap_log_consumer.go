package dt_analytics

type DTMmapLogConsumer struct {
	path                string
	namePrefix          string
	maxFileSizeBytes    uint64
	flushSizeBytes      uint64
}

// NewDTLogConsumer creates an DTConsumer to log the events.
// Event logs will be stored in the path with name_prefix.
// The inputted event will be buffered and be written to log per every max_batch_len length.
// Also, the event log is sharding by either of these circumstances:
//   - Every hour,
//   - File size is over approximated maximum file size max_file_size_bytes in bytes (0 for unlimited).
func NewDTMmapLogConsumer(path string, namePrefix string, maxFileSizeBytes uint64, flushSizeBytes uint64) DTConsumer {
	return DTMmapLogConsumer{
		path,
		namePrefix,
		maxFileSizeBytes,
		flushSizeBytes,
	}
}

func (c DTMmapLogConsumer) getConfig() map[string]interface{} {
	return map[string]interface{}{
		"consumer":             "mlog",
		"path":                 c.path,
		"name_prefix":          c.namePrefix,
		"file_size":            c.maxFileSizeBytes
		"flush_size":           c.flushSizeBytes,
	}
}
