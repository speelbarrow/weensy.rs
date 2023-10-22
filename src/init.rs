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

```
#![no_std]
#![no_main]
use weensy::{entry, init};

#[entry(mut gpt1)]
fn main() -> ! {
    let mut delay = init::delay::<1, 1000>(gpt1);
    // ...
#   loop {}
# }
# #[export_name = "main"]
# fn _main() -> i32 {
#      0
# }
```

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

This function consumes the [USB] instance provided to it.

```
#![no_std]
#![no_main]
use weensy::{entry, init};

#[entry(usb)]
fn main() -> ! {
    init::log(usb);
    // ...
#   loop {}
# }
# #[export_name = "main"]
# fn _main() -> i32 {
#      0
# }
```

[USB]: teensy4_bsp::hal::usbd::Instances<1>
[log]: https://docs.rs/log/latest/log/
*/
#[cfg(feature = "t4bsp-usb-logging")]
pub fn log(usb: Instances<1>) {
    DEFAULT_LOG.register_usb(usb);
}
