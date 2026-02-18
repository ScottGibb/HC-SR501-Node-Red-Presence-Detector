use paho_mqtt as mqtt;
use std::io::{self, BufRead};
use std::process;

/// This example demonstrates a mock transmitter that sends a presence message to an MQTT broker
/// when the user presses Enter. The message is a simple "ON" payload following Home Assistant conventions.
fn main() {
    let host_address = "localhost";
    // let host_address: &str = "192.168.0.69"; //PiLab
    let port = 1883;
    let sensor_id = "2";
    let room = "master_bedroom";
    let device_id = format!("{}_presence_{}", room, sensor_id);
    let state_topic = format!("homeassistant/binary_sensor/{}/state", device_id);
    let availability_topic = format!("homeassistant/binary_sensor/{}/availability", device_id);
    let discovery_topic = format!("homeassistant/binary_sensor/{}/config", device_id);

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(format!("tcp://{host_address}:{port}"))
        .client_id(format!("presence_detector_{}", sensor_id))
        .finalize();

    let client = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {err:?}");
        process::exit(1);
    });

    // Set last will and testament for availability
    let lwt = mqtt::Message::new(&availability_topic, "offline", mqtt::QOS_1);
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(20))
        .clean_session(true)
        .will_message(lwt)
        .finalize();

    // Connect and wait for it to complete or fail
    if let Err(e) = client.connect(conn_opts) {
        println!("Unable to connect:\n\t{e:?}");
        process::exit(1);
    }

    // Publish discovery configuration for Home Assistant
    let discovery_config = serde_json::json!({
        "name": format!("{} Presence Sensor", room.replace('_', " ")),
        "device_class": "occupancy",
        "state_topic": state_topic,
        "availability_topic": availability_topic,
        "payload_on": "ON",
        "payload_off": "OFF",
        "payload_available": "online",
        "payload_not_available": "offline",
        "unique_id": format!("presence_detector_{}", sensor_id),
        "device": {
            "identifiers": [format!("presence_detector_{}", sensor_id)],
            "name": format!("{} Presence Sensor", room.replace('_', " ")),
            "model": "HC-SR501",
            "manufacturer": "Custom"
        }
    });
    let discovery_msg = mqtt::Message::new(&discovery_topic, discovery_config.to_string(), mqtt::QOS_1);
    if let Err(e) = client.publish(discovery_msg) {
        println!("Error publishing discovery config: {e:?}");
    } else {
        println!("Published Home Assistant discovery configuration");
    }

    // Publish online availability
    let availability_msg = mqtt::Message::new(&availability_topic, "online", mqtt::QOS_1);
    if let Err(e) = client.publish(availability_msg) {
        println!("Error publishing availability: {e:?}");
    } else {
        println!("Published availability: online");
    }

    println!("Press Enter to send a presence message (ON). Press Ctrl+C to exit.");

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
            let msg = mqtt::Message::new(&state_topic, "ON", mqtt::QOS_1);

            // Attempt to publish the message
            if let Err(e) = client.publish(msg) {
                println!("Error sending message: {e:?}");
                // If message sending fails, try reconnecting
                if let Err(e) = client.reconnect() {
                    println!("Error reconnecting: {e:?}");
                    break; // If we cannot reconnect, exit the loop
                }
            } else {
                println!("Message sent on topic: [{}]: ON", state_topic);
            }
        }
    }

    // Publish offline availability before disconnecting
    let availability_msg = mqtt::Message::new(&availability_topic, "offline", mqtt::QOS_1);
    let _ = client.publish(availability_msg);

    // Graceful shutdown: disconnect from the MQTT broker
    if let Err(e) = client.disconnect(None) {
        println!("Error disconnecting: {e:?}");
    }

    println!("Disconnected. Exiting...");
}
