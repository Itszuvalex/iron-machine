#[macro_use] extern crate failure;
extern crate gl;
extern crate sdl2;
extern crate resources;
extern crate render_gl;

use resources::Resources;
use std::path::Path;

fn main() {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let width = 900;
    let height = 700;

    let window = video_subsystem
    .window("Game", width, height)
    .opengl()
    .resizable()
    .maximized()
    .allow_highdpi()
    .build()
    .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut viewport = render_gl::Viewport::for_window(width as i32, height as i32);
    viewport.set_used(&gl);

    unsafe {
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let triangle = render_gl::Triangle::new(&res, &gl).unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::SizeChanged(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                },
                _ => {},
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        triangle.render(&gl);

        window.gl_swap_window();
    }
}
