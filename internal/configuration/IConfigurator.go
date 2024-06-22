package configuration

type SystemSettings struct {
	sensor    IPresenceSensor
	sensing   ISensing
	connector IConnector
}

type IConfigurator interface {
	GetSettings() error
}
