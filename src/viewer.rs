pub trait ViewerWidget<T> {
    fn show_viewer(&mut self, ui: &mut egui::Ui, asset: &mut T);

    fn show_info(&mut self, ui: &mut egui::Ui);

    fn show_help(&mut self, ui: &mut egui::Ui);
}
