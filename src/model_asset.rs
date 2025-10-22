use crate::asset::Asset;
use anyhow::{Context, Ok, bail};
use three_d::{self, CpuGeometry};
use three_d_asset;
use three_d_asset::Model;
use three_d_asset::io::RawAssets;

pub struct MeshModel {
    name: String,
    pub verts: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl Asset for MeshModel {
    fn from_dropped_file(ctx: &egui::Context, file: &egui::DroppedFile) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let name = file.name.clone();
        let bytes = file.bytes.as_ref().context("No bytes in dropped file")?;

        let mut raws = RawAssets::new();
        raws.insert(&name, bytes.to_vec());

        let model = raws
            .deserialize::<Model>(&file.name)
            .context("Failed to Deserialize")?;

        // let model = three_d_asset::io::deserialize::<Model>(bytes.to_vec())
        // .context("Failed to deserialize")?;
        let prim = model.geometries.get(0).context("No geometry in model")?;
        let geo = &prim.geometry;

        let CpuGeometry::Triangles(mesh) = geo else {
            bail!("Only triangle meshes are supported");
        };

        let positions = mesh
            .positions
            .to_f32()
            .iter()
            .map(|v| [v.x, v.y, v.z])
            .collect::<Vec<[f32; 3]>>();

        let indices = mesh.indices.to_u32().context("Require Indices")?;

        Ok(MeshModel {
            name,
            verts: positions,
            indices: indices,
        })
    }

    fn get_id(&self) -> &str {
        &self.name
    }
}
