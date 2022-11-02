pub enum Texture {
    Normal([SingleTexture; 6]),
    Struct(StructTexture)
}

pub struct SingleTexture {}

pub struct StructTexture {}