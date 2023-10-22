use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, ItemFn, ReturnType, Signature, Token, Type};

mod entry;
use entry::Arg;

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

[resources]: teensy4_bsp::board::Resources
[`t41`]: teensy4_bsp::board::t41
*/
#[proc_macro_attribute]
pub fn entry(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<Arg, Token![,]>::parse_separated_nonempty)
        .into_iter();

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);
    verify_signature(&sig);

    let resources = {
        if args.len() != 0 {
            Some(quote! {
                let teensy4_bsp::board::Resources {
                    #(#args),* , ..
                } = teensy4_bsp::board::t41(teensy4_bsp::board::instances());
            })
        } else {
            None
        }
    };

    quote! {
        #[no_mangle]
        #(#attrs)*
        #vis #sig {
            #resources

            #block
        }
    }
    .into()
}

fn verify_signature(signature: &Signature) {
    macro_rules! is {
        (@some $( $prop: ident ).+, $value: expr) => {
            is!(@ne $( $prop ).+, Some($value))
        };
        (@none $( $prop: ident ).+) => {
            is!(@ne $( $prop ).+, None)
        };
        (@eq $( $prop: ident ).+, $value: expr) => {
            is!( signature.$( $prop ).+ == $value, &signature.$( $prop ).+ )
        };
        (@ne $( $prop: ident ).+, $value: expr) => {
            is!( signature.$( $prop ).+ != $value, &signature.$( $prop ).+ );
        };
        (@empty $( $prop: ident ).+) => {
            is!( signature.$( $prop ).+.is_empty(), &signature.$( $prop ).+ );
        };
        (@matches $lhs: pat, $( $prop: ident ).+, $out: expr) => {
            if let $lhs = $( $prop ).+ {
                is!(@err $out);
            }
        };
        ($bool: expr, $out: expr) => {
            if $bool {
                is!(@err $out);
            }
        };
        (@err $out: expr) => {
            syn::Error::new_spanned(
                $out,
                "this attribute may only be applied to a function with the signature `fn() main -> !`",
            ).to_compile_error();
        };
    }

    is!(@none constness);
    is!(@none asyncness);
    is!(@none unsafety);
    is!(@none abi);
    is!(@eq ident, "main");
    is!(@none generics.lt_token);
    is!(@none generics.gt_token);
    is!(@empty generics.params);
    is!(@none generics.where_clause);
    is!(@empty inputs);
    is!(@none variadic);

    if let ReturnType::Type(_, boxed) = &signature.output {
        if let Type::Never(_) = **boxed {
            return;
        }
    }

    is!(@err &signature.output);
}
