use std::error::Error;

use crate::config::HomeAssistantConfig;
use log::info;
use paho_mqtt as mqtt;
use serde_json::json;
pub struct MqttClient {
    topic: String,
    client: mqtt::Client,
}

impl MqttClient {
    pub fn new(
        host: String,
        port: u16,
        client_id: String,
        topic: String,
    ) -> Result<MqttClient, Box<dyn Error>> {
        info!("Initializing MQTT client...");
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(format!("tcp://{host}:{port}"))
            .client_id(&client_id)
            .finalize();
        let client = match mqtt::Client::new(create_opts) {
            Ok(client) => client,
            Err(e) => return Err(Box::new(e)),
        };
        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(std::time::Duration::from_secs(30))
            .clean_session(true)
            .finalize();
        match client.connect(conn_opts) {
            Ok(_) => (),
            Err(e) => return Err(Box::new(e)),
        };
        info!("MQTT client initialized");
        Ok(MqttClient { topic, client })
    }

    pub fn send_message(&self, message: String) -> Result<(), Box<dyn Error>> {
        let msg = mqtt::Message::new(self.topic.clone(), message, mqtt::QOS_1);
        match self.client.publish(msg) {
            Ok(_) => (),
            Err(e) => return Err(Box::new(e)),
        };
        Ok(())
    }

    pub fn send_home_assistant_presence(
        &self,
        config: &HomeAssistantConfig,
        presence_detected: bool,
    ) -> Result<(), Box<dyn Error>> {
        let discovery_payload = json!({
            "name": config.entity_name,
            "unique_id": config.unique_id,
            "state_topic": config.state_topic,
            "payload_on": "ON",
            "payload_off": "OFF",
            "device_class": "occupancy",
            "device": {
                "identifiers": [config.device_identifier],
                "name": config.device_name,
            }
        });

        let discovery_message = mqtt::MessageBuilder::new()
            .topic(config.discovery_topic.clone())
            .payload(discovery_payload.to_string())
            .qos(mqtt::QOS_1)
            .retained(true)
            .finalize();

        match self.client.publish(discovery_message) {
            Ok(_) => (),
            Err(e) => return Err(Box::new(e)),
        };

        let state_message = mqtt::MessageBuilder::new()
            .topic(config.state_topic.clone())
            .payload(if presence_detected { "ON" } else { "OFF" })
            .qos(mqtt::QOS_1)
            .retained(true)
            .finalize();

        match self.client.publish(state_message) {
            Ok(_) => (),
            Err(e) => return Err(Box::new(e)),
        };

        info!(
            "Published Home Assistant presence state: {}",
            if presence_detected { "ON" } else { "OFF" }
        );
        Ok(())
    }
}
