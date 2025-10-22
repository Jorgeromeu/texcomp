use crate::image::image::ImageAsset;

pub struct ImageViewerWidget {
    filter_mode: egui::TextureFilter,
    state: ImageViewerState,
    zoom_box: Option<ZoomBox>,
}

struct ImageViewerState {
    zoom: f32,
    pan_offset: egui::Vec2,
    image_size: egui::Vec2,
    viewer_rect: egui::Rect,
}

struct ZoomBox {
    start_pos: egui::Pos2,
    current_pos: egui::Pos2,
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

    /// Zoom to fit a rectangle in the viewer
    pub fn zoom_to_rect(&mut self, viewer_rect: egui::Rect) {
        // Convert to image coordinates
        let image_min = self.viewer_to_image_pos(viewer_rect.min);
        let image_max = self.viewer_to_image_pos(viewer_rect.max);
        let image_rect_size = (image_max - image_min).abs();
        let image_rect_center = (image_min.to_vec2() + image_max.to_vec2()) / 2.0;

        // Calculate new zoom
        let viewer_size = self.viewer_rect.size();
        let new_zoom = (viewer_size.x / image_rect_size.x).min(viewer_size.y / image_rect_size.y);

        let image_center = self.image_size / 2.0;
        let offset_from_image_center = image_rect_center - image_center;

        self.zoom = new_zoom;
        self.pan_offset = -offset_from_image_center * new_zoom;
    }

    /// Fit the entire image to the viewer
    pub fn fit_to_viewer(&mut self) {
        let viewer_size = self.viewer_rect.size();

        // Calculate zoom to fit entire image in viewer
        let zoom_x = viewer_size.x / self.image_size.x;
        let zoom_y = viewer_size.y / self.image_size.y;

        // Use the smaller zoom to ensure entire image is visible
        self.zoom = zoom_x.min(zoom_y);
        self.pan_offset = egui::Vec2::ZERO;
    }

    /// Get the rectangle of the image in viewer coordinates
    pub fn get_image_rect(&self) -> egui::Rect {
        let zoomed_size = self.image_size * self.zoom;
        egui::Rect::from_center_size(self.viewer_rect.center() + self.pan_offset, zoomed_size)
    }

    pub fn get_zoom_percent(&self) -> f32 {
        self.zoom * 100.0
    }

    pub fn viewer_to_image_pos(&self, viewer_pos: egui::Pos2) -> egui::Pos2 {
        let image_rect = self.get_image_rect();
        let image_size = self.image_size;

        // Get position relative to image rect
        let x_ratio = (viewer_pos.x - image_rect.min.x) / image_rect.width();
        let y_ratio = (viewer_pos.y - image_rect.min.y) / image_rect.height();

        // Map to image coordinates
        egui::pos2(x_ratio * image_size.x, y_ratio * image_size.y)
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
            zoom_box: None,
        }
    }
}

impl ImageViewerWidget {
    const SCROLL_SPEED: f32 = 0.0015;

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

    /// Draw the zoom selection box
    fn draw_zoom_box(&self, painter: &egui::Painter, zoom_box: &ZoomBox) {
        let rect = egui::Rect::from_two_pos(zoom_box.start_pos, zoom_box.current_pos);

        // Draw semi-transparent fill
        painter.rect_filled(
            rect,
            0.0,
            egui::Color32::from_rgba_unmultiplied(100, 150, 255, 50),
        );

        // Draw border
        painter.rect_stroke(
            rect,
            0.0,
            egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 150, 255)),
            egui::StrokeKind::Inside,
        );
    }

    pub fn show_viewer(&mut self, ui: &mut egui::Ui, asset: &mut ImageAsset) {
        // Allocate space for viewer
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

        // setup state
        self.state.image_size = asset.image_size();
        self.state.viewer_rect = response.rect;

        // Handle fit to frame shortcut
        if response.hovered() {
            let f_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::F));
            if f_pressed {
                self.state.fit_to_viewer();
            }
        }

        // Handle scroll wheel zoom
        if response.hovered() {
            let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
            if scroll_delta.abs() > 0.0 {
                let zoom_delta = 1.0 + scroll_delta * Self::SCROLL_SPEED;
                if let Some(hover_pos) = response.hover_pos() {
                    self.state.zoom(zoom_delta, hover_pos);
                }
            }
        }

        // Handle drag to pan
        if response.dragged_by(egui::PointerButton::Primary) {
            self.state.pan(response.drag_delta());
        }

        // Initialize zoom box on right click drag
        if response.drag_started_by(egui::PointerButton::Secondary) {
            if let Some(start_pos) = response.interact_pointer_pos() {
                self.zoom_box = Some(ZoomBox {
                    start_pos,
                    current_pos: start_pos,
                });
            }
        }

        // While dragging, update zoom box current position
        if response.dragged_by(egui::PointerButton::Secondary) {
            if let Some(zoom_box) = &mut self.zoom_box {
                if let Some(current_pos) = response.interact_pointer_pos() {
                    zoom_box.current_pos = current_pos;
                }
            }
        }

        // Cancel zoom box on escape
        if self.zoom_box.is_some() {
            let escape_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::Escape));
            if escape_pressed {
                self.zoom_box = None;
            }
        }

        // apply zoom on release
        if response.drag_stopped_by(egui::PointerButton::Secondary) {
            if let Some(zoom_box) = self.zoom_box.take() {
                let rect = egui::Rect::from_two_pos(zoom_box.start_pos, zoom_box.current_pos);
                // Only zoom if the box has meaningful size
                if rect.width() > 5.0 && rect.height() > 5.0 {
                    self.state.zoom_to_rect(rect);
                }
            }
        }

        // Draw Image
        let texture = asset.get_texture(ui.ctx(), self.filter_mode);
        let image_rect = self.state.get_image_rect();
        self.draw_image(&painter, image_rect, &texture);

        // Draw zoom box if active
        if let Some(zoom_box) = &self.zoom_box {
            self.draw_zoom_box(&painter, zoom_box);
        }
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
        ui.add(egui::Label::new("- Left click and drag to pan the image"));
        ui.add(egui::Label::new(
            "- Right click and drag to select zoom area",
        ));
        ui.add(egui::Label::new("- Press F to fit image to frame"));
    }
}
