#[macro_use] extern crate failure;

pub mod engine;

use std::rc::Rc;

fn main() {
    let sdl = Rc::new(sdl2::init().unwrap());
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

    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let width = 900;
    let height = 700;
    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut viewport = render_gl::Viewport::for_window(width as i32, height as i32);
    viewport.set_used(&gl);
    
    let mut engine = engine::Engine::new(&sdl, &gl, viewport).unwrap();
    engine.init();
    engine.main_loop(&window).unwrap();
}
