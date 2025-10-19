use crate::asset::Asset;
use anyhow::{Context, Result};
use egui::{self};

pub struct ImageAsset {
    pub id: String,
    pub image: egui::ColorImage,
}

impl ImageAsset {
    pub fn size(&self) -> egui::Vec2 {
        egui::vec2(self.image.size[0] as f32, self.image.size[1] as f32)
    }

    pub fn to_texture(
        &self,
        ctx: &egui::Context,
        name: &str,
        filter_mode: egui::TextureFilter,
    ) -> egui::TextureHandle {
        let opt = egui::TextureOptions {
            magnification: filter_mode,
            minification: filter_mode,
            ..Default::default()
        };

        ctx.load_texture(name, self.image.clone(), opt)
    }
}

impl Asset for ImageAsset {
    fn from_dropped_file(ctx: &egui::Context, file: &egui::DroppedFile) -> Result<Self>
    where
        Self: Sized,
    {
        let bytes = file.bytes.as_ref().context("No file data")?;
        let image = image::load_from_memory(bytes).context("Failed to load image")?;

        let size = [image.width() as usize, image.height() as usize];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        Ok(ImageAsset {
            image: color_image,
            id: file.name.clone(),
        })
    }

    fn get_id(&self) -> &str {
        return &self.id;
    }
}
