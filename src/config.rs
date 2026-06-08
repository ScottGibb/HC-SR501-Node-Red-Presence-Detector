#[derive(Debug)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub client_id: String,
    pub topic: String,
}

#[derive(Debug)]
pub struct HomeAssistantConfig {
    pub discovery_topic: String,
    pub state_topic: String,
    pub unique_id: String,
    pub entity_name: String,
    pub device_name: String,
    pub device_identifier: String,
}

#[derive(Debug)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub pin: String,
    pub sensor_id: String,
    pub home_assistant: HomeAssistantConfig,
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

    let room_slug = room.to_lowercase().replace(' ', "-");
    let unique_id = format!("{room_slug}-{sensor_id}-presence");
    let entity_name = format!("{room} Presence");
    let device_name = format!("{room} Presence Sensor {sensor_id}");
    let device_identifier = format!("{room_slug}-{sensor_id}");
    let ha_state_topic = format!("presence/{room}/{sensor_id}/ha_state");

    let mqtt_topic = format!("presence/{room}/");
    info!("Configuration read successfully");
    Config {
        mqtt: MqttConfig {
            host: mqtt_host,
            port: mqtt_port.parse().unwrap(),
            client_id: format!("transmitter-{sensor_id}"),
            topic: mqtt_topic,
        },
        pin,
        sensor_id,
        home_assistant: HomeAssistantConfig {
            discovery_topic: format!("homeassistant/binary_sensor/{unique_id}/config"),
            state_topic: ha_state_topic,
            unique_id,
            entity_name,
            device_name,
            device_identifier,
        },
    }
}
#[cfg(feature = "dev-config")]
pub fn get_config() -> Config {
    info!("Using development configuration");
    let room = "master-bedroom".to_string();
    let sensor_id = "1".to_string();
    let unique_id = format!("{room}-{sensor_id}-presence");
    Config {
        mqtt: MqttConfig {
            host: "localhost".to_string(),
            port: 1883,
            client_id: format!("transmitter-{}", sensor_id),
            topic: format!("presence/{room}/"),
        },
        // pin: "4".to_string(),  // BCM pin number
        pin: "C0".to_string(), // FTDI port and pin number
        sensor_id,
        home_assistant: HomeAssistantConfig {
            discovery_topic: format!("homeassistant/binary_sensor/{unique_id}/config"),
            state_topic: format!("presence/{room}/1/ha_state"),
            unique_id,
            entity_name: "Master Bedroom Presence".to_string(),
            device_name: "Master Bedroom Presence Sensor 1".to_string(),
            device_identifier: "master-bedroom-1".to_string(),
        },
    }
}
