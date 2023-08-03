//! ```sh
//! 
//! $ cargo hf2 --example led_button
//! 
//! ```


#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::hal::gpio::*; // GPIOの構造体やトレイトをインポートする
use wio::pac::Peripherals;
use wio::prelude::*; // 主要な構造体やトレイトをインポートする

#[entry]
fn main() -> ! {

    let peripherals: Peripherals = Peripherals::take().unwrap();
    let pins = wio::Pins::new(peripherals.PORT);

    let mut led: Led = Led::new(pins.user_led);
    let button1: Button1 = Button1::new(pins.button1);

    loop {

        if button1.is_pressed() {

            led.turn_on();

        } else {

            led.turn_off();

        }

    }
}

struct Button1 {
    pin: Pin<PC26, Input<Floating>>,
}

impl Button1 {
    fn new(pin: Pin<PC26, Reset>) -> Button1 {
        Button1 {
            pin: pin.into_floating_input(),
        }
    }

    fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }

    fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }
}

struct Led {
    pin: Pin<PA15, Output<PushPull>>,
}

impl Led {
    fn new(pin: Pin<PA15, Reset>) -> Led {
        Led {
            pin: pin.into_push_pull_output(),
        }
    }

    fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }

    fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }

    fn toggle(&mut self){
        self.pin.toggle();
    }
}