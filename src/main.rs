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
        Point3::new(0., 1., 0.),
        Vec3::new(0., 1., 1.),
        0.0, 1.0
    ));

    // let cuboid = Primitive::Cuboid(Point3::new(0.0, 1., 6.), Vec3::new(0.5, 0.75, 0.5));
    let sphere = Primitive::Sphere(Point3::new(0., 1., 6.), 0.75, Rgb::new(0.2, 1.0, 0.2));
    scene.add(Primitive::AAPlane(Axis::Y, 0., Rgb::new(1.0, 1.0, 1.0)));
    scene.add(sphere);
    // scene.add_csg(Csg::new(CsgOp::SmoothMax(5.0), [Some(cuboid), Some(sphere)]));
    scene.add_light(Point3::new(6., 5., -6.));
    return scene
}

fn input(scene: &mut Scene, held_keys: &[bool; 255]) {
    let movement_speed = 0.025;
    let rotate_speed = 0.02;

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
            glutin::event::Event::MainEventsCleared => {
                input(&mut scene, &held_keys);
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
        let scene_settings = glium::uniforms::UniformBuffer::new(&display, SceneSettingsBlock {
            background_color: [0.7, 0.7, 0.9],
        }).unwrap();
        target.draw(&vertex_buffer, &indices, &program, &uniform! {
            time: time, 
            resolution: [display.get_framebuffer_dimensions().0 as f32, display.get_framebuffer_dimensions().1 as f32],
            mouse: mouse,

            camera: scene.camera.as_data(),
            camera_origin: scene.camera.origin.to_tuple(),
            camera_focal_length: scene.camera.focal_length,

            scene_settings: &scene_settings,

            scene_objects: &buffer_objects,
            scene_lights: &buffer_lights,
            scene_csgs: &buffer_csgs,
        }, &Default::default()).unwrap();
        target.finish().unwrap();
    });


}