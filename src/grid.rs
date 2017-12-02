extern crate nalgebra as na;

use sprite::{SpriteFactory, Sprite};

use std::collections::HashMap;

use rand::{thread_rng, Rng};

use glium;

use std::sync::Arc;


const BLOCK_SIZE: f32 = 32.;


#[derive(Hash, PartialEq, Eq, Clone)]
pub enum BlockType {
    Stone,
    StoneLadder
}


impl BlockType {
    pub fn is_colliding(&self) -> bool {
        match *self {
            _ => true
        }
    }

    pub fn is_climbable(&self) -> bool {
        match *self {
            BlockType::StoneLadder => true,
            _ => false
        }
    }
}

fn sprite_for_block(
        block_type: &BlockType,
        position: na::Vector2<i32>,
        sprite_factory: &SpriteFactory,
        textures: &HashMap<BlockType, Vec<Arc<glium::texture::SrgbTexture2d>>>
    ) -> Sprite
{
    let mut rng = thread_rng();
    let texture = rng.choose(textures.get(block_type).unwrap()).unwrap();

    let mut sprite = sprite_factory.create_sprite(texture.clone());
    sprite.set_position(na::convert::<_, na::Vector2<f32>>(position) * BLOCK_SIZE);
    sprite
}

pub struct Block {
    pub block_type: BlockType,
    pub sprite: Sprite
}

pub struct Grid {
    pub blocks: HashMap<na::Vector2<i32>, Block>
}

impl Grid {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new()
        }
    }
    pub fn add_prefab(
            mut self,
            blocks: Vec<(BlockType, na::Vector2<i32>)>,
            offset: na::Vector2<i32>,
            sprite_factory: &SpriteFactory,
            textures: &HashMap<BlockType, Vec<Arc<glium::texture::SrgbTexture2d>>>
        )
    {
        let actual_pos = blocks.iter()
            .map(|&(ref t, ref pos)| (t, pos+offset))
            .collect::<Vec<_>>();

        for (block_type, pos) in actual_pos {
            let sprite = sprite_for_block(&block_type, pos, sprite_factory, textures);
            self.blocks.insert(pos, Block{block_type: block_type.clone(), sprite});
        }
    }
}
