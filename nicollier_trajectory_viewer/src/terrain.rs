use crate::coordinate_systems;
use anyhow::{Error, Result};
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::primitives::Aabb;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Deserialize)]
struct TerrainPixel {
    /// EAST coordinate
    x: f32,
    /// NORTH coordinate
    y: f32,
    /// UP coordinate
    z: f32,
}

impl From<&TerrainPixel> for Vec3 {
    fn from(value: &TerrainPixel) -> Self {
        Vec3::new(value.x, value.y, value.z)
    }
}

struct TerrainChunk {
    pub raw_data: Vec<TerrainPixel>,
}

impl TerrainChunk {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut file_reader = csv::ReaderBuilder::new().delimiter(b' ').from_reader(bytes);

        let raw_data = file_reader.deserialize().map(|r| r.unwrap()).collect();

        Ok(Self { raw_data })
    }

    pub fn compute_bounding_box(&self) -> Aabb {
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);

        for pixel in &self.raw_data {
            let p = Vec3::from(pixel);
            min = p.min(min);
            max = p.max(max);
        }

        Aabb::from_min_max(min, max)
    }
}

impl From<TerrainChunk> for Mesh {
    fn from(value: TerrainChunk) -> Self {
        let bbox = value.compute_bounding_box();
        let origin = bbox.min();
        let extents = bbox.max() - bbox.min();
        let x_len = extents.x;
        let y_len = extents.y;
        let grid_spacing = 0.5f32;
        let x_count = (x_len / grid_spacing) as usize + 1;
        let y_count = (y_len / grid_spacing) as usize + 1;
        assert_eq!(
            x_count * y_count,
            value.raw_data.len(),
            "Grid Spacing is probably incorrect"
        );

        let num_tris = (x_count - 1) * (y_count - 1) * 2;

        let mut vertices = vec![[0.0, 0.0, 0.0]; x_count * y_count];

        // Set normals to just be a unit vector in the y direction,
        // meshes are unlit anyway so no normal calculation needed
        let normals = vec![[0.0, 1.0, 0.0]; x_count * y_count];

        let mut uv_coords = vec![[0.0, 0.0]; x_count * y_count];

        for pixel in value.raw_data {
            let enu_coords = Vec3::from(&pixel);
            let relative_coords = enu_coords - Vec3::from(origin);
            let x = (relative_coords.x / grid_spacing) as usize;
            let y = (relative_coords.y / grid_spacing) as usize;

            // Convert from East-North-Up to Right-handed y-up coords (Bevy coordinate system, North-Up-East)
            let rh_yup_coords = coordinate_systems::enu_to_engine(enu_coords);
            vertices[x * y_count + y] = rh_yup_coords.into();

            uv_coords[x * y_count + y] = [
                x as f32 / x_count as f32,
                (y_count - y) as f32 / y_count as f32,
            ];
        }

        let mut indices = Vec::with_capacity(num_tris * 3);

        for x in 0..x_count - 1 {
            for y in 0..y_count - 1 {
                let compute_index = |c_x, c_y| (c_x * y_count + c_y) as u32;

                // TRI 1
                indices.push(compute_index(x, y));
                indices.push(compute_index(x + 1, y));
                indices.push(compute_index(x + 1, y + 1));

                // TRI 2
                indices.push(compute_index(x, y));
                indices.push(compute_index(x + 1, y + 1));
                indices.push(compute_index(x, y + 1));
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_coords);
        mesh.set_indices(Some(Indices::U32(indices)));
        return mesh;
    }
}

#[derive(Component)]
struct Terrain;

#[derive(Debug, Default)]
pub struct TerrainMeshLoader;

impl AssetLoader for TerrainMeshLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let chunk = TerrainChunk::from_bytes(bytes)?;
            let mesh = Mesh::from(chunk);
            load_context.set_default_asset(LoadedAsset::new(mesh));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["xyz"]
    }
}

#[derive(Default, Debug)]
struct TerrainChunkHandles {
    mesh: Option<Handle<Mesh>>,
    texture: Option<Handle<Image>>,
}

fn load_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    info!("Loading Terrain chunks...");
    let mut chunks = HashMap::new();
    let _loaded_folder = asset_server
        .load_folder("terrain_data/wichlen")
        .expect("Failed to load terrain assets");
    for h in _loaded_folder {
        let asset_path = asset_server
            .get_handle_path(h.clone())
            .unwrap()
            .path()
            .to_path_buf();

        let main_path = asset_path.as_path().parent().unwrap().to_path_buf();
        if !chunks.contains_key(&main_path) {
            chunks.insert(main_path.clone(), TerrainChunkHandles::default());
        }

        // UNWRAP: chunks will always contain main_path because of the previous block
        let handles = chunks.get_mut(&main_path).unwrap();

        if asset_path.as_path().extension().unwrap() == "xyz" {
            handles.mesh = Some(h.typed::<Mesh>());
        } else if asset_path.as_path().extension().unwrap() == "png" {
            handles.texture = Some(h.typed::<Image>());
        }
    }

    for (_, handles) in chunks {
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(handles.texture.unwrap().clone()),
            alpha_mode: AlphaMode::Opaque,
            unlit: true,
            ..default()
        });

        commands.spawn((
            PbrBundle {
                mesh: handles.mesh.unwrap().clone(),
                material,
                transform: Transform::IDENTITY,
                ..default()
            },
            Terrain,
        ));
    }
}

pub struct TerrainLoaderPlugin;

impl Plugin for TerrainLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<TerrainMeshLoader>()
            .add_systems(Startup, load_terrain);
    }
}
