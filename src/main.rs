use eframe::egui;
mod shapes;
mod calculator;

fn main() -> Result<(), eframe::Error>{
    let options = eframe::NativeOptions{
        min_window_size: Some(egui::vec2(500.0, 300.0)),
        initial_window_size: Some(egui::vec2(500.0, 600.0)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "My GUI App",
        options,
        Box::new(|_cc| Box::new(calculator::Calculator::default()))
    )
}
