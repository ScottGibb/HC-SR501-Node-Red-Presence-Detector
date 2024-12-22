#[derive(Debug)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub client_id: String,
    pub topic: String,
}

#[derive(Debug)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub pin: String,
    pub sensor_id: String,
}

#[cfg(not(feature = "dev-config"))]
pub fn get_config() -> Config {
    use std::env;
    let mqtt_host = env::var("MQTT_HOST").expect("MQTT_HOST environment variable not set");
    let mqtt_port = env::var("MQTT_PORT").expect("MQTT_PORT environment variable not set");
    let pin = env::var("PIN").expect("PIN environment variable not set");
    let room = env::var("ROOM").expect("ROOM environment variable not set");
    let sensor_id = env::var("SENSOR_ID").expect("SENSOR_ID environment variable not set");
    let mqtt_topic = format!("presence/{}/", room);
    Config {
        mqtt: MqttConfig {
            host: mqtt_host,
            port: mqtt_port.parse().unwrap(),
            client_id: format!("transmitter-{}", sensor_id),
            topic: mqtt_topic,
        },
        pin,
        sensor_id,
    }
}
#[cfg(feature = "dev-config")]
pub fn get_config() -> Config {
    let sensor_id = "1".to_string();
    Config {
        mqtt: MqttConfig {
            host: "localhost".to_string(),
            port: 1883,
            client_id: format!("transmitter-{}", sensor_id),
            topic: "presence/room/1".to_string(),
        },
        // pin: "4".to_string(),  // BCM pin number
        pin: "C0".to_string(), // FTDI port and pin number
        sensor_id,
    }
}
