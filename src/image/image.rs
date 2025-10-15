use crate::asset::Asset;
use anyhow::{Context, Result};

use egui::{self};

pub struct ImageAsset {
    pub texture: egui::TextureHandle,
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

        let opt = egui::TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
            ..Default::default()
        };

        let texture = ctx.load_texture(&file.name, color_image, opt);

        Ok(ImageAsset { texture })
    }
}
