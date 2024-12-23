#[cfg(feature = "dev")]
use embedded_hal::digital::InputPin;

mod config;
mod pins;
mod transmit;

const POLL_INTERVAL: u64 = 5; // seconds
const PRINT_INTERVAL: u64 = 60 * 30; // seconds
fn main() {
    let last_print_time = std::time::Instant::now();
    println!("Initializing...");
    let config = config::get_config();
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
    let mut has_pin_changed = false;
    let mut last_pin_state = false;
    println!("Starting loop");
    loop {
        {
            #[cfg(feature = "dev")]
            let current_pin_state = pin.is_high().unwrap();
            #[cfg(feature = "prod")]
            let current_pin_state = pin.is_high();

            if current_pin_state != last_pin_state {
                has_pin_changed = true;
                last_pin_state = current_pin_state;
            } else {
            }
            if last_print_time.elapsed().as_secs() > PRINT_INTERVAL {
                println!("Current Pin State: {}", current_pin_state);
            }
            if has_pin_changed {
                println!("Presence Change Detected");
                let message = serde_json::json!({
                    "presence": current_pin_state,
                    "timestamp": chrono::Utc::now().to_string(),
                    "sensor_id": config.sensor_id,
                });
                println!("Sending message: {}", message);
                mqtt.send_message(message.to_string()).unwrap();
                has_pin_changed = false;
            }
            std::thread::sleep(std::time::Duration::from_secs(POLL_INTERVAL));
        }
    }
}
