package main

import (
	"encoding/json"
	"fmt"
	dtAnalytics "github.com/datatower-ai/dt-golang-sdk"
	"net/http"
	"os"
)

var dt dtAnalytics.DTAnalytics
var inited bool = false

func main() {
	http.HandleFunc("/init", thInit)
	http.HandleFunc("/track", thTrack)
	http.HandleFunc("/user_set", thUserSet)
	http.HandleFunc("/user_set_once", thUserSetOnce)
	http.HandleFunc("/user_add", thUserAdd)
	http.HandleFunc("/user_unset", thUserUnset)
	http.HandleFunc("/user_delete", thUserDelete)
	http.HandleFunc("/user_append", thUserAppend)
	http.HandleFunc("/user_uniq_append", thUserUniqAppend)
	http.HandleFunc("/flush", thFlush)
	http.HandleFunc("/close", thClose)
	http.HandleFunc("/log/enable", thEnableLog)
	http.HandleFunc("/log/disable", thDisableLog)

	port := os.Args[1]
	if port == "" {
		port = "10085"
	}
	println("Running on port " + port)
	err := http.ListenAndServe(fmt.Sprintf(":%s", port), nil)
	if err != nil {
		panic(err)
	}
}

type InitRequest struct {
	Path        string `json:"path"`
	MaxBatchLen uint32 `json:"max_batch_length"`
	LogPrefix   string `json:"log_prefix"`
	MaxLogSize  uint64 `json:"max_log_size"`
	Debug       bool   `json:"debug"`
}

type EventRequest struct {
	DtId      string                 `json:"dt_id"`
	AcId      string                 `json:"acid"`
	EventName string                 `json:"event_name"`
	Props     map[string]interface{} `json:"props"`
}

type UserRequest struct {
	DtId  string                 `json:"dt_id"`
	AcId  string                 `json:"acid"`
	Props map[string]interface{} `json:"props"`
}

func thInit(w http.ResponseWriter, r *http.Request) {
	var p InitRequest

	err := json.NewDecoder(r.Body).Decode(&p)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	path := p.Path
	if path == "" {
		path = "log_test"
	}
	maxBatchLen := p.MaxBatchLen
	if maxBatchLen == 0 {
		maxBatchLen = 200
	}
	consumer := dtAnalytics.NewDTLogConsumer(path, maxBatchLen, p.LogPrefix, p.MaxLogSize)
	dt, _ = dtAnalytics.New(consumer, p.Debug)
	inited = true

	fmt.Fprintf(w, "Received Init\n")
}

func thTrack(w http.ResponseWriter, r *http.Request) {
	var p EventRequest

	err := json.NewDecoder(r.Body).Decode(&p)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if !inited {
		fmt.Fprintf(w, "Received Track, but sdk is not initialized\n")
	} else {
		err2 := dt.Track(p.DtId, p.AcId, p.EventName, p.Props)
		if err2 != nil {
			fmt.Fprintf(w, "Received Track, result: fail\n")
		} else {
			fmt.Fprintf(w, "Received Track, result: success\n")
		}
	}
}

func thUserSet(w http.ResponseWriter, r *http.Request) {
	var p UserRequest

	err := json.NewDecoder(r.Body).Decode(&p)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if !inited {
		fmt.Fprintf(w, "Received user_set, but sdk is not initialized\n")
	} else {
		err2 := dt.UserSet(p.DtId, p.AcId, p.Props)
		if err2 != nil {
			fmt.Fprintf(w, "Received user_set, result: fail\n")
		} else {
			fmt.Fprintf(w, "Received user_set, result: success\n")
		}
	}
}

func thUserSetOnce(w http.ResponseWriter, r *http.Request) {
	var p UserRequest

	err := json.NewDecoder(r.Body).Decode(&p)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if !inited {
		fmt.Fprintf(w, "Received user_set_once, but sdk is not initialized\n")
	} else {
		err2 := dt.UserSetOnce(p.DtId, p.AcId, p.Props)
		if err2 != nil {
			fmt.Fprintf(w, "Received user_set_once, result: fail\n")
		} else {
			fmt.Fprintf(w, "Received user_set_once, result: success\n")
		}
	}
}

func thUserAdd(w http.ResponseWriter, r *http.Request) {
	var p UserRequest

	err := json.NewDecoder(r.Body).Decode(&p)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if !inited {
		fmt.Fprintf(w, "Received user_add, but sdk is not initialized\n")
	} else {
		err2 := dt.UserAdd(p.DtId, p.AcId, p.Props)
		if err2 != nil {
			fmt.Fprintf(w, "Received user_add, result: fail\n")
		} else {
			fmt.Fprintf(w, "Received user_add, result: success\n")
		}
	}
}

func thUserUnset(w http.ResponseWriter, r *http.Request) {
	var p UserRequest

	err := json.NewDecoder(r.Body).Decode(&p)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if !inited {
		fmt.Fprintf(w, "Received user_unset, but sdk is not initialized\n")
	} else {
		err2 := dt.UserUnset(p.DtId, p.AcId, p.Props)
		if err2 != nil {
			fmt.Fprintf(w, "Received user_unset, result: fail\n")
		} else {
			fmt.Fprintf(w, "Received user_unset, result: success\n")
		}
	}
}

func thUserDelete(w http.ResponseWriter, r *http.Request) {
	var p UserRequest

	err := json.NewDecoder(r.Body).Decode(&p)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if !inited {
		fmt.Fprintf(w, "Received user_delete, but sdk is not initialized\n")
	} else {
		err2 := dt.UserDelete(p.DtId, p.AcId, p.Props)
		if err2 != nil {
			fmt.Fprintf(w, "Received user_delete, result: fail\n")
		} else {
			fmt.Fprintf(w, "Received user_delete, result: success\n")
		}
	}
}

func thUserAppend(w http.ResponseWriter, r *http.Request) {
	var p UserRequest

	err := json.NewDecoder(r.Body).Decode(&p)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if !inited {
		fmt.Fprintf(w, "Received user_append, but sdk is not initialized\n")
	} else {
		err2 := dt.UserAppend(p.DtId, p.AcId, p.Props)
		if err2 != nil {
			fmt.Fprintf(w, "Received user_append, result: fail\n")
		} else {
			fmt.Fprintf(w, "Received user_append, result: success\n")
		}
	}
}

func thUserUniqAppend(w http.ResponseWriter, r *http.Request) {
	var p UserRequest

	err := json.NewDecoder(r.Body).Decode(&p)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if !inited {
		fmt.Fprintf(w, "Received user_uniq_append, but sdk is not initialized\n")
	} else {
		err2 := dt.UserUniqAppend(p.DtId, p.AcId, p.Props)
		if err2 != nil {
			fmt.Fprintf(w, "Received user_uniq_append, result: fail\n")
		} else {
			fmt.Fprintf(w, "Received user_uniq_append, result: success\n")
		}
	}
}

func thFlush(w http.ResponseWriter, r *http.Request) {
	if !inited {
		fmt.Fprintf(w, "Received Flush, but sdk is not initialized\n")
	} else {
		dt.Flush()
		fmt.Fprintf(w, "Received Flush\n")
	}
}

func thClose(w http.ResponseWriter, r *http.Request) {
	if !inited {
		fmt.Fprintf(w, "Received Close, but sdk is not initialized\n")
	} else {
		dt.Close()
		fmt.Fprintf(w, "Received Close\n")
	}
}

func thEnableLog(w http.ResponseWriter, r *http.Request) {
	dtAnalytics.ToggleLogger(true)
	fmt.Fprintf(w, "Received /log/enable\n")
}

func thDisableLog(w http.ResponseWriter, r *http.Request) {
	dtAnalytics.ToggleLogger(false)
	fmt.Fprintf(w, "Received /log/disable\n")
}
