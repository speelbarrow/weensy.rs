use proc_macro::TokenStream;

mod entry;
/**
Configures your `main` function to be exported properly.

Arguments may be passed to the attribute which will then be used to unpack the [resources] returned by the [`t41`]
bootstraper.

``` no_run
use weensy::{init, entry};
#[entry(usb, mut gpio2, pins[mut 12, 13])]
fn main() -> ! {
    init::log(usb);
    // ...
#   panic!()
# }
```
``` compile_fail no_run
use weensy::entry;
#[entry]
fn main() { // Signature must be `fn main() -> !`
    // ...
#    loop{}
# }
```

[resources]: teensy4_bsp::board::Resources
[`t41`]: teensy4_bsp::board::t41
*/
#[proc_macro_attribute]
pub fn entry(attr: TokenStream, input: TokenStream) -> TokenStream {
    entry::entry(attr, input)
}
