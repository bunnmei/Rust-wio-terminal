//! ```sh
//! 
//! $ cargo hf2 --example led_blinking
//! 
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::hal::gpio::*; 
use wio::hal::delay::Delay;
use wio::hal::clock::GenericClockController;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals: Peripherals = Peripherals::take().unwrap();
    let core: CorePeripherals = CorePeripherals::take().unwrap();
    let mut clocks: GenericClockController = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins: wio::Pins = wio::Pins::new(peripherals.PORT);

    // ledを取得
    let mut led: Pin<PA15, Output<PushPull>> = pins.user_led.into_push_pull_output();

    // delayの取得
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {

        led.set_high().unwrap();

        delay.delay_ms(1000u16);
        
        led.set_low().unwrap();

        delay.delay_ms(1000u16);

    }

}
