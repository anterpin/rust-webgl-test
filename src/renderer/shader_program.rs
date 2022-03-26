use super::loader;
use crate::renderer::Renderer;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

pub trait UniformLocations {
    fn init_locations(&mut self, program: &WebGlProgram) -> Result<(), String>;
    fn new() -> Self;
}

pub struct ShaderProgram<T: UniformLocations> {
    shaders: Vec<WebGlShader>,
    program: Option<WebGlProgram>,
    pub uniforms: T,
}

impl<T: UniformLocations> ShaderProgram<T> {
    pub fn new() -> Self {
        ShaderProgram {
            shaders: Vec::new(),
            program: None,
            uniforms: UniformLocations::new(),
        }
    }
    pub async fn from_file_name(
        vertex_shader_file: &str,
        fragment_shader_file: &str,
    ) -> Result<Self, JsValue> {
        let vert_shader_source = loader::load_file(vertex_shader_file).await?;
        let fragment_shader_source = loader::load_file(fragment_shader_file).await?;
        let to_utf_str = |bytes| -> Result<String, JsValue> {
            let s = std::str::from_utf8(bytes)
                .map_err(|_| JsValue::from_str("the value is not a string"))?;
            Ok(s.to_string())
        };
        let vert_shader_source = to_utf_str(&vert_shader_source[..])?;
        let fragment_shader_source = to_utf_str(&fragment_shader_source[..])?;

        let mut program = Self::new();
        program.add_shader(
            WebGl2RenderingContext::VERTEX_SHADER,
            &vert_shader_source,
            Some(vertex_shader_file),
        )?;
        program.add_shader(
            WebGl2RenderingContext::FRAGMENT_SHADER,
            &fragment_shader_source,
            Some(fragment_shader_file),
        )?;
        program.link_program()?;
        Ok(program)
    }
    #[allow(dead_code)]
    pub fn get_program(&self) -> &WebGlProgram {
        self.program.as_ref().unwrap()
    }
    pub fn add_shader(
        &mut self,
        shader_type: u32,
        source: &str,
        file_name: Option<&str>,
    ) -> Result<(), String> {
        let context = &Renderer::get_instance().get_context_instance();
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| "Unable to create a shader object".to_string())?;

        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            self.shaders.push(shader);
            Ok(())
        } else {
            let compile_error = context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error creating shader".to_string());
            Err(format!(
                "Compile error: {}\n source {}",
                compile_error,
                file_name.unwrap_or("")
            ))
        }
    }

    pub fn use_program(&self) {
        let context = Renderer::get_instance().get_context_instance();
        context.use_program(self.program.as_ref());
    }

    pub fn link_program(&mut self) -> Result<(), String> {
        let context = Renderer::get_instance().get_context_instance();
        let program = context
            .create_program()
            .ok_or_else(|| "Unable to create the program".to_string())?;
        for shader in &self.shaders {
            context.attach_shader(&program, &shader)
        }
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            self.program = Some(program);
            self.uniforms.init_locations(self.program.as_ref().unwrap())
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown error creating the program".to_string()))
        }
    }
}

pub struct StdUniforms {
    transformation_matrix_location: Option<WebGlUniformLocation>,
    view_matrix_location: Option<WebGlUniformLocation>,
    projection_matrix_location: Option<WebGlUniformLocation>,
}
impl UniformLocations for StdUniforms {
    fn new() -> Self {
        StdUniforms {
            transformation_matrix_location: None,
            view_matrix_location: None,
            projection_matrix_location: None,
        }
    }
    fn init_locations(&mut self, program: &WebGlProgram) -> Result<(), String> {
        let context = Renderer::get_instance().get_context_instance();
        self.transformation_matrix_location = Some(
            context
                .get_uniform_location(program, "transformationMatrix")
                .expect(&format!("Unknown {}", "transformationMatrix")),
        );
        self.view_matrix_location = Some(
            context
                .get_uniform_location(program, "viewMatrix")
                .expect(&format!("Unknown {}", "viewMatrix")),
        );
        self.projection_matrix_location = Some(
            context
                .get_uniform_location(program, "projectionMatrix")
                .expect(&format!("Unknown {}", "projectionMatrix")),
        );
        // texture 0
        // context.use_program(Some(program));
        // let sampler_location = Some(
        //     context
        //         .get_uniform_location(program, "image")
        //         .expect(&format!("Unknown image")),
        // );
        // context.uniform1i(sampler_location.as_ref(), 0);
        Ok(())
    }
}
impl StdUniforms {
    pub fn load_tranformation_matrix(&self, data: &[f32]) {
        let context = Renderer::get_instance().get_context_instance();
        context.uniform_matrix4fv_with_f32_array(
            self.transformation_matrix_location.as_ref(),
            true,
            data,
        );
    }
    pub fn load_view_matrix(&self, data: &[f32]) {
        let context = Renderer::get_instance().get_context_instance();
        context.uniform_matrix4fv_with_f32_array(self.view_matrix_location.as_ref(), true, data);
    }
    pub fn load_projection_matrix(&self, data: &[f32]) {
        let context = Renderer::get_instance().get_context_instance();
        context.uniform_matrix4fv_with_f32_array(
            self.projection_matrix_location.as_ref(),
            true,
            data,
        );
    }
}

use std::ops::{Deref, DerefMut};
impl<T: UniformLocations> Deref for ShaderProgram<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.uniforms
    }
}
impl<T: UniformLocations> DerefMut for ShaderProgram<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.uniforms
    }
}

use wasm_bindgen::prelude::*;

extern crate derive_more;
use derive_more::{Deref, DerefMut};

#[wasm_bindgen]
#[derive(Deref, DerefMut)]
pub struct Shader(ShaderProgram<StdUniforms>);

impl Shader {
    pub fn new(program: ShaderProgram<StdUniforms>) -> Self {
        Self { 0: program }
    }
}
