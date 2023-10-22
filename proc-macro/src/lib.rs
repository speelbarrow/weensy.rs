use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

mod arg;
use arg::Arg;

/**
Configures your `main` function to be exported properly.

Arguments may be passed to the attribute which will then be used to unpack the [resources] returned by the [`t41`]
bootstraper.

```
#![no_std]
#![no_main]

use weensy::{init, entry};
#[entry(usb, mut gpio2, pins[mut 12, 13])]
# #[export_name = "_not_main"]
fn main() -> ! {
    init::log(usb);
}
# #[export_name = "main"]
# fn _main() -> i32 {
#    0
# }
```

[resources]: teensy4_bsp::board::Resources
[`t41`]: teensy4_bsp::board::t41
*/
#[proc_macro_attribute]
pub fn entry(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let main = parse_macro_input!(input as ItemFn);
    quote! {
        #[no_mangle]
        #main
    }
    .into()
}
