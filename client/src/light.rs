use std::{ffi::CString, os::raw::c_void};

use raylib::ffi::{
    Color, GetShaderLocation, SetShaderValue, Shader, ShaderUniformDataType, Vector3,
};

#[derive(Debug)]
pub struct Light {
    pub id: i32,
    pub enabled: i32,
    pub kind: i32,
    pub position: Vector3,
    pub target: Vector3,
    pub color: Color,
    pub enabled_loc: i32,
    pub kind_loc: i32,
    pub position_loc: i32,
    pub target_loc: i32,
    pub color_loc: i32,
}

impl Light {
    pub fn new(shader: Shader, id: i32) -> Self {
        unsafe {
            let enable_name = CString::new(format!("lights[{}].enabled", id)).unwrap();
            let type_name = CString::new(format!("lights[{}].type", id)).unwrap();
            let position_name = CString::new(format!("lights[{}].position", id)).unwrap();
            let target_name = CString::new(format!("lights[{}].target", id)).unwrap();
            let color_name = CString::new(format!("lights[{}].color", id)).unwrap();

            Self {
                id,
                enabled: 1,
                kind: 1,
                position: Vector3 {
                    x: 2.0,
                    y: 3.0,
                    z: 2.0,
                },
                target: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                color: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
                enabled_loc: GetShaderLocation(shader, enable_name.as_ptr()),
                kind_loc: GetShaderLocation(shader, type_name.as_ptr()),
                position_loc: GetShaderLocation(shader, position_name.as_ptr()),
                target_loc: GetShaderLocation(shader, target_name.as_ptr()),
                color_loc: GetShaderLocation(shader, color_name.as_ptr()),
            }
        }
    }

    pub fn update(&self, shader: Shader) {
        unsafe {
            let enabled = [self.enabled].as_ptr();
            SetShaderValue(
                shader,
                self.enabled_loc,
                // enabled as *const c_void,
                &self.enabled as *const i32 as _,
                ShaderUniformDataType::SHADER_UNIFORM_INT as i32,
            );
            let kind = [self.kind].as_ptr();
            SetShaderValue(
                shader,
                self.kind_loc,
                // kind as *const c_void,
                &self.kind as *const i32 as _,
                ShaderUniformDataType::SHADER_UNIFORM_INT as i32,
            );

            let position = [self.position.x, self.position.y, self.position.z].as_ptr();
            SetShaderValue(
                shader,
                self.position_loc,
                position as *const c_void,
                ShaderUniformDataType::SHADER_UNIFORM_VEC3 as i32,
            );

            let target = [self.target.x, self.target.y, self.target.z].as_ptr();
            SetShaderValue(
                shader,
                self.target_loc,
                target as *const c_void,
                ShaderUniformDataType::SHADER_UNIFORM_VEC3 as i32,
            );

            let color = [
                self.color.r as f32 / 255.0,
                self.color.g as f32 / 255.0,
                self.color.b as f32 / 255.0,
                self.color.a as f32 / 255.0,
            ]
            .as_ptr();
            SetShaderValue(
                shader,
                self.color_loc,
                color as _,
                ShaderUniformDataType::SHADER_UNIFORM_VEC4 as i32,
            );
        }
    }
}
