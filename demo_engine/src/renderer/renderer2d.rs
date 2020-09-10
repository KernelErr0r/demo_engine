use crate::renderer::{Vertex, OrtographicCamera};
use crate::Color;
use glam::{Mat4, Quat, Vec3};
use glium::backend::Facade;
use glium::index::PrimitiveType;
use glium::texture::RawImage2d;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{Display, Frame, IndexBuffer, Program, Surface, Texture2d, VertexBuffer};
use image::RgbaImage;
use std::borrow::Cow;
use std::fs;

pub struct Quad {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
    color: Color,
    texture: Option<Texture2d>,
}

struct Texture<'a> {
    dimensions: (u32, u32),
    data: Cow<'a, [u8]>,
}

impl<'a> From<&'a RgbaImage> for Texture<'a> {
    fn from(texture: &'a RgbaImage) -> Self {
        Self {
            dimensions: texture.dimensions(),
            data: texture.as_raw().into(),
        }
    }
}

#[derive(Default)]
pub struct QuadBuilder<'a> {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
    color: Color,
    texture: Option<Texture<'a>>,
}

impl<'a> QuadBuilder<'a> {
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

    pub fn texture(&mut self, texture: &'a RgbaImage) -> &mut Self {
        self.texture = Some(Texture::from(texture));
        self
    }

    pub fn build<F: Facade>(&self, facade: &F) -> Quad {
        let texture = {
            match self.texture {
                Some(ref texture) => {
                    let image =
                        RawImage2d::from_raw_rgba_reversed(&texture.data, texture.dimensions);

                    Some(Texture2d::new(facade, image).unwrap())
                }
                None => None,
            }
        };

        Quad {
            position: self.position,
            rotation: self.rotation,
            scale: self.scale,
            color: self.color,
            texture: texture,
        }
    }
}

pub trait Renderer {
    fn begin_rendering(&mut self, camera: &OrtographicCamera);
    fn end_rendering(&mut self);
    fn clear<C: Into<Color>>(&mut self, color: C);
    fn draw_quad(&mut self, quad_builder: &QuadBuilder);
}

pub struct Renderer2D {
    quad_vertex_buffer: VertexBuffer<Vertex>,
    quad_index_buffer: IndexBuffer<u16>,
    color_program: Program,
    color_and_texture_program: Program,
    display: Display,
    frame: Option<Frame>,
    view_projection_matrix: Mat4,
}

impl Renderer2D {
    pub fn new(display: Display) -> Self {
        let quad_shape = vec![
            Vertex::new([0.5, 0.5], [1.0, 1.0]),
            Vertex::new([0.5, -0.5], [1.0, 0.0]),
            Vertex::new([-0.5, -0.5], [0.0, 0.0]),
            Vertex::new([-0.5, 0.5], [0.0, 1.0]),
        ];
        let quad_indices: [u16; 6] = [0, 1, 3, 1, 2, 3];

        Self {
            quad_vertex_buffer: VertexBuffer::new(&display, &quad_shape)
                .expect("Failed to create the vertex buffer"),
            quad_index_buffer: IndexBuffer::new(
                &display,
                PrimitiveType::TrianglesList,
                &quad_indices,
            )
            .expect("Failed to create the index buffer"),
            color_program: Renderer2D::create_program(&display, "color.vert", "color.frag"),
            color_and_texture_program: Renderer2D::create_program(
                &display,
                "color_and_texture.vert",
                "color_and_texture.frag",
            ),
            display: display,
            frame: None,
            view_projection_matrix: Mat4::zero(),
        }
    }

    fn create_program(
        display: &Display,
        vertex_shader_location: &str,
        fragment_shader_location: &str,
    ) -> Program {
        let vertex_shader_source =
            fs::read_to_string(vertex_shader_location).expect("Cannot read vertex shader");
        let vertex_shader_source = vertex_shader_source.as_str();

        let fragment_shader_source =
            fs::read_to_string(fragment_shader_location).expect("Cannot read fragment shader");
        let fragment_shader_source = fragment_shader_source.as_str();

        Program::from_source(display, vertex_shader_source, fragment_shader_source, None)
            .expect("Failed to create the program")
    }
}

impl Renderer for Renderer2D {
    fn begin_rendering(&mut self, camera: &OrtographicCamera) {
        self.frame = Some(self.display.draw());
        self.view_projection_matrix = camera.get_view_projection_matrix();
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
            let quad = quad_builder.build(&self.display);

            let transform =
                Mat4::from_scale_rotation_translation(quad.scale, quad.rotation, quad.position);

            match quad.texture {
                Some(ref texture) => {
                    frame
                        .draw(
                            &self.quad_vertex_buffer,
                            &self.quad_index_buffer,
                            &self.color_and_texture_program,
                            &uniform! {
                                u_ViewProjection: self.view_projection_matrix.to_cols_array_2d(),
                                u_Transform: transform.to_cols_array_2d(),
                                u_Color: quad.color,
                                u_Texture: texture.sampled().minify_filter(MinifySamplerFilter::Nearest).magnify_filter(MagnifySamplerFilter::Nearest)
                            },
                            &Default::default(),
                        )
                        .unwrap();
                }
                None => {
                    frame
                        .draw(
                            &self.quad_vertex_buffer,
                            &self.quad_index_buffer,
                            &self.color_program,
                            &uniform! {
                                u_ViewProjection: self.view_projection_matrix.to_cols_array_2d(),
                                u_Transform: transform.to_cols_array_2d(),
                                u_Color: quad.color
                            },
                            &Default::default(),
                        )
                        .unwrap();
                }
            }
        } else {
            panic!("Rendering wasn't initiated. Call start_rendering before drawing");
        }
    }
}
