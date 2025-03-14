//! GPIO interrupt
//!
//! This prints "Interrupt" when the boot button is pressed.
//! It also blinks an LED like the blinky example.
//!
//! The following wiring is assumed:
//! - LED => GPIO2
//! - BUTTON => GPIO0 (ESP32, ESP32-S2, ESP32-S3) / GPIO9

//% CHIPS: esp32 esp32c2 esp32c3 esp32c6 esp32h2 esp32s2 esp32s3
//% FEATURES: esp-hal/unstable

#![no_std]
#![no_main]

use core::cell::RefCell;

use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Event, Input, InputConfig, Io, Level, Output, OutputConfig, Pull},
    handler,
    main,
    ram,
};

static BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static MOTOR: Mutex<RefCell<Option<Output>>> = Mutex::new(RefCell::new(None));

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(handler);

    let button = peripherals.GPIO9;
    let motor = peripherals.GPIO4;
    let mut button = Input::new(button, InputConfig::default().with_pull(Pull::Up));
    let motor = Output::new(motor, Level::Low, OutputConfig::default());
    critical_section::with(|cs| {
        button.listen(Event::AnyEdge);
        MOTOR.borrow_ref_mut(cs).replace(motor);
        BUTTON.borrow_ref_mut(cs).replace(button);
    });

    let delay = Delay::new();
    loop {
        delay.delay_millis(500);
    }
}

#[handler]
#[ram]
fn handler() {
    esp_println::println!(
        "State: {:?}",
        critical_section::with(|cs| { BUTTON.borrow_ref_mut(cs).as_mut().unwrap().is_low() })
    );

    critical_section::with(|cs| {
        MOTOR.borrow_ref_mut(cs).as_mut().unwrap().set_level(
            if BUTTON.borrow_ref_mut(cs).as_mut().unwrap().is_low() {
                Level::High
            } else {
                Level::Low
            },
        );
    });

    critical_section::with(|cs| {
        BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt()
    });
}
