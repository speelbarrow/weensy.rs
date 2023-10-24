use proc_macro::TokenStream;
use proc_macro2::{Ident as Ident2, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    ItemFn, LitInt, Pat, PatIdent, Result as SynResult, ReturnType, Signature, Token, Type,
};

pub fn entry(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);
    if let Err(e) = verify_signature(&sig) {
        return e.into();
    };

    let resources = {
        if attr.is_empty() {
            None
        } else {
            Some(parse_macro_input!(attr with Punctuated::<Arg, Token![,]>::parse_separated_nonempty)
                .into_iter())
        }
    }.map(|args| {
        quote! {
            let teensy4_bsp::board::Resources {
                #(#args),* , ..
            } = teensy4_bsp::board::t41(teensy4_bsp::board::instances());
        }
    });

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

fn verify_signature(signature: &Signature) -> Result<(), TokenStream2> {
    macro_rules! is {
        (@some $( $prop: ident ).+, $value: expr) => {
            is!(@eq $( $prop ).+, Some($value))
        };
        (@none $( $prop: ident ).+) => {
            is!(@eq $( $prop ).+, None)
        };
        (@eq $( $prop: ident ).+, $value: expr) => {
            is!( signature.$( $prop ).+ != $value, &signature.$( $prop ).+ )
        };
        (@ne $( $prop: ident ).+, $value: expr) => {
            is!( signature.$( $prop ).+ == $value, &signature.$( $prop ).+ );
        };
        (@empty $( $prop: ident ).+) => {
            is!( !signature.$( $prop ).+.is_empty(), &signature.$( $prop ).+ );
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
            return Err(syn::Error::new_spanned(
                $out,
                "this attribute may only be applied to a function with the signature `fn() main -> !`",
            ).to_compile_error());
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
            return Ok(());
        }
    }

    is!(@err &signature.output);
}

pub enum Arg {
    Ident(PatIdent),
    Pins(Punctuated<PatIdent, Token![,]>),
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if let Ok(Pat::Ident(ident)) = Pat::parse_single(input) {
            return if ident.ident == "pins" {
                let content;
                bracketed!(content in input);
                Ok(Self::Pins(Punctuated::parse_separated_nonempty_with(
                    &content, parse_pin,
                )?))
            } else {
                Ok(Self::Ident(ident))
            };
        }
        Err(input.error(format!("failed to parse argument: {}", input)))
    }
}

fn parse_pin(input: ParseStream) -> SynResult<PatIdent> {
    let mutability = input.parse::<Token![mut]>().ok();
    let num = input.parse::<LitInt>()?;
    let ident = Ident2::new(&("p".to_owned() + num.base10_digits()), num.span());
    if let Pat::Ident(r) = parse_quote!(#mutability #ident) {
        Ok(r)
    } else {
        Err(input.error(format!("failed to parse pin: {}", input)))
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Pins(pins) => {
                let i = pins.into_iter();
                quote!(pins: teensy4_bsp::pins::t41::Pins { #(#i),* , .. }).to_tokens(tokens);
            }
        }
    }
}
