use bevy::{color::{Color, ColorToComponents}, ecs::{component::Component, entity::Entity}, math::IVec3, utils::HashMap};

use crate::MeshData;

#[derive(Component)]
pub struct Block {
    pub color: Color,
}

#[derive(Component)]
pub struct Transparent;

#[derive(Component, PartialEq, Eq, Hash)]
pub struct LocalPosition(pub IVec3);

pub fn get_visible_voxel_faces(
    position: IVec3,
    blocks: &HashMap<IVec3, Entity>,
    transparency_cache: &HashMap<Entity, bool>,
) -> u8 {
    const NEIGHBOUR_OFFSETS: [IVec3; 6] = [
        IVec3::NEG_Z, // Front (-Z) - First in cube data
        IVec3::Z,     // Back (+Z)  - Second in cube data
        IVec3::X,     // Right (+X) - Third in cube data
        IVec3::NEG_X, // Left (-X)  - Fourth in cube data
        IVec3::Y,     // Top (+Y)   - Fifth in cube data
        IVec3::NEG_Y  // Bottom (-Y) - Sixth in cube data
    ];

    let mut mask = 0u8;
    for (index, offset) in NEIGHBOUR_OFFSETS.iter().enumerate() {
        let neighbour_pos = position + offset;

        let is_visible = match blocks.get(&neighbour_pos) {
            Some(neighbour_entity) => *transparency_cache.get(neighbour_entity).unwrap_or(&true),
            None => true
        };

        if is_visible {
            mask |= 1 << index;
        }

    }

    mask
}

pub fn generate_voxel_mesh(
    position: IVec3,
    color: Color,
    face_visibility_mask: u8
) -> MeshData {
    const CUBE_VERTICES: [[f32; 3]; 24] = [
        // Front face (-Z)
        [-0.5, -0.5, -0.5], // 0
        [-0.5,  0.5, -0.5], // 1
        [ 0.5,  0.5, -0.5], // 2
        [ 0.5, -0.5, -0.5], // 3
        
        // Back face (+Z)
        [-0.5, -0.5,  0.5], // 4
        [-0.5,  0.5,  0.5], // 5
        [ 0.5,  0.5,  0.5], // 6
        [ 0.5, -0.5,  0.5], // 7
        
        // Right face (+X)
        [ 0.5, -0.5, -0.5], // 8
        [ 0.5, -0.5,  0.5], // 9
        [ 0.5,  0.5,  0.5], // 10
        [ 0.5,  0.5, -0.5], // 11
        
        // Left face (-X)
        [-0.5, -0.5, -0.5], // 12
        [-0.5, -0.5,  0.5], // 13
        [-0.5,  0.5,  0.5], // 14
        [-0.5,  0.5, -0.5], // 15
        
        // Top face (+Y)
        [-0.5,  0.5, -0.5], // 16
        [ 0.5,  0.5, -0.5], // 17
        [ 0.5,  0.5,  0.5], // 18
        [-0.5,  0.5,  0.5], // 19
        
        // Bottom face (-Y)
        [-0.5, -0.5, -0.5], // 20
        [ 0.5, -0.5, -0.5], // 21
        [ 0.5, -0.5,  0.5], // 22
        [-0.5, -0.5,  0.5], // 23
    ];
    
    // Corrected normals
    const CUBE_NORMALS: [[f32; 3]; 6] = [
        [ 0.0,  0.0, -1.0], // Front (-Z)
        [ 0.0,  0.0,  1.0], // Back (+Z)
        [ 1.0,  0.0,  0.0], // Right (+X)
        [-1.0,  0.0,  0.0], // Left (-X)
        [ 0.0,  1.0,  0.0], // Top (+Y)
        [ 0.0, -1.0,  0.0], // Bottom (-Y)
    ];

    let mut data = MeshData::default();

    let offset = position.as_vec3();

    for index in 0..6 {
        let is_face_visible = face_visibility_mask & (1 << index) != 0;
        if !is_face_visible { continue; }

        let vertices_start = index * 4;

        let vertices = [
            CUBE_VERTICES[vertices_start + 0],
            CUBE_VERTICES[vertices_start + 1],
            CUBE_VERTICES[vertices_start + 2],
            CUBE_VERTICES[vertices_start + 3],
        ];

        let normal = CUBE_NORMALS[index];

        for vertex in vertices {
            data.vertices.push([
                vertex[0] + offset.x,
                vertex[1] + offset.y,
                vertex[2] + offset.z,
            ]);

            data.normals.push(normal);
            data.colors.push(color.to_srgba().to_f32_array());
        }

        let face_indices = match index {
            0 | 3 | 5 => [0, 1, 2, 2, 3, 0],
            _ => [0, 3, 2, 2, 1, 0]
        };
        data.indices.extend(face_indices.iter().map(|i| *i + data.vertex_offset));

        data.vertex_offset += 4;
    }

    data
}