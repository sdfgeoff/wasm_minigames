use std::env;
use std::fs;

mod app;
use app::keyboard;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

fn debug_log(message: &str) {
    println!("{}", message);
}

fn main() {
    // Attempt to read the data package
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // Make some assumptions at this point
    let target_resolution = [1024, 768];
    let pixels_per_centimeter = 96.0 / 2.54;

    // Create our window
    let (gl, window, event_loop) = {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Wasm Minigames")
            .with_inner_size(glutin::dpi::LogicalSize::new(
                target_resolution[0] as f32,
                target_resolution[1] as f32,
            ));

        let window = unsafe {
            glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap()
        };
        let gl = unsafe {
            glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _)
        };
        (gl, window, event_loop)
    };

    let since_the_epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time is pre 1970???");

    let mut app = app::App::new(
        gl,
        "".to_string(),
        since_the_epoch.as_secs_f64(),
        target_resolution,
        pixels_per_centimeter,
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {
                return;
            }
            Event::MainEventsCleared => {
                window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Put rendering in here apparently

                let since_the_epoch = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time is pre 1970???");

                app.animation_frame(since_the_epoch.as_secs_f64());
                window.swap_buffers().unwrap();
            }
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    app.update_resolution(
                        [physical_size.width as i32, physical_size.height as i32],
                        96.0 / 2.54, // TODO
                    );

                    window.resize(*physical_size);
                }
                WindowEvent::CloseRequested => {
                    // Put something in here too
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::KeyboardInput { ref input, .. } => {
                    if let Some(meaning) = input.virtual_keycode {
                        let keycode = to_keycode(meaning);
                        if let Some(code) = keycode {
                            app.key_event(
                                code,
                                input.state == glutin::event::ElementState::Pressed,
                            );
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    });
}

fn to_keycode(keycode: glutin::event::VirtualKeyCode) -> Option<keyboard::KeyCode> {
    match keycode {
        glutin::event::VirtualKeyCode::W => Some(keyboard::KeyCode::W),
        glutin::event::VirtualKeyCode::A => Some(keyboard::KeyCode::A),
        glutin::event::VirtualKeyCode::S => Some(keyboard::KeyCode::S),
        glutin::event::VirtualKeyCode::D => Some(keyboard::KeyCode::D),
        glutin::event::VirtualKeyCode::Q => Some(keyboard::KeyCode::Q),
        glutin::event::VirtualKeyCode::E => Some(keyboard::KeyCode::E),
        glutin::event::VirtualKeyCode::R => Some(keyboard::KeyCode::R),
        glutin::event::VirtualKeyCode::F => Some(keyboard::KeyCode::F),
        glutin::event::VirtualKeyCode::Space => Some(keyboard::KeyCode::Space),
        glutin::event::VirtualKeyCode::RShift => Some(keyboard::KeyCode::Shift),
        glutin::event::VirtualKeyCode::LShift => Some(keyboard::KeyCode::Shift),
        glutin::event::VirtualKeyCode::RControl => Some(keyboard::KeyCode::Ctrl),
        glutin::event::VirtualKeyCode::LControl => Some(keyboard::KeyCode::Ctrl),
        glutin::event::VirtualKeyCode::RAlt => Some(keyboard::KeyCode::Alt),
        glutin::event::VirtualKeyCode::LAlt => Some(keyboard::KeyCode::Alt),
        glutin::event::VirtualKeyCode::Escape => Some(keyboard::KeyCode::Escape),
        glutin::event::VirtualKeyCode::Left => Some(keyboard::KeyCode::Left),
        glutin::event::VirtualKeyCode::Up => Some(keyboard::KeyCode::Up),
        glutin::event::VirtualKeyCode::Right => Some(keyboard::KeyCode::Right),
        glutin::event::VirtualKeyCode::Down => Some(keyboard::KeyCode::Down),
        _ => None,
    }
}
