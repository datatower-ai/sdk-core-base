package dt_analytics

/*
#cgo CFLAGS: -I${SRCDIR}/include
#cgo darwin,amd64 LDFLAGS: -L${SRCDIR}/lib -Wl,-rpath,${SRCDIR}/lib -ldt_core_clib-macos-amd64
#cgo darwin,arm64 LDFLAGS: -L${SRCDIR}/lib -Wl,-rpath,${SRCDIR}/lib -ldt_core_clib-macos-arm64
#cgo linux,amd64 LDFLAGS: -L${SRCDIR}/lib -Wl,-rpath,${SRCDIR}/lib -ldt_core_clib-linux-amd64
#cgo linux,arm64 LDFLAGS: -L${SRCDIR}/lib -Wl,-rpath,${SRCDIR}/lib -ldt_core_clib-linux-arm64
#cgo windows,amd64 LDFLAGS: -L${SRCDIR}/lib -Wl,-rpath,${SRCDIR}/lib -ldt_core_clib-windows-amd64
#cgo windows,arm64 LDFLAGS: -L${SRCDIR}/lib -Wl,-rpath,${SRCDIR}/lib -ldt_core_clib-windows-arm64

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
	_sdkType = "dt_server_sdk_go"
)

type DTConsumer interface {
	getConfig() map[string]interface{}
}

type DTAnalytics struct{}

// New for initialization of the DTAnalytics with given consumer.
// If isDebug is set to true, the data will not be inserted to production environment.
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

// Track an event (custom or preset).
// returns
//   - nil if given event and properties is valid, or
//   - error if there are invalid and will not be processed.
func (dta DTAnalytics) Track(dtId string, acId string, eventName string, properties map[string]interface{}) error {
	return dta.add(dtId, acId, eventName, "track", properties)
}

// UserSet Set user properties for the user with given dtId and acId.
// returns
//   - nil if given event and properties is valid, or
//   - error if there are invalid and will not be processed.
func (dta DTAnalytics) UserSet(dtId string, acId string, properties map[string]interface{}) error {
	return dta.add(dtId, acId, "#user_set", "user", properties)
}

// UserSetOnce Set user properties only once for user with given dtId and acId.
// The value will not override existed property.
// returns
//   - nil if given event and properties is valid, or
//   - error if there are invalid and will not be processed.
func (dta DTAnalytics) UserSetOnce(dtId string, acId string, properties map[string]interface{}) error {
	return dta.add(dtId, acId, "#user_set_once", "user", properties)
}

// UserAdd Arithmetic add the value of property by given number for user with given dtId and acId.
// Hence, the type of value for 'custom properties' should be a number.
// returns
//   - nil if given event and properties is valid, or
//   - error if there are invalid and will not be processed.
func (dta DTAnalytics) UserAdd(dtId string, acId string, properties map[string]interface{}) error {
	return dta.add(dtId, acId, "#user_add", "user", properties)
}

// UserUnset Unset properties for user with given dtId and acId.
// Only the key of 'custom properties' will be used and its value is meaningless here.
// returns
//   - nil if given event and properties is valid, or
//   - error if there are invalid and will not be processed.
func (dta DTAnalytics) UserUnset(dtId string, acId string, properties map[string]interface{}) error {
	return dta.add(dtId, acId, "#user_unset", "user", properties)
}

// UserDelete Delete the user with given dtId and acId.
// returns
//   - nil if given event and properties is valid, or
//   - error if there are invalid and will not be processed.
func (dta DTAnalytics) UserDelete(dtId string, acId string, properties map[string]interface{}) error {
	return dta.add(dtId, acId, "#user_delete", "user", properties)
}

// UserAppend Append values to property for the user with given dtId and acId.
// Hence, the type of value for 'custom properties' should be an array.
// returns
//   - nil if given event and properties is valid, or
//   - error if there are invalid and will not be processed.
func (dta DTAnalytics) UserAppend(dtId string, acId string, properties map[string]interface{}) error {
	return dta.add(dtId, acId, "#user_append", "user", properties)
}

// UserUniqAppend Append values to property without duplications for the user with given dtId and acId.
// Hence, the type of value for 'custom properties' should be an array.
// returns
//   - nil if given event and properties is valid, or
//   - error if there are invalid and will not be processed.
func (dta DTAnalytics) UserUniqAppend(dtId string, acId string, properties map[string]interface{}) error {
	return dta.add(dtId, acId, "#user_uniq_append", "user", properties)
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

// Flush the data buffer manually.
func (_ DTAnalytics) Flush() {
	C.dt_flush()
}

// Close the DTAnalytics, remember to call this before the program finishes to preventing data loss!
func (_ DTAnalytics) Close() {
	C.dt_close()
}

// ToggleLogger to enable and disable the logging.
func ToggleLogger(enable bool) {
	enabled := 0
	if enable {
		enabled = 1
	}
	cEnabled := C.uint8_t(enabled)
	C.dt_toggle_logger(cEnabled)
}
