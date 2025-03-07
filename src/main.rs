use std::f32::consts::PI;

use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_flycam::prelude::*;
use bracket_noise::prelude::{FastNoise, NoiseType};
use chunk::{spawn_chunk, ChunkPlugin};
use rand::{rngs::StdRng, SeedableRng};
use world::World;

mod chunk;
mod block;
mod world;

pub const CHUNK_SIZE: IVec3 = IVec3::new(32, 32, 32);
pub const WORLD_SIZE: IVec3 = IVec3::new(48,  48, 48);
const WORLD_MINIMUM_HEIGHT: i32 = 40;
const WORLD_MAXIMUM_HEIGHT: i32 = 150;

fn main() {
    App::new()
        .insert_resource(GameNoise::new(420, 0.1))
        .insert_resource(GameRng::new(420))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)
        .add_systems(Startup, setup)
        .add_plugins(ChunkPlugin)
        // .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, spawn_world)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 200.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },

    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(128., 256., 128.),
        FlyCam, // From bevy_flycam
        SpotLight {
            color: Color::WHITE,
            intensity: 1600.0, // Lumens
            range: 100.0,
            shadows_enabled: true,
            inner_angle: 0.5,
            outer_angle: 0.6,
            ..default()
        }
    ));
}

#[derive(Default)]
struct MeshData {
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
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

#[derive(Resource)]
struct GameNoise(FastNoise);

impl GameNoise {
    fn new(seed: u64, frequency: f32) -> Self {
        let mut noise = FastNoise::seeded(seed);
        noise.set_noise_type(NoiseType::Perlin);
        noise.set_frequency(frequency);

        Self(noise)
    }
}

fn spawn_world(
    mut commands: Commands,
    mut noise: ResMut<GameNoise>
) {
    let mut world = World::default();

    for x in 0..WORLD_SIZE.x {
        for y in 0..WORLD_SIZE.y {
            for z in 0..WORLD_SIZE.z {
                let chunk = spawn_chunk(
                    IVec3::new(x, y, z),

                    WORLD_MINIMUM_HEIGHT,
                    WORLD_MAXIMUM_HEIGHT,

                    &mut commands,
                    &mut noise.0,
                );
                println!("Generating chunk: {} {} {}", x, y, z);
                world.chunks.insert(IVec3::new(x, y, z), chunk);
            }
        }
    }

    commands.insert_resource(world);
}