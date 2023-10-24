use proc_macro::TokenStream;

mod entry;
/**
Configures your `main` function to be exported properly.

Arguments may be passed to the attribute which will then be used to unpack the [resources] returned by the [`t41`]
bootstraper.

```
# #![no_main]
use weensy::{init, entry};
#[entry(usb, mut gpio2, pins[mut 12, 13])]
# #[export_name = "_not_main"]
fn main() -> ! {
    init::log(usb);
    // ...
#    loop{}
# }
# #[export_name = "main"]
# fn _main() -> i32 {
#    0
# }
```
``` compile_fail
# #![no_main]
use weensy::entry;
#[entry]
# #[export_name = "_not_main"]
// Signature must be `fn main() -> !`
fn main() {
    // ...
#    loop{}
# }
# #[export_name = "main"]
# fn _main() -> i32 {
#    0
# }
```

[resources]: teensy4_bsp::board::Resources
[`t41`]: teensy4_bsp::board::t41
*/
#[proc_macro_attribute]
pub fn entry(attr: TokenStream, input: TokenStream) -> TokenStream {
    entry::entry(attr, input)
}
