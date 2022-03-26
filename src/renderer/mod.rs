extern crate wasm_bindgen;
extern crate web_sys;

mod camera;
mod element;
mod loader;
mod mesh;
pub mod meshes;
mod shader_program;
mod texture;
mod texture_coords;
mod vao;

use super::alg::{utils, Mat4, Vec4};
pub use camera::*;
pub use element::*;
pub use mesh::*;
pub use shader_program::*;
pub use texture::*;
pub use texture_coords::*;
pub use vao::*;

use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

#[allow(dead_code)]
enum RenderMode {
    M2D,
    M3D,
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[derive(Serialize)]
struct ContextOptions {
    antialias: bool,
}

pub struct Renderer<'a> {
    context: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    pub camera: Camera,
    render_mode: RenderMode,
    std_program: Option<Shader>,
    line_program: Option<Shader>,
    plane: Option<Element>,
    selected: Option<&'a Element>,
    dragged: Option<&'a Element>,
    hover: Option<&'a Element>,
    mouse_x: i32,
    mouse_y: i32,
}
pub static mut RENDERER: Option<Box<Renderer>> = None;

impl<'a> Renderer<'a> {
    const FOV: f32 = 90.;
    const NEAR_PLANE: f32 = 0.1;
    const FAR_PLANE: f32 = 1000.;
    #[allow(dead_code)]
    pub fn get_aspect_ratio(&self) -> f32 {
        self.canvas.width() as f32 / self.canvas.height() as f32
    }
    #[allow(dead_code)]
    fn check_hover(&'a self) -> Option<&Element> {
        let camera = &self.camera;
        let x = self.mouse_x;
        let y = self.mouse_y;
        if let Some(ref plane) = self.plane {
            let mat = self.get_projection_matrix() * camera.get_view_matrix();
            let mat = mat.inverse().unwrap();
            let glx = x as f32 * 2.0 / self.get_width() as f32 - 1.0;
            let gly = y as f32 * 2.0 / self.get_height() as f32 - 1.0;
            let gly = gly * -1.0;

            // work because two 2d
            let mwp = mat.mul(&Vec4([glx, gly, 0., 1.0]));
            let x = mwp[0];
            let y = mwp[1];
            let element = plane.select(x, y);
            return element;
        }
        None
    }
    #[allow(dead_code)]
    pub fn get_width(&self) -> u32 {
        self.canvas.width()
    }
    #[allow(dead_code)]
    pub fn get_height(&self) -> u32 {
        self.canvas.height()
    }
    #[allow(dead_code)]
    pub fn get_std_program(&self) -> &Shader {
        self.std_program.as_ref().unwrap()
    }
    #[allow(dead_code)]
    pub fn set_std_program(&mut self, program: Shader) {
        self.std_program = Some(program);
    }
    #[allow(dead_code)]
    pub fn get_line_program(&self) -> &Shader {
        self.line_program.as_ref().unwrap()
    }
    #[allow(dead_code)]
    pub fn set_line_program(&mut self, program: Shader) {
        self.line_program = Some(program);
    }
    #[allow(dead_code)]
    pub fn get_projection_matrix(&self) -> Mat4 {
        let aspect_ratio = self.get_aspect_ratio();
        match self.render_mode {
            RenderMode::M2D => utils::orthographic_matrix(
                aspect_ratio,
                self.camera.get_zoom(),
                Self::NEAR_PLANE,
                Self::FAR_PLANE,
            ),
            RenderMode::M3D => utils::perspective_matrix(
                aspect_ratio,
                Self::FOV,
                Self::NEAR_PLANE,
                Self::FAR_PLANE,
            ),
        }
    }
    fn new() -> Result<Renderer<'a>, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        let context_options = ContextOptions { antialias: true };
        let context_options =
            JsValue::from_serde(&context_options).expect("could not serialize context options");
        let context = canvas
            .get_context_with_context_options("webgl2", &context_options)?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            // const LEFT_BUTTON: u16 = 1;
            let renderer = Renderer::get_instance();
            renderer.mouse_x = event.offset_x();
            renderer.mouse_y = event.offset_y();
            let element = renderer.check_hover();

            let renderer = Renderer::get_instance();
            renderer.hover = element;
            let camera = &mut renderer.camera;
            // left button not pressed
            // if event.buttons() & LEFT_BUTTON == 0 {
            //     return;
            // }
            if !event.ctrl_key() {
                return;
            }
            match renderer.render_mode {
                RenderMode::M2D => {
                    camera.mouse_move_2d(event.movement_x() as f32, event.movement_y() as f32);
                }
                RenderMode::M3D => {
                    camera.mouse_move_3d(event.movement_x() as f32, event.movement_y() as f32);
                }
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();

        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let renderer = Renderer::get_instance();
            let camera = &mut renderer.camera;
            let key = event.key();
            match renderer.render_mode {
                RenderMode::M2D => {
                    camera.key_move_2d(key.as_ref());
                }
                RenderMode::M3D => {
                    camera.key_move_3d(key.as_ref());
                }
            }

            let element = renderer.check_hover();
            let renderer = Renderer::get_instance();
            renderer.hover = element;
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        return Ok(Renderer {
            context,
            canvas,
            camera: Camera::default(),
            render_mode: RenderMode::M2D,
            std_program: None,
            line_program: None,
            plane: None,
            selected: None,
            dragged: None,
            hover: None,
            mouse_x: 0,
            mouse_y: 0,
        });
    }
    pub fn get_instance() -> &'static mut Renderer<'a> {
        // return Renderer::new().unwrap();
        unsafe {
            if RENDERER.is_none() {
                RENDERER = Some(Box::new(Renderer::new().unwrap()))
            }
            RENDERER.as_deref_mut().unwrap()
        }
    }
    pub fn get_context_instance(&self) -> &WebGl2RenderingContext {
        &self.context
    }
    pub fn prepare(&self) {
        let context = &self.context;
        context.clear_color(0.0, 0.0, 1.0, 1.0);
        context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
    }
    pub async fn create_area(&mut self, image_name: &str) -> Result<(), JsValue> {
        if self.plane.is_none() {
            return Err(JsValue::from("cannot create area if plane is not created"));
        }
        let plane = self.plane.as_ref().unwrap();
        let texture = Texture::new(image_name).await?;
        let (width, height) = plane.rect.dimensions;
        let mesh = meshes::textured_rectangle(texture)?;
        let vao = Vao::new(mesh).map_err(|s| JsValue::from_str(&s))?;

        let mesh = meshes::line_rectangle(width, height);
        let frame = Vao::new(mesh).map_err(|s| JsValue::from_str(&s))?;

        let area = Element::area(vao, frame, width, height);
        if let ElementKind::PLANE(ref plane) = plane.kind {
            // plane.areas.push();
        }
        Ok(())
    }
    pub async fn create_plane(&mut self, image_name: &str) -> Result<(), JsValue> {
        let texture = Texture::new(image_name).await?;
        let width = texture.width;
        let height = texture.height;
        let mesh = meshes::textured_rectangle(texture)?;
        let vao = Vao::new(mesh).map_err(|s| JsValue::from_str(&s))?;

        let mesh = meshes::line_rectangle(width, height);
        let frame = Vao::new(mesh).map_err(|s| JsValue::from_str(&s))?;

        let plane = Element::plane(vao, frame, width, height);
        self.plane = Some(plane);
        Ok(())
    }
    pub fn draw(&self) {
        let program = self.get_std_program();
        program.use_program();
        let mat = Mat4::identity();
        // let mat = mat.rotate(&vec4::Vec4([0.0, 0.0, rotation, 1.0]));
        // let mat = mat.scale(&vec4::Vec4([0.5, 0.5, 0.5, 1.0]));
        program.load_tranformation_matrix(mat.data());
        program.load_view_matrix(self.camera.get_view_matrix().data());
        program.load_projection_matrix(self.get_projection_matrix().data());
        if self.plane.is_none() {
            return;
        }

        let plane = self.plane.as_ref().unwrap();
        if self.dragged.is_none() || !std::ptr::eq(plane, self.dragged.unwrap()) {
            plane.vao.draw();
        }

        if self.hover.is_none() {
            return;
        }

        let program = self.get_line_program();
        program.use_program();
        let mat = Mat4::identity();
        // let mat = mat.rotate(&vec4::Vec4([0.0, 0.0, rotation, 1.0]));
        // let mat = mat.scale(&vec4::Vec4([0.5, 0.5, 0.5, 1.0]));
        program.load_tranformation_matrix(mat.data());
        program.load_view_matrix(self.camera.get_view_matrix().data());
        program.load_projection_matrix(self.get_projection_matrix().data());
        self.hover.unwrap().frame.draw_line_loop();
    }
}
