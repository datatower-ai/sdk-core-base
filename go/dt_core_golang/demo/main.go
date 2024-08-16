package main

import (
	"fmt"
	dtAnalytics "github.com/datatower-ai/dt-golang-sdk"
	"log"
	"sort"
	"strconv"
	"strings"
	"time"
)

func main() {
	dtAnalytics.ToggleLogger(true)
// 	consumer := dtAnalytics.NewDTLogConsumer("log", 1000, "dt_go_demo", 0)
	consumer := dtAnalytics.NewDTMmapLogConsumer("log", "dt_go_test", 10 * 1024 * 1024, 1 * 1024 * 1024)
	dt, _ := dtAnalytics.New(consumer, true)
	dtAnalytics.ToggleLogger(false)

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

	for i := 0; i < 20; i++ {
		properties[fmt.Sprintf("a%d", i)] = strings.Repeat("asd", i)
	}

	n := 100000
	start := time.Now()
	tm := int64(0)
	lst := []int64{}
	for i := 0; i < n; i++ {
		properties["$_event_call_time"] = strconv.FormatInt(time.Now().UnixMicro(), 10)
		st := time.Now()
		err := dt.Track("dtiddd", "", "simple_event", properties)
		tmp := time.Since(st).Microseconds()
		tm = tm + tmp
		lst = append(lst, tmp)
		if err != nil {
			println(err)
		}
	}
	log.Printf("Time elapsed: %fms", float64(time.Since(start).Microseconds())/1000)
	log.Printf("Time elapsed avg: %fms", float64(tm)/float64(n)/1000)
	sort.Slice(lst, func(i, j int) bool { return lst[i] < lst[j] })
	println(fmt.Sprintf("min: %fms", float64(lst[0])/1000.0))
	println(fmt.Sprintf("max: %fms", float64(lst[len(lst)-1])/1000.0))
	println(fmt.Sprintf("50': %fms", float64(lst[(len(lst)-1)/2])/1000.0))
	println(fmt.Sprintf("80': %fms", float64(lst[(len(lst)-1)*8/10])/1000.0))
	println(fmt.Sprintf("90': %fms", float64(lst[(len(lst)-1)*9/10])/1000.0))
	println(fmt.Sprintf("95': %fms", float64(lst[(len(lst)-1)*95/100])/1000.0))
	println(fmt.Sprintf("99': %fms", float64(lst[(len(lst)-1)*99/100])/1000.0))
	numWrite := n / 200
	println(fmt.Sprintf("%f': %fms", float64((n-numWrite))/float64(n), float64(lst[len(lst)-numWrite-1])/1000.0))
	allExceptWrite := lst[:len(lst)-numWrite]
	println(fmt.Sprintf("avg (except write): %fms", float64(sumArr(allExceptWrite))/float64(len(allExceptWrite))/1000.0))
	allOnlyWrite := lst[len(lst)-numWrite:]
	println(fmt.Sprintf("avg (write only): %fms", float64(sumArr(allOnlyWrite))/float64(len(allOnlyWrite))/1000.0))

	dt.Flush()
	dt.Close()
}

func sumArr(numbers []int64) int64 {
	sum := int64(0)
	for _, num := range numbers {
		sum += num
	}
	return sum
}
