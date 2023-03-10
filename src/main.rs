#[macro_use]
extern crate glium;
use glium::glutin;
use glium::Surface;


use program::*;
use scene::*;
#[path = "program.rs"] mod program;
#[path = "scene.rs"] mod scene;


fn basic_scene() -> Scene {
    let mut scene = Scene::new();
    scene.set_camera(Camera::new(
        Point3::new(0., 2., 0.),
        Vec3::new(-0.15, 1.8, 1.0),
        0.0, 2.5
    ));

    let cuboid = Primitive::Cuboid(Point3::new(0.0, 1., 6.), Vec3::new(0.5, 0.75, 0.5));
    let sphere = Primitive::Sphere(Point3::new(-1.25, 1., 6.), 0.8);
    scene.add(Primitive::AAPlane(Axis::Y, 0.));
    scene.add_csg(Csg::new(CsgOp::SmoothMax(5.0), [Some(cuboid), Some(sphere)]));
    scene.add_light(Point3::new(6., 5., -6.));
    return scene
}



fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let (vertex_buffer, indices, program) = load_program(&display);

    let scene = basic_scene();
    let mut time = 0f32;
    let mut held_keys = [false; 255];
    let mut mouse = [0f32; 4];
    event_loop.run(move |ev, _, control_flow| {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput {
                    input: glutin::event::KeyboardInput {
                        virtual_keycode: Some(keycode),
                        state,
                        .. }, .. } => { match state {
                        glutin::event::ElementState::Pressed => {
                            held_keys[keycode as usize] = true;
                        },
                        glutin::event::ElementState::Released => {
                            held_keys[keycode as usize] = false;
                        }
                    }
                },
                glutin::event::WindowEvent::MouseInput { button, state, .. } => { match state {
                        glutin::event::ElementState::Pressed => {
                            match button {
                                glutin::event::MouseButton::Left => { mouse[2] = 1.0; },
                                glutin::event::MouseButton::Right => { mouse[3] = 1.0; },
                                _ => ()
                            }
                        },
                        glutin::event::ElementState::Released => {
                            match button {
                                glutin::event::MouseButton::Left => { mouse[2] = 0.0; },
                                glutin::event::MouseButton::Right => { mouse[3] = 0.0; },
                                _ => ()
                            }
                        }
                    }
                },
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    mouse[0] = position.x as f32;
                    mouse[1] = position.y as f32;
                }
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
        let buffer_objects = glium::uniforms::UniformBuffer::new(&display, UniformBlockObjects {
            objects: scene.get_objects(), 
        }).unwrap();
        let buffer_lights = glium::uniforms::UniformBuffer::new(&display, UniformBlockLights {
            lights: scene.get_lights(), 
        }).unwrap();
        let buffer_csgs = glium::uniforms::UniformBuffer::new(&display, UniformBlockCsgs {
            csgs: scene.get_csgs(),
        }).unwrap();
        target.draw(&vertex_buffer, &indices, &program, &uniform! {
            time: time, 
            resolution: [display.get_framebuffer_dimensions().0 as f32, display.get_framebuffer_dimensions().1 as f32],
            mouse: mouse,

            camera: scene.camera.as_data(),
            camera_origin: scene.camera.origin.to_tuple(),
            camera_focal_length: scene.camera.focal_length,
            scene_objects: &buffer_objects,
            scene_lights: &buffer_lights,
            scene_csgs: &buffer_csgs,
        }, &Default::default()).unwrap();
        target.finish().unwrap();
    });


}