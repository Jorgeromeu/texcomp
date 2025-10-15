pub trait ViewerWidget<T> {
    fn show_viewer(&mut self, ui: &mut egui::Ui, asset: &T);

    fn show_info(&mut self, ui: &mut egui::Ui);
}
