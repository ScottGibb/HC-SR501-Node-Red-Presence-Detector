package sensors

type PresenceSensorMode uint8

const (
	POLLED PresenceSensorMode = iota
	THREADED
	CALLBACK
)

type PresenceSensorSettings struct {
	name string
}
type IPresenceSensor interface {
	init() error
	Detected() bool
}

type IPresenceCallBack interface {
	init() error
}
