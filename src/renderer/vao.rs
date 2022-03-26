use super::mesh::Mesh;
use crate::renderer::Renderer;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

#[wasm_bindgen]
pub struct Vao {
    _buffers: Vec<WebGlBuffer>,
    vao: Option<WebGlVertexArrayObject>,
    size: i32,
    mesh: Mesh,
}

impl Vao {
    pub fn get_mesh(&self) -> &Mesh {
        &self.mesh
    }
    pub fn new(mesh: Mesh) -> Result<Vao, String> {
        let context = Renderer::get_instance().get_context_instance();
        let mut _buffers: Vec<WebGlBuffer> = Vec::new();
        let vao = Some(
            context
                .create_vertex_array()
                .ok_or("Could not create vertex array object")?,
        );
        context.bind_vertex_array(vao.as_ref());

        Self::bind_indices(&mut _buffers, mesh.indices.as_slice())?;
        Self::store_in_attribute_list(&mut _buffers, 0, mesh.vertices.as_slice(), 3)?;
        if let Some(ref texture_coords) = mesh.texture_coords {
            Self::store_in_attribute_list(&mut _buffers, 1, texture_coords.coords.as_slice(), 2)?;
        }
        if let Some(ref normals) = mesh.normals {
            Self::store_in_attribute_list(&mut _buffers, 2, normals.as_slice(), 3)?;
        }

        let size = mesh.indices.len() as i32;

        Ok(Vao {
            _buffers,
            vao,
            size,
            mesh,
        })
    }
    pub fn get_vao(&self) -> &WebGlVertexArrayObject {
        self.vao.as_ref().unwrap()
    }
    pub fn get_size(&self) -> i32 {
        self.size
    }
    fn store_in_attribute_list(
        _buffers: &mut Vec<WebGlBuffer>,
        attribute_number: u32,
        data: &[f32],
        size: i32,
    ) -> Result<(), String> {
        let context = Renderer::get_instance().get_context_instance();
        let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        _buffers.push(buffer);

        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(data);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        };

        context.vertex_attrib_pointer_with_i32(
            attribute_number,
            size,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        Ok(())
    }
    fn bind_indices(_buffers: &mut Vec<WebGlBuffer>, data: &[u32]) -> Result<(), String> {
        let context = Renderer::get_instance().get_context_instance();
        let index_buffer = context.create_buffer().ok_or("failed to create a buffer")?;
        context.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&index_buffer),
        );
        _buffers.push(index_buffer);

        unsafe {
            let positions_array_buf_view = js_sys::Uint32Array::view(data);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        };
        Ok(())
    }
    fn bind(&self) {
        let context = Renderer::get_instance().get_context_instance();
        context.bind_vertex_array(self.vao.as_ref());
        // position
        context.enable_vertex_attrib_array(0);
        if let Some(ref texture_coords) = self.mesh.texture_coords {
            context.enable_vertex_attrib_array(1);
            context.active_texture(WebGl2RenderingContext::TEXTURE0);
            texture_coords.texture.bind();
        }
        if self.mesh.normals.is_some() {
            context.enable_vertex_attrib_array(2);
        }
    }
    pub fn draw(&self) {
        let context = Renderer::get_instance().get_context_instance();
        self.bind();
        context.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.size,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );
    }
    pub fn draw_line_loop(&self) {
        let context = Renderer::get_instance().get_context_instance();
        self.bind();
        context.draw_elements_with_i32(
            WebGl2RenderingContext::LINE_LOOP,
            self.size,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );
    }
}
