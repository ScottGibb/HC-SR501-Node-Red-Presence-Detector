package sensors

import "fmt"

type PresenceSensorType uint8

const (
	HC_SR04 PresenceSensorType = iota
	UNKNOWN
)

func GetType(str string) (PresenceSensorType, error) {
	var initError error
	switch str {
	case "HC_SR04":
		return nil, initError
	default:
		initError = fmt.Errorf("Presence Senor not supported (%v)", str)
		return UNKNOWN, initError
	}
}
