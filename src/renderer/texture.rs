use crate::Renderer;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlTexture};

#[wasm_bindgen]
extern "C" {
    async fn parseImage(image_data: String) -> JsValue;
    // #[wasm_bindgen(js_namespace = console)]
    // fn log(value: &JsValue);
}

#[wasm_bindgen]
pub struct Texture {
    texture: Option<WebGlTexture>,
    pub width: f32,
    pub height: f32,
}

impl Texture {
    pub fn aspect_ratio(&self) -> f32 {
        self.width / self.height
    }

    pub async fn new(image_name: &str) -> Result<Texture, JsValue> {
        let context = Renderer::get_instance().get_context_instance();
        let texture = context.create_texture().expect("cannot create a texture");

        let image = parseImage(String::from(image_name)).await;
        let image = image.dyn_into::<HtmlImageElement>()?;

        let texture = Texture {
            texture: Some(texture),
            width: image.width() as f32,
            height: image.height() as f32,
        };

        texture.bind();

        context.tex_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            &image,
        )?;
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );

        context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        Ok(texture)
    }
    pub fn bind(&self) {
        let context = Renderer::get_instance().get_context_instance();
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, self.texture.as_ref());
    }
}
