use super::{texture::Texture, states::BlockState};

pub struct Block {
    pub position: (f32, f32, f32),
    pub textures: Texture,
    pub state: BlockState
}