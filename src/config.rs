use std::env;

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
    pub pin: u8,
}

pub fn get_config_from_env() -> Config {
    let mqtt_host = env::var("MQTT_HOST").unwrap_or_else(|_| "localhost".to_string());
    let mqtt_port = env::var("MQTT_PORT").unwrap_or_else(|_| "1883".to_string());
    let mqtt_client_id = env::var("MQTT_CLIENT_ID").unwrap_or_else(|_| "ftdi".to_string());
    let pin = env::var("PIN").unwrap_or_else(|_| "0".to_string());
    let room = env::var("ROOM").unwrap_or_else(|_| "living-room".to_string());
    let sensor_id = env::var("SENSOR_ID").unwrap_or_else(|_| "1".to_string());
    let mqtt_topic = format!("presence/{}/{}", room, sensor_id);
    Config {
        mqtt: MqttConfig {
            host: mqtt_host,
            port: mqtt_port.parse().unwrap(),
            client_id: mqtt_client_id,
            topic: mqtt_topic,
        },
        pin: pin.parse().unwrap(),
    }
}
