#[macro_use]
extern crate glium;

fn main() {
    use glium::{Display, Surface, Program, VertexBuffer};
    use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
    use glium::glutin::{Event, WindowEvent};
    use glium::uniforms::EmptyUniforms;
    use glium::index::{IndexBuffer,PrimitiveType};

    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new().with_dimensions(600, 600);
    let context = ContextBuilder::new();
    let display = Display::new(window, context, &events_loop).unwrap();

    let vertex_shader_src = include_str!("../assets/tri.glslv");
    let fragment_shader_src = include_str!("../assets/tri.glslf");
    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.5, -0.5] };
    let vertex3 = Vertex { position: [ 0.5,  0.5] };
    let vertex4 = Vertex { position: [-0.5,  0.5] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
    let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u32,1,2, 0,2,3]).unwrap();

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        // target.clear_color(0.0, 0.0, 1.0, 0.0);
        target.draw(&vertex_buffer, &indices, &program, &EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            if let Event::WindowEvent { event: WindowEvent::Closed, .. } = ev {
                closed = true;
            }
        });
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);