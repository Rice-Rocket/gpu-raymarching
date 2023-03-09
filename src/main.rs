#[macro_use]
extern crate glium;
use glium::glutin;
use glium::Surface;


#[path = "program.rs"] mod program;
use program::*;



fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let (vertex_buffer, indices, program) = load_program(&display);

    let mut time = 0f32;
    event_loop.run(move |ev, _, control_flow| {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => (),
        }

        let next_frame_time = std::time::Instant::now() + 
        std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        time += 0.005;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! {
            time: time, 
            resolution: (display.get_framebuffer_dimensions().0 as f32, display.get_framebuffer_dimensions().1 as f32),
        }, &Default::default()).unwrap();
        target.finish().unwrap();
    });


}