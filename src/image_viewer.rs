use eframe::egui;

// Reusable image viewer widget
pub struct ImageViewerWidget {
    zoom: f32,
    pan_offset: egui::Vec2,
}

impl Default for ImageViewerWidget {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan_offset: egui::Vec2::ZERO,
        }
    }
}

impl ImageViewerWidget {
    pub fn show(&mut self, ui: &mut egui::Ui, texture: &egui::TextureHandle) {
        let image_size = texture.size_vec2();

        // Allocate the entire available space for the image viewer
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

        // Handle scroll wheel zoom (no Ctrl needed!)
        if response.hovered() {
            let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
            if scroll_delta.abs() > 0.0 {
                let zoom_delta = 1.0 + scroll_delta * 0.002;

                // Zoom towards mouse cursor
                if let Some(hover_pos) = response.hover_pos() {
                    let viewer_rect = response.rect;

                    // Point in viewer space
                    let point_in_viewer = hover_pos - viewer_rect.center();

                    // Adjust pan to zoom towards cursor
                    self.pan_offset =
                        (self.pan_offset - point_in_viewer) * zoom_delta + point_in_viewer;
                }

                self.zoom *= zoom_delta;
                self.zoom = self.zoom.clamp(0.1, 20.0);
            }
        }

        // Handle drag to pan
        if response.dragged() {
            self.pan_offset += response.drag_delta();
        }

        // Calculate image rectangle
        let viewer_rect = response.rect;
        let viewer_center = viewer_rect.center();

        // Calculate zoomed image size (maintaining aspect ratio)
        let zoomed_size = image_size * self.zoom;

        // Center the image with pan offset
        let image_rect = egui::Rect::from_center_size(viewer_center + self.pan_offset, zoomed_size);

        // Draw checkerboard background (optional)
        draw_checkerboard(&painter, viewer_rect);

        // Draw the image
        painter.image(
            texture.id(),
            image_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}

fn draw_checkerboard(painter: &egui::Painter, rect: egui::Rect) {
    let checker_size = 16.0;
    let color1 = egui::Color32::from_gray(128);
    let color2 = egui::Color32::from_gray(160);

    let min_x = (rect.min.x / checker_size).floor() as i32;
    let max_x = (rect.max.x / checker_size).ceil() as i32;
    let min_y = (rect.min.y / checker_size).floor() as i32;
    let max_y = (rect.max.y / checker_size).ceil() as i32;

    for y in min_y..max_y {
        for x in min_x..max_x {
            let color = if (x + y) % 2 == 0 { color1 } else { color2 };
            let square_rect = egui::Rect::from_min_size(
                egui::pos2(x as f32 * checker_size, y as f32 * checker_size),
                egui::vec2(checker_size, checker_size),
            );
            painter.rect_filled(square_rect, 0.0, color);
        }
    }
}
