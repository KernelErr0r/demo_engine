use demo_engine::image::{load, ImageFormat};
use demo_engine::renderer::{QuadBuilder, Renderer, Renderer2D, OrtographicCamera};
use demo_engine::{glium::Display, glutin::{
    dpi::LogicalSize,
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
}, Quat, Vec3};
use std::io::Cursor;
use std::time::{Duration, Instant};

fn main() {
    println!("Hello demo world!");

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1024.0, 768.0))
        .with_title("DemoSandbox");
    let context_builder = ContextBuilder::new();
    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

    let mut renderer = Renderer2D::new(display);
    let mut camera = OrtographicCamera::new(-1.0, 1.0, -1.0, 1.0);

    camera.set_position(Vec3::new(1.0, 1.0, 1.0));

    let image = load(
        Cursor::new(&include_bytes!("../../image.png")[..]),
        ImageFormat::Png,
    )
    .unwrap()
    .to_rgba();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        renderer.begin_rendering(&camera);

        renderer.clear((0.3, 0.3, 0.3));
        renderer.draw_quad(
            QuadBuilder::default()
                .position((0.0, 0.0, 0.0))
                .rotation(Quat::from_rotation_z(45.0_f32.to_radians()))
                .scale((1.0, 1.0, 1.0))
                .color((0.0, 1.0, 0.0))
                .texture(&image),
        );

        renderer.end_rendering();
    });
}
