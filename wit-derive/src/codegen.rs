use anyhow::{Context, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use wit_parser::{
    Function, Interface, Package, Resolve, Type, TypeDefKind, World, WorldItem, WorldKey,
};

pub struct CodeGenerator<'a> {
    resolve: &'a Resolve,
    package: &'a Package,
    world: &'a World,
    wit_source: WitSourceContent,
}

pub enum WitSourceContent {
    Files(Vec<(String, String)>), // Vec<(filename, content)>
    Inline(String),
}

impl<'a> CodeGenerator<'a> {
    pub fn new(resolve: &'a Resolve, package: &'a Package, world: &'a World, wit_source: WitSourceContent) -> Self {
        Self {
            resolve,
            package,
            world,
            wit_source,
        }
    }

    pub fn generate(&mut self) -> Result<TokenStream> {
        let package_name = self.package.name.name.replace('-', "_");
        let mod_name = format_ident!("{}", package_name);

        let mut import_traits = Vec::new();
        let mut export_traits = Vec::new();
        let mut import_impls = Vec::new();
        let mut export_impls = Vec::new();
        let mut import_fields = Vec::new();
        let mut export_fields = Vec::new();

        // Process imports
        for (key, item) in &self.world.imports {
            match item {
                WorldItem::Interface { id, .. } => {
                    let interface = &self.resolve.interfaces[*id];
                    let (trait_def, impl_def, field_name) =
                        self.generate_import_interface(key, interface)?;
                    import_traits.push(trait_def);
                    import_impls.push(impl_def);
                    import_fields.push(field_name);
                }
                WorldItem::Function(_) => {
                    // Handle top-level functions if needed
                }
                WorldItem::Type(_) => {
                    // Handle top-level types if needed
                }
            }
        }

        // Process exports
        for (key, item) in &self.world.exports {
            match item {
                WorldItem::Interface { id, .. } => {
                    let interface = &self.resolve.interfaces[*id];
                    let (trait_def, impl_def, field_name) =
                        self.generate_export_interface(key, interface)?;
                    export_traits.push(trait_def);
                    export_impls.push(impl_def);
                    export_fields.push(field_name);
                }
                WorldItem::Function(_) => {
                    // Handle top-level functions if needed
                }
                WorldItem::Type(_) => {
                    // Handle top-level types if needed
                }
            }
        }

        let instantiate_fn = self.generate_instantiate_function(&import_fields, &export_fields)?;
        let wit_module = self.generate_wit_module()?;

        let result = quote! {
            pub mod #mod_name {
                use std::sync::{Arc, Mutex};
                use wasm_component_layer::{
                    self,
                    Component,
                    Func,
                    FuncType,
                    InterfaceIdentifier,
                    Linker,
                    PackageIdentifier,
                    PackageName,
                    Store,
                    TypedFunc,
                    Value,
                    ValueType,
                };
                use wasm_runtime_layer::backend::WasmEngine;
                use anyhow::Result;

                #wit_module

                #(#import_traits)*
                #(#export_traits)*

                pub struct Imports {
                    #(#import_fields)*
                }

                pub struct Exports<T, E>
                where
                    E: wasm_runtime_layer::backend::WasmEngine,
                {
                    #(#export_fields),*,
                    // Add phantom data to use the generic parameters
                    _phantom: std::marker::PhantomData<(T, E)>,
                }

                #(#import_impls)*
                #(#export_impls)*

                #instantiate_fn
            }
        };

        Ok(result)
    }

    fn generate_wit_module(&self) -> Result<TokenStream> {
        let wit_content = match &self.wit_source {
            WitSourceContent::Files(files) => {
                let file_constants = files.iter().map(|(filename, content)| {
                    let const_name = format_ident!("{}", filename.replace('.', "_").replace('-', "_").to_uppercase());
                    quote! {
                        pub const #const_name: &'static str = #content;
                    }
                });
                
                quote! {
                    pub mod wit {
                        pub mod files {
                            #(#file_constants)*
                        }
                    }
                }
            }
            WitSourceContent::Inline(content) => {
                quote! {
                    pub mod wit {
                        pub const INLINE: &'static str = #content;
                    }
                }
            }
        };

        Ok(wit_content)
    }

    fn generate_import_interface(
        &self,
        key: &WorldKey,
        interface: &Interface,
    ) -> Result<(TokenStream, TokenStream, TokenStream)> {
        let interface_name = self.get_interface_name(key);
        let trait_name = format_ident!("{}", interface_name);
        let field_name = format_ident!("{}", interface_name.to_lowercase());

        let mut trait_methods = Vec::new();
        for (_, function) in &interface.functions {
            let method = self.generate_trait_method(function)?;
            trait_methods.push(method);
        }

        let trait_def = quote! {
            pub trait #trait_name {
                #(#trait_methods)*
            }
        };

        let impl_def = quote! {
            // Import implementation will be handled in instantiate function
        };

        let field_def = quote! {
            pub #field_name: Box<dyn #trait_name + Send>
        };

        Ok((trait_def, impl_def, field_def))
    }

    fn generate_export_interface(
        &self,
        key: &WorldKey,
        interface: &Interface,
    ) -> Result<(TokenStream, TokenStream, TokenStream)> {
        let interface_name = self.get_interface_name(key);
        let trait_name = format_ident!("{}", interface_name);
        let impl_name = format_ident!("{}Impl", interface_name);
        let field_name = format_ident!("{}", interface_name.to_lowercase());

        let mut trait_methods = Vec::new();
        let mut impl_methods = Vec::new();
        let mut struct_fields = Vec::new();

        for (func_name, function) in &interface.functions {
            let trait_method = self.generate_trait_method(function)?;
            let impl_method = self.generate_impl_method(func_name, function)?;
            let func_field = format_ident!("{}", func_name);

            trait_methods.push(trait_method);
            impl_methods.push(impl_method);

            let param_types = self.get_function_param_types(function)?;
            let return_type = self.get_function_return_type(function)?;

            struct_fields.push(quote! {
                #func_field: wasm_component_layer::TypedFunc<#param_types, #return_type>
            });
        }

        let trait_def = quote! {
            pub trait #trait_name {
                #(#trait_methods)*
            }
        };

        let impl_def = quote! {
            struct #impl_name<T, E>
            where
                E: wasm_runtime_layer::backend::WasmEngine,
            {
                store: std::sync::Arc<std::sync::Mutex<wasm_component_layer::Store<T, E>>>,
                #(#struct_fields,)*
            }

            impl<T, E> #trait_name for #impl_name<T, E>
            where
                E: wasm_runtime_layer::backend::WasmEngine,
            {
                #(#impl_methods)*
            }
        };

        let field_def = quote! {
            pub #field_name: Box<dyn #trait_name>
        };

        Ok((trait_def, impl_def, field_def))
    }

    fn generate_trait_method(&self, function: &Function) -> Result<TokenStream> {
        let func_name = format_ident!("{}", function.name);
        let params = self.generate_function_params(function)?;
        let return_type = self.generate_return_type(function)?;

        Ok(quote! {
            fn #func_name(&mut self, #(#params),*) #return_type;
        })
    }

    fn generate_impl_method(&self, func_name: &str, function: &Function) -> Result<TokenStream> {
        let method_name = format_ident!("{}", function.name);
        let func_field = format_ident!("{}", func_name);
        let params = self.generate_function_params(function)?;
        let return_type = self.generate_return_type(function)?;
        let param_names: Vec<_> = function
            .params
            .iter()
            .map(|(name, _)| format_ident!("{}", name))
            .collect();

        let param_tuple = if param_names.len() == 1 {
            quote! { #(#param_names)* }
        } else {
            quote! { (#(#param_names),*) }
        };

        Ok(quote! {
            fn #method_name(&mut self, #(#params),*) #return_type {
                let mut store_guard = self.store.lock().unwrap();
                self.#func_field.call(&mut *store_guard, #param_tuple).unwrap()
            }
        })
    }

    fn generate_function_params(&self, function: &Function) -> Result<Vec<TokenStream>> {
        let mut params = Vec::new();
        for (name, ty) in &function.params {
            let param_name = format_ident!("{}", name);
            let param_type = self.type_to_rust_type(ty)?;
            params.push(quote! { #param_name: #param_type });
        }
        Ok(params)
    }

    fn generate_return_type(&self, function: &Function) -> Result<TokenStream> {
        match &function.result {
            Some(ty) => {
                let rust_type = self.type_to_rust_type(ty)?;
                Ok(quote! { -> #rust_type })
            }
            None => Ok(TokenStream::new()),
        }
    }

    fn get_function_param_types(&self, function: &Function) -> Result<TokenStream> {
        let param_types: Result<Vec<_>> = function
            .params
            .iter()
            .map(|(_, ty)| self.type_to_rust_type(ty))
            .collect();
        let param_types = param_types?;

        if param_types.len() == 1 {
            Ok(quote! { #(#param_types)* })
        } else {
            Ok(quote! { (#(#param_types),*) })
        }
    }

    fn get_function_return_type(&self, function: &Function) -> Result<TokenStream> {
        match &function.result {
            Some(ty) => self.type_to_rust_type(ty),
            None => Ok(quote! { () }),
        }
    }

    fn type_to_rust_type(&self, ty: &Type) -> Result<TokenStream> {
        match ty {
            Type::Bool => Ok(quote! { bool }),
            Type::U8 => Ok(quote! { u8 }),
            Type::U16 => Ok(quote! { u16 }),
            Type::U32 => Ok(quote! { u32 }),
            Type::U64 => Ok(quote! { u64 }),
            Type::S8 => Ok(quote! { i8 }),
            Type::S16 => Ok(quote! { i16 }),
            Type::S32 => Ok(quote! { i32 }),
            Type::S64 => Ok(quote! { i64 }),
            Type::F32 => Ok(quote! { f32 }),
            Type::F64 => Ok(quote! { f64 }),
            Type::Char => Ok(quote! { char }),
            Type::String => Ok(quote! { String }),
            Type::ErrorContext => {
                // Handle error context type
                Ok(quote! { anyhow::Error })
            }
            Type::Id(type_id) => {
                let type_def = &self.resolve.types[*type_id];
                match &type_def.kind {
                    TypeDefKind::List(element_type) => {
                        let element_rust_type = self.type_to_rust_type(element_type)?;
                        Ok(quote! { Vec<#element_rust_type> })
                    }
                    TypeDefKind::Option(some_type) => {
                        let some_rust_type = self.type_to_rust_type(some_type)?;
                        Ok(quote! { Option<#some_rust_type> })
                    }
                    _ => {
                        // For now, fallback to a generic type name
                        let type_name = type_def
                            .name
                            .as_ref()
                            .context("Type definition missing name")?;
                        let rust_name = format_ident!("{}", type_name);
                        Ok(quote! { #rust_name })
                    }
                }
            }
        }
    }

    fn type_to_value_type(&self, ty: &Type) -> Result<TokenStream> {
        match ty {
            Type::Bool => Ok(quote! { wasm_component_layer::ValueType::Bool }),
            Type::U8 => Ok(quote! { wasm_component_layer::ValueType::U8 }),
            Type::U16 => Ok(quote! { wasm_component_layer::ValueType::U16 }),
            Type::U32 => Ok(quote! { wasm_component_layer::ValueType::U32 }),
            Type::U64 => Ok(quote! { wasm_component_layer::ValueType::U64 }),
            Type::S8 => Ok(quote! { wasm_component_layer::ValueType::S8 }),
            Type::S16 => Ok(quote! { wasm_component_layer::ValueType::S16 }),
            Type::S32 => Ok(quote! { wasm_component_layer::ValueType::S32 }),
            Type::S64 => Ok(quote! { wasm_component_layer::ValueType::S64 }),
            Type::F32 => Ok(quote! { wasm_component_layer::ValueType::F32 }),
            Type::F64 => Ok(quote! { wasm_component_layer::ValueType::F64 }),
            Type::Char => Ok(quote! { wasm_component_layer::ValueType::Char }),
            Type::String => Ok(quote! { wasm_component_layer::ValueType::String }),
            Type::Id(type_id) => {
                let type_def = &self.resolve.types[*type_id];
                match &type_def.kind {
                    TypeDefKind::List(element_type) => {
                        let element_value_type = self.type_to_value_type(element_type)?;
                        Ok(
                            quote! { wasm_component_layer::ValueType::List(Box::new(#element_value_type)) },
                        )
                    }
                    TypeDefKind::Option(some_type) => {
                        let some_value_type = self.type_to_value_type(some_type)?;
                        Ok(
                            quote! { wasm_component_layer::ValueType::Option(Box::new(#some_value_type)) },
                        )
                    }
                    _ => {
                        // For complex types like records, resources, etc.
                        Ok(quote! { wasm_component_layer::ValueType::Record })
                    }
                }
            }
            _ => {
                // For unsupported types, fall back to a generic approach
                Ok(quote! { wasm_component_layer::ValueType::Record })
            }
        }
    }

    fn generate_param_extraction(
        &self,
        param_type: &Type,
        name: &str,
        func_name: &str,
        i_literal: &proc_macro2::Literal,
    ) -> Result<TokenStream> {
        let param_name = format_ident!("{}", name);

        match param_type {
            Type::String => {
                // Handle Arc<str> to String conversion for ergonomic API
                Ok(quote! {
                    let #param_name = if let Some(wasm_component_layer::Value::String(val)) =
                        params.get(#i_literal).take()
                    {
                        val.to_string()
                    } else {
                        panic!("Invalid parameter type for parameter '{}' in function '{}', expected String", #name, #func_name);
                    };
                })
            }
            Type::Bool => Ok(quote! {
                let #param_name = if let Some(wasm_component_layer::Value::Bool(val)) =
                    params.get(#i_literal).take()
                {
                    val
                } else {
                    panic!("Invalid parameter type for parameter '{}' in function '{}', expected Bool", #name, #func_name);
                };
            }),
            Type::S8 => Ok(quote! {
                let #param_name = if let Some(wasm_component_layer::Value::S8(val)) =
                    params.get(#i_literal).take()
                {
                    val
                } else {
                    panic!("Invalid parameter type for parameter '{}' in function '{}', expected S8", #name, #func_name);
                };
            }),
            Type::U8 => Ok(quote! {
                let #param_name = if let Some(wasm_component_layer::Value::U8(val)) =
                    params.get(#i_literal).take()
                {
                    val
                } else {
                    panic!("Invalid parameter type for parameter '{}' in function '{}', expected U8", #name, #func_name);
                };
            }),
            Type::S16 => Ok(quote! {
                let #param_name = if let Some(wasm_component_layer::Value::S16(val)) =
                    params.get(#i_literal).take()
                {
                    val
                } else {
                    panic!("Invalid parameter type for parameter '{}' in function '{}', expected S16", #name, #func_name);
                };
            }),
            Type::U16 => Ok(quote! {
                let #param_name = if let Some(wasm_component_layer::Value::U16(val)) =
                    params.get(#i_literal).take()
                {
                    val
                } else {
                    panic!("Invalid parameter type for parameter '{}' in function '{}', expected U16", #name, #func_name);
                };
            }),
            Type::S32 => Ok(quote! {
                let #param_name = if let Some(wasm_component_layer::Value::S32(val)) =
                    params.get(#i_literal).take()
                {
                    val
                } else {
                    panic!("Invalid parameter type for parameter '{}' in function '{}', expected S32", #name, #func_name);
                };
            }),
            Type::U32 => Ok(quote! {
                let #param_name = if let Some(wasm_component_layer::Value::U32(val)) =
                    params.get(#i_literal).take()
                {
                    val
                } else {
                    panic!("Invalid parameter type for parameter '{}' in function '{}', expected U32", #name, #func_name);
                };
            }),
            Type::S64 => Ok(quote! {
                let #param_name = if let Some(wasm_component_layer::Value::S64(val)) =
                    params.get(#i_literal).take()
                {
                    val
                } else {
                    panic!("Invalid parameter type for parameter '{}' in function '{}', expected S64", #name, #func_name);
                };
            }),
            Type::U64 => Ok(quote! {
                let #param_name = if let Some(wasm_component_layer::Value::U64(val)) =
                    params.get(#i_literal).take()
                {
                    val
                } else {
                    panic!("Invalid parameter type for parameter '{}' in function '{}', expected U64", #name, #func_name);
                };
            }),
            Type::F32 => Ok(quote! {
                let #param_name = if let Some(wasm_component_layer::Value::F32(val)) =
                    params.get(#i_literal).take()
                {
                    val
                } else {
                    panic!("Invalid parameter type for parameter '{}' in function '{}', expected F32", #name, #func_name);
                };
            }),
            Type::F64 => Ok(quote! {
                let #param_name = if let Some(wasm_component_layer::Value::F64(val)) =
                    params.get(#i_literal).take()
                {
                    val
                } else {
                    panic!("Invalid parameter type for parameter '{}' in function '{}', expected F64", #name, #func_name);
                };
            }),
            Type::Char => Ok(quote! {
                let #param_name = if let Some(wasm_component_layer::Value::Char(val)) =
                    params.get(#i_literal).take()
                {
                    val
                } else {
                    panic!("Invalid parameter type for parameter '{}' in function '{}', expected Char", #name, #func_name);
                };
            }),
            Type::Id(type_id) => {
                let type_def = &self.resolve.types[*type_id];
                match &type_def.kind {
                    TypeDefKind::List(_) => {
                        // Placeholder for List handling
                        Ok(quote! {
                            let #param_name = if let Some(wasm_component_layer::Value::List(val)) =
                                params.get(#i_literal).take()
                            {
                                // TODO: Implement proper List conversion
                                unimplemented!("List parameter conversion not yet implemented for parameter '{}' in function '{}'", #name, #func_name)
                            } else {
                                panic!("Invalid parameter type for parameter '{}' in function '{}', expected List", #name, #func_name);
                            };
                        })
                    }
                    TypeDefKind::Option(_) => {
                        // Placeholder for Option handling
                        Ok(quote! {
                            let #param_name = if let Some(wasm_component_layer::Value::Option(val)) =
                                params.get(#i_literal).take()
                            {
                                // TODO: Implement proper Option conversion
                                unimplemented!("Option parameter conversion not yet implemented for parameter '{}' in function '{}'", #name, #func_name)
                            } else {
                                panic!("Invalid parameter type for parameter '{}' in function '{}', expected Option", #name, #func_name);
                            };
                        })
                    }
                    _ => {
                        // Placeholder for other complex types (Record, Variant, etc.)
                        Ok(quote! {
                            let #param_name = if let Some(wasm_component_layer::Value::Record(val)) =
                                params.get(#i_literal).take()
                            {
                                // TODO: Implement proper Record/complex type conversion
                                unimplemented!("Complex type parameter conversion not yet implemented for parameter '{}' in function '{}'", #name, #func_name)
                            } else {
                                panic!("Invalid parameter type for parameter '{}' in function '{}', expected complex type", #name, #func_name);
                            };
                        })
                    }
                }
            }
            _ => {
                // Fallback for any other unsupported types
                Ok(quote! {
                    let #param_name = {
                        // TODO: Implement conversion for unsupported type
                        unimplemented!("Unsupported parameter type conversion for parameter '{}' in function '{}'", #name, #func_name)
                    };
                })
            }
        }
    }

    fn get_interface_name(&self, key: &WorldKey) -> String {
        match key {
            WorldKey::Name(name) => name.clone(),
            WorldKey::Interface(interface_id) => {
                // Try to get a reasonable name from the interface
                self.resolve.interfaces[*interface_id]
                    .name
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| format!("interface_{}", interface_id.index()))
            }
        }
    }

    fn generate_instantiate_function(
        &self,
        _import_fields: &[TokenStream],
        _export_fields: &[TokenStream],
    ) -> Result<TokenStream> {
        let mut import_definitions = Vec::new();
        let mut export_initializations = Vec::new();

        // Process imports
        for (key, item) in &self.world.imports {
            if let WorldItem::Interface { id, .. } = item {
                let interface = &self.resolve.interfaces[*id];
                let interface_name = self.get_interface_name(key);
                let field_name = format_ident!("{}", interface_name.to_lowercase());

                // Extract the parts we need for the generated code
                let namespace = &self.package.name.namespace;
                let pkg_name = &self.package.name.name;
                let version = if let Some(version) = &self.package.name.version {
                    let major = version.major;
                    let minor = version.minor;
                    let patch = version.patch;
                    quote! {
                        Some(semver::Version {
                            major: #major,
                            minor: #minor,
                            patch: #patch,
                            pre: semver::Prerelease::EMPTY,
                            build: semver::BuildMetadata::EMPTY,
                        })
                    }
                } else {
                    quote! { None }
                };

                let mut function_definitions = Vec::new();

                for (func_name, function) in &interface.functions {
                    let func_ident = format_ident!("{}", func_name);
                    let param_names: Vec<_> = function
                        .params
                        .iter()
                        .map(|(name, _)| format_ident!("{}", name))
                        .collect();

                    // Create a series of statements to extract parameters, not using repetition
                    let mut extract_params_stmts = Vec::new();
                    for (i, (name, param_type)) in function.params.iter().enumerate() {
                        let i_literal = proc_macro2::Literal::usize_unsuffixed(i);
                        let extraction_code = self
                            .generate_param_extraction(param_type, name, func_name, &i_literal)?;

                        extract_params_stmts.push(extraction_code);
                    } // Pass the field_name (like "console") to the closure for better context

                    // Generate parameter types for the function signature
                    let param_types: Result<Vec<_>> = function
                        .params
                        .iter()
                        .map(|(_, ty)| self.type_to_value_type(ty))
                        .collect();
                    let param_types = param_types?;

                    // Generate return type for the function signature
                    let return_types = match &function.result {
                        Some(ret_ty) => {
                            let ret_type = self.type_to_value_type(ret_ty)?;
                            vec![ret_type]
                        }
                        None => vec![],
                    };

                    function_definitions.push(quote! {
                        let field_name_clone = #field_name.clone();
                        interface_instance
                            .define_func(
                                #func_name,
                                wasm_component_layer::Func::new(
                                    &mut store,
                                    wasm_component_layer::FuncType::new(
                                        vec![#(#param_types),*],
                                        vec![#(#return_types),*],
                                    ),
                                    move |_caller, params, results| {
                                        // Extract parameters
                                        #(#extract_params_stmts)*

                                        // Call the function
                                        let mut guard = field_name_clone.lock().unwrap();
                                        let result = guard.#func_ident(#(#param_names),*);

                                        // Handle return value based on function signature
                                        if results.len() > 0 {
                                            // Function has a return value, set it in results
                                            // For now, we handle simple cases; a complete implementation
                                            // would need to convert the result to the appropriate Value type
                                            // based on the function's return type
                                            // This is a placeholder - specific conversion would depend on the return type
                                        }

                                        Ok(())
                                    },
                                ),
                            )?;
                    });
                }

                import_definitions.push(quote! {
                    // Use the parsed values directly (no runtime parsing needed)
                    let namespace = #namespace;
                    let pkg_name = #pkg_name;
                    let interface_name = #interface_name;

                    let pkg_id = wasm_component_layer::PackageIdentifier::new(
                        wasm_component_layer::PackageName::new(namespace, pkg_name),
                        #version,
                    );

                    let interface_id = wasm_component_layer::InterfaceIdentifier::new(pkg_id, interface_name);

                    let interface_instance = linker.define_instance(interface_id)?;

                    let #field_name = std::sync::Arc::new(std::sync::Mutex::new(imports.#field_name));

                    #(#function_definitions)*
                });
            }
        }

        // Process exports
        for (key, item) in &self.world.exports {
            if let WorldItem::Interface { id, .. } = item {
                let interface = &self.resolve.interfaces[*id];
                let interface_name = self.get_interface_name(key);
                let field_name = format_ident!("{}", interface_name.to_lowercase());
                let impl_name = format_ident!("{}Impl", interface_name);

                // Extract the parts we need for the generated code
                let namespace = &self.package.name.namespace;
                let pkg_name = &self.package.name.name;
                let package_version = &self.package.name.version;

                // Generate field initializations dynamically based on the interface functions
                let mut field_inits = Vec::new();
                for (func_name, _function) in &interface.functions {
                    let func_field = format_ident!("{}", func_name);
                    field_inits.push(quote! {
                        #func_field: interface_instance.func(#func_name).unwrap().typed().unwrap()
                    });
                }

                // Generate the version part
                let version_expr = if let Some(version) = package_version {
                    let major = version.major;
                    let minor = version.minor;
                    let patch = version.patch;
                    quote! {
                        Some(semver::Version {
                            major: #major,
                            minor: #minor,
                            patch: #patch,
                            pre: semver::Prerelease::EMPTY,
                            build: semver::BuildMetadata::EMPTY,
                        })
                    }
                } else {
                    quote! { None }
                };

                export_initializations.push(quote! {
                    // Use the parsed values directly
                    let namespace = #namespace;
                    let pkg_name = #pkg_name;
                    let interface_name = #interface_name;

                    let interface_instance = instance
                        .exports()
                        .instance(&wasm_component_layer::InterfaceIdentifier::new(
                            wasm_component_layer::PackageIdentifier::new(
                                wasm_component_layer::PackageName::new(namespace, pkg_name),
                                #version_expr,
                            ),
                            interface_name,
                        ))
                        .unwrap();

                    // Create the implementation
                    let store_arc = std::sync::Arc::new(std::sync::Mutex::new(store));
                    let #field_name = #impl_name {
                        store: store_arc,
                        #(#field_inits),*
                    };
                });
            }
        }

        // Create proper field initializers from field names
        let mut export_fields_init = Vec::new();

        for (key, item) in &self.world.exports {
            if let WorldItem::Interface { .. } = item {
                let interface_name = self.get_interface_name(key);
                let field_name = format_ident!("{}", interface_name.to_lowercase());

                export_fields_init.push(quote! {
                    #field_name: Box::new(#field_name)
                });
            }
        }

        Ok(quote! {
            pub fn instantiate<T, E>(
                mut store: wasm_component_layer::Store<T, E>,
                component: &wasm_component_layer::Component,
                imports: Imports,
            ) -> anyhow::Result<Exports<T, E>>
            where
                E: wasm_runtime_layer::backend::WasmEngine,
                T: 'static
            {
                let mut linker = wasm_component_layer::Linker::default();

                // Set up imports in the linker
                #(#import_definitions)*

                // Instantiate the component
                let instance = linker.instantiate(&mut store, component)?;

                // Extract exports and create implementations
                #(#export_initializations)*

                // Return the exports struct
                Ok(Exports {
                    #(#export_fields_init),*,
                    _phantom: std::marker::PhantomData
                })
            }
        })
    }
}
