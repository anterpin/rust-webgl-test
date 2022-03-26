use super::mesh::Mesh;
use super::texture::Texture;
use super::texture_coords::TextureCoords;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
pub fn quad(width: f32) -> Mesh {
    rectangle(width, width)
}

#[allow(dead_code)]
pub fn rectangle(width: f32, height: f32) -> Mesh {
    let hw = width / 2.0;
    let hh = height / 2.0;
    let vertices = vec![-hw, hh, 0.0, hw, hh, 0.0, -hw, -hh, 0.0, hw, -hh, 0.0];
    let indices = vec![0, 1, 2, 2, 1, 3];
    Mesh {
        vertices,
        indices,
        texture_coords: None,
        normals: None,
    }
}
#[allow(dead_code)]
pub fn cube() -> Mesh {
    let vertices = vec![
        -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, 0.5, -0.5, 0.5, 0.5,
        -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5,
    ];
    let indices = vec![
        0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7, 0, 1, 4, 1, 4, 5, 4, 2, 0, 6, 2, 4, 2, 6, 3, 6, 7, 3,
        3, 7, 5, 5, 3, 1,
    ];
    Mesh {
        vertices,
        indices,
        texture_coords: None,
        normals: None,
    }
}
#[allow(dead_code)]
pub fn line_rectangle(width: f32, height: f32) -> Mesh {
    let hw = width / 2.0;
    let hh = height / 2.0;

    let vertices = vec![-hw, hh, 0.0, hw, hh, 0.0, -hw, -hh, 0.0, hw, -hh, 0.0];
    let indices = vec![0, 1, 3, 2];

    Mesh {
        vertices,
        indices,
        texture_coords: None,
        normals: None,
    }
}
#[allow(dead_code)]
pub fn triangle() -> Mesh {
    let vertices = vec![
        0.0,
        1.0,
        0.0,
        -0.8660254037844386,
        -0.5,
        0.0,
        0.8660254037844386,
        -0.5,
        0.0,
    ];
    let indices = vec![0, 1, 2];
    Mesh {
        vertices,
        indices,
        texture_coords: None,
        normals: None,
    }
}

#[allow(dead_code)]
pub fn textured_rectangle(texture: Texture) -> Result<Mesh, JsValue> {
    let width = texture.width;
    let mut mesh = rectangle(width, width / texture.aspect_ratio());
    let text_coords = TextureCoords {
        texture,
        coords: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    };
    mesh.texture_coords = Some(text_coords);
    Ok(mesh)
}
