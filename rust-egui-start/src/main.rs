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
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 在 `ui` 方法内部，你可以直接使用传入的 `ui` 参数
        ui.heading("Hello, egui!");
        if ui.button("Click Me").clicked() {
            println!("Button clicked");
        }
    }
}
