use demo_engine::{
    glium::{Display, Surface},
    glutin::{
        dpi::LogicalSize,
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
};
use std::time::{Duration, Instant};

fn main() {
    println!("Hello demo world!");

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1024.0, 768.0))
        .with_title("DemoEditor");
    let context_builder = ContextBuilder::new();
    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

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

        let mut frame = display.draw();

        frame.clear_color(0.3, 0.3, 0.3, 1.0);

        frame.finish().unwrap();
    });
}
