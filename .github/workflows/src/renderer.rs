use egui::{Color32, Context, Pos2, Rect, Stroke};
use crate::sdk::{EspConfig, PlayerEntity, world_to_screen};

pub fn draw_esp(ctx: &Context, config: &EspConfig) {
    let screen = ctx.screen_rect();
    
    egui::Area::new("esp_layer".into())
        .fixed_pos(Pos2::ZERO)
        .show(ctx, |ui| {
            let entities = get_entities_from_memory();
            let view_matrix = get_view_matrix_from_memory();
            
            for entity in entities {
                if let Some(screen_pos) = world_to_screen(
                    entity.position,
                    view_matrix,
                    screen.width(),
                    screen.height(),
                ) {
                    if config.show_boxes {
                        draw_box(ui, screen_pos, 60.0, 120.0, config.esp_color);
                    }
                    if config.show_health {
                        draw_health_bar(ui, screen_pos, entity.health, entity.max_health);
                    }
                    if config.show_distance {
                        draw_distance(ui, screen_pos, entity.distance, config.esp_color);
                    }
                }
            }
        });
}

fn draw_box(ui: &mut egui::Ui, pos: Pos2, width: f32, height: f32, color: Color32) {
    let rect = Rect::from_center_size(pos, egui::vec2(width, height));
    ui.painter().rect_stroke(rect, 0.0, Stroke::new(2.0, color));
}

fn draw_health_bar(ui: &mut egui::Ui, pos: Pos2, health: f32, max: f32) {
    let bar_width = 4.0;
    let bar_height = 60.0;
    let pct = (health / max).clamp(0.0, 1.0);
    
    let bg_rect = Rect::from_min_size(
        pos + egui::vec2(-35.0, -30.0),
        egui::vec2(bar_width, bar_height),
    );
    ui.painter().rect_filled(bg_rect, 0.0, Color32::BLACK);
    
    let fill_height = bar_height * pct;
    let fill_rect = Rect::from_min_size(
        pos + egui::vec2(-35.0, -30.0 + (bar_height - fill_height)),
        egui::vec2(bar_width, fill_height),
    );
    let color = if pct > 0.6 { Color32::GREEN } 
        else if pct > 0.3 { Color32::YELLOW } 
        else { Color32::RED };
    ui.painter().rect_filled(fill_rect, 0.0, color);
}

fn draw_distance(ui: &mut egui::Ui, pos: Pos2, distance: f32, color: Color32) {
    let text = format!("{:.0}m", distance);
    ui.painter().text(
        pos + egui::vec2(0.0, 40.0),
        egui::Align2::CENTER_TOP,
        text,
        egui::FontId::monospace(12.0),
        color,
    );
}

fn get_entities_from_memory() -> Vec<PlayerEntity> {
    vec![]
}

fn get_view_matrix_from_memory() -> glam::Mat4 {
    glam::Mat4::IDENTITY
}
