package main

func main() {
	println("Hello World")
}

func setup() {

	var settings SystemSettings
	settings.GetSettings()

	var sensor IPresenceSensor
	var connector IConnector

	var sensing ISensing
}
