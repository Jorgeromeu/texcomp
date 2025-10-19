use anyhow::bail;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::asset::Asset;
use anyhow::{Context, Ok, Result};
use egui::{self};

pub struct ImageAsset {
    pub id: String,
    pub image: egui::ColorImage,
    texture_cache: HashMap<egui::TextureFilter, egui::TextureHandle>,
    file_path: Option<PathBuf>,
}

impl ImageAsset {
    pub fn get_texture(
        &mut self,
        ctx: &egui::Context,
        filter_mode: egui::TextureFilter,
    ) -> &egui::TextureHandle {
        self.texture_cache.entry(filter_mode).or_insert_with(|| {
            let opt = egui::TextureOptions {
                magnification: filter_mode,
                minification: filter_mode,
                ..Default::default()
            };
            ctx.load_texture(&self.id, self.image.clone(), opt)
        })
    }

    pub fn image_from_bytes(bytes: &[u8]) -> Result<egui::ColorImage> {
        let image = image::load_from_memory(bytes).context("Failed to load image")?;
        let size = [image.width() as usize, image.height() as usize];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
        Ok(color_image)
    }

    pub fn image_size(&self) -> egui::Vec2 {
        egui::vec2(self.image.width() as f32, self.image.height() as f32)
    }
}

impl Asset for ImageAsset {
    fn from_dropped_file(ctx: &egui::Context, file: &egui::DroppedFile) -> Result<Self>
    where
        Self: Sized,
    {
        let bytes = file.bytes.as_ref().context("No file data")?;
        let color_image = Self::image_from_bytes(bytes)?;

        Ok(ImageAsset {
            image: color_image,
            id: file.name.clone(),
            texture_cache: HashMap::new(),
            file_path: file.path.clone(),
        })
    }

    fn get_id(&self) -> &str {
        return &self.id;
    }
}
