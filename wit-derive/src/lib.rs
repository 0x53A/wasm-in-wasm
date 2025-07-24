use anyhow::{Context, Result};
use proc_macro::TokenStream;
use quote::quote;
use std::path::Path;
use syn::parse_macro_input;

mod codegen;
mod parser;

use crate::codegen::CodeGenerator;
use crate::parser::MacroInput;
use wit_parser::{Resolve, UnresolvedPackageGroup};

/// Generates host bindings for a WIT world.
///
/// # Example
///
/// ```rust,ignore
/// wit_derive::generate!({
///     world: "calculator",
///     path: "../wit",
/// });
/// ```
#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MacroInput);

    match generate_bindings(input) {
        Ok(tokens) => tokens,
        Err(e) => {
            let error_msg = format!("wit-derive error: {:?}", e);
            quote! {
                compile_error!(#error_msg);
            }
            .into()
        }
    }
}

fn generate_bindings(input: MacroInput) -> Result<TokenStream> {
    // Resolve the path relative to the crate root
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR not set")?;
    let wit_path = Path::new(&manifest_dir).join(&input.path);

    // Parse the WIT files
    let group = UnresolvedPackageGroup::parse_dir(&wit_path)
        .with_context(|| format!("Failed to parse WIT files in {}", wit_path.display()))?;

    let mut resolve = Resolve::new();
    let pkg_id = resolve.push(group.main, &group.source_map)?;
    for pkg in group.nested {
        resolve.push(pkg, &group.source_map)?;
    }

    // Find the world
    let world_id = resolve
        .select_world(pkg_id, Some(&input.world))
        .with_context(|| format!("World '{}' not found in package", input.world))?;

    let world = &resolve.worlds[world_id];
    let package = &resolve.packages[world.package.unwrap()];

    // Generate the code
    let mut generator = CodeGenerator::new(&resolve, package, world);
    let generated = generator.generate()?;

    Ok(generated.into())
}
