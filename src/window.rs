use glium::{Display, Surface, Program};
use glium::uniforms::EmptyUniforms;

use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
use glium::glutin::{Event, WindowEvent};

use world;

pub struct Window {
    pub display: Display,
    pub closed: bool,

    events_loop: EventsLoop,
    program: Program,
}

impl Window {
    pub fn new() -> Window {
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new().with_dimensions(600, 600);
        let context = ContextBuilder::new();
        let display = Display::new(window, context, &events_loop).unwrap();

        let vertex_shader_src = include_str!("../assets/tri.glslv");
        let fragment_shader_src = include_str!("../assets/tri.glslf");
        let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        Window {
            display: display,
            closed: false,
            events_loop: events_loop,
            program: program,
        }
    }

    pub fn draw(&self, world: &world::World) {
        let mut target = self.display.draw();

        target.clear_color(0.0, 0.0, 1.0, 0.0);
        target.draw(&world.vertex_buffer, &world.index_buffer, &self.program, &EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }

    pub fn events(&mut self) {
        let mut closed = self.closed;

        self.events_loop.poll_events(|ev| {
            if let Event::WindowEvent { event: WindowEvent::Closed, .. } = ev {
                closed = true;
            }
        });
        
        self.closed = closed;
    }
}