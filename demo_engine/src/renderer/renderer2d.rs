use crate::renderer::Vertex;
use glam::{Mat4, Vec2, Vec3, Vec4};
use glium::index::PrimitiveType;
use glium::{Display, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use std::fs;

pub trait Renderer {
    fn begin_rendering(&mut self);
    fn end_rendering(&mut self);
}

pub trait Clear<C> {
    fn clear(&mut self, color: C);
}

pub trait DrawQuad<P, S, C> {
    fn draw_quad(&mut self, position: P, size: S, color: C);
}

pub struct Renderer2D {
    quad_vertex_buffer: VertexBuffer<Vertex>,
    quad_index_buffer: IndexBuffer<u16>,
    program: Program,
    display: Display,
    frame: Option<Frame>,
}

impl Renderer2D {
    pub fn new(display: Display) -> Self {
        let quad_vertex1 = Vertex {
            position: [0.5, 0.5],
        };
        let quad_vertex2 = Vertex {
            position: [0.5, -0.5],
        };
        let quad_vertex3 = Vertex {
            position: [-0.5, -0.5],
        };
        let quad_vertex4 = Vertex {
            position: [-0.5, 0.5],
        };
        let quad_shape = vec![quad_vertex1, quad_vertex2, quad_vertex3, quad_vertex4];
        let quad_indices: [u16; 6] = [0, 1, 3, 1, 2, 3];

        let vertex_shader_source =
            fs::read_to_string("main.vert").expect("Cannot read data from the file main.vert");
        let vertex_shader_source = vertex_shader_source.as_str();
        let fragment_shader_source =
            fs::read_to_string("main.frag").expect("Cannot read data from the file main.frag");
        let fragment_shader_source = fragment_shader_source.as_str();

        Self {
            quad_vertex_buffer: VertexBuffer::new(&display, &quad_shape)
                .expect("Failed to create the vertex buffer"),
            quad_index_buffer: IndexBuffer::new(
                &display,
                PrimitiveType::TrianglesList,
                &quad_indices,
            )
            .expect("Failed to create the index buffer"),
            program: Program::from_source(
                &display,
                vertex_shader_source,
                fragment_shader_source,
                None,
            )
            .expect("Failed to create the program"),
            display: display,
            frame: None,
        }
    }
}

impl Renderer for Renderer2D {
    fn begin_rendering(&mut self) {
        self.frame = Some(self.display.draw());
    }

    fn end_rendering(&mut self) {
        let frame = self
            .frame
            .take()
            .expect("You can't stop rendering if you didn't initialize it");
        frame
            .finish()
            .expect("Something went wrong during rendering");
    }
}

impl Clear<f32> for Renderer2D {
    fn clear(&mut self, color: f32) {
        self.clear(Vec4::new(color, color, color, 1.0));
    }
}

impl Clear<Vec3> for Renderer2D {
    fn clear(&mut self, color: Vec3) {
        self.clear(Vec4::new(color.x(), color.y(), color.z(), 1.0));
    }
}

impl Clear<Vec4> for Renderer2D {
    fn clear(&mut self, color: Vec4) {
        if let Some(ref mut frame) = self.frame {
            frame.clear_color(color.x(), color.y(), color.z(), color.w());
        } else {
            panic!("Rendering wasn't initiated. Call start_rendering before drawing");
        }
    }
}

impl DrawQuad<Vec2, Vec2, f32> for Renderer2D {
    fn draw_quad(&mut self, position: Vec2, size: Vec2, color: f32) {
        self.draw_quad(
            Vec3::new(position.x(), position.y(), 0.0),
            size,
            Vec4::new(color, color, color, 1.0),
        );
    }
}

impl DrawQuad<Vec3, Vec2, f32> for Renderer2D {
    fn draw_quad(&mut self, position: Vec3, size: Vec2, color: f32) {
        self.draw_quad(position, size, Vec4::new(color, color, color, 1.0));
    }
}

impl DrawQuad<Vec2, Vec2, Vec3> for Renderer2D {
    fn draw_quad(&mut self, position: Vec2, size: Vec2, color: Vec3) {
        self.draw_quad(
            Vec3::new(position.x(), position.y(), 0.0),
            size,
            Vec4::new(color.x(), color.y(), color.z(), 1.0),
        );
    }
}

impl DrawQuad<Vec3, Vec2, Vec3> for Renderer2D {
    fn draw_quad(&mut self, position: Vec3, size: Vec2, color: Vec3) {
        self.draw_quad(
            position,
            size,
            Vec4::new(color.x(), color.y(), color.z(), 1.0),
        );
    }
}

impl DrawQuad<Vec2, Vec2, Vec4> for Renderer2D {
    fn draw_quad(&mut self, position: Vec2, size: Vec2, color: Vec4) {
        self.draw_quad(Vec3::new(position.x(), position.y(), 0.0), size, color);
    }
}

impl DrawQuad<Vec3, Vec2, Vec4> for Renderer2D {
    fn draw_quad(&mut self, position: Vec3, size: Vec2, color: Vec4) {
        if let Some(ref mut frame) = self.frame {
            let transform = Mat4::from_translation(position)
                * Mat4::from_scale(Vec3::new(size.x(), size.y(), 0.0));

            frame
                .draw(
                    &self.quad_vertex_buffer,
                    &self.quad_index_buffer,
                    &self.program,
                    &uniform! {
                        u_Transform: transform.to_cols_array_2d(),
                        u_Color: [color.x(), color.y(), color.z(), color.w()]
                    },
                    &Default::default(),
                )
                .unwrap();
        } else {
            panic!("Rendering wasn't initiated. Call start_rendering before drawing");
        }
    }
}
