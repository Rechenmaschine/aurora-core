use std::f32::consts::PI;
use std::fs;
use std::fs::File;
use std::path::Path;
use serde::Deserialize;
use anyhow::{Context, Result};
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::primitives::Aabb;

#[derive(Copy, Clone, Debug, Deserialize)]
struct TerrainPixel {
    x: f32,
    y: f32,
    z: f32
}

impl From<&TerrainPixel> for Vec3 {
    fn from(value: &TerrainPixel) -> Self {
        Vec3::new(value.x, value.y, value.z)
    }
}

struct TerrainChunk {
    raw_data: Vec<TerrainPixel>
}

impl TerrainChunk {
    pub fn from_file(file: impl AsRef<Path>) -> Result<Self> {
        let mut file_reader = csv::ReaderBuilder::new()
            .delimiter(b' ')
            .from_reader(
            File::open(file).with_context(|| "Failed to open terrain chunk")?
        );

        let raw_data = file_reader.deserialize().map(|r| r.unwrap()).collect();

        Ok(Self {
            raw_data
        })
    }

    pub fn merge_chunks<'a>(chunks: Vec<Self>) -> Self {
        let capacity = chunks.as_slice().iter().map(|c| c.raw_data.len()).sum();

        let mut raw_data = Vec::with_capacity(capacity);

        for mut chunk in chunks {
            raw_data.append(&mut chunk.raw_data);
        }

        TerrainChunk {
            raw_data
        }
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

    pub fn get_pixel_count(&self) -> usize {
        self.raw_data.len()
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
        //assert_eq!(x_count * y_count, value.raw_data.len(), "Grid Spacing is probably incorrect");

        let num_tris = (x_count - 1) * (y_count - 1) * 2;

        let mut vertices = vec![[0.0, 0.0, 0.0]; x_count * y_count];
        let mut normals = vec![[0.0, 0.0, 1.0]; x_count * y_count];
        let mut uv_coords = vec![[0.0, 0.0]; x_count * y_count];

        for pixel in value.raw_data {
            let rel_coord = Vec3::from(&pixel) - Vec3::from(origin);
            let x = (rel_coord.x / grid_spacing) as usize;
            let y = (rel_coord.y / grid_spacing) as usize;
            vertices[x * y_count + y] = rel_coord.into();

            // TODO: Do proper vertex normal computation here
        }

        let mut indices = Vec::with_capacity(num_tris * 3);

        for x in 0..x_count {
            for y in 0..y_count {
                let compute_index = |c_x, c_y| (c_x * y_count + c_y) as u32;

                // TRI 1
                indices.push(compute_index(x, y));
                indices.push(compute_index(x+1, y));
                indices.push(compute_index(x+1, y+1));

                // TRI 2
                indices.push(compute_index(x, y));
                indices.push(compute_index(x+1, y+1));
                indices.push(compute_index(x, y+1));
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

fn load_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    info!("Loading Terrain chunks...");
    let chunks: Vec<_> = fs::read_dir("assets/terrain_data/wichlen")
        .unwrap()
        .map(|p| {
            let path = p.unwrap().path();
            info!("Loading chunk {}", path.display());
            TerrainChunk::from_file(path).unwrap()
        })
        .collect();

    info!("Merging Terrain chunks...");
    let merged_terrain_chunk = TerrainChunk::merge_chunks(chunks);

    info!("Bounding box of merged terrain: {:?}", merged_terrain_chunk.compute_bounding_box());
    info!("Loaded {} height field points.", merged_terrain_chunk.get_pixel_count());

    let terrain_mesh = meshes.add(merged_terrain_chunk.into());

    let material_non_emissive = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..default()
    });

    commands.spawn((PbrBundle {
        mesh: terrain_mesh,
        material: material_non_emissive,
        transform: Transform {
            rotation: Quat::from_rotation_x(PI/2.),
            ..default()
        },
        ..default()
    }, Terrain));
}

pub struct TerrainLoaderPlugin;

impl Plugin for TerrainLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_terrain);
    }
}