use paho_mqtt as mqtt;
use serde_json::Value;

/// This example demonstrates a mock receiver that listens for presence messages on an MQTT broker.
/// The messages are expected to be JSON objects with a boolean `presence` field and a `timestamp`
/// field. The receiver will print the received JSON object to the console.
fn main() {
    // let host_address = "localhost";
    let host_address: &str = "192.168.0.69"; //PiLab
    let port = 1883;
    let topic = "presence/master-bedroom/";

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(format!("tcp://{}:{}", host_address, port))
        .client_id("receiver")
        .finalize();

    let client = mqtt::Client::new(create_opts).expect("Failed to create client");

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(30))
        .clean_session(true)
        .finalize();

    client
        .connect(conn_opts)
        .expect("Failed to connect, are you sure the broker is running?");

    client
        .subscribe(topic, 1)
        .expect("Failed to subscribe, are you sure the topic is correct?");

    for msg in client.start_consuming() {
        if let Some(msg) = msg {
            let payload_str = msg.payload_str();
            match serde_json::from_str::<Value>(&payload_str) {
                Ok(json) => println!("Received JSON: {:?}", json),
                Err(e) => println!("Error parsing JSON: {:?}", e),
            }
        }
    }
}
