use egui;

pub trait InputStateExt {
    fn any_pressed(&self, keys: &[egui::Key]) -> bool;
}

impl InputStateExt for egui::InputState {
    fn any_pressed(&self, keys: &[egui::Key]) -> bool {
        keys.iter().any(|&k| self.key_pressed(k))
    }
}
