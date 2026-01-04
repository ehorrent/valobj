use syn::Token;
use syn::parse::Parse;

#[derive(Default)]
pub struct Config {
    pub normalize: bool,
    pub validate: bool,
}

impl Parse for Config {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut cfg = Config::default();

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                if ident == "Normalize" {
                    cfg.normalize = true;
                } else if ident == "Validate" {
                    cfg.validate = true;
                } else {
                    return Err(syn::Error::new_spanned(
                        ident,
                        "Unknown flag, expected one of: Normalize, Validate",
                    ));
                }
            } else {
                return Err(lookahead.error());
            }

            // Consume optional comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(cfg)
    }
}
