use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::{Indices, PrimitiveTopology}};
use bracket_noise::prelude::FastNoise;
use rand::Rng;

use crate::{block::{generate_voxel_mesh, Block}, MeshData};

pub const CHUNK_SIZE: IVec3 = IVec3::new(32, 32, 32);

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_chunk_mesh);
    }
}

#[derive(Component)]
pub struct Chunk {
    position: IVec3,
    blocks: Vec<Vec<Vec<Block>>>,
}

impl Chunk {
    pub fn create_filled(position: IVec3, fill_block: Block) -> Chunk {
        Chunk {
            position,
            blocks: vec![vec![vec![fill_block; CHUNK_SIZE.x as usize]; CHUNK_SIZE.y as usize]; CHUNK_SIZE.z as usize]
        }
    }

    fn is_position_valid(position: IVec3) -> bool {
        position.x >= 0 && position.x < CHUNK_SIZE.x &&
        position.y >= 0 && position.y < CHUNK_SIZE.y &&
        position.z >= 0 && position.z < CHUNK_SIZE.z
    }

    pub fn get_block(&self, position: IVec3) -> Option<&Block> {
        if Chunk::is_position_valid(position) {
            Some(&self.blocks[position.x as usize][position.y as usize][position.z as usize])
        } else {
            None
        }
    }

    pub fn set_block(&mut self, position: IVec3, block: Block) -> bool {
        if Chunk::is_position_valid(position) {
            self.blocks[position.x as usize][position.y as usize][position.z as usize] = block;

            true
        } else {
            false
        }
    }
}

#[derive(Component)]
pub struct UpdateMesh;

pub fn spawn_chunk(
    chunk_position: IVec3,

    minimum_height: i32,
    maximum_height: i32,

    commands: &mut Commands,
    rng: &mut impl Rng,
    noise: &mut FastNoise,
) {
    let air_block = Block { transparent: true, color: None };
    let mut chunk = Chunk::create_filled(chunk_position, air_block);

    let world_position = chunk_position * CHUNK_SIZE;

    for x in 0..CHUNK_SIZE.x {
        for z in 0..CHUNK_SIZE.z {

            let noise_x = (world_position.x + x) as f32 * 0.05;
            let noise_z = (world_position.z + z) as f32 * 0.05;

            let noise_value = noise.get_noise(noise_x, noise_z);
            let normalized_noise = (noise_value + 1.0) / 2.0;
            let height = (minimum_height as f32 + normalized_noise * ((maximum_height - minimum_height) as f32)) as i32;

            for y in 0..CHUNK_SIZE.y {
                if world_position.y + y <= height {
                    let block = Block {
                        transparent: false,
                        color: Some(Color::srgb(rng.random(), rng.random(), rng.random()))
                    };
    
                    chunk.set_block(IVec3::new(x, y, z), block);
                }
            }
        }
    }

    commands.spawn((
        Transform::from_translation((chunk_position * CHUNK_SIZE).as_vec3()),
        chunk,
        UpdateMesh,
        Name::new("Chunk")
    ));
}

fn update_chunk_mesh(
    mut commands: Commands,
    chunk_query: Query<(Entity, &Chunk), With<UpdateMesh>>,

    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, chunk) in &chunk_query {
        let mut entity_commands = commands.entity(entity);

        let mut chunk_mesh_data = MeshData::default();

        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                for z in 0..CHUNK_SIZE.z {
                    let block_position = IVec3::new(x, y, z);
                    if let Some(block) = chunk.get_block(block_position) {
                        if block.transparent { continue; }

                        let color = block.color.unwrap_or(Color::WHITE);
                        let block_mesh_data = generate_voxel_mesh(block_position, color, &chunk);

                        chunk_mesh_data.merge(block_mesh_data);
                    }
                }
            }
        }

        let mesh = meshes.add(Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, chunk_mesh_data.vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, chunk_mesh_data.normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, chunk_mesh_data.colors)
            .with_inserted_indices(Indices::U32(chunk_mesh_data.indices))
        );

        let material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        });

        entity_commands.insert((
            MeshMaterial3d(material),
            Mesh3d(mesh)
        ));

        entity_commands.remove::<UpdateMesh>();
    }
}