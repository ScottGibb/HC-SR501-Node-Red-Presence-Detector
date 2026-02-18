#[derive(Debug)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub client_id: String,
    pub state_topic: String,
    pub availability_topic: String,
    pub discovery_topic: String,
}

#[derive(Debug)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub pin: String,
    pub sensor_id: String,
    pub room: String,
}

use log::info;

#[cfg(not(feature = "dev-config"))]
pub fn get_config() -> Config {
    use std::env;

    info!("Reading configuration from environment variables");
    let mqtt_host = env::var("MQTT_HOST").expect("MQTT_HOST environment variable not set");
    let mqtt_port = env::var("MQTT_PORT").expect("MQTT_PORT environment variable not set");
    let pin = env::var("PIN").expect("PIN environment variable not set");
    let room = env::var("ROOM").expect("ROOM environment variable not set");
    let sensor_id = env::var("SENSOR_ID").expect("SENSOR_ID environment variable not set");
    let device_id = format!("{}_presence_{}", room.replace('-', "_"), sensor_id);
    info!("Configuration read successfully");
    Config {
        mqtt: MqttConfig {
            host: mqtt_host,
            port: mqtt_port.parse().unwrap(),
            client_id: format!("presence_detector_{}", sensor_id),
            state_topic: format!("homeassistant/binary_sensor/{}/state", device_id),
            availability_topic: format!("homeassistant/binary_sensor/{}/availability", device_id),
            discovery_topic: format!("homeassistant/binary_sensor/{}/config", device_id),
        },
        pin,
        sensor_id,
        room,
    }
}
#[cfg(feature = "dev-config")]
pub fn get_config() -> Config {
    info!("Using development configuration");
    let sensor_id = "1".to_string();
    let room = "master_bedroom".to_string();
    let device_id = format!("{}_presence_{}", room, sensor_id);
    Config {
        mqtt: MqttConfig {
            host: "localhost".to_string(),
            port: 1883,
            client_id: format!("presence_detector_{}", sensor_id),
            state_topic: format!("homeassistant/binary_sensor/{}/state", device_id),
            availability_topic: format!("homeassistant/binary_sensor/{}/availability", device_id),
            discovery_topic: format!("homeassistant/binary_sensor/{}/config", device_id),
        },
        // pin: "4".to_string(),  // BCM pin number
        pin: "C0".to_string(), // FTDI port and pin number
        sensor_id,
        room,
    }
}
