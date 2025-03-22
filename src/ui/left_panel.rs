use eframe::egui;
pub fn left_panel(ctx: &egui::Context, config: &mut Config) {
    egui::SidePanel::left("Tickers:")
        .min_width(200.0)
        .show(ctx, |ui| {
            ctx.set_pixels_per_point(config.zoom);
            ui.heading("Tickers:");
            ui.horizontal_top(|ui| {
                if ui.button("zoom-in").clicked() {
                    //FIXME: make config a separate Struct with Impl for set/get
                    config.zoom_in()
                }
                if ui.button("zoom-out").clicked() {
                    config.zoom_out()
                }
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    for item in &self.store.lock().unwrap().prices {
                        ui.horizontal(|ui| {
                            ui.label(item.symbol.to_string());
                            ui.strong(item.price.to_string());
                        });
                    }
                })
            })
        });
}
