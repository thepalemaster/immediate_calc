use std::sync::Arc;

use eframe::egui;
use egui::IconData;
mod calculator;
pub mod literals;
mod shapes;

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
