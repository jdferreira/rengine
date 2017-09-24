use glium::{Display, VertexBuffer};
use glium::index::{IndexBuffer, PrimitiveType};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct World {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u32>,
}

impl World {

    pub fn new(display: &Display) -> World {
        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [ 0.5, -0.5] };
        let vertex3 = Vertex { position: [ 0.5,  0.5] };
        let vertex4 = Vertex { position: [-0.5,  0.5] };
        let shape = vec![vertex1, vertex2, vertex3, vertex4];
        let indices = [0, 1, 2,
                       0, 2, 3u32];

        let vertex_buffer = VertexBuffer::new(display, &shape).unwrap();
        let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).unwrap();

        World {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        }
    }
    
}