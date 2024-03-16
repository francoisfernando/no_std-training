#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO, Rmt};
use smart_leds::{SmartLedsWrite, RGB8};
use esp_hal_smartled::{SmartLedsAdapter, smartLedBuffer};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    println!("Hello world from Blinky!");

    // Set GPIO21 as an output, and set its state high initially.

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let rmt = Rmt::new(peripherals.RMT, 80u32.MHz(), &clocks).unwrap();

    let rmt_buffer = smartLedBuffer!(3);
    let mut led = SmartLedsAdapter::new(rmt.channel0, io.pins.gpio21, rmt_buffer, &clocks);

    // Initialize the Delay peripheral, and use it to toggle the LED state in a loop.
    let mut delay = Delay::new(&clocks);

    let mut data: [RGB8; 3] = [RGB8::default(); 3];
    let empty: [RGB8; 3] = [RGB8::default(); 3];

    loop {
        data[0] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[1] = RGB8 {
            r: 0,
            g: 0x10,
            b: 0,
        };
        data[2] = RGB8 {
            r: 0x10,
            g: 0,
            b: 0,
        };

        led.write(data.iter().cloned()).unwrap();
        delay.delay_ms(500 as u16);
        led.write(empty.iter().cloned()).unwrap();
        delay.delay_ms(500 as u16);
        println!("In the loop!");
    }
}
