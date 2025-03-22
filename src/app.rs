use binance::{api::Binance, market::Market};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

// UI
use crate::ui::{central_panel, left_panel, right_panel};

use binance::model::KlineSummary;
use catppuccin_egui::{Theme, MOCHA};
use chrono::{TimeZone, Utc};
use eframe::egui;
use egui::{menu, Color32, Stroke};
use egui_plot::*;
use news::Article;
use store::Store;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct Config {
    up_color: Color32,
    down_color: Color32,
    chart_refresh: usize,
    theme: Theme,
}

impl Config {
    fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
}

pub struct Terminal {
    zoom: f32,
    store: Store,
    config: Config,
}

// impl Terminal {}

impl Default for Terminal {
    fn default() -> Self {
        // TODO: move this logic to actors in main.rs
        let store = Arc::new(Mutex::new(Store::default()));
        let store_clone = store.clone();
        let store_clone2 = store.clone();
        let egui_ctx = eframe::egui::Context::default();
        let egui_ctx2 = eframe::egui::Context::default();
        Store::get_news(&store.lock().unwrap()).unwrap();

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));
            if let Ok(_) = store_clone.lock().unwrap().get_news() {
                egui_ctx.request_repaint();
                println!("news updated");
            }
        });

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            Store::update_tickers(&mut store_clone2.lock().unwrap());
            egui_ctx2.request_repaint();
        });

        // Store::update_news(&Store::default(), store.clone());
        let config = Config {
            up_color: Color32::from_rgb(0, 255, 0),
            down_color: Color32::from_rgb(255, 0, 0),
            chart_refresh: 10,
            theme: MOCHA,
        };
        Self {
            zoom: 2.0,
            store,
            config,
        }
    }
}

impl Terminal {
    pub fn new(news: Receiver<Vec<Article>>, tickers: Receiver<Vec<KlineSummary>>) -> Self {
        let store = Store::new(news, tickers);

        let config = Config {
            up_color: Color32::from_rgb(0, 255, 0),
            down_color: Color32::from_rgb(255, 0, 0),
            chart_refresh: 10,
            theme: MOCHA,
        };

        Self {
            zoom: 2.0,
            store,
            config,
        }
    }
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
}

impl eframe::App for Terminal {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after_secs(1.0);
        // catppuccin_egui::set_theme(&ctx, catppuccin_egui::MOCHA);
        // menu
        left_panel(self, ctx);
        right_panel(self, ctx);
        central_panel(self, ctx);
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
                    kline.open.parse::<f64>().unwrap(),
                    median(&kline.high, &kline.low) as f64,
                    kline.close.parse::<f64>().unwrap(),
                    kline.high.parse::<f64>().unwrap(),
                ),
            )
            .whisker_width(2.0)
            .box_width(4.0)
            .fill(Color32::from_rgb(255, 0, 0))
            .stroke(Stroke::new(6.0, Color32::from_rgb(255, 0, 0)))
        };

        let down_candle = |kline: &KlineSummary| {
            BoxElem::new(
                kline.open_time as f64 / 1000.0,
                BoxSpread::new(
                    kline.low.parse::<f64>().unwrap(),
                    kline.close.parse::<f64>().unwrap(),
                    median(&kline.high, &kline.low) as f64,
                    kline.open.parse::<f64>().unwrap(),
                    kline.high.parse::<f64>().unwrap(),
                ),
            )
            .whisker_width(100.0)
            .box_width(4.0)
            .fill(Color32::from_rgb(0, 255, 0))
            .stroke(Stroke::new(6.0, Color32::from_rgb(0, 255, 0)))
        };

        let candles: Vec<BoxElem> = value
            .iter()
            .map(|kline| {
                if uptrend(&kline) {
                    let data = up_candle(&kline);
                    // dbg!(&data);
                    return data;
                } else {
                    let data = down_candle(&kline);
                    // dbg!(&data);
                    return data;
                }
            })
            .collect();

        Ok(ChartCandle {
            data: BoxPlot::new(candles),
        })
    }
}

pub struct MyApp {
    zoom: f32,
    prices: binance::model::Prices,
}

impl Default for MyApp {
    fn default() -> Self {
        let market: Market = Binance::new(None, None);
        let prices = market.get_all_prices().unwrap();
        Self { prices, zoom: 1.0 }
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
}
