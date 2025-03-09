use crate::ui::MyApp;
use egui;
mod store;
mod ui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(), //.with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ferristrade",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
