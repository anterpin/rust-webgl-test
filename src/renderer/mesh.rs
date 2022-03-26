use super::texture_coords::TextureCoords;

pub struct Mesh {
    pub vertices: Vec<f32>,
    pub texture_coords: Option<TextureCoords>,
    pub normals: Option<Vec<f32>>,
    pub indices: Vec<u32>,
}
