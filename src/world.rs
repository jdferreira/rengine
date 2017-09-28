use super::glium::{Display, VertexBuffer, Frame, Program, DrawParameters, Surface};
use super::glium::index::{IndexBuffer, PrimitiveType};

use vertex::Vertex;
use renderable::{self, Renderable};

pub struct Square {
    vertex_buffer: VertexBuffer<Vertex>,
    indices_source: IndexBuffer<u32>,
    shader: Program
}

impl Square {
    fn new(display: &Display) -> Self {
        let vertex1 = Vertex { position: [-0.5, -0.5, 0.0] };
        let vertex2 = Vertex { position: [ 0.5, -0.5, 0.0] };
        let vertex3 = Vertex { position: [ 0.5,  0.5, 0.0] };
        let vertex4 = Vertex { position: [-0.5,  0.5, 0.0] };
        let shape = vec![vertex1, vertex2, vertex3, vertex4];
        let indices = [0, 1, 2,
                       0, 2, 3u32];

        let vertex_buffer = VertexBuffer::new(display, &shape).unwrap();
        let indices_source = IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).unwrap();
        let shader = {
            let vertex_shader = include_str!("../assets/tri.glslv");
            let fragment_shader = include_str!("../assets/tri.glslf");
            Program::from_source(display, vertex_shader, fragment_shader, None).unwrap()
        };
        
        Square {
            vertex_buffer: vertex_buffer,
            indices_source: indices_source,
            shader: shader,
        }
    }
}

impl<'a> renderable::Renderable<'a> for Square {
    
    fn render(&self, target: &mut Frame, params: &DrawParameters) {
        target.draw(
            &self.vertex_buffer,
            &self.indices_source,
            &self.shader,
            &uniform!{},
            params)
        .unwrap();
    }

}

pub struct World<'a> {
    pub square: Square,
    params: DrawParameters<'a>,
}

impl<'a> World<'a> {

    pub fn new<'b>(display: &'b Display) -> World<'a> {
        World {
            square: Square::new(display),
            params: Default::default(),
        }
    }
    
    pub fn draw(&self, target: &mut Frame) {
        self.square.render(target, &self.params);
    }

}