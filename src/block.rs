use bevy::{color::{Color, ColorToComponents}, math::IVec3};

use crate::{chunk::Chunk, MeshData};

#[derive(Clone, Copy, Default)]
pub struct Block {
    pub transparent: bool,
    pub color: Option<Color>,
}

#[derive(Clone, Copy)]
enum Face {
    Front,
    Back,
    Left,
    Right,
    Bottom,
    Top
}

impl Face {
    fn all() -> [Face; 6] {
        [
            Self::Front,
            Self::Back,
            Self::Left,
            Self::Right,
            Self::Bottom,
            Self::Top
        ]
    }

    fn opposite(&self) -> Face {
        match self {
            Self::Front => Self::Back,
            Self::Back => Self::Front,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
            Self::Top => Self::Bottom
        }
    }

    fn as_ivec3(&self) -> IVec3 {
        match self {
            Self::Front => IVec3::NEG_Z,
            Self::Back => IVec3::Z,
            Self::Left => IVec3::NEG_X,
            Self::Right => IVec3::X,
            Self::Bottom => IVec3::NEG_Y,
            Self::Top => IVec3::Y
        }
    }
}

fn get_visible_block_faces(position: IVec3, chunk: &Chunk) -> Vec<Face> {
    let mut visible_faces = Vec::new();

    for face in Face::all() {
        let is_visible = match chunk.get_block(position + face.as_ivec3()) {
            Some(Block { transparent: true, .. }) | None => true,
            _ => false
        };

        if is_visible {
            visible_faces.push(face);
        }
    }

    visible_faces
}

fn get_block_face_mesh_data(position: IVec3, face: Face, color: Color) -> MeshData {
    let vertices = match face {
        Face::Front => [
            [-0.5, -0.5, -0.5], 
            [-0.5,  0.5, -0.5], 
            [ 0.5,  0.5, -0.5], 
            [ 0.5, -0.5, -0.5], 
        ],
        Face::Back => [
            [-0.5, -0.5,  0.5], 
            [-0.5,  0.5,  0.5], 
            [ 0.5,  0.5,  0.5], 
            [ 0.5, -0.5,  0.5], 
        ],
        Face::Right => [
            [ 0.5, -0.5, -0.5], 
            [ 0.5, -0.5,  0.5], 
            [ 0.5,  0.5,  0.5],
            [ 0.5,  0.5, -0.5],
        ],
        Face::Left => [
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5,  0.5],
            [-0.5,  0.5,  0.5],
            [-0.5,  0.5, -0.5],
        ],
        Face::Top => [
            [-0.5,  0.5, -0.5],
            [ 0.5,  0.5, -0.5],
            [ 0.5,  0.5,  0.5],
            [-0.5,  0.5,  0.5],
        ],
        Face::Bottom => [
            [-0.5, -0.5, -0.5],
            [ 0.5, -0.5, -0.5],
            [ 0.5, -0.5,  0.5],
            [-0.5, -0.5,  0.5],
        ]
    }.iter()
    .map(|vertex| {
        [
            vertex[0] + position.x as f32,
            vertex[1] + position.y as f32,
            vertex[2] + position.z as f32,
        ]
    }).collect::<Vec<[f32; 3]>>();

    let normal = face.as_ivec3().as_vec3().to_array();
    let normals = vec![normal; 4];

    let indices = match face {
        Face::Front | Face::Left | Face::Bottom => vec![0, 1, 2, 2, 3, 0],
        _ =>                                       vec![0, 3, 2, 2, 1, 0]
    };

    let colors = vec![color.to_srgba().to_f32_array(); 4];

    MeshData {
        vertices,
        normals,
        indices,
        colors,
    }
}

pub fn generate_voxel_mesh(
    position: IVec3,
    color: Color,
    chunk: &Chunk
) -> MeshData {
    let visible_faces = get_visible_block_faces(position, chunk);

    let mut block_mesh_data = MeshData::default();

    for face in visible_faces {
        block_mesh_data.merge(get_block_face_mesh_data(position, face, color));
    }

    block_mesh_data
}