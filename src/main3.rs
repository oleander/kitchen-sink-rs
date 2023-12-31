// // #![no_std]
// // #![feature(start)]
// // #![no_main]

// use esp32_hal::peripherals::Peripherals;
// use esp32_hal::clock::ClockControl;
// use esp32_hal::gpio::{AlternateFunction, Event, InputSignal, Pin};
// use esp32_hal::{Delay, IO};
// use esp32_hal::prelude::*;
// use log::log_enabled;

// // #[entry]
// // #[no_mangle]
// fn main() {
//   log_enabled!(log::Level::Info);

//   log::info!("Peripherals initialized");
//   let dp = Peripherals::take();

//   log::info!("System setup");
//   let system = dp.SYSTEM.split();

//   log::info!("Clock setup");
//   let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

//   log::info!("Delay setup");
//   let mut delay = Delay::new(&clocks);

//   log::info!("GPIO setup");
//   let io = IO::new(dp.GPIO, dp.IO_MUX);

//   log::info!("GPIO13 setup");
//   let mut pin = io.pins.gpio13.into_pull_up_input();

//   log::info!("GPIO13 alternate function setup");
//   pin.set_alternate_function(AlternateFunction::Function0);

//   log::info!("GPIO13 interrupt enable");
//   pin.enable_input(true);

//   log::info!("GPIO13 pull-up enable");
//   pin.internal_pull_up(true);

//   log::info!("GPIO13 interrupt setup");
//   pin.listen(Event::LowLevel);

//   pin.connect_input_to_peripheral(InputSignal::GPIO_BT_ACTIVE);

//   loop {
//     log::info!("GPIO13 state: {}", pin.is_low().unwrap());
//     delay.delay_ms(1000 as u32);
//   }
// }

// #[interrupt]
// fn GPIO() {
//   log::info!("GPIO13 interrupt");
// }
