package main

import dtanalytics "github.com/datatower-ai/sdk-core-golang/src/dt_analytics"

func main() {
	consumer := dtanalytics.NewDTLogConsumer("log", 200, "dt_go_demo", 10*1024*1024)
	dtanalytics.ToggleLogger(true)
	dt, _ := dtanalytics.New(consumer, true)
	err := dt.Track("dtiddd", "", "simple_event", map[string]interface{}{
		"#app_id":    "appid123",
		"#bundle_id": "com.example",
	})
	if err != nil {
		println(err)
	}
	dt.Flush()
	dt.Close()
}
