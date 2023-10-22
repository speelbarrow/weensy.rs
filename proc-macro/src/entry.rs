use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    LitInt, Pat, PatIdent, Result, Token,
};

pub enum Arg {
    Ident(PatIdent),
    Pins(Punctuated<PatIdent, Token![,]>),
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
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

fn parse_pin(input: ParseStream) -> Result<PatIdent> {
    let mutability = input.parse::<Token![mut]>().ok();
    let num = input.parse::<LitInt>()?;
    let ident = Ident::new(&("p".to_owned() + num.base10_digits()), num.span());
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
