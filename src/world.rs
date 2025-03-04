use bevy::{ecs::{entity::Entity, system::{Query, Resource}}, math::IVec3, utils::HashMap};

use crate::{block::Block, chunk::Chunk, CHUNK_SIZE};

#[derive(Resource, Default)]
pub struct World {
    pub chunks: HashMap<IVec3, Entity>
}

impl World {
    pub fn get_block(&self, global_pos: IVec3, chunk_query: &Query<&Chunk>) -> Option<Block> {
        let chunk_pos = global_pos.div_euclid(CHUNK_SIZE);
        let block_pos = global_pos.rem_euclid(CHUNK_SIZE);

        self.chunks.get(&chunk_pos)
            .and_then(|e| chunk_query.get(*e).ok())
            .and_then(|chunk| chunk.get_block(block_pos))
    }
}