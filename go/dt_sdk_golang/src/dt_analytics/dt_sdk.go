package dt_analytics

/*
#cgo CFLAGS: -I.
#cgo darwin,arm64 LDFLAGS: -L. -ldt_core_clib-macos-arm64
#cgo darwin,amd64 LDFLAGS: -L. -ldt_core_clib-macos-amd64
#cgo linux,arm64 LDFLAGS: -L. -ldt_core_clib-linux-arm64
#cgo linux,amd64 LDFLAGS: -L. -ldt_core_clib-linux-amd64
#cgo windows,arm64 LDFLAGS: -L. -ldt_core_clib-windows-arm64
#cgo windows,amd64 LDFLAGS: -L. -ldt_core_clib-windows-amd64

#include "stdlib.h"
#include "dt_core_clib.h"
*/
import "C"

import (
	"errors"
	jsoniter "github.com/json-iterator/go"
	"unsafe"
)

const (
	SDK_TYPE    = "dt_server_sdk_go"
	SDK_VERSION = "1.0.0"
)

type DTConsumer interface {
	getConfig() map[string]interface{}
}

type DTAnalytics struct{}

func New(consumer DTConsumer, isDebug bool) (DTAnalytics, error) {
	configMap := consumer.getConfig()

	if isDebug {
		configMap["_debug"] = 1
	}

	b, _ := jsoniter.Marshal(configMap)
	configStr := string(b)
	configCStr := C.CString(configStr)
	defer C.free(unsafe.Pointer(configCStr))

	res := C.dt_init(configCStr)
	if res != 0 {
		return DTAnalytics{}, nil
	} else {
		return DTAnalytics{}, errors.New("failed to init DTAnalytics")
	}
}

func (dta DTAnalytics) Track(dtId string, acId string, eventName string, properties map[string]interface{}) error {
	return dta.add(dtId, acId, eventName, "track", properties)
}

func (_ DTAnalytics) add(dtId string, acId string, eventName string, eventType string, properties map[string]interface{}) error {
	event := make(map[string]interface{}, len(properties)+6)

	for k, v := range properties {
		event[k] = v
	}
	event["#dt_id"] = dtId
	event["#acid"] = acId
	event["#event_name"] = eventName
	event["#event_type"] = eventType
	event["#sdk_type"] = SDK_TYPE
	event["#sdk_version_name"] = SDK_VERSION

	b, err := jsoniter.Marshal(event)
	if err != nil {
		return err
	}
	eventJson := string(b)
	cEventJson := C.CString(eventJson)

	if C.dt_add_event(cEventJson) != 0 {
		return nil
	} else {
		return errors.New("given event is not valid")
	}
}

func (_ DTAnalytics) Flush() {
	C.dt_flush()
}

func (_ DTAnalytics) Close() {
	C.dt_close()
}

func ToggleLogger(enable bool) {
	enabled := 0
	if enable {
		enabled = 1
	}
	cEnabled := C.uint8_t(enabled)
	C.dt_toggle_logger(cEnabled)
}
