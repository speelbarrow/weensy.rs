#![no_std]

use embedded_hal::digital::v2::OutputPin;
use teensy4_bsp::{
    board::{led as get_led, Led},
    hal::{gpio::Port, gpt::Gpt, timer::Blocking},
    pins::t41::P13,
};

/**
Flashes the LED three times with a 500ms delay, then three times with a 250ms delay. Useful if you want to open the
serial console before your actual program starts running. Returns the [`Led`] instance afterwards.

[`Led`]: teensy4_bsp::board::Led
*/
pub fn countdown<const N: u8, const HZ: u32>(
    gpio2: &mut Port<2>,
    p13: P13,
    delay: &mut Blocking<Gpt<N>, HZ>,
) -> Led {
    let mut led = get_led(gpio2, p13);

    // Set the LED initially high.
    led.set_high().unwrap();

    // Used to avoid code duplication here. Defined in this scope to allow manipulation of scope variables.
    macro_rules! loop_time {
        ($block_ms: literal) => {
            for _ in 0..6 {
                led.toggle();
                delay.block_ms($block_ms);
            }
        };
    }

    loop_time!(500);
    loop_time!(250);

    // Make sure the LED is off afterward.
    led.set_low().unwrap();

    led
}
