#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;

impl eframe::App for app::WasmInWasmApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.ui(ui);
    }
}

// When compiling natively:
#[cfg(not(any(target_arch = "wasm32")))]
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([520.0, 820.0])
            .with_min_inner_size([360.0, 300.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Wasm In Wasm",
        native_options,
        Box::new(|cc| {
            app::theme::apply(&cc.egui_ctx);
            Ok(Box::new(app::WasmInWasmApp::default()))
        }),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    use eframe::wasm_bindgen::prelude::*;
    use egui_web_component::EguiMount;
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    thread_local! {
        static NEXT_ID: Cell<u32> = const { Cell::new(0) };
        static MOUNTS: RefCell<HashMap<u32, EguiMount>> = RefCell::new(HashMap::new());
    }

    let connect = Closure::wrap(Box::new(move |element: web_sys::HtmlElement| {
        let id = NEXT_ID.with(|next_id| {
            let id = next_id.get();
            next_id.set(id + 1);
            id
        });

        element
            .set_attribute("data-wasm-in-wasm-id", &id.to_string())
            .expect("Failed to set wasm-in-wasm element id");

        wasm_bindgen_futures::spawn_local(async move {
            let start_result = EguiMount::connect(
                &element,
                eframe::WebOptions::default(),
                Box::new(|cc| {
                    app::theme::apply(&cc.egui_ctx);
                    Ok(Box::new(app::WasmInWasmApp::default()))
                }),
            )
            .await;
            let document = web_sys::window()
                .expect("No window")
                .document()
                .expect("No document");

            match start_result {
                Ok(mount) => {
                    MOUNTS.with(|mounts| {
                        mounts.borrow_mut().insert(id, mount);
                    });
                    if let Some(loading_text) = document.get_element_by_id("loading_text") {
                        loading_text.remove();
                    }
                }
                Err(e) => {
                    if let Some(loading_text) = document.get_element_by_id("loading_text") {
                        loading_text.set_inner_html(
                            "<p> The app has crashed. See the developer console for details. </p>",
                        );
                    }
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        });
    }) as Box<dyn FnMut(web_sys::HtmlElement)>);

    let disconnect = Closure::wrap(Box::new(move |element: web_sys::HtmlElement| {
        if let Some(id) = element
            .get_attribute("data-wasm-in-wasm-id")
            .and_then(|id| id.parse::<u32>().ok())
        {
            MOUNTS.with(|mounts| {
                if let Some(mount) = mounts.borrow_mut().remove(&id) {
                    mount.disconnect();
                }
            });
        }
    }) as Box<dyn FnMut(web_sys::HtmlElement)>);

    let window = web_sys::window().expect("No window");
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("__wasm_in_wasm_connect"),
        connect.as_ref().unchecked_ref(),
    )
    .expect("Failed to install connect callback");
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("__wasm_in_wasm_disconnect"),
        disconnect.as_ref().unchecked_ref(),
    )
    .expect("Failed to install disconnect callback");

    connect.forget();
    disconnect.forget();

    js_sys::eval(
        r#"
        class WasmInWasmApp extends HTMLElement {
            connectedCallback() {
                window.__wasm_in_wasm_connect(this);
            }

            disconnectedCallback() {
                window.__wasm_in_wasm_disconnect(this);
            }
        }

        if (!customElements.get('wasm-in-wasm-app')) {
            customElements.define('wasm-in-wasm-app', WasmInWasmApp);
        }
        "#,
    )
    .expect("Failed to register wasm-in-wasm-app custom element");
}
