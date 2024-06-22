package sensors

type HCSR04Core struct {
	PinNumber uint
	PresenceSensorSettings
}

type PolledHCSR04Sensor struct {
	HCSR04Core
}

func (s *PolledHCSR04Sensor) init() error {
	return nil
}

func (s *PolledHCSR04Sensor) Detected() bool {
	return false
}

type ThreadedHCSR04Sensor struct {
	UpdatePeriod uint64
	HCSR04Core
}

func (s *ThreadedHCSR04Sensor) init() error {

	go s.loop()
	return nil
}

func (s *ThreadedHCSR04Sensor) Detected() bool {
	return false
}

func (s *ThreadedHCSR04Sensor) loop() {

}

type CallBackHCSR04Sensor struct {
	callbackFunction func()
	HCSR04Core
}

func (s *CallBackHCSR04Sensor) init() {

}
