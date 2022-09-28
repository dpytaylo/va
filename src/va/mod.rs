pub mod graphics;
pub mod utils;
pub mod widgets;

pub mod global;
pub mod layer;
pub mod manager;
pub mod object_data;
pub mod object;
pub mod time;
pub mod window;

// abcdefghijklmnopqrstuvwxyz
use std::rc::Rc;

use log::error;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use graphics::render::Render;

use global::Va;

pub trait MainLoop {
    fn run(
        &mut self, // event: Event<()>,
                   // control_flow: &mut ControlFlow
    );
}

pub struct Application();

impl Application {
    pub fn run<T>(va: Va, mut main_loop: T)
    where
        T: MainLoop + 'static,
    {
        let event_loop = va.take_event_loop();
        va.set_event_loop_window_target(&*event_loop);

        event_loop.run(move |event, event_loop_window_target, control_flow| {
            *control_flow = ControlFlow::Wait;
            
            main_loop.run();
            va.graphics.update();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    window_id,
                } => {
                    for window in va.windows().iter() {
                        if window.winit_window().id() == window_id {
                            window.graphics().set_recreate_swapchain(true);
                        }
                    }
                }
                Event::RedrawRequested(window_id) => {
                    for window in va.windows().iter() {
                        if window.winit_window().id() == window_id {
                            match Render::draw(&va, &window) {
                                Ok(_) => (),
                                Err(err) => error!("{:?}", err),
                            }
                            break;
                        }
                    }
                }
                // Event::RedrawEventsCleared => {
                //     for window in va.windows().iter() {
                //         match Render::draw(&va, &window) {
                //             Ok(_) => (),
                //             Err(err) => error!("{:?}", err),
                //         }
                //         break;
                //     }
                // }
                _ => (),
            }
        });
    }
}
