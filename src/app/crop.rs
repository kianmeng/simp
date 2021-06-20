use glium::{
    backend::glutin::Display, draw_parameters::DrawParameters, implement_vertex,
    index::PrimitiveType, program::Program, uniform, IndexBuffer, Surface, VertexBuffer, Blend,
};

use super::super::Vec2;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
    }
}

implement_vertex!(Vertex, position);

pub struct Crop {
    pub inner: Option<Inner>,
    pub cropping: bool,
    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<u8>,
    shader: Program,
}

pub struct Inner {
    pub start: Vec2<f32>,
    pub current: Vec2<f32>,
}

impl Crop {
    pub fn new(display: &Display) -> Self {
        let shader = Program::from_source(
            display,
            include_str!("../shader/crop.vert"),
            include_str!("../shader/crop.frag"),
            None,
        )
        .unwrap();

        let shape = [
            Vertex::new(-1.0, 1.0),
            Vertex::new(-1.0, -1.0),
            Vertex::new(1.0, 1.0),
            Vertex::new(1.0, -1.0),
        ];
        let index_buffer = &[0, 1, 2, 2, 1, 3];

        let vertices = VertexBuffer::new(display, &shape).unwrap();
        let indices =
            IndexBuffer::new(display, PrimitiveType::TrianglesList, index_buffer).unwrap();

        Self {
            inner: None,
            cropping: false,
            vertices,
            indices,
            shader,
        }
    }

    pub fn start() {}

    pub fn render(&self, target: &mut glium::Frame, size: Vec2<f32>) {
        if let Some(ref inner) = self.inner { 
            target
                .draw(
                    &self.vertices,
                    &self.indices,
                    &self.shader,
                    &uniform! { start: *inner.start, end: *inner.current, size: *size },
                    &DrawParameters {
                        blend: Blend::alpha_blending(),
                        ..DrawParameters::default()
                    },
                )
                .unwrap();
        }
    }
}