mod overlay;
mod memory;
mod sdk;
mod renderer;

use overlay::EspOverlay;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_fullscreen(true)
            .with_mouse_passthrough(true),
        ..Default::default()
    };

    eframe::run_native(
        "ESP",
        options,
        Box::new(|cc| Box::new(EspOverlay::new(cc))),
    ).unwrap();
}
