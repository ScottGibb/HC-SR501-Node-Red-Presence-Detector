use paho_mqtt as mqtt;

fn main() {
    let host_address = "localhost";
    let port = 1883;
    let topic = "presence/room/1";

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
            println!("Received: {:?}", msg);
        }
    }
}
