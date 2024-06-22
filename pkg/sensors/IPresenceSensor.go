package sensors

type IPresenceSensor interface {
	init()
	Detected() bool
}
