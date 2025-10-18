use crate::debug::model_viewer::ModelViewer;

mod model_viewer;

pub struct DebugManager {
    pub show_debug_pane: bool,
    pub show_debug_window: bool,
    model_viewer: ModelViewer,
}

impl Default for DebugManager {
    fn default() -> Self {
        Self {
            model_viewer: ModelViewer::default(),
            show_debug_pane: false,
            show_debug_window: false,
        }
    }
}

impl DebugManager {
    pub fn show_debug_ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menu bar
        egui::TopBottomPanel::top("debug_menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(&mut self.show_debug_pane, "Debug Pane");
                ui.toggle_value(&mut self.show_debug_window, "Debug Window");
            });
        });
    }

    pub fn show_debug_pane(&mut self, ctx: &egui::Context) {
        if self.show_debug_pane {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.model_viewer.show(ctx, ui);
            });
        }

        if self.show_debug_window {
            egui::Window::new("Debug Window").show(ctx, |ui| {
                let fps = ctx.input(|i| i.stable_dt).recip();
                ui.label(format!("FPS: {:.1}", fps));
            });
        }
    }
}
