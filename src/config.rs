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

#[cfg(not(feature = "dev"))]
pub fn get_config_from_env() -> Config {
    use std::env;
    let mqtt_host = env::var("MQTT_HOST").expect("MQTT_HOST environment variable not set");
    let mqtt_port = env::var("MQTT_PORT").expect("MQTT_PORT environment variable not set");
    let mqtt_client_id =
        env::var("MQTT_CLIENT_ID").expect("MQTT_CLIENT_ID environment variable not set");
    let pin = env::var("PIN").expect("PIN environment variable not set");
    let room = env::var("ROOM").expect("ROOM environment variable not set");
    let sensor_id = env::var("SENSOR_ID").expect("SENSOR_ID environment variable not set");
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
#[cfg(feature = "dev")]
pub fn default() -> Config {
    Config {
        mqtt: MqttConfig {
            host: "localhost".to_string(),
            port: 1883,
            client_id: "receiver".to_string(),
            topic: "presence/room/1".to_string(),
        },
        pin: 4,
    }
}
