package sensors

import "fmt"

type PresenceDetectorType uint8

const (
	HC_SR04 PresenceDetectorType = iota
	UNKNOWN
)

func GetType(str string) (PresenceDetectorType, error) {
	var initError error
	switch str {
	case "HC_SR04":
		return HC_SR04, initError
	default:
		initError = fmt.Errorf("Presence Senor not supported (%v)", str)
		return UNKNOWN, initError
	}
}
