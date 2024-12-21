use std::error::Error;

use paho_mqtt as mqtt;
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
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(format!("tcp://{}:{}", host, port))
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
}
