use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Egui Test",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    counter: u32,
}

impl eframe::App for MyApp {
    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Hello, egui!");
        ui.label(format!("Counter: {}", self.counter));
        if ui.button("Click Me").clicked() {
            self.counter += 1;
        }
    }
}
