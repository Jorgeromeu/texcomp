use crate::image::image::ImageAsset;

pub struct ImageViewerWidget {
    filter_mode: egui::TextureFilter,
    state: ImageViewerState,
}

struct ImageViewerState {
    zoom: f32,
    pan_offset: egui::Vec2,
    image_size: egui::Vec2,
    viewer_rect: egui::Rect,
}

impl ImageViewerState {
    pub fn pan(&mut self, delta: egui::Vec2) {
        self.pan_offset += delta;
    }

    pub fn zoom(&mut self, zoom_delta: f32, target_pos: egui::Pos2) {
        let new_zoom = self.zoom * zoom_delta;

        // update pan
        let point_in_viewer = target_pos - self.viewer_rect.center();
        self.pan_offset = (self.pan_offset - point_in_viewer) * zoom_delta + point_in_viewer;

        // zoom in
        self.zoom = new_zoom;
    }

    pub fn get_image_rect(&self) -> egui::Rect {
        let zoomed_size = self.image_size * self.zoom;
        egui::Rect::from_center_size(self.viewer_rect.center() + self.pan_offset, zoomed_size)
    }

    pub fn get_zoom_percent(&self) -> f32 {
        self.zoom * 100.0
    }
}

impl Default for ImageViewerWidget {
    fn default() -> Self {
        Self {
            filter_mode: egui::TextureFilter::Nearest,
            state: ImageViewerState {
                zoom: 1.0,
                pan_offset: egui::Vec2::ZERO,
                image_size: egui::Vec2::ZERO,
                viewer_rect: egui::Rect::NOTHING,
            },
        }
    }
}

impl ImageViewerWidget {
    /// Draw the image given the current state
    fn draw_image(
        &self,
        painter: &egui::Painter,
        image_rect: egui::Rect,
        texture: &egui::TextureHandle,
    ) {
        painter.image(
            texture.id(),
            image_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }

    pub fn show_viewer(&mut self, ui: &mut egui::Ui, asset: &mut ImageAsset) {
        // Allocate space for viewer
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

        // setup state
        self.state.image_size = asset.image_size();
        self.state.viewer_rect = response.rect;

        // Handle scroll wheel zoom
        if response.hovered() {
            let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
            if scroll_delta.abs() > 0.0 {
                let zoom_delta = 1.0 + scroll_delta * 0.002;
                if let Some(hover_pos) = response.hover_pos() {
                    self.state.zoom(zoom_delta, hover_pos);
                }
            }
        }

        // Handle drag to pan
        if response.dragged() {
            self.state.pan(response.drag_delta());
        }

        // Draw Image
        let texture = asset.get_texture(ui.ctx(), self.filter_mode);
        let image_rect = self.state.get_image_rect();
        self.draw_image(&painter, image_rect, &texture);
    }

    pub fn show_info(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new(format!(
            "({}, {}) [{:.2}%]",
            self.state.image_size.x as u32,
            self.state.image_size.y as u32,
            self.state.get_zoom_percent()
        )));

        ui.horizontal(|ui| {
            ui.label("Interpolation:");
            ui.selectable_value(
                &mut self.filter_mode,
                egui::TextureFilter::Nearest,
                "Nearest",
            );
            ui.selectable_value(&mut self.filter_mode, egui::TextureFilter::Linear, "Linear");
        });
    }

    pub fn show_help(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::Label::new("Image Viewer Help:"));
        ui.add(egui::Label::new("- Scroll to zoom in/out"));
        ui.add(egui::Label::new("- Click and drag to pan the image"));
    }
}
