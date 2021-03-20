extern crate failure;

extern crate gl;
extern crate sdl2;
extern crate resources;
extern crate render_gl;

use resources::Resources;
use std::path::Path;
use std::rc::Rc;

use failure::Fail;

use crate::engine::scene;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "SDL2 Encountered an error: {}", err)]
    SDL2Error{ err: String },
    #[fail(display = "SDL2 Video subsystem  error")]
    SDL2Window(#[cause] sdl2::video::WindowBuildError),
    #[fail(display = "Resource error")]
    Resource(#[cause] resources::Error),
}

impl From<resources::Error> for Error {
    fn from(other: resources::Error) -> Self {
        Error::Resource(other)
    }
}

impl From<sdl2::video::WindowBuildError> for Error {
    fn from(other: sdl2::video::WindowBuildError) -> Self {
        Error::SDL2Window(other)
    }
}

pub struct Engine {
    scenes: Vec<Box<dyn scene::Scene>>,
    resources: Resources,
    sdl: Rc<sdl2::Sdl>,
    gl: gl::Gl,
    viewport: render_gl::Viewport,
    // Temp
    triangle: render_gl::Triangle,
}

impl Engine {
    pub fn new(sdl: &Rc<sdl2::Sdl>, gl: &gl::Gl, viewport: render_gl::Viewport) -> Result<Engine, Error> {
        let resources = Resources::from_relative_exe_path(Path::new("assets"))?;

        let triangle = render_gl::Triangle::new(&resources, &gl).unwrap();

        Ok(Engine {
            scenes: Vec::new(),
            resources: resources,
            sdl: sdl.clone(),
            viewport: viewport,
            gl: gl.clone(),
            // Temp
            triangle: triangle,
        })
    }

    pub fn init(&mut self) {
        self.viewport.set_used(&self.gl);
        unsafe {
            self.gl.ClearColor(0.3, 0.3, 0.5, 1.0);
        }
    }

    pub fn main_loop(&mut self, window: &sdl2::video::Window) -> Result<(), Error> {
        let mut event_pump = self.sdl.event_pump().map_err(|s| Error::SDL2Error{err: s})?;
        'main: loop {
            let cont = self.handle_events(&mut event_pump).unwrap();
            if !cont {
                break 'main;
            }
            self.update();
            self.render(window);
        }
        Ok(())
    }

    fn handle_events(&mut self, event_pump: &mut sdl2::EventPump) -> Result<bool, Error> {
        for event in event_pump.poll_iter()
        {
            match event {
                sdl2::event::Event::Quit {..} => return Ok(false),
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::SizeChanged(w, h),
                    ..
                } => {
                    self.viewport.update_size(w, h);
                    self.viewport.set_used(&self.gl);
                },
                _ => {},
            }
        }
        return Ok(true);
    }

    fn render(&self, window: &sdl2::video::Window) {
        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        self.triangle.render(&self.gl);

        window.gl_swap_window();
    }

    fn update(&self) {

    }
}
