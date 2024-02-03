mod peripherals;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use std::thread::sleep;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    info!("Hello, world! Eyo");
    let mut led = peripherals::get_led()?;
    loop {
        info!("LED on");
        led.set_high()?;
        sleep(std::time::Duration::from_secs(1));
        info!("LED off");
        led.set_low()?;
        sleep(std::time::Duration::from_secs(1));
    }
}
