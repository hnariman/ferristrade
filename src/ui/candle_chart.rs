use chrono::{TimeZone, Utc};
use egui_plot::{AxisHints, GridMark, Legend, Plot};

use super::ChartCandle;

pub fn candlestick_chart(ui: &mut egui::Ui, data: ChartCandle) {
    let custom_x_axis = vec![AxisHints::new_x().formatter(
        |grid_mark: GridMark, _range: &std::ops::RangeInclusive<f64>| {
            Utc.timestamp_opt(grid_mark.value as i64, 0)
                .single()
                .unwrap()
                .format("%Y-%b-%d")
                .to_string()
        },
    )];

    let plot = Plot::new("candlestick chart")
        .legend(Legend::default())
        .view_aspect(2.0)
        .allow_zoom(true)
        .allow_boxed_zoom(true)
        .show_x(true)
        .show_y(true)
        .custom_x_axes(custom_x_axis)
        .custom_y_axes(vec![AxisHints::new_y().label("Price")])
        .center_x_axis(false)
        .center_y_axis(false);

    plot.show(ui, |plot_ui| {
        plot_ui.box_plot(data.data);
    });
}
