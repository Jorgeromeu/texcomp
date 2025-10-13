use crate::app::NamedTexture;
use anyhow::{Context as _, Result, bail};
use gltf::Gltf;

pub enum AssetType {
    Image(NamedTexture),
    Gltf(NamedTexture),
}

fn load_image(ctx: &egui::Context, file: &egui::DroppedFile) -> Result<AssetType> {
    let bytes = file.bytes.as_ref().context("No file data")?;
    let image = image::load_from_memory(bytes).context("Failed to load image")?;

    let size = [image.width() as usize, image.height() as usize];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
    let texture = ctx.load_texture(&file.name, color_image, Default::default());

    Ok(AssetType::Image(NamedTexture {
        name: file.name.clone(),
        texture,
    }))
}

fn load_glb(ctx: &egui::Context, file: &egui::DroppedFile) -> Result<AssetType> {
    let bytes = file.bytes.as_ref().context("No file data")?;
    let gltf = Gltf::from_slice(bytes).context("Failed to parse GLB")?;
    let (_document, _buffers, images) = gltf::import_slice(bytes)?;

    let material = gltf.materials().next().context("No materials found")?;
    let texture_info = material
        .pbr_metallic_roughness()
        .base_color_texture()
        .context("No base color texture")?;

    let texture = texture_info.texture();
    let gltf_image = texture.source();
    let data = images.get(gltf_image.index()).context("Image not found")?;

    // data.pixels is already decoded! Use it directly
    let (width, height) = (data.width, data.height);

    // Convert based on format
    let color_image = match data.format {
        gltf::image::Format::R8G8B8 => {
            // RGB -> RGBA
            let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
            for chunk in data.pixels.chunks(3) {
                rgba.push(chunk[0]);
                rgba.push(chunk[1]);
                rgba.push(chunk[2]);
                rgba.push(255); // Alpha
            }
            egui::ColorImage::from_rgba_unmultiplied([width as usize, height as usize], &rgba)
        }
        gltf::image::Format::R8G8B8A8 => egui::ColorImage::from_rgba_unmultiplied(
            [width as usize, height as usize],
            &data.pixels,
        ),
        _ => bail!("Unsupported image format in GLB: {:?}", data.format),
    };

    let egui_texture = ctx.load_texture(&file.name, color_image, Default::default());

    Ok(AssetType::Gltf(NamedTexture {
        name: file.name.clone(),
        texture: egui_texture,
    }))
}

pub fn load_asset(ctx: &egui::Context, file: &egui::DroppedFile) -> Result<AssetType> {
    let filename = file.name.to_lowercase();
    let extension = filename.rsplit('.').next().context("No file extension")?;

    match extension {
        "png" | "jpg" | "jpeg" | "bmp" => load_image(ctx, file),
        "glb" => load_glb(ctx, file),
        _ => bail!("Unsupported format: {extension}"),
    }
}
