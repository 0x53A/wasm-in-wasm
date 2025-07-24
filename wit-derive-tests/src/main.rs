#![feature(liballoc_internals)]

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

fn main() {

    let wasmi_engine = wasmi_runtime_layer::Engine::default();
    let engine = wasm_component_layer::Engine::new(wasmi_engine);
    let mut store = wasm_component_layer::Store::new(&engine, ());

    let component = wasm_component_layer::Component::new(&engine, &WASM_BLOB).unwrap();

    let mut linker = wasm_component_layer::Linker::default();

    let console_impl = MyConsoleImpl;
    let mut imports = crate::calculator::Imports {
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
