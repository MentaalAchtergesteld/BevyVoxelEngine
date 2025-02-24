use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_flycam::prelude::*;
use chunk::{spawn_chunk, ChunkPlugin};
use rand::{rngs::StdRng, SeedableRng};

mod chunk;
mod block;

const CHUNK_SIZE: IVec3 = IVec3::new(16, 16, 16);
const WORLD_SIZE: IVec3 = IVec3::new(16,  8, 16);

fn main() {
    App::new()
        .insert_resource(GameRng::new(420))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(ChunkPlugin)
        // .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, spawn_world)
        .run();
}

#[derive(Default)]
struct MeshData {
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
    vertex_offset: u32,
}

impl MeshData {
    fn merge(&mut self, other: MeshData) {
        let index_offset = self.vertices.len() as u32;

        self.vertices.extend(other.vertices);
        self.normals.extend(other.normals);
        self.colors.extend(other.colors);
        self.indices.extend(other.indices.iter().map(|i| i + index_offset));
    }
}

#[derive(Resource)]
struct GameRng(StdRng);

impl GameRng {
    fn new(seed: u64) -> Self {
        Self(StdRng::seed_from_u64(seed))
    }
}

fn spawn_world(
    mut commands: Commands,
    mut rng: ResMut<GameRng>
) {
    for x in 0..WORLD_SIZE.x {
        for y in 0..WORLD_SIZE.y {
            for z in 0..WORLD_SIZE.z {
                spawn_chunk(
                    CHUNK_SIZE,
                    IVec3::new(x, y, z),
                    &mut commands,
                    &mut rng.0,
                )
            }
        }
    }
}