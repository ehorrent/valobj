mod builder;
mod config;
mod getter;
mod value_object;

use crate::config::Config;
use proc_macro::TokenStream;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn value_object(attr: TokenStream, item: TokenStream) -> TokenStream {
    let cfg = match syn::parse2::<Config>(attr.into()) {
        Ok(cfg) => cfg,
        Err(e) => return e.into_compile_error().into(),
    };

    let input = parse_macro_input!(item as ItemStruct);
    value_object::expand(cfg, input)
}
