use std::sync::{Arc, Mutex};

// UI
use crate::store::Store;
use eframe::egui;
use egui::{Color32, Stroke};
use egui_plot::*;
use rss::Item;

pub struct MyApp {
    zoom: f32,
    store: Arc<Mutex<Store>>,
    // news: Arc<Mutex<Vec<Item>>>, // so we can share rss data between gui and update methods
}

impl Default for MyApp {
    fn default() -> Self {
        let store = Arc::new(Mutex::new(Store::default()));
        Store::update_news(store.clone());
        Self { zoom: 1.0, store }
    }
}

impl MyApp {
    fn zoomin(&mut self) {
        if self.zoom > 4.0 {
            return;
        }
        self.zoom += 0.2;
    }

    fn zoomout(&mut self) {
        if self.zoom < 1.0 {
            return;
        }
        self.zoom -= 0.2;
    }

    // fn candle_chart(&mut self) { todo!() }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after_secs(1.0);
        egui::SidePanel::left("Tickers:").show(ctx, |ui| {
            ctx.set_pixels_per_point(self.zoom);
            ui.heading("Tickers:");
            ui.horizontal_top(|ui| {
                if ui.button("zoom-in").clicked() {
                    self.zoomin();
                }
                if ui.button("zoom-out").clicked() {
                    self.zoomout();
                }
            });
            // egui::ScrollArea::vertical().show(ui, |ui| {
            //     ui.vertical(|ui| {
            //         for item in &self.store.prices {
            //             ui.horizontal(|ui| {
            //                 ui.label(item.symbol.to_string());
            //                 ui.strong(item.price.to_string());
            //             });
            //         }
            //     })
            // })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            candlestick_chart(ui);
        });

        egui::SidePanel::right("News").show(ctx, |ui| {
            ui.heading("News");
            ui.vertical(|ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for item in self.store.lock().unwrap().news.lock().unwrap().clone() {
                        // for item in self.store.lock().news.lock().unwrap().clone() {
                        ui.label(item.title().unwrap_or("no title"));
                        ui.strong(item.description().unwrap_or("no description"));
                        ui.separator();
                    }
                })
            })
        });
    }
}

fn candlestick_chart(ui: &mut egui::Ui) {
    let red = Color32::from_rgb(255, 0, 0);
    let green = Color32::from_rgb(0, 255, 0);
    let data = BoxPlot::new(vec![
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1))
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.5, BoxSpread::new(1.5, 2.4, 2.4, 2.8, 3.5))
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(2.0, BoxSpread::new(1.8, 2.0, 2.4, 2.5, 2.7))
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(2.5, BoxSpread::new(1.5, 1.8, 1.8, 2.1, 2.2))
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(3.0, BoxSpread::new(1.4, 1.6, 1.6, 1.8, 2.1))
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(3.5, BoxSpread::new(0.5, 1.5, 1.5, 1.6, 1.7))
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(4.0, BoxSpread::new(1.2, 1.4, 1.4, 2.9, 3.2))
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(4.5, BoxSpread::new(2.1, 2.3, 2.3, 2.6, 2.7))
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(5.0, BoxSpread::new(1.9, 2.1, 2.1, 2.7, 3.5))
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(5.5, BoxSpread::new(2.0, 2.1, 2.1, 2.9, 3.3))
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(6.0, BoxSpread::new(2.3, 2.9, 2.9, 3.7, 4.1))
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(6.5, BoxSpread::new(3.1, 3.4, 3.4, 4.0, 4.2))
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
    ]);

    let plot = Plot::new("candlestick chart")
        .legend(Legend::default())
        .view_aspect(1.0)
        .allow_zoom(true)
        .center_x_axis(false)
        .center_y_axis(true);

    plot.show(ui, |plot_ui| {
        plot_ui.box_plot(data);
    });
}
