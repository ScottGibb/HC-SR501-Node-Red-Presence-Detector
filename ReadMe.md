# HC-SR501 Node Red Presence Detector

[![MegaLinter](https://github.com/ScottGibb/HC-SR501-Node-Red-Presence-Detector/actions/workflows/megalinter.yaml/badge.svg)](https://github.com/ScottGibb/HC-SR501-Node-Red-Presence-Detector/actions/workflows/megalinter.yaml)
[![Continuous Build](https://github.com/ScottGibb/HC-SR501-Node-Red-Presence-Detector/actions/workflows/continuous-build.yaml/badge.svg)](https://github.com/ScottGibb/HC-SR501-Node-Red-Presence-Detector/actions/workflows/continuous-build.yaml)

![Languages and Tool](./docs/Languages%20And%20Tools.drawio.svg)

## Summary

This is a simple project in which a HC-SR501 PIR sensor is used to detect motion in a room. MQTT is used as the message broke between NodeRed and Rust running on a Raspberry Pi 4. The code detects the presence and then notifies the server on a change in presence. Node Red then tells Alexa to turn the lights on and off within a time frame.

## System Architecture

The architecture of the system is shown in the diagram below:

![System Architecture](./docs/System%20Architecture.drawio.svg)

The Key parts are outlined below:

- **Rust Presence Detector**: Is a simple Script that uses polling to detect when the PIR Sensor is triggered. The PIR Sensor is configured for Repeatable Trigger Mode at. The code is constantly checking the pin every 5 seconds or so (This can be configured). At which point it sends an MQTT message on a state change.

- **MQTT Broker** Receives this message before forwarding it on to NodeRed as its subscribed to the same room topic.

- **Node-Red** Will then set its internal presence state before thend decided what to do. Based on the time of day the lights may come on or off depending on the state.

## Node Red Control FLow

On the NodeRed side the following flow is used:


This flow will turn the lights in the master bedroom on and off dependent on the presence and the time of day.

## Getting Started

Hardware wise, you must make sure the HC-SR501 is in Repeatable Trigger Mode and that the delay and sensitivity triggers are set accordingly.

Then configure your docker-compose.yaml file like how its configured in the [example](./examples/docker/docker-compose.yaml).

Then when you are ready run the docker compose command:

```bash
docker compose up 
```

## Developer Notes

If you are developing with this project, I like to use the [FT232H Breakout Board](https://www.adafruit.com/product/2264) which provides desktop with GPIO, I2C and SPI connections. This can be done using the following command

```bash
cargo run --features=dev
```

This will used the dev-config found [here](./src/config.rs). You may also need to install the following dependencies for the FTD232H Board if you choose to work with it:

```bash
apt install libftdi1 libftdi1-dev
```

## Extra Examples

When developing this code I wrote some simple MQTT code which could send and receive from the broker. It can be ran like the following:

```bash
cargo run --example mock_transmitter
cargo run --example receiver
```

## Useful Links

- [HC-SR501](https://dronebotworkshop.com/using-pir-sensors-with-arduino-raspberry-pi/)
- [Node-Red](https://cookbook.nodered.org/mqtt/connect-to-broker)
