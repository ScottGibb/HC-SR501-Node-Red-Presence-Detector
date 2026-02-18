use paho_mqtt as mqtt;

/// This example demonstrates a mock receiver that listens for presence messages on an MQTT broker.
/// The messages are expected to be simple "ON" or "OFF" payloads following Home Assistant conventions.
/// The receiver will print the received payload to the console.
fn main() {
    let host_address = "localhost";
    // let host_address: &str = "192.168.0.69"; //PiLab
    let port = 1883;
    let device_id = "master_bedroom_presence_1";
    let state_topic = format!("homeassistant/binary_sensor/{}/state", device_id);

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(format!("tcp://{host_address}:{port}"))
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
        .subscribe(&state_topic, 1)
        .expect("Failed to subscribe, are you sure the topic is correct?");

    println!("Subscribed to topic: {}", state_topic);
    println!("Waiting for presence messages...");

    for msg in client.start_consuming().into_iter().flatten() {
        let payload_str = msg.payload_str();
        println!("Received state: {}", payload_str);
    }
}
