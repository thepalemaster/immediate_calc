use eframe::egui;
pub mod literals;
mod shapes;
mod calculator;

fn main() -> Result<(), eframe::Error>{
    let options = eframe::NativeOptions{
        min_window_size: Some(egui::vec2(literals::STEP * 10.0, literals::STEP * 6.)),
        initial_window_size: Some(egui::vec2(literals::STEP * 10.0, literals::STEP * 12.)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        literals::APP_TITLE,
        options,
        Box::new(|_cc| Box::new(calculator::Calculator::default()))
    )
}
