package main

import (
	"fmt"
	dtanalytics "github.com/datatower-ai/sdk-core-golang/src/dt_analytics"
	"log"
	"strings"
	"time"
)

func main() {
	//dtanalytics.ToggleLogger(true)
	consumer := dtanalytics.NewDTLogConsumer("log", 200, "dt_go_demo", 10*1024*1024)
	dt, _ := dtanalytics.New(consumer, true)

	properties := map[string]interface{}{
		"productNames": []string{"Lua", "hello"},
		"productType":  "Lua book",
		"producePrice": 80,
		"shop":         "xx-shop",
		"#os":          "1.1.1.1",
		"sex":          "female",
		"#app_id":      "appid_1234567890",
		"#bundle_id":   "com.example",
	}

	for i := 0; i < 5; i++ {
		properties[fmt.Sprintf("a%d", i)] = strings.Repeat("asd", i)
	}

	n := 10000
	start := time.Now()
	tm := int64(0)
	for i := 0; i < n; i++ {
		st := time.Now()
		err := dt.Track("dtiddd", "", "simple_event", properties)
		tm = tm + time.Since(st).Microseconds()
		if err != nil {
			println(err)
		}
	}
	log.Printf("Time elapsed: %d", time.Since(start).Microseconds())
	log.Printf("Time elapsed avg: %d", tm/int64(n))

	dt.Flush()
	dt.Close()
}
