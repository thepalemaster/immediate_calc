#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use eframe::egui;
use egui::IconData;

mod calculator;
pub mod literals;
mod shapes;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let icon = include_bytes!("../assets/icon.rgb").to_vec();
    let window_size = egui::ViewportBuilder {
        min_inner_size: Some(egui::vec2(literals::STEP * 9.6, literals::STEP * 8.)),
        max_inner_size: Some(egui::vec2(literals::STEP * 9.6, literals::STEP * 24.)),
        icon: Some(Arc::new(IconData {
            rgba: icon,
            width: 256,
            height: 256,
        })),
        ..Default::default()
    };
    let options = eframe::NativeOptions {
        viewport: window_size,
        ..Default::default()
    };
    eframe::run_native(
        literals::APP_TITLE,
        options,
        Box::new(|_cc| Box::<calculator::Calculator>::default()),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    console_error_panic_hook::set_once();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Box::<calculator::Calculator>::default()),
            )
            .await
            .expect("failed to start calculator-wasm");
    });
}
