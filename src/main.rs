use std::sync::mpsc::{channel, Receiver, Sender};
use svc::hal::task::watchdog::TWDTDriver;
use hal::task::watchdog::TWDTConfig;
use std::sync::Mutex as StdMutex;
use hal::prelude::Peripherals;
use lazy_static::lazy_static;
use critical_section::Mutex;
use hal::gpio::PinDriver;
use std::time::Duration;
use svc::hal::cpu::Core;
use std::cell::RefCell;
use esp_idf_svc as svc;
use svc::hal::gpio::*;
use esp_idf_svc::hal;
use sys::EspError;
use svc::sys;

use esp_idf_svc::timer::EspTimer;

mod keyboard;
use keyboard::Keyboard;

static M1: Mutex<RefCell<Option<PinDriver<Gpio12, Input>>>> = Mutex::new(RefCell::new(None));
static M2: Mutex<RefCell<Option<PinDriver<Gpio13, Input>>>> = Mutex::new(RefCell::new(None));
// 6 more buttons will be added

static EVENT: Mutex<RefCell<Option<i32>>> = Mutex::new(RefCell::new(None));
static STATE: Mutex<RefCell<Option<i32>>> = Mutex::new(RefCell::new(None));

lazy_static! {
  static ref CHANNEL: (Mutex<Sender<i32>>, StdMutex<Receiver<i32>>) = {
    let (send, recv) = channel();
    let recv = StdMutex::new(recv);
    let send = Mutex::new(send);
    (send, recv)
  };
}

// static BUTTON_TIMER: Mutex<RefCell<Option<FreeRtosTimer>>> = Mutex::new(RefCell::new(None));

fn button_timer_callback() {
  critical_section::with(|cs| {
      // Handle the button press event here after 200ms delay
      // ...
  });
}

macro_rules! setup_button_interrupt {
  ($mutex:ident, $pin:expr) => {
    let mut btn = PinDriver::input($pin)?;

    // Trigger when button is pushed
    btn.set_interrupt_type(InterruptType::LowLevel)?;

    EspTimer::new(200, button_timer_callback)?;

    // Default is pull up
    btn.set_pull(hal::gpio::Pull::Up)?;

    unsafe {
      // On click
      btn
        .subscribe(|| {
          critical_section::with(|cs| {
            let mut bbrn = $mutex.borrow_ref_mut(cs);
            let btn = bbrn.as_mut().unwrap();
            EVENT.borrow_ref_mut(cs).replace(btn.pin());
            // btn.enable_interrupt().unwrap();
            let timer = BUTTON_TIMER.borrow_ref(cs).as_ref().unwrap();
            timer.start().unwrap();
          });
        })
        .unwrap();
    }

    btn.enable_interrupt()?;
    critical_section::with(|cs| $mutex.borrow_ref_mut(cs).replace(btn));
  };
}

fn event_id() -> Option<i32> {
  critical_section::with(|cs| {
    let curr = EVENT.borrow_ref_mut(cs).take();
    let prev = STATE.borrow_ref_mut(cs).take();

    // If no event, return
    match (curr, prev) {
      // No new event
      (None, Some(prev)) => {
        STATE.borrow_ref_mut(cs).replace(prev);
        None
      },

      // Same as previous event
      // (Some(curr), Some(prev)) if curr == prev => {
      //   STATE.borrow_ref_mut(cs).replace(prev);
      //   None
      // },

      // New event
      (Some(curr), _) => {
        STATE.borrow_ref_mut(cs).replace(curr);
        Some(curr)
      }

      (None, None) => None
    }
  })
}

fn main() -> Result<(), EspError> {
  sys::link_patches();
  svc::log::EspLogger::initialize_default();

  let peripherals = Peripherals::take().unwrap();

  let mut keyboard = Keyboard::new();

  setup_button_interrupt!(M1, peripherals.pins.gpio12);
  setup_button_interrupt!(M2, peripherals.pins.gpio13);

  // let config = TWDTConfig {
  //   duration: Duration::from_secs(10), panic_on_trigger: false, subscribed_idle_tasks: Core::Core0.into()
  // };

  // let mut driver = TWDTDriver::new(peripherals.twdt, &config)?;

  // let mut watchdog = driver.watch_current_task()?;

  log::info!("Starting loop");
  loop {
    // watchdog.feed().unwrap();

    let Some(id) = event_id() else {
      hal::delay::FreeRtos::delay_ms(5);
      continue;
    };

    log::info!("Button {} pressed", id);

    if keyboard.connected() {
      keyboard.write(id.to_string().as_str());
    }

    hal::delay::FreeRtos::delay_ms(5);
  }
}
