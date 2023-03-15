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
        point3(0., 1., 0.),
        vec3(0., 1., 1.),
        0.0, 1.0
    ));

    scene.add(Primitive::Plane(vec3(0., 1., 0.), 0., rgb(1.0, 1.0, 1.0)), Transform::none());
    let sphere = scene.add(Primitive::Sphere(0.75, rgb(0.2, 1.0, 0.2)), Transform::translation(vec3(0., 1., 6.)));
    let cuboid = scene.add(Primitive::Cuboid(vec3(0.5, 0.5, 0.65), 0.05, rgb(1.0, 0.2, 0.2)), Transform::translation(vec3(1., 1., 6.)));
    scene.add_bool_op(BooleanOp::new(BooleanOpType::SmoothUnion(8.0), vec![sphere, cuboid]));
    scene.add_light(point3(6., 5., -6.));
    return scene
}

fn input(scene: &mut Scene, held_keys: &[bool; 255], prev_keys: &[bool; 255]) {
    let movement_speed = 0.04;
    let rotate_speed = 0.03;

    if held_keys[glutin::event::VirtualKeyCode::A as usize] {
        scene.camera.move_x(movement_speed); }
    if held_keys[glutin::event::VirtualKeyCode::D as usize] {
        scene.camera.move_x(-movement_speed); }
    if held_keys[glutin::event::VirtualKeyCode::W as usize] {
        scene.camera.move_z(movement_speed); }
    if held_keys[glutin::event::VirtualKeyCode::S as usize] {
        scene.camera.move_z(-movement_speed); }
    if held_keys[glutin::event::VirtualKeyCode::Q as usize] {
        scene.camera.move_y(movement_speed); }
    if held_keys[glutin::event::VirtualKeyCode::E as usize] {
        scene.camera.move_y(-movement_speed); }

    if held_keys[glutin::event::VirtualKeyCode::Left as usize] {
        scene.camera.rotate_x(rotate_speed); }
    if held_keys[glutin::event::VirtualKeyCode::Right as usize] {
        scene.camera.rotate_x(-rotate_speed); }
    // if held_keys[glutin::event::VirtualKeyCode::Up as usize] {
    //     scene.camera.rotate_y(rotate_speed); }
    // if held_keys[glutin::event::VirtualKeyCode::Down as usize] {
    //     scene.camera.rotate_y(-rotate_speed); }
}



fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let (vertex_buffer, indices, program) = load_program(&display);

    let mut scene = basic_scene();
    let mut time = 0f32;
    let mut held_keys = [false; 255];
    let mut prev_keys = [false; 255];
    let mut mouse = [0f32; 4];
    event_loop.run(move |ev, _, control_flow| {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => { match event {
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
                };
            },
            glutin::event::Event::MainEventsCleared => {
                input(&mut scene, &held_keys, &prev_keys);
            },
            glutin::event::Event::NewEvents(cause) => { match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => {
                        display.gl_window().window().request_redraw();
                    },
                    glutin::event::StartCause::Init => (),
                    _ => return,
                }
                prev_keys.copy_from_slice(&held_keys);
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
            resolution: [display.get_framebuffer_dimensions().0 as f32, display.get_framebuffer_dimensions().1 as f32],
            mouse: mouse,

            camera: scene.camera.as_data(),
            camera_origin: scene.camera.origin.to_tuple(),
            camera_focal_length: scene.camera.focal_length,
        }, &Default::default()).unwrap();
        target.finish().unwrap();
    });


}