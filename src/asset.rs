use crate::image::image::ImageAsset;
use crate::model_asset::MeshModel;
use anyhow::{Context, Result, bail};
use egui;

pub trait Asset {
    fn from_dropped_file(ctx: &egui::Context, file: &egui::DroppedFile) -> Result<Self>
    where
        Self: Sized;

    fn get_id(&self) -> &str;
}

pub enum AssetEnum {
    Image(ImageAsset),
    Model(MeshModel),
}

impl Asset for AssetEnum {
    fn get_id(&self) -> &str {
        match self {
            AssetEnum::Image(image_asset) => &image_asset.get_id(),
            AssetEnum::Model(model_asset) => &model_asset.get_id(),
        }
    }

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
            "obj" | "glb" => Ok(AssetEnum::Model(MeshModel::from_dropped_file(ctx, file)?)),
            s => bail!("Unsupported format {s}"),
        }
    }
}
