use super::{backpack::Backpack};

pub struct Player {
    pub name: String,
    pub position: (f32, f32, f32),
    pub backpack: Backpack
}