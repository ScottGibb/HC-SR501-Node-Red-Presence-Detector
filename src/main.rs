use chrono;
use embedded_hal::digital::InputPin;
use serde_json as json;
mod config;
mod pins;
mod transmit;

fn main() {
    println!("Initializing...");
    let config = config::get_config_from_env();
    println!("Config: {:?}", config);
    let mut pin = match pins::get_pin(config.pin) {
        Ok(pin) => pin,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    println!("Pin initialized");
    println!("Connecting to MQTT broker...");
    let mqtt = match transmit::MqttClient::new(
        config.mqtt.host,
        config.mqtt.port,
        config.mqtt.client_id,
        config.mqtt.topic,
    ) {
        Ok(mqtt) => mqtt,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    println!("Connected to MQTT broker");
    loop {
        if pin.is_high().unwrap() {
            println!("Presence Detected");
            let message = json::json!({
                "presence": true,
                "timestamp": chrono::Utc::now().to_string(),
            });
            mqtt.send_message(message.to_string()).unwrap();
        }
    }
}
