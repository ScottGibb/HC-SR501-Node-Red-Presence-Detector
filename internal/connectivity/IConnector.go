package connectivity

type IpConnector struct {
	PortNumber int
	IpAddress  string
	IConnector
}

type IConnector interface {
	Send()
	Receive()
}
