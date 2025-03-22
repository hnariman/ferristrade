pub fn right_panel(ctx: &egui::Context) {
    egui::SidePanel::right("News")
        .min_width(300.0)
        .show(ctx, |ui| {
            ui.heading("News");

            let available_height = ui.available_height();
            let top_height = available_height * 0.8;
            let bottom_height = available_height - top_height;

            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), top_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.vertical(|ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            // for item in self.store.lock().unwrap().news.lock().unwrap().clone() {
                            //     ui.label(&item.title);
                            //     ui.strong(&item.title);
                            //     ui.separator();
                            // }
                        })
                    })
                },
            );

            ui.separator();
            ui.heading("Market Sentiment Outlook");
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_height(), bottom_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.vertical(|ui| ui.strong("and some market analysis text here from AI agent"))
                },
            )
        });
}
