use crate::image::image::ImageAsset;
use anyhow::{Context, Result, bail};
use egui;

pub trait Asset {
    fn from_dropped_file(ctx: &egui::Context, file: &egui::DroppedFile) -> Result<Self>
    where
        Self: Sized;
}

pub enum AssetEnum {
    Image(ImageAsset),
}

impl Asset for AssetEnum {
    fn from_dropped_file(ctx: &egui::Context, file: &egui::DroppedFile) -> Result<Self>
    where
        Self: Sized,
    {
        let filename = file.name.clone();
        let extension = filename.rsplit('.').next().context("No File Extension")?;

        match extension {
            "png" | "jpg" | "jpeg" | "bmp" => {
                Ok(AssetEnum::Image(ImageAsset::from_dropped_file(ctx, file)?))
            }
            s => bail!("Unsupported format {s}"),
        }
    }
}
