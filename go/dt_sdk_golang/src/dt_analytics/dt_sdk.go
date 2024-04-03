package dt_analytics

/*
#cgo CFLAGS: -I.
#cgo darwin,arm64 LDFLAGS: -L. -ldt_core_clib-macos-arm64
#cgo darwin,amd64 LDFLAGS: -L. -ldt_core_clib
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
	_sdkType    = "dt_server_sdk_go"
	_sdkVersion = "1.0.0"
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

	ret := C.dt_init(configCStr)
	clear(configMap)
	if ret != 0 {
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
	event["#sdk_type"] = _sdkType
	event["#sdk_version_name"] = _sdkVersion

	b, err := jsoniter.Marshal(event)
	if err != nil {
		return err
	}
	eventJson := string(b)
	cEventJson := C.CString(eventJson)
	ret := C.dt_add_event(cEventJson)
	clear(event)

	if ret != 0 {
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
