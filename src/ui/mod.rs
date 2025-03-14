use core::f64;
use std::{
    fs::symlink_metadata,
    sync::{Arc, Mutex},
    vec,
};

// UI
use crate::store::Store;
use binance::{
    api::Binance,
    model::{KlineSummary, Symbol},
};
use chrono::DateTime;
use eframe::egui;
use egui::{gui_zoom::kb_shortcuts, Color32, Stroke};
use egui_plot::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Config {
    up_color: Color32,
    down_color: Color32,
    chart_refresh: usize,
}

pub struct MyApp {
    zoom: f32,
    store: Arc<Mutex<Store>>,
    config: Config,
}

impl Default for MyApp {
    fn default() -> Self {
        let store = Arc::new(Mutex::new(Store::default()));
        Store::update_news(&Store::default(), store.clone());
        let config = Config {
            up_color: Color32::from_rgb(0, 255, 0),
            down_color: Color32::from_rgb(255, 0, 0),
            chart_refresh: 10,
        };
        Self {
            zoom: 2.0,
            store,
            config,
        }
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
    //
    pub fn left_panel(&mut self, ctx: &egui::Context) {
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
    }
    pub fn right_panel(&mut self, ctx: &egui::Context) {
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

    pub fn central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let data: ChartCandle = self
                .store
                .lock()
                .expect("Mutex lock is poisoned central_panel()")
                .chart_data
                .clone()
                .try_into()
                .expect("unable to convert store data into chart_candle: central_panel()");
            MyApp::candlestick_chart(self, ui, data);
        });
    }

    // pub fn candlestick_chart2(ui: &mut egui::Ui) {
    // let market = Binance::new(None, None);
    // let data: Vec<KlineSummary> = vec![];
    // let data = data.into_iter().map(|k| k.try_into()).collect();
    // let plot = Plot::new("candlestick chart")
    //     .legend(Legend::default())
    //     .view_aspect(1.0)
    //     .allow_zoom(true)
    //     .center_x_axis(false)
    //     .center_y_axis(true);

    // plot.show(ui, |plot_ui| plot_ui.box_plot(data));
    // }

    pub fn candlestick_chart(&self, ui: &mut egui::Ui, data: ChartCandle) {
        // TODO: move to settings and have default constants?
        // let down = self.config.down_color;
        // let up = self.config.up_color;

        let plot = Plot::new("candlestick chart")
            .legend(Legend::default())
            .view_aspect(2.0)
            .allow_zoom(true)
            .show_x(true)
            .show_y(true)
            .center_x_axis(false)
            .center_y_axis(false);

        plot.show(ui, |plot_ui| {
            plot_ui.box_plot(data.data);
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after_secs(1.0);
        catppuccin_egui::set_theme(&ctx, catppuccin_egui::MOCHA);
        self.left_panel(ctx);
        self.central_panel(ctx);
        self.right_panel(ctx);
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ChartValue {
    arg: f64,
    r#box: (f64, f64, f64, f64, f64),
    whisker: f64,
    fill: Color32,
    stroke: (f64, Color32),
}

pub struct ChartCandle {
    data: BoxPlot,
}

impl TryFrom<Vec<KlineSummary>> for ChartCandle {
    type Error = String;
    fn try_from(value: Vec<KlineSummary>) -> Result<Self, Self::Error> {
        let median = |h: &str, l: &str| {
            let high = h.parse::<f64>().unwrap();
            let low = l.parse::<f64>().unwrap();
            (high + low) / 2.0
        };

        let uptrend = |k: &KlineSummary| -> bool {
            let open = k.open.parse::<f64>().unwrap();
            let close = k.close.parse::<f64>().unwrap();
            open > close
        };

        // let human_readable_date = |kline:&KlineSummary|{
        //     let timestamp = kline.open_time as f64 /1000.0;
        //     let naive = DateTime::from_timestamp(kline.open_time, 0);
        // }

        let up_candle = |kline: &KlineSummary| {
            BoxElem::new(
                kline.open_time as f64 / 1000.0,
                BoxSpread::new(
                    kline.low.parse::<f64>().unwrap(),
                    kline.close.parse::<f64>().unwrap(),
                    median(&kline.high, &kline.low),
                    kline.open.parse::<f64>().unwrap(),
                    kline.high.parse::<f64>().unwrap(),
                ),
            )
            .whisker_width(0.0)
            .fill(Color32::from_rgb(255, 0, 0))
            .stroke(Stroke::new(2.0, Color32::from_rgb(255, 0, 0)))
        };

        let down_candle = |kline: &KlineSummary| {
            BoxElem::new(
                kline.open_time as f64 / 1000.0,
                BoxSpread::new(
                    kline.low.parse::<f64>().unwrap(),
                    kline.open.parse::<f64>().unwrap(),
                    median(&kline.high, &kline.low),
                    kline.close.parse::<f64>().unwrap(),
                    kline.high.parse::<f64>().unwrap(),
                ),
            )
            .whisker_width(0.0)
            .fill(Color32::from_rgb(0, 255, 0))
            .stroke(Stroke::new(2.0, Color32::from_rgb(0, 255, 0)))
        };

        let candles: Vec<BoxElem> = value
            .iter()
            .map(|kline| {
                if uptrend(&kline) {
                    up_candle(&kline)
                } else {
                    down_candle(&kline)
                }
            })
            .collect();

        Ok(ChartCandle {
            data: BoxPlot::new(candles),
        })
    }
}
