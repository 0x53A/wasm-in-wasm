// Example demonstrating the inline WIT functionality

use anyhow::Result;
use wit_component::ComponentEncoder;

wit_derive::generate!({
    world: "calculator",
    inline: r#"
        package example:calculator@0.1.0;

        interface math {
            add: func(a: s32, b: s32) -> s32;
            multiply: func(a: s32, b: s32) -> s32;
        }

        interface console {
            print: func(line: string);
        }

        world calculator {
            import console;
            export math;
        }
    "#,
});

// Client
// ----------------------------------------------------------

pub fn create_inline_wat_component() -> Result<Vec<u8>> {
    // First, parse the WIT to get the resolve and world
    let mut resolve = wit_parser::Resolve::default();
    let wit_content = calculator::wit::INLINE;
    let pkg_id = resolve.push_str("calculator.wit", wit_content)?;

    let world_id = resolve.select_world(pkg_id, Some("calculator"))?;

    // WAT implementation of the math interface
    let wat_source = r#"
(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;4;) (func))
  (import "cm32p2|example:calculator/console@0.1" "print" (func (;0;) (type 0)))
  (memory (;0;) 0)
  (export "cm32p2|example:calculator/math@0.1|add" (func 1))
  (export "cm32p2|example:calculator/math@0.1|add_post" (func 2))
  (export "cm32p2|example:calculator/math@0.1|multiply" (func 3))
  (export "cm32p2|example:calculator/math@0.1|multiply_post" (func 4))
  (export "cm32p2_memory" (memory 0))
  (export "cm32p2_realloc" (func 5))
  (export "cm32p2_initialize" (func 6))
  (func (;1;) (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )
  (func (;2;) (type 2) (param i32))
  (func (;3;) (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.mul
  )
  (func (;4;) (type 2) (param i32))
  (func (;5;) (type 3) (param i32 i32 i32 i32) (result i32)
    unreachable
  )
  (func (;6;) (type 4))
)
"#;

    // Compile WAT to WASM bytes (core module)
    let mut core_wasm = wat::parse_str(wat_source)?;

    // Embed component metadata - this is the equivalent of `wasm-tools component embed`
    wit_component::embed_component_metadata(
        &mut core_wasm,
        &resolve,
        world_id,
        wit_component::StringEncoding::UTF8,
    )?;

    // Now create the component using ComponentEncoder
    let component_bytes = wit_component::ComponentEncoder::default()
        .module(&core_wasm)?
        .validate(true)
        .encode()?;

    Ok(component_bytes)
}

// Host
// ----------------------------------------------------------

pub struct MyConsoleImpl;

impl crate::calculator::console for MyConsoleImpl {
    fn print(&mut self, message: String) {
        println!("[WASM]: {}", message);
    }
}

// Test
// ----------------------------------------------------------

#[test]
fn test_inline_wit() -> Result<()> {
    let wasm_blob = create_inline_wat_component().unwrap();

    let wasmi_engine = wasmi_runtime_layer::Engine::default();
    let engine = wasm_component_layer::Engine::new(wasmi_engine);
    let store = wasm_component_layer::Store::new(&engine, ());

    let component = wasm_component_layer::Component::new(&engine, &wasm_blob).unwrap();

    let console_impl = MyConsoleImpl;
    let imports = crate::calculator::Imports {
        console: Box::new(console_impl),
    };

    let mut instance = crate::calculator::instantiate(store, &component, imports).unwrap();

    let add_result = instance.math.add(7, 8);
    println!("Result of 7 + 8 = {}", add_result);
    assert!(
        add_result == 15,
        "Expected 7 + 8 to equal 15, got {}",
        add_result
    );
    Ok(())
}
