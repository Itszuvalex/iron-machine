#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;
extern crate gl;
extern crate sdl2;

pub mod render_gl;
pub mod resources;

use crate::resources::Resources;
use std::path::Path;
use render_gl::data;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    clr: data::f32_f32_f32
}

fn main() {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
    .window("Game", 900, 700)
    .opengl()
    .resizable()
    .build()
    .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let shader_program = render_gl::Program::from_res(&gl, &res, "shaders/triangle").unwrap();

    let vertices: Vec<Vertex> = vec![
        // positions     // colors
        Vertex { pos : (-0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },
        Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() },
        Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() },
    ];

    let mut vertexbufferobject: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vertexbufferobject);
        gl.BindBuffer(gl::ARRAY_BUFFER, vertexbufferobject);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vertexarrayobject: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vertexarrayobject);
        gl.BindVertexArray(vertexarrayobject);
        gl.BindBuffer(gl::ARRAY_BUFFER, vertexbufferobject);
    }

    Vertex::vertex_attrib_pointers(&gl);

    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }
        shader_program.set_used();
        unsafe {
            gl.BindVertexArray(vertexarrayobject);
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                3
            );
        }

        window.gl_swap_window();
    }
}
