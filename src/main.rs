#[macro_use]
extern crate glium;

mod world;
mod window;

fn main() {
    let mut window = window::Window::new();
    let world = world::World::new(&window.display);

    while !window.closed {
        window.draw(&world);
        window.events();
    }
}