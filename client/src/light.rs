use std::os::raw::c_void;

use raylib_ffi::{
    colors, enums, rl_str, Color, GetShaderLocation, SetShaderValue, Shader, Vector3,
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
        let me = unsafe {
            Self {
                id,
                enabled: 1,
                kind: 1,
                position: Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
                target: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                color: colors::WHITE,
                enabled_loc: GetShaderLocation(shader, rl_str!(format!("lights[{}].enabled", id))),
                kind_loc: GetShaderLocation(shader, rl_str!(format!("lights[{}].type", id))),
                position_loc: GetShaderLocation(
                    shader,
                    rl_str!(format!("lights[{}].position", id)),
                ),
                target_loc: GetShaderLocation(shader, rl_str!(format!("lights[{}].target", id))),
                color_loc: GetShaderLocation(shader, rl_str!(format!("lights[{}].color", id))),
            }
        };

        me
    }

    pub fn update(&self, shader: Shader) {
        unsafe {
            let enabled = [self.enabled].as_ptr();
            SetShaderValue(
                shader,
                self.enabled_loc,
                enabled as *const c_void,
                enums::ShaderUniformDataType::Int as i32,
            );
            let kind = [self.kind].as_ptr();
            SetShaderValue(
                shader,
                self.kind_loc,
                kind as *const c_void,
                enums::ShaderUniformDataType::Int as i32,
            );

            let position = [self.position.x, self.position.y, self.position.z].as_ptr();
            SetShaderValue(
                shader,
                self.position_loc,
                position as *const c_void,
                enums::ShaderUniformDataType::Vec3 as i32,
            );

            let target = [self.position.x, self.position.y, self.position.z].as_ptr();
            SetShaderValue(
                shader,
                self.target_loc,
                target as *const c_void,
                enums::ShaderUniformDataType::Vec3 as i32,
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
                color as *const c_void,
                enums::ShaderUniformDataType::Vec4 as i32,
            );
        }
    }
}