use quote::quote;

pub fn impl_getter(ident: &syn::Ident, ty: &syn::Type) -> proc_macro2::TokenStream {
    let is_string = matches!(ty, syn::Type::Path(type_path) if type_path.path.is_ident("String"));

    if is_string {
        quote! {
            impl #ident {
                pub fn get(&self) -> &str {
                    &self.0
                }
            }

            impl core::convert::AsRef<str> for #ident {
                fn as_ref(&self) -> &str {
                    &self.0
                }
            }

            impl std::ops::Deref for #ident {
                type Target = str;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    } else {
        quote! {
            impl #ident {
                pub fn get(&self) -> #ty {
                    self.0
                }
            }

            impl core::convert::AsRef<#ty> for #ident {
                fn as_ref(&self) -> &#ty {
                    &self.0
                }
            }

            impl std::ops::Deref for #ident {
                type Target = #ty;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    }
}
