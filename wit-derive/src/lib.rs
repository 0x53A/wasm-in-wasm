use anyhow::{Context, Result};
use proc_macro::TokenStream;
use quote::quote;
use std::path::Path;
use syn::parse_macro_input;

mod codegen;
mod parser;

use crate::codegen::{CodeGenerator, WitSourceContent};
use crate::parser::MacroInput;
use wit_parser::{Resolve, UnresolvedPackageGroup};

/// Generates host bindings for a WIT world.
///
/// # Example
///
/// Using a path to WIT files:
/// ```rust,ignore
/// wit_derive::generate!({
///     world: "calculator",
///     path: "../wit",
/// });
/// ```
///
/// Using inline WIT content:
/// ```rust,ignore
/// wit_derive::generate!({
///     world: "calculator",
///     inline: r#"
///         package example:calculator@0.1.0;
///         
///         interface math {
///             add: func(a: s32, b: s32) -> s32;
///             multiply: func(a: s32, b: s32) -> s32;
///         }
///         
///         interface console {
///             print: func(line: string);
///         }
///         
///         world calculator {
///             import console;
///             export math;
///         }
///     "#,
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
    // Parse the WIT files based on the source type and collect source content
    let (group, wit_source) = match &input.source {
        crate::parser::WitSource::Path(path) => {
            // Resolve the path relative to the crate root
            let manifest_dir =
                std::env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR not set")?;
            let wit_path = Path::new(&manifest_dir).join(path);

            let group = UnresolvedPackageGroup::parse_dir(&wit_path)
                .with_context(|| format!("Failed to parse WIT files in {}", wit_path.display()))?;

            // Collect all WIT files and their contents
            let mut files = Vec::new();
            collect_wit_files(&wit_path, &mut files)?;

            let wit_source = WitSourceContent::Files(files);
            (group, wit_source)
        }
        crate::parser::WitSource::Inline(content) => {
            let group = UnresolvedPackageGroup::parse("inline.wit", content)
                .with_context(|| "Failed to parse inline WIT content")?;

            let wit_source = WitSourceContent::Inline(content.clone());
            (group, wit_source)
        }
    };

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
    let mut generator = CodeGenerator::new(&resolve, package, world, wit_source);
    let generated = generator.generate()?;

    Ok(generated.into())
}

fn collect_wit_files(dir: &Path, files: &mut Vec<(String, String)>) -> Result<()> {
    use std::fs;

    if dir.is_file() {
        // If it's a single file, read it
        if let Some(ext) = dir.extension() {
            if ext == "wit" {
                let content = fs::read_to_string(dir)
                    .with_context(|| format!("Failed to read WIT file: {}", dir.display()))?;
                let filename = dir
                    .file_name()
                    .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?
                    .to_string_lossy()
                    .to_string();
                files.push((filename, content));
            }
        }
        return Ok(());
    }

    if !dir.is_dir() {
        return Err(anyhow::anyhow!(
            "Path is neither a file nor a directory: {}",
            dir.display()
        ));
    }

    // Recursively collect all .wit files
    for entry in
        fs::read_dir(dir).with_context(|| format!("Failed to read directory: {}", dir.display()))?
    {
        let entry = entry.with_context(|| "Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            collect_wit_files(&path, files)?;
        } else if let Some(ext) = path.extension() {
            if ext == "wit" {
                let content = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read WIT file: {}", path.display()))?;
                let filename = path
                    .file_name()
                    .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?
                    .to_string_lossy()
                    .to_string();
                files.push((filename, content));
            }
        }
    }

    Ok(())
}
