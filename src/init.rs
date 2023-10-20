use teensy4_bsp::{
    board::PERCLK_FREQUENCY,
    hal::{
        gpt::{ClockSource, Gpt},
        timer::Blocking,
    },
};

#[cfg(feature = "t4bsp-usb-logging")]
use teensy4_bsp::{hal::usbd::Instances, LoggingFrontend};

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

#[cfg(feature = "t4bsp-usb-logging")]
const DEFAULT_LOG: LoggingFrontend = LoggingFrontend::default_log();
/**
Initializes a [USB] peripheral for logging over a serial connection. See [log] for more info.

[USB]: teensy4_bsp::hal::usbd::Instances<1>
[log]: ::log
*/
#[cfg(feature = "t4bsp-usb-logging")]
pub fn log(usb: Instances<1>) {
    DEFAULT_LOG.register_usb(usb);
}
