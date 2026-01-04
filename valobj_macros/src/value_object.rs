use crate::builder::impl_builder;
use crate::config::Config;
use ident_case::RenameRule;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, ItemStruct};
use crate::getter::impl_getter;

pub fn expand(cfg: Config, input: ItemStruct) -> TokenStream {
    if input.fields.len() != 1 {
        return syn::Error::new_spanned(
            input.ident,
            "`value_object` attribute requires a tuple struct with exactly one field",
        )
        .to_compile_error()
        .into();
    }

    // Extract the type of the single field
    let field: &Field = match &input.fields {
        syn::Fields::Unnamed(fields) => &fields.unnamed[0],
        _ => {
            return syn::Error::new_spanned(
                input.ident,
                "`value_object` attribute can only be applied to tuple structs with exactly one field",
            )
            .to_compile_error()
            .into();
        }
    };

    let ty = &field.ty;
    let vis = input.vis;
    let ident = input.ident;
    let attrs = input.attrs; // preserve other attributes

    let snake_case_ident = RenameRule::SnakeCase.apply_to_variant(ident.to_string());
    let mod_ident = format_ident!("__private_mod_{}", snake_case_ident);
    
    let builder_block = impl_builder(&cfg, &ident, ty);
    let getter_block = impl_getter(&ident, ty);
    let struct_block = quote! {
        mod #mod_ident {
            #(#attrs)*
            #vis struct #ident(#ty);

            #builder_block

            #getter_block
        }

        use #mod_ident::#ident;
    };

    TokenStream::from(struct_block)
}
