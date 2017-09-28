use super::glium::{Display, Surface};
use super::glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
use super::glium::glutin::{Event, WindowEvent};

use world;

pub struct Window {
    pub display: Display,
    pub closed: bool,
    events_loop: EventsLoop,
}

impl Window {
    pub fn new() -> Window {
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new().with_dimensions(600, 600);
        let context = ContextBuilder::new();
        let display = Display::new(window, context, &events_loop).unwrap();

        Window {
            display: display,
            closed: false,
            events_loop: events_loop,
        }
    }

    pub fn draw(&self, world: &world::World) {
        let mut target = self.display.draw();

        target.clear_color(0.0, 0.0, 1.0, 0.0);
        world.draw(&mut target);
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