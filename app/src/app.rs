use egui::{self, Color32, Stroke, vec2};
use egui_typed_input::ValText;
use std::f32::consts::PI;

use wasm_component_layer::*;

// you need to manually build the project beforehand using 'cargo component build --release'
const DEFAULT_WASM_BLOB: &[u8] =
    include_bytes!(r#"../../demo-component/target/wasm32-wasip1/release/demo_component.wasm"#);

const WIT_TEXT: &str = include_str!(r#"../../wit/world.wit"#);

mod wit_bindings {
    wit_derive::generate!({
        world: "calculator",
        path: "../wit",
    });
}

pub struct MyConsoleImpl;

impl wit_bindings::calculator::console for MyConsoleImpl {
    fn print(&mut self, message: String) {
        println!("[WASM]: {}", message);
    }
}

pub struct WasmInWasmApp {
    blob: Option<Box<[u8]>>,
    text: Option<String>,

    exported_interfaces: Vec<String>,
    output: Vec<String>,

    upload: Option<std::sync::mpsc::Receiver<Box<[u8]>>>,

    frame_count: u64,
}

impl Default for WasmInWasmApp {
    fn default() -> Self {
        Self {
            blob: None,
            text: None,
            exported_interfaces: Vec::new(),
            output: Vec::new(),
            upload: None,
            frame_count: 0,
        }
    }
}

impl WasmInWasmApp {
    pub fn ui(&mut self, ctx: &egui::Context) {
        // performance: keep track of frame count and render time
        self.frame_count += 1;
        #[cfg(target_arch = "wasm32")]
        let (performance, render_start_time) = {
            // Get performance object for timing
            let window = web_sys::window().expect("should have window");
            let performance = window
                .performance()
                .expect("should have performance available");
            let start_time = performance.now();
            (performance, start_time)
        };
        #[cfg(not(target_arch = "wasm32"))]
        let render_start_time = std::time::Instant::now();

        if let Some(rx) = &self.upload {
            if let Ok(blob) = rx.try_recv() {
                self.blob = Some(blob);
                self.process_blob();
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Set dark mode
            ui.ctx().set_visuals(egui::Visuals::dark());

            // Helper function to draw a separator line
            let draw_separator = |ui: &mut egui::Ui| {
                let separator_height = 2.0;
                let separator_color = Color32::from_rgb(100, 100, 100); // Dark gray

                let response = ui.allocate_rect(
                    egui::Rect::from_min_size(
                        ui.cursor().min,
                        egui::Vec2::new(ui.available_width(), separator_height),
                    ),
                    egui::Sense::hover(),
                );

                let rect = response.rect;
                ui.painter().rect_filled(rect, 0.0, separator_color);

                ui.add_space(5.0); // Space after separator
            };

            ui.label(r"You can load any WASM component which adheres to the following interface:");

            render_code(ui, WIT_TEXT);

            if ui.button("Load default wasm blob").clicked() {
                self.blob = Some(DEFAULT_WASM_BLOB.to_vec().into_boxed_slice());
                self.process_blob();
            }

            if ui.button("Upload blob").clicked() {
                // open a file dialog to select a wasm blob
                #[cfg(target_arch = "wasm32")]
                {
                    // For web: use rfd's async file dialog
                    let ctx = ui.ctx().clone();
                    let (tx, rx) = std::sync::mpsc::channel();
                    self.upload = Some(rx);
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Some(file_handle) = rfd::AsyncFileDialog::new()
                            .add_filter("WebAssembly", &["wasm"])
                            .pick_file()
                            .await
                        {
                            let data = file_handle.read().await;
                            let blob_data = data.into_boxed_slice();
                            tx.send(blob_data).expect("Failed to send blob data");
                            ctx.request_repaint();
                        }
                    });
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // For native: use rfd's blocking file dialog
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("WebAssembly", &["wasm"])
                        .pick_file()
                    {
                        match std::fs::read(&path) {
                            Ok(data) => {
                                self.blob = Some(data.into_boxed_slice());
                                self.process_blob();
                            }
                            Err(e) => {
                                self.output.push(format!("Error reading file: {}", e));
                            }
                        }
                    }
                }
            }

            if ui.button("Download current blob").clicked() {
                match &self.blob {
                    Some(blob) => {
                        #[cfg(target_arch = "wasm32")]
                        {
                            // For web: use rfd's async file dialog
                            let blob_data = blob.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                if let Some(file_handle) = rfd::AsyncFileDialog::new()
                                    .set_file_name("component.wasm")
                                    .save_file()
                                    .await
                                {
                                    let _ = file_handle.write(&blob_data).await;
                                }
                            });
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            // For native: use rfd's blocking file dialog
                            if let Some(path) = rfd::FileDialog::new()
                                .set_file_name("component.wasm")
                                .save_file()
                            {
                                match std::fs::write(&path, blob) {
                                    Ok(()) => {
                                        self.output.push(format!("Saved to: {}", path.display()));
                                    }
                                    Err(e) => {
                                        self.output.push(format!("Error saving file: {}", e));
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        self.output.push("No blob loaded to download".to_string());
                    }
                }
            }

            ui.add_space(15.0);
            draw_separator(ui);

            ui.label("Exported Interfaces:");
            for interface in &self.exported_interfaces {
                render_code(ui, interface);
            }

            ui.collapsing("Decompiled WAT", |ui| {
                if let Some(text) = &self.text {
                    ui.label("Decompiled WAT:");

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.monospace(text);
                    });
                } else {
                    ui.label("No WAT available");
                }
            });

            ui.add_space(15.0);
            draw_separator(ui);

            ui.label("Output:");
            for line in &self.output {
                ui.label(line);
            }

            ui.add_space(15.0);
            draw_separator(ui);

            #[cfg(target_arch = "wasm32")]
            let time = performance.now() - render_start_time;
            #[cfg(not(target_arch = "wasm32"))]
            let time = render_start_time.elapsed().as_secs_f32() * 1000.0;

            ui.label(format!("Frame {}: {:.2} ms", self.frame_count, time));
        });
    }
}

impl WasmInWasmApp {
    pub fn process_blob(&mut self) {
        let blob = match &self.blob {
            Some(blob) => blob,
            None => {
                self.output.push("No WASM blob loaded".to_string());
                return;
            }
        };

        // try to decompile
        let wat = wasmprinter::print_bytes(blob);
        match wat {
            Ok(wat) => {
                self.text = Some(wat);
            }
            Err(e) => {
                self.text = None;
                self.output
                    .push(format!("Failed to decompile WASM blob: {}", e));
                return;
            }
        }

        self.output
            .push(format!("loaded blob of size: {}", blob.len()));

        self.exported_interfaces.clear();

        let Ok(actual_wit) = wit_component::decode(&blob) else {
            self.output.push("Failed to decode WASM blob".to_string());
            return;
        };

        let mut printer = wit_component::WitPrinter::default();

        match actual_wit {
            wit_component::DecodedWasm::Component(resolve, world_id) => {
                self.output.push("Wasm blob is a Component".to_string());
                let world = &resolve.worlds[world_id];
                if let Some(pkg_id) = world.package {
                    let pkg = &resolve.packages[pkg_id];

                    self.output.push("world is a package".to_string());
                    printer.print(&resolve, pkg_id, &[]).unwrap();

                    // for x in pkg.
                    // printer.print_interface(resolve, id);
                    self.exported_interfaces.push(printer.output.to_string());
                } else {
                    self.output.push("No package defined".to_string());
                }
            }
            wit_component::DecodedWasm::WitPackage(resolve, pkg_id) => {
                self.output.push("Wasm blob is a Package".to_string());
                printer.print(&resolve, pkg_id, &[]).unwrap();
                self.output.push(printer.output.to_string());
            }
        }

        // Create a new engine for instantiating a component.
        let wasmi_engine = wasmi_runtime_layer::Engine::default();
        let engine = wasm_component_layer::Engine::new(wasmi_engine);

        // Create a store for managing WASM data and any custom user-defined state.
        let mut store = wasm_component_layer::Store::new(&engine, ());

        // Parse the component bytes and load its imports and exports.
        let component = wasm_component_layer::Component::new(&engine, &blob).unwrap();

        // Create a linker that will be used to resolve the component's imports, if any.
        let mut linker = Linker::default();

        let console_impl = MyConsoleImpl;
        let mut imports = wit_bindings::calculator::Imports {
            console: Box::new(console_impl),
        };

        let mut instance =
            wit_bindings::calculator::instantiate(store, &component, imports).unwrap();

        let add_result = instance.math.add(7, 8);
        self.output
            .push(format!("Result of 7 + 8 = {}", add_result));

        let multiply_result = instance.math.multiply(7, 5);
        self.output
            .push(format!("Result of 7 * 5 = {}", multiply_result));
    }
}

pub fn render_code(ui: &mut egui::Ui, code: &str) {
    let language = "wit";
    let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
    egui_extras::syntax_highlighting::code_view_ui(ui, &theme, code, language);
}
