//! ```sh
//! 
//! $ cargo hf2 --example uart
//! 
//! ```

// ArduinoでUARTを受信するコード Arduino-uart-receiver/uart_receiver.ino
// ArduinoIDEの Serial Monitor で確認用

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals,Peripherals};
use wio::prelude::*;
use wio::{entry, Pins, Sets};
#[entry]
fn main() -> ! {

    let mut peripherals = Peripherals::take().unwrap();
    // クロックを初期化する

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL
    );

    // TODO: UARTドライバオブジェクトを初期化する

    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let mut serial: wio::HalUart = sets.uart.init(
        &mut clocks,
        115200.Hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
    );
    
    // Delay用
    let core: CorePeripherals = CorePeripherals::take().unwrap();
    let mut delay: Delay = Delay::new(core.SYST, &mut clocks);

    loop {
        for c in b"hello world\n".iter() {
            nb::block!(serial.write(*c)).unwrap();
        }
        
        delay.delay_ms(1000u16)
    }

}