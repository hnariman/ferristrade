use catppuccin_egui::{Theme, MOCHA};
use egui::Color32;

#[derive(Debug)]
pub struct Config {
    up_color: Color32,
    down_color: Color32,
    chart_refresh: usize,
    theme: Theme,
    pub zoom: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            up_color: Color32::from_rgb(0, 255, 0),
            down_color: Color32::from_rgb(255, 0, 0),
            chart_refresh: 10,
            theme: MOCHA,
            zoom: 2.0,
        }
    }
}

impl Config {
    fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn zoom_in(&mut self) {
        if self.zoom > 4.0 {
            return;
        }
        self.zoom += 0.2;
    }
    pub fn zoom_out(&mut self) {
        if self.zoom < 1.0 {
            return;
        }
        self.zoom -= 0.2;
    }
}
