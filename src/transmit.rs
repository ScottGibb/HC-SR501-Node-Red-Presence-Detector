use std::error::Error;

use log::info;
use paho_mqtt as mqtt;
pub struct MqttClient {
    state_topic: String,
    availability_topic: String,
    client: mqtt::Client,
}

impl MqttClient {
    pub fn new(
        host: String,
        port: u16,
        client_id: String,
        state_topic: String,
        availability_topic: String,
        discovery_topic: String,
        device_name: String,
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
        
        // Set last will and testament for availability
        let lwt = mqtt::Message::new(&availability_topic, "offline", mqtt::QOS_1);
        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(std::time::Duration::from_secs(30))
            .clean_session(true)
            .will_message(lwt)
            .finalize();
        match client.connect(conn_opts) {
            Ok(_) => (),
            Err(e) => return Err(Box::new(e)),
        };
        
        // Publish discovery configuration for Home Assistant
        let discovery_config = serde_json::json!({
            "name": device_name,
            "device_class": "occupancy",
            "state_topic": state_topic,
            "availability_topic": availability_topic,
            "payload_on": "ON",
            "payload_off": "OFF",
            "payload_available": "online",
            "payload_not_available": "offline",
            "unique_id": client_id,
            "device": {
                "identifiers": [client_id.clone()],
                "name": device_name,
                "model": "HC-SR501",
                "manufacturer": "Custom"
            }
        });
        let discovery_msg = mqtt::Message::new(
            discovery_topic,
            discovery_config.to_string(),
            mqtt::QOS_1,
        );
        match client.publish(discovery_msg) {
            Ok(_) => info!("Published Home Assistant discovery configuration"),
            Err(e) => return Err(Box::new(e)),
        };
        
        // Publish online availability
        let availability_msg = mqtt::Message::new(&availability_topic, "online", mqtt::QOS_1);
        match client.publish(availability_msg) {
            Ok(_) => info!("Published availability: online"),
            Err(e) => return Err(Box::new(e)),
        };
        
        info!("MQTT client initialized");
        Ok(MqttClient {
            state_topic,
            availability_topic,
            client,
        })
    }

    pub fn send_state(&self, occupied: bool) -> Result<(), Box<dyn Error>> {
        let payload = if occupied { "ON" } else { "OFF" };
        let msg = mqtt::Message::new(&self.state_topic, payload, mqtt::QOS_1);
        match self.client.publish(msg) {
            Ok(_) => (),
            Err(e) => return Err(Box::new(e)),
        };
        Ok(())
    }
}

impl Drop for MqttClient {
    fn drop(&mut self) {
        // Publish offline availability before disconnecting
        let availability_msg = mqtt::Message::new(&self.availability_topic, "offline", mqtt::QOS_1);
        if let Err(e) = self.client.publish(availability_msg) {
            log::warn!("Failed to publish offline status: {}", e);
        }
        if let Err(e) = self.client.disconnect(None) {
            log::warn!("Failed to disconnect gracefully: {}", e);
        }
    }
}
