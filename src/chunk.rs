use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::{Indices, PrimitiveTopology}, utils::HashMap};
use rand::Rng;

use crate::{block::{generate_voxel_mesh, get_visible_voxel_faces, Block, LocalPosition, Transparent}, MeshData};

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
    size: IVec3,
    blocks: HashMap<IVec3, Entity>,
}

#[derive(Component)]
pub struct UpdateMesh;

pub fn spawn_chunk(
    chunk_size: IVec3,
    chunk_position: IVec3,
    commands: &mut Commands,
    rng: &mut impl Rng,
) {
    let mut blocks = HashMap::new();

    for x in 0..chunk_size.x {
        for z in 0..chunk_size.z {
            for y in 0..chunk_size.y {
                let local_position = IVec3::new(x, y, z);
                
                let block = commands.spawn((
                    LocalPosition(local_position),
                    Block {
                        color: Color::srgb(rng.random(), rng.random(), rng.random())
                    },
                    Name::new("Block")
                )).id();

                // if rng.random_bool(0.25) {
                //     commands.entity(block).insert(Transparent);
                // }

                blocks.insert(local_position, block);
            }
        }
    }

    let chunk = Chunk {
        position: chunk_position,
        size: chunk_size,
        blocks
    };

    commands.spawn((
        Transform::from_translation((chunk_position * chunk_size).as_vec3()),
        chunk,
        UpdateMesh,
        Name::new("Chunk")
    ));
}

fn update_chunk_mesh(
    mut commands: Commands,
    chunk_query: Query<(Entity, &Chunk), With<UpdateMesh>>,
    block_query: Query<(&Block, Option<&Transparent>)>,

    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, chunk) in &chunk_query {
        let transparency_cache = chunk.blocks.iter()
            .map(|(_, entity)| {
                let is_transparent = match block_query.get(*entity) {
                    Ok((_, transparent)) => transparent.is_some(),
                    Err(_) => false 
                };

                (*entity, is_transparent)
            })
            .collect::<HashMap<Entity, bool>>();

        let mut entity_commands = commands.entity(entity);

        let mut chunk_mesh_data = MeshData::default();

        for(block_pos, block_entity) in chunk.blocks.iter() {
            let (block, transparent) = if let Ok(entity) = block_query.get(*block_entity) {
                entity
            } else {
                continue;
            };

            if transparent.is_some() { continue; }

            let color = block.color;

            let face_visibility_mask = get_visible_voxel_faces(*block_pos, &chunk.blocks, &transparency_cache);
            let voxel_mesh_data = generate_voxel_mesh(*block_pos, color, face_visibility_mask);

            chunk_mesh_data.merge(voxel_mesh_data);
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