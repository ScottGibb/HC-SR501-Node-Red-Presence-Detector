use std::error::Error;

#[cfg(feature = "dev")]
mod dev_dependencies {
    pub use ftdi;
    pub use ftdi::Device;
    pub use ftdi_embedded_hal;
}
#[cfg(feature = "dev")]
use dev_dependencies::*;

#[cfg(feature = "prod")]
use rppal;

#[cfg(feature = "prod")]
pub fn get_pin(pin: String) -> Result<rppal::gpio::InputPin, Box<dyn Error>> {
    let pin = pin.parse()?;
    let pin = rppal::gpio::Gpio::new()?.get(pin)?.into_input();
    Ok(pin)
}
#[cfg(feature = "dev")]
pub fn get_pin(pin: String) -> Result<ftdi_embedded_hal::InputPin<Device>, Box<dyn Error>> {
    const BAUDRATE: u32 = 115200;
    const DEVICE_VID: u16 = 0x0403;
    const DEVICE_PID: u16 = 0x6014;
    println!("Initializing FTDI device...");
    let device = ftdi::find_by_vid_pid(DEVICE_VID, DEVICE_PID)
        .interface(ftdi::Interface::A)
        .open()?;
    println!("FTDI device initialized");
    let hal = match ftdi_embedded_hal::FtHal::init_freq(device, BAUDRATE) {
        Ok(hal) => hal,
        Err(e) => return Err(Box::new(e)),
    };

    // Parse the pin string to get the port and pin number
    let port = pin.chars().nth(0);
    let pin = pin.chars().nth(1);

    // Get the pin from the port and pin number
    let pin = match port {
        Some('A') => match pin {
            Some('0') => hal.ci0(),
            Some('1') => hal.ci1(),
            Some('2') => hal.ci2(),
            Some('3') => hal.ci3(),
            Some('4') => hal.ci4(),
            Some('5') => hal.ci5(),
            Some('6') => hal.ci6(),
            Some('7') => hal.ci7(),
            _ => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid pin",
                )))
            }
        },
        Some('D') => match pin {
            Some('0') => hal.adi0(),
            Some('1') => hal.adi1(),
            Some('2') => hal.adi2(),
            Some('3') => hal.adi3(),
            Some('4') => hal.adi4(),
            Some('5') => hal.adi5(),
            Some('6') => hal.adi6(),
            Some('7') => hal.adi7(),
            _ => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid pin",
                )))
            }
        },
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid port",
            )))
        }
    };
    println!("FTDI HAL initialized");
    let pin = match pin {
        Ok(pin) => pin,
        Err(e) => return Err(Box::new(e)),
    };
    println!("Pin initialized");
    Ok(pin)
}
