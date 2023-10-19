use teensy4_bsp::{
    board::PERCLK_FREQUENCY,
    hal::{
        gpt::{ClockSource, Gpt},
        timer::Blocking,
    },
};

const CLOCK_SOURCE: ClockSource = ClockSource::HighFrequencyReferenceClock;

/**
Initializes the provided [GPT] to run at the specified `HZ` using the [high frequency reference clock]. Returns a
[Blocking] timer to allow 'sleeping'.

This function consumes the [GPT] instance provided to it.

[GPT]: teensy4_bsp::hal::gpt::Gpt
[high frequency reference clock]: teensy4_bsp::hal::gpt::ClockSource::HighFrequencyReferenceClock
[Blocking]: teensy4_bsp::hal::timer::Blocking
*/
pub fn delay<const N: u8, const HZ: u32>(mut gpt: Gpt<N>) -> Blocking<Gpt<N>, HZ> {
    gpt.disable();
    gpt.set_divider(PERCLK_FREQUENCY / HZ);
    gpt.set_clock_source(CLOCK_SOURCE);

    Blocking::from_gpt(gpt)
}
