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
pub fn get_pin(pin: u8) -> Result<rppal::gpio::InputPin, Box<dyn Error>> {
    let pin = rppal::gpio::Gpio::new()?.get(pin)?.into_input();
    Ok(pin)
}
#[cfg(feature = "dev")]
pub fn get_pin(pin: u8) -> Result<ftdi_embedded_hal::InputPin<Device>, Box<dyn Error>> {
    const BAUDRATE: u32 = 115200;
    const DEVICE_VID: u16 = 0x0403;
    const DEVICE_PID: u16 = 0x6014;

    let device = ftdi::find_by_vid_pid(DEVICE_VID, DEVICE_PID)
        .interface(ftdi::Interface::A)
        .open()?;

    let hal = match ftdi_embedded_hal::FtHal::init_freq(device, BAUDRATE) {
        Ok(hal) => hal,
        Err(e) => return Err(Box::new(e)),
    };

    let pin = match hal.ci0() {
        Ok(pin) => pin,
        Err(e) => return Err(Box::new(e)),
    };
    Ok(pin)
}
