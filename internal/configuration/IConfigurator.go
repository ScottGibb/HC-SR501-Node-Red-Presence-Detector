package configuration

type SystemSettings struct {
}
type IConfigurator interface {
	GetSettings() error
}
