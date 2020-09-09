use crate::renderer::Vertex;
use crate::Color;
use glam::{Mat4, Quat, Vec3};
use glium::index::PrimitiveType;
use glium::{Display, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use std::fs;

#[derive(Default)]
pub struct QuadBuilder {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
    color: Color,
}

impl QuadBuilder {
    pub fn position<P: Into<Vec3>>(&mut self, position: P) -> &mut Self {
        self.position = position.into();
        self
    }

    pub fn rotation<R: Into<Quat>>(&mut self, rotation: R) -> &mut Self {
        self.rotation = rotation.into();
        self
    }

    pub fn scale<S: Into<Vec3>>(&mut self, scale: S) -> &mut Self {
        self.scale = scale.into();
        self
    }

    pub fn color<C: Into<Color>>(&mut self, color: C) -> &mut Self {
        self.color = color.into();
        self
    }
}

pub trait Renderer {
    fn begin_rendering(&mut self);
    fn end_rendering(&mut self);
    fn clear<C: Into<Color>>(&mut self, color: C);
    fn draw_quad(&mut self, quad_builder: &QuadBuilder);
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

    fn clear<C: Into<Color>>(&mut self, color: C) {
        if let Some(ref mut frame) = self.frame {
            let color = color.into();

            frame.clear_color(color.r(), color.g(), color.b(), color.a());
        } else {
            panic!("Rendering wasn't initiated. Call start_rendering before drawing");
        }
    }

    fn draw_quad(&mut self, quad_builder: &QuadBuilder) {
        if let Some(ref mut frame) = self.frame {
            let transform = Mat4::from_scale_rotation_translation(
                quad_builder.scale,
                quad_builder.rotation,
                quad_builder.position,
            );

            frame
                .draw(
                    &self.quad_vertex_buffer,
                    &self.quad_index_buffer,
                    &self.program,
                    &uniform! {
                        u_Transform: transform.to_cols_array_2d(),
                        u_Color: quad_builder.color
                    },
                    &Default::default(),
                )
                .unwrap();
        } else {
            panic!("Rendering wasn't initiated. Call start_rendering before drawing");
        }
    }
}
