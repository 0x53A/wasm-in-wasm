use wit_parser::ManglingAndAbi;

use anyhow::Result;

wit_derive::generate!({
    world: "calculator",
    path: "../wit",
});

pub struct MyConsoleImpl;

impl crate::calculator::console for MyConsoleImpl {
    fn print(&mut self, message: String) {
        println!("Console log: {}", message);
    }
}

const WASM_BLOB: &[u8] =
    include_bytes!(r#"../../demo-component/target/wasm32-wasip1/release/demo_component.wasm"#);

// // Add this helper function to see what the expected module looks like
// pub fn generate_dummy_module() -> Result<Vec<u8>> {
//     let mut resolve = wit_parser::Resolve::default();
//     let pkg_id = resolve.push_str("calculator.wit", r#"
//         package example:calculator@0.1.0;

//         interface math {
//             add: func(a: s32, b: s32) -> s32;
//             multiply: func(a: s32, b: s32) -> s32;
//         }

//         interface console {
//             print: func(line: string);
//         }

//         world calculator {
//             import console;
//             export math;
//         }
//     "#)?;

//     let world_id = resolve.select_world(pkg_id, Some("calculator"))?;

//     let dummy = wit_component::dummy_module(&resolve, world_id, ManglingAndAbi::Standard32);

//     // Print the WAT to see the expected structure
//     println!("{}", wasmprinter::print_bytes(&dummy)?);

//     Ok(dummy)
// }

fn main() {
    let wasmi_engine = wasmi_runtime_layer::Engine::default();
    let engine = wasm_component_layer::Engine::new(wasmi_engine);
    let store = wasm_component_layer::Store::new(&engine, ());

    let component = wasm_component_layer::Component::new(&engine, &WASM_BLOB).unwrap();

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
}
