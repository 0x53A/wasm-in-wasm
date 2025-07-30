use syn::{
    braced,
    parse::{Parse, ParseStream},
    Ident, LitStr, Result, Token,
};

pub struct MacroInput {
    pub world: String,
    pub source: WitSource,
}

pub enum WitSource {
    Path(String),
    Inline(String),
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut world = None;
        let mut path = None;
        let mut inline = None;

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "world" => {
                    let lit: LitStr = content.parse()?;
                    world = Some(lit.value());
                }
                "path" => {
                    let lit: LitStr = content.parse()?;
                    path = Some(lit.value());
                }
                "inline" => {
                    let lit: LitStr = content.parse()?;
                    inline = Some(lit.value());
                }
                _ => {
                    return Err(syn::Error::new_spanned(key, "Unknown field"));
                }
            }

            if !content.is_empty() {
                content.parse::<Token![,]>()?;
            }
        }

        let world =
            world.ok_or_else(|| syn::Error::new(input.span(), "Missing required field 'world'"))?;

        // Ensure exactly one of path or inline is provided
        let source = match (path, inline) {
            (Some(path), None) => WitSource::Path(path),
            (None, Some(inline)) => WitSource::Inline(inline),
            (Some(_), Some(_)) => {
                return Err(syn::Error::new(
                    input.span(),
                    "Cannot specify both 'path' and 'inline' fields",
                ));
            }
            (None, None) => {
                return Err(syn::Error::new(
                    input.span(),
                    "Must specify either 'path' or 'inline' field",
                ));
            }
        };

        Ok(MacroInput { world, source })
    }
}
