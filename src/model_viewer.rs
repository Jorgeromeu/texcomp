use crate::model_asset::MeshModel;
use crate::viewer::ViewerWidget;

pub struct ModelViewerWidget;

impl Default for ModelViewerWidget {
    fn default() -> Self {
        ModelViewerWidget {}
    }
}

impl ViewerWidget<MeshModel> for ModelViewerWidget {
    fn show_viewer(&mut self, ui: &mut egui::Ui, asset: &mut MeshModel) {
        ui.label(format!("Verts:{}", asset.verts.len()));
    }

    fn show_info(&mut self, ui: &mut egui::Ui) {}

    fn show_help(&mut self, ui: &mut egui::Ui) {}
}
