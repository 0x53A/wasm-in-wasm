use syn::{
    braced,
    parse::{Parse, ParseStream},
    Ident, LitStr, Result, Token,
};

pub struct MacroInput {
    pub world: String,
    pub path: String,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut world = None;
        let mut path = None;

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

        let path =
            path.ok_or_else(|| syn::Error::new(input.span(), "Missing required field 'path'"))?;

        Ok(MacroInput { world, path })
    }
}
