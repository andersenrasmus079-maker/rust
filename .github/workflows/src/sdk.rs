use egui::Color32;
use glam::{Vec2, Vec3, Mat4};

#[derive(Clone, Copy)]
pub struct EspConfig {
    pub show_boxes: bool,
    pub show_health: bool,
    pub show_distance: bool,
    pub show_ui: bool,
    pub esp_color: Color32,
}

#[derive(Clone, Copy)]
pub struct PlayerEntity {
    pub position: Vec3,
    pub health: f32,
    pub max_health: f32,
    pub distance: f32,
}

pub fn world_to_screen(
    world: Vec3,
    view_matrix: Mat4,
    screen_w: f32,
    screen_h: f32,
) -> Option<Vec2> {
    let w = world.x * view_matrix.x_axis.w
          + world.y * view_matrix.y_axis.w
          + world.z * view_matrix.z_axis.w
          + view_matrix.w_axis.w;
    
    if w < 0.1 {
        return None;
    }

    let x = world.x * view_matrix.x_axis.x
          + world.y * view_matrix.y_axis.x
          + world.z * view_matrix.z_axis.x
          + view_matrix.w_axis.x;
          
    let y = world.x * view_matrix.x_axis.y
          + world.y * view_matrix.y_axis.y
          + world.z * view_matrix.z_axis.y
          + view_matrix.w_axis.y;

    let ndc_x = x / w;
    let ndc_y = y / w;

    let screen_x = (screen_w / 2.0) * (ndc_x + 1.0);
    let screen_y = (screen_h / 2.0) * (1.0 - ndc_y);

    Some(Vec2::new(screen_x, screen_y))
}
