#[cfg(feature = "dev")]
use embedded_hal::digital::InputPin;
mod config;
mod pins;
mod transmit;

fn main() {
    println!("Initializing...");
    #[cfg(not(feature = "dev"))]
    let config = config::get_config_from_env();
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
        #[cfg(feature = "dev")]
        if pin.is_high().unwrap() {
            println!("Presence Detected");
            let message = serde_json::json!({
                "presence": true,
                "timestamp": chrono::Utc::now().to_string(),
                "sensor_id": config.sensor_id,
            });
            println!("Sending message: {}", message);
            mqtt.send_message(message.to_string()).unwrap();
        }
        #[cfg(not(feature = "dev"))]
        if pin.is_high() {
            println!("Presence Detected");
            let message = serde_json::json!({
                "presence": true,
                "timestamp": chrono::Utc::now().to_string(),
                "sensor_id": config.sensor_id,
            });
            println!("Sending message: {}", message);
            mqtt.send_message(message.to_string()).unwrap();
        }
    }
}
