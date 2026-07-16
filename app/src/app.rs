use egui::{self, CornerRadius, RichText, Stroke};

/// Rectangular, dark "industrial console" theme with signal-orange accents.
pub mod theme {
    use egui::{
        Color32, CornerRadius, FontFamily, FontId, Stroke, TextStyle, Theme, Vec2, style::Selection,
    };

    pub const ACCENT: Color32 = Color32::from_rgb(255, 94, 0); // signal orange
    pub const ACCENT_BRIGHT: Color32 = Color32::from_rgb(255, 140, 50);
    pub const ERROR: Color32 = Color32::from_rgb(255, 70, 54);

    pub const BG_WINDOW: Color32 = Color32::from_rgb(14, 14, 16);
    pub const BG_PANEL: Color32 = Color32::from_rgb(19, 19, 22);
    pub const BG_WIDGET: Color32 = Color32::from_rgb(28, 28, 32);
    pub const BG_HOVER: Color32 = Color32::from_rgb(38, 38, 43);
    pub const BG_DEEP: Color32 = Color32::from_rgb(9, 9, 10);

    pub const STROKE_DIM: Color32 = Color32::from_rgb(52, 52, 58);
    pub const TEXT: Color32 = Color32::from_rgb(222, 222, 216);
    pub const TEXT_DIM: Color32 = Color32::from_rgb(140, 140, 134);

    pub fn apply(ctx: &egui::Context) {
        ctx.set_theme(Theme::Dark);
        ctx.style_mut_of(Theme::Dark, |style| {
            style.text_styles = [
                (TextStyle::Heading, FontId::new(16.0, FontFamily::Monospace)),
                (TextStyle::Body, FontId::new(13.5, FontFamily::Proportional)),
                (
                    TextStyle::Monospace,
                    FontId::new(12.5, FontFamily::Monospace),
                ),
                (TextStyle::Button, FontId::new(13.0, FontFamily::Monospace)),
                (TextStyle::Small, FontId::new(10.5, FontFamily::Monospace)),
            ]
            .into();

            style.spacing.item_spacing = Vec2::new(8.0, 6.0);
            style.spacing.button_padding = Vec2::new(14.0, 6.0);

            let v = &mut style.visuals;
            v.dark_mode = true;
            v.window_fill = BG_WINDOW;
            v.panel_fill = BG_PANEL;
            v.extreme_bg_color = BG_DEEP;
            v.code_bg_color = BG_DEEP;
            v.faint_bg_color = BG_WIDGET;
            v.warn_fg_color = ACCENT_BRIGHT;
            v.error_fg_color = ERROR;
            v.hyperlink_color = ACCENT_BRIGHT;
            v.selection = Selection {
                bg_fill: ACCENT.gamma_multiply(0.55),
                stroke: Stroke::new(1.0, Color32::WHITE),
            };

            // Sharp edges everywhere.
            v.window_corner_radius = CornerRadius::ZERO;
            v.menu_corner_radius = CornerRadius::ZERO;
            let w = &mut v.widgets;
            for wv in [
                &mut w.noninteractive,
                &mut w.inactive,
                &mut w.hovered,
                &mut w.active,
                &mut w.open,
            ] {
                wv.corner_radius = CornerRadius::ZERO;
            }

            w.noninteractive.bg_fill = BG_PANEL;
            w.noninteractive.bg_stroke = Stroke::new(1.0, STROKE_DIM);
            w.noninteractive.fg_stroke = Stroke::new(1.0, TEXT);

            w.inactive.bg_fill = BG_WIDGET;
            w.inactive.weak_bg_fill = BG_WIDGET;
            w.inactive.bg_stroke = Stroke::new(1.0, STROKE_DIM);
            w.inactive.fg_stroke = Stroke::new(1.0, TEXT);

            w.hovered.bg_fill = BG_HOVER;
            w.hovered.weak_bg_fill = BG_HOVER;
            w.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
            w.hovered.fg_stroke = Stroke::new(1.5, ACCENT_BRIGHT);

            w.active.bg_fill = ACCENT;
            w.active.weak_bg_fill = ACCENT;
            w.active.bg_stroke = Stroke::new(1.0, ACCENT);
            w.active.fg_stroke = Stroke::new(1.5, Color32::BLACK);

            w.open.bg_fill = BG_WIDGET;
            w.open.weak_bg_fill = BG_WIDGET;
            w.open.bg_stroke = Stroke::new(1.0, ACCENT);
            w.open.fg_stroke = Stroke::new(1.0, ACCENT_BRIGHT);

            v.interact_cursor = Some(egui::CursorIcon::PointingHand);
        });
    }
}

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

    // Interactive math inputs
    add_input_a: String,
    add_input_b: String,
    multiply_input_a: String,
    multiply_input_b: String,

    // Flag to indicate if a component is loaded
    component_loaded: bool,
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
            add_input_a: "7".to_string(),
            add_input_b: "8".to_string(),
            multiply_input_a: "7".to_string(),
            multiply_input_b: "5".to_string(),
            component_loaded: false,
        }
    }
}

impl WasmInWasmApp {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
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

        if let Some(rx) = &self.upload
            && let Ok(blob) = rx.try_recv()
        {
            self.blob = Some(blob);
            self.process_blob();
        }

        self.header_bar(ui);

        #[cfg(target_arch = "wasm32")]
        let time = performance.now() - render_start_time;
        #[cfg(not(target_arch = "wasm32"))]
        let time = render_start_time.elapsed().as_secs_f64() * 1000.0;
        // NOTE: panels are laid out before the central content, so the status
        // bar shows the time up to this point plus last frame's count.
        self.status_bar(ui, time);

        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    self.interface_section(ui);
                    ui.add_space(12.0);
                    self.component_section(ui);
                    if self.component_loaded {
                        ui.add_space(12.0);
                        self.math_section(ui);
                    }
                    ui.add_space(12.0);
                    self.log_section(ui);
                });
        });
    }

    fn header_bar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("header")
            .frame(
                egui::Frame::new()
                    .fill(theme::BG_DEEP)
                    .inner_margin(egui::Margin::symmetric(12, 10)),
            )
            .show_separator_line(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (mark, _) =
                        ui.allocate_exact_size(egui::Vec2::new(10.0, 22.0), egui::Sense::hover());
                    ui.painter().rect_filled(mark, 0.0, theme::ACCENT);
                    ui.heading(RichText::new("WASM-IN-WASM").color(theme::TEXT).strong());
                    ui.label(
                        RichText::new("// component playground")
                            .monospace()
                            .color(theme::TEXT_DIM),
                    );
                });
                // Signal-orange rule under the header.
                let (line, _) = ui.allocate_exact_size(
                    egui::Vec2::new(ui.available_width(), 2.0),
                    egui::Sense::hover(),
                );
                ui.painter().rect_filled(line, 0.0, theme::ACCENT);
            });
    }

    fn status_bar(&self, ui: &mut egui::Ui, render_ms: f64) {
        egui::Panel::bottom("status")
            .frame(
                egui::Frame::new()
                    .fill(theme::BG_DEEP)
                    .inner_margin(egui::Margin::symmetric(12, 4)),
            )
            .show_separator_line(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("FRAME {}", self.frame_count))
                            .small()
                            .color(theme::TEXT_DIM),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("{render_ms:.2} ms"))
                                .small()
                                .color(theme::ACCENT_BRIGHT),
                        );
                        ui.label(
                            RichText::new(if self.component_loaded {
                                "COMPONENT LOADED"
                            } else {
                                "NO COMPONENT"
                            })
                            .small()
                            .color(if self.component_loaded {
                                theme::ACCENT
                            } else {
                                theme::TEXT_DIM
                            }),
                        );
                    });
                });
            });
    }

    fn section<R>(
        ui: &mut egui::Ui,
        title: &str,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> R {
        ui.horizontal(|ui| {
            let (tick, _) =
                ui.allocate_exact_size(egui::Vec2::new(4.0, 12.0), egui::Sense::hover());
            ui.painter().rect_filled(tick, 0.0, theme::ACCENT);
            ui.label(RichText::new(title).small().color(theme::ACCENT_BRIGHT));
        });
        egui::Frame::new()
            .fill(theme::BG_WINDOW)
            .stroke(Stroke::new(1.0, theme::STROKE_DIM))
            .corner_radius(CornerRadius::ZERO)
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                add_contents(ui)
            })
            .inner
    }

    fn interface_section(&mut self, ui: &mut egui::Ui) {
        Self::section(ui, "INTERFACE", |ui| {
            ui.label("Load any WASM component which adheres to the following interface:");
            ui.add_space(4.0);
            render_code(ui, WIT_TEXT);
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if ui.button("▶ LOAD DEMO").clicked() {
                    self.blob = Some(DEFAULT_WASM_BLOB.to_vec().into_boxed_slice());
                    self.process_blob();
                }
                if ui.button("⬆ UPLOAD .WASM").clicked() {
                    self.upload_blob(ui);
                }
                if ui
                    .add_enabled(self.blob.is_some(), egui::Button::new("⬇ DOWNLOAD"))
                    .clicked()
                {
                    self.download_blob();
                }
            });
        });
    }

    fn upload_blob(&mut self, ui: &mut egui::Ui) {
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
            let _ = ui;
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

    fn download_blob(&mut self) {
        let Some(blob) = &self.blob else {
            self.output.push("No blob loaded to download".to_string());
            return;
        };
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

    fn component_section(&mut self, ui: &mut egui::Ui) {
        Self::section(ui, "COMPONENT", |ui| {
            if self.exported_interfaces.is_empty() {
                ui.label(
                    RichText::new("no component loaded — exported interfaces show up here")
                        .color(theme::TEXT_DIM)
                        .italics(),
                );
            } else {
                ui.label("Exported interfaces:");
                for interface in &self.exported_interfaces {
                    render_code(ui, interface);
                }
            }

            ui.add_space(4.0);
            ui.collapsing(RichText::new("DECOMPILED WAT").small(), |ui| {
                if let Some(text) = &self.text {
                    egui::ScrollArea::vertical()
                        .id_salt("wat_scroll")
                        .max_height(300.0)
                        .show(ui, |ui| {
                            ui.monospace(text);
                        });
                } else {
                    ui.label(RichText::new("No WAT available").color(theme::TEXT_DIM));
                }
            });
        });
    }

    fn math_section(&mut self, ui: &mut egui::Ui) {
        Self::section(ui, "MATH CONSOLE", |ui| {
            let mut results = Vec::new();

            ui.horizontal(|ui| {
                ui.label(RichText::new("ADD").monospace().color(theme::TEXT_DIM));
                ui.add(egui::TextEdit::singleline(&mut self.add_input_a).desired_width(64.0));
                ui.label("+");
                ui.add(egui::TextEdit::singleline(&mut self.add_input_b).desired_width(64.0));
                if ui.button("= RUN").clicked() {
                    if let (Ok(a), Ok(b)) = (
                        self.add_input_a.parse::<i32>(),
                        self.add_input_b.parse::<i32>(),
                    ) {
                        if let Some(result) = self.execute_add(a, b) {
                            results.push(format!("Addition: {} + {} = {}", a, b, result));
                        } else {
                            results.push("Failed to execute addition".to_string());
                        }
                    } else {
                        results.push(
                            "Invalid input for addition - please enter valid integers".to_string(),
                        );
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new("MUL").monospace().color(theme::TEXT_DIM));
                ui.add(egui::TextEdit::singleline(&mut self.multiply_input_a).desired_width(64.0));
                ui.label("×");
                ui.add(egui::TextEdit::singleline(&mut self.multiply_input_b).desired_width(64.0));
                if ui.button("= RUN").clicked() {
                    if let (Ok(a), Ok(b)) = (
                        self.multiply_input_a.parse::<i32>(),
                        self.multiply_input_b.parse::<i32>(),
                    ) {
                        if let Some(result) = self.execute_multiply(a, b) {
                            results.push(format!("Multiplication: {} × {} = {}", a, b, result));
                        } else {
                            results.push("Failed to execute multiplication".to_string());
                        }
                    } else {
                        results.push(
                            "Invalid input for multiplication - please enter valid integers"
                                .to_string(),
                        );
                    }
                }
            });

            self.output.extend(results);
        });
    }

    fn log_section(&self, ui: &mut egui::Ui) {
        Self::section(ui, "LOG", |ui| {
            egui::Frame::new()
                .fill(theme::BG_DEEP)
                .stroke(Stroke::new(1.0, theme::STROKE_DIM))
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    if self.output.is_empty() {
                        ui.label(
                            RichText::new("— log is empty —")
                                .monospace()
                                .color(theme::TEXT_DIM),
                        );
                    }
                    for line in &self.output {
                        let is_error = line.starts_with("Error")
                            || line.starts_with("Failed")
                            || line.starts_with("Invalid");
                        ui.label(RichText::new(line).monospace().color(if is_error {
                            theme::ERROR
                        } else {
                            theme::TEXT
                        }));
                    }
                });
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

        // Clear previous instance
        self.component_loaded = false;
        self.output.clear();

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

        let Ok(actual_wit) = wit_component::decode(blob) else {
            self.output.push("Failed to decode WASM blob".to_string());
            return;
        };

        let mut printer = wit_component::WitPrinter::default();

        match actual_wit {
            wit_component::DecodedWasm::Component(resolve, world_id) => {
                self.output.push("Wasm blob is a Component".to_string());
                let world = &resolve.worlds[world_id];
                if let Some(pkg_id) = world.package {
                    let _pkg = &resolve.packages[pkg_id];

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
        let store = wasm_component_layer::Store::new(&engine, ());

        // Parse the component bytes and load its imports and exports.
        let component = wasm_component_layer::Component::new(&engine, blob).unwrap();

        let console_impl = MyConsoleImpl;
        let imports = wit_bindings::calculator::Imports {
            console: Box::new(console_impl),
        };

        let mut instance =
            wit_bindings::calculator::instantiate(store, &component, imports).unwrap();

        // Demonstrate the functions work with initial values
        let add_result = instance.math.add(7, 8);
        self.output.push(format!("Demo: 7 + 8 = {}", add_result));

        let multiply_result = instance.math.multiply(7, 5);
        self.output
            .push(format!("Demo: 7 × 5 = {}", multiply_result));

        // Mark component as loaded for interactive use
        self.component_loaded = true;
    }

    fn execute_add(&self, a: i32, b: i32) -> Option<i32> {
        let blob = self.blob.as_ref()?;

        // Create a new engine for instantiating a component.
        let wasmi_engine = wasmi_runtime_layer::Engine::default();
        let engine = wasm_component_layer::Engine::new(wasmi_engine);

        // Create a store for managing WASM data and any custom user-defined state.
        let store = wasm_component_layer::Store::new(&engine, ());

        // Parse the component bytes and load its imports and exports.
        let component = wasm_component_layer::Component::new(&engine, blob).ok()?;

        let console_impl = MyConsoleImpl;
        let imports = wit_bindings::calculator::Imports {
            console: Box::new(console_impl),
        };

        let mut instance =
            wit_bindings::calculator::instantiate(store, &component, imports).ok()?;

        Some(instance.math.add(a, b))
    }

    fn execute_multiply(&self, a: i32, b: i32) -> Option<i32> {
        let blob = self.blob.as_ref()?;

        // Create a new engine for instantiating a component.
        let wasmi_engine = wasmi_runtime_layer::Engine::default();
        let engine = wasm_component_layer::Engine::new(wasmi_engine);

        // Create a store for managing WASM data and any custom user-defined state.
        let store = wasm_component_layer::Store::new(&engine, ());

        // Parse the component bytes and load its imports and exports.
        let component = wasm_component_layer::Component::new(&engine, blob).ok()?;

        let console_impl = MyConsoleImpl;
        let imports = wit_bindings::calculator::Imports {
            console: Box::new(console_impl),
        };

        let mut instance =
            wit_bindings::calculator::instantiate(store, &component, imports).ok()?;

        Some(instance.math.multiply(a, b))
    }
}

pub fn render_code(ui: &mut egui::Ui, code: &str) {
    let language = "wit";
    let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
    egui_extras::syntax_highlighting::code_view_ui(ui, &theme, code, language);
}
