#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub texture_coordinates: [f32; 2],
}

implement_vertex!(Vertex, position, texture_coordinates);

impl Vertex {
    pub fn new(position: [f32; 2], texture_coordinates: [f32; 2]) -> Self {
        Self {
            position: position,
            texture_coordinates: texture_coordinates,
        }
    }
}
