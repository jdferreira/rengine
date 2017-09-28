use super::glium::{DrawParameters, Frame};

pub trait Renderable<'a> {
    fn render(&self, target: &mut Frame, params: &DrawParameters);
}
