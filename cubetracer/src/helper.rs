extern crate gl;

use crate::errors::*;
use crate::{glchk_expr, glchk_stmt};

use std::ffi::{c_void, CString};
use std::{mem, ptr};

use gl::types::*;

pub fn generate_texture(width: u32, height: u32) -> Result<u32, GLError> {
    let mut tex_out = 0;

    glchk_stmt!(
        gl::GenTextures(1, &mut tex_out);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, tex_out);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA32F as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::FLOAT,
            ptr::null(),
        );

        gl::BindImageTexture(0, tex_out, 0, gl::FALSE, 0, gl::WRITE_ONLY, gl::RGBA32F);
    );

    Ok(tex_out)
}

pub fn get_uniform_location(program: u32, var_name: &str) -> Result<i32, GLError> {
    let c_var_name = CString::new(var_name).unwrap();
    let loc = glchk_expr!(gl::GetUniformLocation(program, c_var_name.as_ptr()));
    if loc == -1 {
        Err(GLError::UniformNotFound {
            name: var_name.to_string(),
        })
    } else {
        Ok(loc)
    }
}

pub fn build_program_raytracer() -> Result<u32, GLError> {
    let shader_compute = glchk_expr!(gl::CreateShader(gl::COMPUTE_SHADER));
    let c_str_vert =
        CString::new(include_str!("../shaders/raytracer/main.comp").as_bytes()).unwrap();

    glchk_stmt!(
        gl::ShaderSource(shader_compute, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(shader_compute);
    );

    gl_check_error_shader(shader_compute, gl::COMPILE_STATUS)?;

    let program = glchk_expr!(gl::CreateProgram());
    glchk_stmt!(
        gl::AttachShader(program, shader_compute);
        gl::LinkProgram(program);
    );

    gl_check_error_program(program, gl::LINK_STATUS)
}

pub fn make_quad_vao(program: u32) -> Result<u32, GLError> {
    let vertices: [f32; 8] = [-1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0];

    let (mut vbo, mut vao) = (0, 0);

    glchk_stmt!(
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STREAM_DRAW,
        );
    );

    let c_var_name_pos = CString::new("in_pos").unwrap();
    let attr_pos = glchk_expr!(gl::GetAttribLocation(program, c_var_name_pos.as_ptr()) as u32);

    glchk_stmt!(
        gl::VertexAttribPointer(attr_pos, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(attr_pos);
    );

    Ok(vao)
}

pub fn build_program_quad() -> Result<u32, GLError> {
    let program = glchk_expr!(gl::CreateProgram());
    let shader_vertex = glchk_expr!(gl::CreateShader(gl::VERTEX_SHADER));

    let c_str_vert = CString::new(include_str!("../shaders/vertex.glsl").as_bytes()).unwrap();

    glchk_stmt!(
        gl::ShaderSource(shader_vertex, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(shader_vertex);
    );

    gl_check_error_shader(shader_vertex, gl::COMPILE_STATUS)?;

    glchk_stmt!(
        gl::AttachShader(program, shader_vertex);
    );

    let shader_fragment = glchk_expr!(gl::CreateShader(gl::FRAGMENT_SHADER));
    let c_str_vert = CString::new(include_str!("../shaders/fragment.glsl").as_bytes()).unwrap();

    glchk_stmt!(
        gl::ShaderSource(shader_fragment, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(shader_fragment);
    );
    gl_check_error_shader(shader_fragment, gl::COMPILE_STATUS)?;

    glchk_stmt!(
        gl::AttachShader(program, shader_fragment);
        gl::LinkProgram(program);
    );
    gl_check_error_program(program, gl::LINK_STATUS)?;

    glchk_stmt!(
        gl::DeleteShader(shader_vertex);
        gl::DeleteShader(shader_fragment);
        gl::ProgramUniform1i(
            program,
            gl::GetUniformLocation(program, CString::new("uni_text").unwrap().as_ptr()),
            0,
        );


    );

    Ok(program)
}