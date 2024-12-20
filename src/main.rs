use chrono;
use embedded_hal::digital::InputPin;
use serde_json as json;
mod config;
mod pins;
mod transmit;

fn main() {
    println!("Initializing...");
    #[cfg(not(feature = "dev"))]
    let config = Config::get_config_from_env();
    #[cfg(feature = "dev")]
    let config = config::default();
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
            println!("Sending message: {}", message.to_string());
            mqtt.send_message(message.to_string()).unwrap();
        }
    }
}
