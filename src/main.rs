use commitment_issues::include_metadata;
#[cfg(feature = "dev")]
use embedded_hal::digital::InputPin;

include_metadata!();
mod config;
mod pins;
mod transmit;
use log::{debug, error, info, warn};

// Timing Constants
const POLL_INTERVAL: u64 = 5; // seconds
const PRINT_INTERVAL: u64 = 60 * 30; // seconds
fn main() {
    env_logger::init();
    let last_print_time = std::time::Instant::now();
    info!("Presence Detector Version");
    info!("Tag: {}", metadata::tag_describe());
    info!("Author: {}", metadata::last_author());
    info!("Initializing...");
    let config = config::get_config();
    info!("Config Accepted: {config:?}");
    let mut pin = match pins::get_pin(config.pin) {
        Ok(pin) => pin,
        Err(e) => {
            error!("Error: {e}");
            return;
        }
    };
    let mqtt = match transmit::MqttClient::new(
        config.mqtt.host,
        config.mqtt.port,
        config.mqtt.client_id,
        config.mqtt.topic,
    ) {
        Ok(mqtt) => mqtt,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };
    // Pin State Variables
    let mut has_pin_changed = false;
    let mut last_pin_state = false;
    info!("Starting loop");
    loop {
        {
            #[cfg(feature = "dev")]
            let current_pin_state = match pin.is_high() {
                Ok(current_pin_state) => current_pin_state,
                Err(e) => {
                    warn!("Failed to read pin state: {e}");
                    error!("Exiting...");
                    break;
                }
            };
            #[cfg(feature = "prod")]
            let current_pin_state = pin.is_high();

            if current_pin_state != last_pin_state {
                has_pin_changed = true;
                last_pin_state = current_pin_state;
            }
            if last_print_time.elapsed().as_secs() > PRINT_INTERVAL {
                debug!("Current Pin State: {current_pin_state}");
            }
            if has_pin_changed {
                info!("Presence Change Detected");
                let message = serde_json::json!({
                    "presence": current_pin_state,
                    "timestamp": chrono::Utc::now().to_string(),
                    "sensor_id": config.sensor_id,
                });
                info!("Sending message: {message}");
                if let Err(e) = mqtt.send_message(message.to_string()) {
                    error!("Failed to send message: {e}");
                    return;
                }
                has_pin_changed = false;
            }
            std::thread::sleep(std::time::Duration::from_secs(POLL_INTERVAL));
        }
    }
}
