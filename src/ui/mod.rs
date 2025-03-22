pub mod candle_chart;
pub mod central_panel;
pub mod left_panel;
pub mod menu_panel;
pub mod right_panel;
// UI
use binance::model::KlineSummary;
use eframe::egui;
use egui::{Color32, Stroke};
use egui_plot::*;

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
