package dtanalytics

type DTLogConsumer struct {
	path                string
	max_batch_len       uint32
	name_prefix         string
	max_file_size_types uint64
}

func NewDTLogConsumer(path string, max_batch_len uint32, name_prefix string, max_file_size_bytes uint64) DTConsumer {
	return DTLogConsumer{
		path,
		max_batch_len,
		name_prefix,
		max_file_size_bytes,
	}
}

func (c DTLogConsumer) getConfig() map[string]interface{} {
	return map[string]interface{}{
		"consumer":            "log",
		"path":                c.path,
		"max_batch_len":       c.max_batch_len,
		"name_prefix":         c.name_prefix,
		"max_file_size_bytes": c.max_file_size_types,
	}
}
