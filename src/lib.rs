extern crate wasm_bindgen;
extern crate web_sys;

mod alg;
mod renderer;
use wasm_bindgen::prelude::*;

use alg::{Mat4, Vec4};
use renderer::Renderer;
use renderer::{meshes, Element, Shader, ShaderProgram, StdUniforms, Texture, Vao};
use std::panic;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub async fn init() -> Result<(), JsValue> {
    panic::set_hook(Box::new(|panic_info| {
        error(&format!("panic occurred {:?}", panic_info));
    }));
    let renderer = Renderer::get_instance();
    renderer.camera.set_position(Vec4([0.0, 0.0, 1.0, 1.0]));
    let context = renderer.get_context_instance();
    context.line_width(5.0);

    let program: ShaderProgram<StdUniforms> =
        ShaderProgram::from_file_name("shader.vert", "shader.frag").await?;
    let program = Shader::new(program);
    renderer.set_std_program(program);

    let program: ShaderProgram<StdUniforms> =
        ShaderProgram::from_file_name("line.vert", "line.frag").await?;
    let program = Shader::new(program);
    renderer.set_line_program(program);
    renderer.create_plane("squidgame.png").await?;

    Ok(())
}

#[wasm_bindgen]
pub fn get_width(vao: &Vao) -> f32 {
    let texture_coords = vao.get_mesh().texture_coords.as_ref();
    if texture_coords.is_none() {
        return 0.0;
    }
    texture_coords.unwrap().texture.width
}
#[wasm_bindgen]
pub fn get_height(vao: &Vao) -> f32 {
    let texture_coords = vao.get_mesh().texture_coords.as_ref();
    if texture_coords.is_none() {
        return 0.0;
    }
    texture_coords.unwrap().texture.height
}

#[wasm_bindgen]
pub async fn create_quad(image_name: String) -> Result<Vao, JsValue> {
    let texture = Texture::new(&image_name).await?;
    let mesh = meshes::textured_rectangle(texture)?;
    Vao::new(mesh).map_err(|s| JsValue::from_str(&s))
}
#[wasm_bindgen]
pub fn create_quad_line(width: f32, height: f32) -> Result<Vao, JsValue> {
    let mesh = meshes::line_rectangle(width, height);
    Vao::new(mesh).map_err(|s| JsValue::from_str(&s))
}
#[wasm_bindgen]
pub fn prepare() {
    let renderer = Renderer::get_instance();
    renderer.prepare();
}

#[wasm_bindgen]
pub fn draw_line(vao: &Vao, rotation: f32) -> Result<(), JsValue> {
    let renderer = Renderer::get_instance();
    let program = renderer.get_line_program();
    program.use_program();
    let mat = Mat4::identity();
    // let mat = mat.rotate(&vec4::Vec4([0.0, 0.0, rotation, 1.0]));
    // let mat = mat.scale(&vec4::Vec4([0.5, 0.5, 0.5, 1.0]));
    program.load_tranformation_matrix(mat.data());
    program.load_view_matrix(renderer.camera.get_view_matrix().data());
    program.load_projection_matrix(renderer.get_projection_matrix().data());
    vao.draw_line_loop();
    Ok(())
}

#[wasm_bindgen]
pub fn draw() -> Result<(), JsValue> {
    let renderer = renderer::Renderer::get_instance();
    renderer.draw();
    Ok(())
}
