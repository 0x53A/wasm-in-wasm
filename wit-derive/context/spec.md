I'd like to create a proc-macro, in rust, that generates the host bindings for a wit file. It should use ``wasm_component_layer`` as a backend (which itself is a generic layer over multiple wasm engines like wasmtime or wasmi). Taking this wit file as an example, it should generate:

```rust
// package
pub mod calculator {
  pub trait Math {
    fn add(&mut self, a: i32, b: i32) -> i32;
    fn multiply(&mut self, a: i32, b: i32) -> i32;
  }

  pub trait Console {
    pub fn print(&mut self, line: String);
  }


  pub struct Imports {
    pub console: Box<dyn Console>;
  }

  pub struct Exports {
    pub math: Box<dyn Math>;
  }
  pub fn instantiate(store: wasm_component_layer::Store, component: &wasm_component_layer::Component, imports: Imports) -> Exports {
    // ...
  }
}
```

```rust
// user code:

my_macro!({
        world: "calculator",
        path: "../wit",
});

// get the wasm blob from somewhere/anwhere
let wasm_blob: &[u8] = // ...

// using any engine, create the component
let wasm_component = wasm_component_layer::Component::new(&engine, &wasm_blob).unwrap();

// create the required imports
struct MyConsole { }
impl calculator::Console for MyConsole {
    fn print(&mut self, line: String) {
        println!("{line}");
    }
}
let imports = calculator::Imports { console: Box::new(MyConsole { }) };
let calculator_instance = calculator::instantiate(&wasm_component, imports);

let result = calculator_instance.exports.math.add(7, 9);
assert!(result == 16);
```

Implementation Notes:

Use ``wit-parser`` crate to parse the wit file.
Implementation of ``instantiate``:

``wasm_component_layer`` already provides a way to iterate interfaces and execute functions. 

```rust
pub mod calculator {

    // ...

    struct MathImpl {
        store: Arc<Mutex<wasm_component_layer::Store>>;
        add: TypedFunc<(i32, i32), i32>;
        multiply: TypedFunc<(i32, i32), i32>;
    }

    impl Math for MathImpl {
        fn add(&mut self, a: i32, b: i32) -> i32 {
            let mut store_guard = self.store.lock().unwrap();
            self.add.call(&mut *store_guard, (a, b)).unwrap() // unwrap is fine here, if the function doesn't exist of the signature doesn't match, it should have errored previously in 'instantiate'
        }
        fn multiply(&mut self, a: i32, b: i32) -> i32 {
            // analogous to 'add'
        }
    }

    pub fn instantiate(store: wasm_component_layer::Store,  linker: wasm_component_layer::Linker, component: &wasm_component_layer::Component, imports: Imports) -> Exports {

    // Define the console interface that the WASM component expects to import
    let console_interface = linker
        .define_instance("example:calculator/console".try_into().unwrap())
        .unwrap();

    let console_impl = Arc::new(Mutex::new(imports.console));
    console_interface
        .define_func(
            "print",
            Func::new(
                &mut store,
                FuncType::new([ValueType::String], []),
                move |_caller, params, _results| {
                    let p0 = params.get(0).take();
                    let Some(Value::String(line)) = p0 else { panic!("invalid parameter type blah blah blah"); };
                    let mut console_guard = self.store.lock().unwrap();
                    *console_guard.print(line.clone());
                    Ok(())
                },
            ),
        )
        .unwrap();

        // Create an instance of the component using the linker.
        let instance = linker.instantiate(&mut store, &component).unwrap();

        // get the interface
        let math_interface = instance
            .exports()
            .instance(&InterfaceIdentifier::new(
                PackageIdentifier::new(
                    PackageName::new("example", "calculator"),
                    Some(semver::Version {
                        major: 0,
                        minor: 1,
                        patch: 0,
                        pre: semver::Prerelease::EMPTY,
                        build: semver::BuildMetadata::EMPTY,
                    }),
                ),
                "math",
            ))
            .unwrap();

        // Get the 'add' function
        let func_ptr_add = math_interface
            .func("add")
            .unwrap()
            .typed::<(i32, i32), i32>()
            .unwrap();

        // Get the 'add' function
        let func_ptr_multiply = math_interface
            .func("multiply")
            .unwrap()
            .typed::<(i32, i32), i32>()
            .unwrap();

        let store_arc = Arc::new(Mutex::new(store));
        let math = MathImpl {
            store: store_arc,
            add: func_ptr_add,
            multiply: func_ptr_multiply
        };

        Exports {
            math: Box::new(math)
        }
    }
}
```

For the first iteration, we ignore ``async`` (``stream`` and ``future`` types in WIT/WASM).

Resources (``resource``) must be handled - generate Traits for them.


Important:
In the actual implementation, fully qualify all types, that is, use ``::`` prefix.