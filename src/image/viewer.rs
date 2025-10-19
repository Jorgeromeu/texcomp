use std::collections::HashMap;

use crate::image::image::ImageAsset;

#[derive(Hash, Eq, PartialEq, Clone)]
struct TextureDescriptor {
    id: String,
    filter_mode: egui::TextureFilter,
}

struct TextureCache {
    textures: HashMap<TextureDescriptor, egui::TextureHandle>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn get_texture(
        &mut self,
        ctx: &egui::Context,
        asset: &ImageAsset,
        descriptor: TextureDescriptor,
    ) -> &egui::TextureHandle {
        self.textures
            .entry(descriptor.clone())
            .or_insert_with(|| asset.to_texture(ctx, &descriptor.id, descriptor.filter_mode))
    }
}

pub struct ImageViewerWidget {
    zoom: f32,
    pan_offset: egui::Vec2,
    zoom_min: f32,
    zoom_max: f32,
    filter_mode: egui::TextureFilter,
    texture_cache: TextureCache,
}

impl Default for ImageViewerWidget {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            zoom_max: 20.0,
            zoom_min: 0.1,
            pan_offset: egui::Vec2::ZERO,
            filter_mode: egui::TextureFilter::Nearest,
            texture_cache: TextureCache::new(),
        }
    }
}

impl ImageViewerWidget {
    pub fn show_viewer(&mut self, ui: &mut egui::Ui, asset: &ImageAsset) {
        let image_size = asset.size();

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
                self.zoom = self.zoom.clamp(self.zoom_min, self.zoom_max);
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

        let texture_desc = TextureDescriptor {
            id: asset.id.clone(),
            filter_mode: self.filter_mode,
        };

        let texture = self
            .texture_cache
            .get_texture(ui.ctx(), asset, texture_desc);

        // Draw the image
        painter.image(
            texture.id(),
            image_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }

    pub fn show_info(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::Slider::new(&mut self.zoom, self.zoom_min..=self.zoom_max)
                .logarithmic(true)
                .text("Zoom")
                .suffix("x"),
        );
        ui.horizontal(|ui| {
            ui.label("Filter Mode:");
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
