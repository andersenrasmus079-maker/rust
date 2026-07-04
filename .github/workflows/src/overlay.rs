use egui::Color32;
use crate::sdk::EspConfig;

pub struct EspOverlay {
    pub config: EspConfig,
}

impl EspOverlay {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            config: EspConfig {
                show_boxes: true,
                show_health: true,
                show_distance: true,
                show_ui: true,
                esp_color: Color32::from_rgb(0, 255, 0),
            },
        }
    }
}

impl eframe::App for EspOverlay {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Insert)) {
            self.config.show_ui = !self.config.show_ui;
        }

        if self.config.show_ui {
            egui::Window::new("ESP Settings")
                .frame(egui::Frame::none()
                    .fill(egui::Color32::from_rgba_premultiplied(20, 20, 20, 200))
                    .rounding(8.0))
                .show(ctx, |ui| {
                    ui.heading("ESP");
                    ui.checkbox(&mut self.config.show_boxes, "Player Boxes");
                    ui.checkbox(&mut self.config.show_health, "Health Bars");
                    ui.checkbox(&mut self.config.show_distance, "Distance");
                    ui.label("Color:");
                    ui.color_edit_button_srgba(&mut self.config.esp_color);
                });
        }

        crate::renderer::draw_esp(ctx, &self.config);
    }
}
