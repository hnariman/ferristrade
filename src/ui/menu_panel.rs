pub fn app_menu(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Themes", |ui| {
                if ui.button("Dark").clicked() {
                    ctx.set_theme(egui::Theme::Dark)
                }
                if ui.button("Light").clicked() {
                    ctx.set_theme(egui::Theme::Light)
                    // ctx.set_theme(catppuccin_egui::MOCHA)
                }
                // if ui.button("Latte").clicked() {
                //     ctx.set_theme(egui::Theme::Dark)
                // }
                // if ui.button("Mocha").clicked() {
                //     ctx.set_theme(egui::Theme::Light)
                //     // ctx.set_theme(catppuccin_egui::MOCHA)
                // }
                // if ui.button("Macchiato").clicked() {
                //     // ctx.set_theme(catppuccin_egui::MACCHIATO)
                // }
                if ui.small_button("Debug: chart").clicked() {
                    // let values = self.store.lock().unwrap().chart_data.clone();
                    // dbg!(values);
                    eprintln!("small button")
                }
            })
        });
    });
}
