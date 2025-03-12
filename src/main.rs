use crate::ui::MyApp;
use egui;
use feeds::Feeds;
mod feeds;
mod store;
mod ui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(), //.with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    let feed = feeds::Feeds::default();
    let news = std::thread::spawn(async || Feeds::get_news().await);

    eframe::run_native(
        "Ferristrade",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
