#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Io, Level, Output},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
use esp_println::println;
use esp_wifi::{
    current_millis,
    esp_now::{EspNow, BROADCAST_ADDRESS},
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio2, Level::Low);

    esp_println::logger::init_logger_from_env();

    let timer = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1, &clocks, None).timer0;
    let init = esp_wifi::initialize(
        esp_wifi::EspWifiInitFor::Wifi,
        timer,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
        &clocks,
    )
    .unwrap();
    let wifi = peripherals.WIFI;
    let mut esp_now = EspNow::new(&init, wifi).unwrap();

    let interval = 2000;
    let mut time_slot = current_millis();
    let mut before = time_slot;
    let mut after = None;
    loop {
        if current_millis() >= time_slot {
            time_slot = current_millis() + interval;
            after = None;

            esp_now
                .send(&BROADCAST_ADDRESS, &[0])
                .unwrap()
                .wait()
                .unwrap();

            led.set_high();
            delay.delay(10.millis());
            led.set_low();
        }

        if esp_now.receive().is_some() {
            println!("Received ping");
            if after.is_none() {
                after = Some(current_millis());
                let median = (after.unwrap() - before) / 2;
                time_slot = before + median + interval;
            } else {
                before = current_millis();
            }
        }
    }
}
