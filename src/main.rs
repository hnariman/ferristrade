// UI
use eframe::egui;
use egui::{Color32, Stroke};
use egui_plot::*;
// binance:
use binance::api::*;
use binance::market::*;
use binance::model::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ferristrade",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
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
        .view_aspect(1.0);

    plot.show(ui, |plot_ui| {
        plot_ui.box_plot(data);
    });
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| match &self.prices {
                    Prices::AllPrices(val) => {
                        for item in val {
                            ui.horizontal(|ui| {
                                ui.label(item.symbol.to_string());
                                ui.strong(item.price.to_string());
                            });
                        }
                    }
                })
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            candlestick_chart(ui);
        });
    }
}
