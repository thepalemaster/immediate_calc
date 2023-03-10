mod shapes;
mod calculator;


fn main() {
    println!("Hello, world!");
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "My GUI App",
        options,
        Box::new(|_cc| Box::new(calculator::Calculator::default()))
    )
}