use chrono::Utc;
use paho_mqtt as mqtt;
use serde_json::json;
use std::io::{self, BufRead};
use std::process;

/// This example demonstrates a mock transmitter that sends a presence message to an MQTT broker
/// when the user presses Enter. The message is a JSON object with a boolean `presence` field set to
/// `true` and a `timestamp` field set to the current time in UTC.
fn main() {
    let host_address = "localhost";
    // let host_address: &str = "192.168.0.69"; //PiLab
    let port = 1883;
    let sensor_id = "1";
    let topic = format!("presence/master-bedroom/");

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(format!("tcp://{}:{}", host_address, port))
        .client_id(format!("transmitter-{}", sensor_id))
        .finalize();

    let client = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    // Connect and wait for it to complete or fail
    if let Err(e) = client.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    println!("Press Enter to send a presence message. Press Ctrl+C to exit.");

    let stdin = io::stdin();
    let mut stdin_locked = stdin.lock();

    loop {
        let mut buffer = String::new();
        // Read a line of input (blocking until the user presses Enter)
        if stdin_locked.read_line(&mut buffer).is_err() {
            println!("Error reading input. Exiting...");
            break;
        }

        // If the input is empty (just pressing Enter), send a presence message
        if buffer.trim().is_empty() {
            let message = json!({
                "presence": true,
                "sensor_id": sensor_id,
                "timestamp": Utc::now().to_string(),
            });

            let msg = mqtt::Message::new(topic.clone(), message.to_string(), mqtt::QOS_1);

            // Attempt to publish the message
            if let Err(e) = client.publish(msg) {
                println!("Error sending message: {:?}", e);
                // If message sending fails, try reconnecting
                if let Err(e) = client.reconnect() {
                    println!("Error reconnecting: {:?}", e);
                    break; // If we cannot reconnect, exit the loop
                }
            } else {
                println!("Message sent on topic: [{}]: {}", topic, message);
            }
        }
    }

    // Graceful shutdown: disconnect from the MQTT broker
    if let Err(e) = client.disconnect(None) {
        println!("Error disconnecting: {:?}", e);
    }

    println!("Disconnected. Exiting...");
}
