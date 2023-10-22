use proc_macro2::Ident as Ident2;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitInt, Pat, PatIdent, Result, Token,
};

pub enum Arg {
    Ident(PatIdent),
    Pins(Punctuated<LitInt, Token![,]>),
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(Pat::Ident(ident)) = Pat::parse_single(input) {
            return Ok(Self::Ident(ident));
        } else if let Ok(ident) = input.parse::<Ident2>() {
            if ident == "pins" {
                let content;
                bracketed!(content in input);
                return Ok(Self::Pins(Punctuated::parse_separated_nonempty(&content)?));
            }
        }
        Err(input.error(format!("failed to parse argument: {}", input)))
    }
}
