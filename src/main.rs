#[macro_use]
extern crate glium;
use glium::glutin;
use glium::Surface;
use egui_glium::egui_winit::egui;


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

fn input(scene: &mut Scene, held_keys: &[bool; 255], prev_keys: &[bool; 255], selected_obj: &mut usize) {
    let movement_speed = 0.1;
    let rotate_speed = 0.08;

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

    if held_keys[glutin::event::VirtualKeyCode::Tab as usize] && !prev_keys[glutin::event::VirtualKeyCode::Tab as usize] {
        if held_keys[glutin::event::VirtualKeyCode::LShift as usize] {
            *selected_obj = (*selected_obj - 1) % scene.objects.len();
        } else {
            *selected_obj = (*selected_obj + 1) % scene.objects.len();
        }
    }
}



fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let (vertex_buffer, indices, program) = load_program(&display);
    let mut gui = egui_glium::EguiGlium::new(&display, &event_loop);
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(FONT_NAME.into(), egui::FontData::from_static(FONT_PATH));
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().insert(0, FONT_NAME.into());
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, FONT_NAME.into());
    gui.egui_ctx.set_fonts(fonts);

    let mut scene = basic_scene();
    let mut selected_object = 0;
    let mut time = 0f32;
    let mut held_keys = [false; 255];
    let mut prev_keys = [false; 255];
    let mut mouse = [0f32; 4];
    let mut new_object_choice = Primitive::Sphere(4., rgb(1., 1., 1.));

    let mut scene_fog_color = SceneFogColorBlock {
        fog_color: [0.30, 0.36, 0.60, 1.0],
    };
    let mut scene_params = SceneParamsBlock {
        params: [4.0, 1.0, 1.0, 0.0],
    };
    let mut scene_consts = SceneConstsBlock {
        consts: [scene.objects.len() as f32 + 1.0, 0.0, 0.0, 0.0],
    };
    event_loop.run(move |ev, _, control_flow| {
        let repaint_after = gui.run(&display, |egui_ctx| {
            let pos = scene.obj_transforms[selected_object].translate;
            let mut x = pos.x;
            let mut y = pos.y;
            let mut z = pos.z;
            egui::Window::new(&scene.objects[selected_object].as_str())
                .collapsible(true)
                .current_pos((20., 20.)).show(egui_ctx, |ui| {
                ui.label("Position");
                ui.add(egui::DragValue::new(&mut x).speed(0.05).prefix("X: "));
                ui.add(egui::DragValue::new(&mut y).speed(0.05).prefix("Y: "));
                ui.add(egui::DragValue::new(&mut z).speed(0.05).prefix("Z: "));
            });
            if (pos.x != x) || (pos.y != y) || (pos.z != z) {
                scene.obj_transforms[selected_object].translate = vec3(x, y, z);
            }

            egui::Window::new("Scene")
                .collapsible(true)
                .current_pos((200., 20.)).show(egui_ctx, |ui| {
                    ui.label("Objects");
                    egui::ComboBox::from_label("New Object")
                        .selected_text(format!("{}", new_object_choice.as_str()))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut new_object_choice, Primitive::Sphere(4., rgb(1., 1., 1.)), "Sphere");
                        });
                    // if ui.button("Add").clicked() {
                    //     scene.add(new_object_choice.clone());
                    // }

                    ui.label("Quality");
                    ui.add(egui::DragValue::new(&mut scene_params.params[0]).speed(0.05).prefix("Shadow Quality: "));

                    ui.label("Performance");
                    let mut shadows_enabled = scene_params.params[1] != 0.0;
                    let mut ao_enabled = scene_params.params[2] != 0.0;
                    ui.add(egui::Checkbox::new(&mut shadows_enabled, "Shadows Enabled"));
                    ui.add(egui::Checkbox::new(&mut ao_enabled, "Ambient Occlusion Enabled"));
                    scene_params.params[1] = shadows_enabled as i32 as f32;
                    scene_params.params[2] = ao_enabled as i32 as f32;
                });
        });

        *control_flow = if repaint_after.is_zero() {
            display.gl_window().window().request_redraw();
            glutin::event_loop::ControlFlow::Poll
        // } else if let Some(repaint_after_instant) = std::time::Instant::now().checked_add(repaint_after) {
        //     glutin::event_loop::ControlFlow::WaitUntil(repaint_after_instant)
        } else {
            let next_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
            glutin::event_loop::ControlFlow::WaitUntil(next_time)
        };

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

                let event_response = gui.on_event(&event);
                if event_response.repaint {
                    display.gl_window().window().request_redraw();
                }
            },
            glutin::event::Event::MainEventsCleared => {
                input(&mut scene, &held_keys, &prev_keys, &mut selected_object);
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
        let buffer_objects = glium::uniforms::UniformBuffer::new(&display, UniformBlockObjects {
            objects: scene.get_objects(), 
        }).unwrap();
        let buffer_lights = glium::uniforms::UniformBuffer::new(&display, UniformBlockLights {
            lights: scene.get_lights(), 
        }).unwrap();
        let buffer_bool_ops = glium::uniforms::UniformBuffer::new(&display, UniformBlockBoolOps {
            bool_ops: scene.get_bool_ops(),
        }).unwrap();
        let buffer_transforms = glium::uniforms::UniformBuffer::new(&display, UniformBlockTransforms {
            transformations: scene.get_transformations(),
        }).unwrap();
        let scene_fog_color = glium::uniforms::UniformBuffer::new(&display, scene_fog_color).unwrap();
        let scene_params = glium::uniforms::UniformBuffer::new(&display, scene_params).unwrap();
        let scene_consts = glium::uniforms::UniformBuffer::new(&display, scene_consts).unwrap();
        target.draw(&vertex_buffer, &indices, &program, &uniform! {
            time: time, 
            resolution: [display.get_framebuffer_dimensions().0 as f32, display.get_framebuffer_dimensions().1 as f32],
            mouse: mouse,

            camera: scene.camera.as_data(),
            camera_origin: scene.camera.origin.to_tuple(),
            camera_focal_length: scene.camera.focal_length,

            scene_fog_color: &scene_fog_color,
            scene_params: &scene_params,
            scene_consts: &scene_consts,

            scene_objects: &buffer_objects,
            scene_lights: &buffer_lights,
            scene_bool_ops: &buffer_bool_ops,
            scene_transforms: &buffer_transforms,
        }, &Default::default()).unwrap();
        gui.paint(&display, &mut target);
        target.finish().unwrap();
    });


}