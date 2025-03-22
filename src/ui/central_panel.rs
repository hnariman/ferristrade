// use super::ChartCandle;

pub fn central_panel(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |_ui| {
        egui::Window::new("BTCUSDT")
            .open(&mut true)
            .show(ctx, |_ui| {
                // let data: ChartCandle = self
                //     .store
                //     .lock()
                //     .expect("Mutex lock is poisoned central_panel()")
                //     .chart_data
                //     .clone()
                //     .try_into()
                //     .expect("unable to convert store data into chart_candle: central_panel()");
                // Terminal::candlestick_chart(self, ui, data);
            });
        egui::Window::new("BTCUSDC")
            .open(&mut true)
            .show(ctx, |_ui| {
                // let data: ChartCandle = self
                //     .store
                //     .lock()
                //     .expect("Mutex lock is poisoned central_panel()")
                //     .chart_data
                //     .clone()
                //     .try_into()
                //     .expect("unable to convert store data into chart_candle: central_panel()");
                // Terminal::candlestick_chart(self, ui, data);
            });
    });
}
