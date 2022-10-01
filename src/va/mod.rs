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

use env_logger::Target;
use log::{error, LevelFilter};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use graphics::render::Render;

use global::Va;

pub trait MainLoop {
    fn run(
        &mut self, // event: Event<()>,
                   // control_flow: &mut ControlFlow
    );
}

pub struct DefaultMainLoop;

impl MainLoop for DefaultMainLoop {
    fn run(&mut self) 
    {
        // do nothing
    }
}

#[derive(Default)]
pub struct Application<T = (), U = DefaultMainLoop> 
    where T: FnOnce(&Va),
          U: MainLoop + 'static,
{
    initialize_closure: Option<T>,
    main_loop: U,
}

impl<T, U> Application<T, U> 
    where T: FnOnce(&Va),
          U: MainLoop + 'static,
{
    pub fn new(main_loop: U) -> Self {
        Self {
            initialize_closure: None,
            main_loop,
        }
    }

    pub fn with_initialize(mut self, closure: T) -> Self {
        self.initialize_closure = Some(closure);
        self
    }

    pub fn run(self) {
        env_logger::builder()
            .filter_level(LevelFilter::Info)
            .target(Target::Stdout)
            .init();

        let event_loop = EventLoop::new();
        let va = match Va::new(&event_loop) {
            Ok(val) => val,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        let Self { initialize_closure, mut main_loop } = self;

        if let Some(initialize_closure) = initialize_closure {
            initialize_closure(&va);
        }

        event_loop.run(move |event, event_loop_window_target, control_flow| {
            *control_flow = ControlFlow::Wait;
            
            va.graphics.update();
            main_loop.run();

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
