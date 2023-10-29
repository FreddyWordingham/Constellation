mod camera;
mod hardware;
mod memory;
mod nbody;
mod pipelines;
mod settings;
mod simulation;

pub use camera::Camera;
pub use nbody::NBody;
pub use settings::Settings;

use hardware::Hardware;
use log;
use memory::{Memory, Vertex};
use pipelines::Pipelines;
use simulation::Simulation;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub async fn run(mut settings: Settings, init_conditions: NBody) {
    debug_assert!(init_conditions.is_valid());

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Heavens")
        .with_inner_size(winit::dpi::LogicalSize::new(
            settings.display_width * settings.pixel_size,
            settings.display_height * settings.pixel_size,
        ))
        .build(&event_loop)
        .unwrap();

    let mut simulation = Simulation::new(window, settings, init_conditions).await;

    event_loop.run(move |event, _, control_flow| {
        // control_flow.set_poll(); // Continuously runs the event loop
        control_flow.set_wait(); // Runs the event loop only when an event is received

        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == simulation.hardware.window.id() => {
                if !simulation.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => {
                            log::info!("Escape pressed, closing");
                            *control_flow = ControlFlow::Exit
                        }
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(key_code),
                                    ..
                                },
                            ..
                        } => match key_code {
                            VirtualKeyCode::Minus => {
                                settings.time_step /= 10.0;
                            }
                            VirtualKeyCode::Equals => {
                                settings.time_step *= 10.0;
                            }
                            VirtualKeyCode::Q => {
                                settings.zoom /= 2.0;
                            }
                            VirtualKeyCode::E => {
                                settings.zoom *= 2.0;
                            }
                            VirtualKeyCode::Z => {
                                settings.blur_radius /= 2.0;
                            }
                            VirtualKeyCode::X => {
                                settings.blur_radius *= 2.0;
                            }
                            VirtualKeyCode::F => {
                                settings.gravitational_constant /= 2.0;
                            }
                            VirtualKeyCode::G => {
                                settings.gravitational_constant *= 2.0;
                            }
                            VirtualKeyCode::A => {
                                settings.camera_x += 10.0;
                            }
                            VirtualKeyCode::D => {
                                settings.camera_x -= 10.0;
                            }
                            VirtualKeyCode::W => {
                                settings.camera_y += 10.0;
                            }
                            VirtualKeyCode::S => {
                                settings.camera_y -= 10.0;
                            }
                            VirtualKeyCode::O => {
                                settings.ghost_stack_visible_limit /= 2.0;
                            }
                            VirtualKeyCode::P => {
                                settings.ghost_stack_visible_limit *= 2.0;
                            }
                            _ => {
                                println!("Unbound key pressed: {:?}", key_code);
                            }
                        },
                        WindowEvent::Resized(physical_size) => {
                            simulation.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            simulation.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::MainEventsCleared => {
                log::debug!("Main events cleared");
                // simulation.hardware.window.request_redraw();
            }
            Event::RedrawRequested(window_id) if window_id == simulation.hardware.window.id() => {
                log::debug!("Redraw requested");
                simulation.update(&settings);
                match simulation.render() {
                    Ok(_) => {
                        log::debug!("Redraw complete");
                    }
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        simulation.resize(simulation.hardware.window.inner_size())
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                log::debug!("Redraw events cleared");
                simulation.hardware.window.request_redraw();
            }
            _ => (),
        }
    });
}
