use commitment_issues::include_metadata;
#[cfg(feature = "dev")]
use embedded_hal::digital::InputPin;
#[cfg(feature = "prod")]
use std::sync::Arc;

include_metadata!();
mod config;
mod pins;
mod transmit;
use log::{error, info};
#[cfg(feature = "dev")]
use log::{debug, warn};

// Timing Constants
#[cfg(feature = "dev")]
const POLL_INTERVAL: u64 = 5; // seconds
#[cfg(feature = "dev")]
const PRINT_INTERVAL: u64 = 60 * 30; // seconds
#[cfg(feature = "prod")]
const DEBOUNCE_DURATION_MS: u64 = 50; // milliseconds

#[cfg(feature = "dev")]
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
    info!("Starting polling loop (dev mode)");
    loop {
        {
            let current_pin_state = match pin.is_high() {
                Ok(current_pin_state) => current_pin_state,
                Err(e) => {
                    warn!("Failed to read pin state: {e}");
                    error!("Exiting...");
                    break;
                }
            };

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

#[cfg(feature = "prod")]
fn main() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc;
    
    env_logger::init();
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
        config.mqtt.host.clone(),
        config.mqtt.port,
        config.mqtt.client_id.clone(),
        config.mqtt.topic.clone(),
    ) {
        Ok(mqtt) => mqtt,
        Err(e) => {
            error!("Error: {e}");
            return;
        }
    };
    
    let sensor_id = Arc::new(config.sensor_id.clone());
    let sensor_id_callback = sensor_id.clone();
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    
    // Setup signal handler for graceful shutdown
    ctrlc::set_handler(move || {
        info!("Received shutdown signal");
        running_clone.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    
    // Create channel for sending messages from interrupt handler to MQTT thread
    let (tx, rx) = mpsc::channel();
    
    // Spawn a thread to handle MQTT sending
    let running_mqtt = running.clone();
    std::thread::spawn(move || {
        while running_mqtt.load(Ordering::SeqCst) {
            match rx.recv_timeout(std::time::Duration::from_secs(1)) {
                Ok(message) => {
                    info!("Sending message: {message}");
                    if let Err(e) = mqtt.send_message(message) {
                        error!("Failed to send message: {e}");
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    info!("Message channel disconnected, exiting MQTT thread");
                    break;
                }
            }
        }
    });
    
    info!("Setting up async interrupt handler");
    
    // Get and send initial pin state
    let initial_state = pin.is_high();
    info!("Initial pin state: {}", initial_state);
    
    // Send initial state message
    let initial_message = serde_json::json!({
        "presence": initial_state,
        "timestamp": chrono::Utc::now().to_string(),
        "sensor_id": sensor_id.as_ref(),
    })
    .to_string();
    info!("Sending initial message: {}", initial_message);
    if let Err(e) = tx.send(initial_message) {
        error!("Failed to send initial message to MQTT thread: {e}");
    }
    
    // Setup async interrupt on both rising and falling edges
    let result = pin.set_async_interrupt(
        rppal::gpio::Trigger::Both,
        Some(std::time::Duration::from_millis(DEBOUNCE_DURATION_MS)),
        move |event| {
            // The trigger field tells us what edge triggered this interrupt
            let presence = match event.trigger {
                rppal::gpio::Trigger::RisingEdge => true,
                rppal::gpio::Trigger::FallingEdge => false,
                _ => {
                    error!("Unexpected trigger type");
                    return;
                }
            };
            
            info!("Presence Change Detected: {}", presence);
            let message = serde_json::json!({
                "presence": presence,
                "timestamp": chrono::Utc::now().to_string(),
                "sensor_id": sensor_id_callback.as_ref(),
            })
            .to_string();
            
            // Send message to MQTT thread via channel
            if let Err(e) = tx.send(message) {
                error!("Failed to send message to MQTT thread: {e}");
            }
        },
    );
    
    if let Err(e) = result {
        error!("Failed to set async interrupt: {e}");
        return;
    }
    
    info!("Async interrupt handler configured successfully");
    info!("Waiting for PIR sensor events (press Ctrl-C to exit)...");
    
    // Keep the main thread alive
    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    info!("Shutting down gracefully...");
}
