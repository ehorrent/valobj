use crate::config::Config;
use quote::quote;

pub fn impl_builder(cfg: &Config, ident: &syn::Ident, ty: &syn::Type) -> proc_macro2::TokenStream {
    let normalize_block = if cfg.normalize {
        quote! { #ident::normalize(value); }
    } else {
        quote! { value; }
    };

    let where_clause = match (cfg.validate, cfg.normalize) {
        (true, true) => quote! { where #ident: ::valobj::Validate<#ty> + ::valobj::Normalize<#ty> },
        (true, false) => quote! { where #ident: ::valobj::Validate<#ty> },
        (false, true) => quote! { where #ident: ::valobj::Normalize<#ty> },
        (false, false) => quote! {},
    };

    if cfg.validate {
        // TryFrom implementation
        quote! {
            use ::valobj::{Normalize, Validate};

            impl ::core::convert::TryFrom<#ty> for #ident #where_clause {
                type Error = ::valobj::Error;
                fn try_from(value: #ty) -> Result<Self, Self::Error> {
                    let inner_value = #normalize_block
                    #ident::validate(&inner_value)?;
                    Ok(#ident(inner_value))
                }
            }
        }
    } else {
        // TODO Error if Validate is implemented

        // From implementation
        quote! {
            use ::valobj::Normalize;

            impl ::core::convert::From<#ty> for #ident #where_clause {
                fn from(value: #ty) -> Self {
                    let inner_value = #normalize_block
                    #ident(inner_value)
                }
            }
        }
    }
}
