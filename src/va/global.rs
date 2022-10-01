use std::cell::{Cell, Ref, RefCell, RefMut};
use std::rc::Rc;

use env_logger::Target;
use log::LevelFilter;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};

use crate::time::Time;

use super::graphics::Graphics;
use super::manager::Manager;
use super::window::Window;

pub struct Va {
    event_loop_window_target: *const EventLoopWindowTarget<()>,

    pub time: Time,
    pub graphics: Rc<Graphics>,
    pub manager: Rc<Manager>,

    windows: RefCell<Vec<Rc<Window>>>,
}

impl Va {
    pub fn new(event_loop_window_target: &EventLoopWindowTarget<()>) -> anyhow::Result<Self> {
        let graphics = Graphics::new()?;
        let manager = Manager::new(Rc::clone(&graphics));

        let va = Self {
            event_loop_window_target: event_loop_window_target,

            time: Time::new(),
            graphics,
            manager,

            windows: RefCell::default(),
        };

        // va.event_loop_window_target.set(Some(
        //     va.event_loop.borrow().as_deref().unwrap() as *const EventLoopWindowTarget<()>
        // ));

        // TODO
        va.graphics.setup_device_and_queues(va.event_loop_window_target())?;
        Ok(va)
    }

    pub fn event_loop_window_target(&self) -> &EventLoopWindowTarget<()> {
        // unsafe {
        //     &*self
        //         .event_loop_window_target
        //         .get()
        //         .expect("invalid event loop window target")
        // }

        unsafe {
            &*self.event_loop_window_target
        }
    }

    pub fn windows(&self) -> Ref<Vec<Rc<Window>>> {
        self.windows.borrow()
    }

    pub fn windows_mut(&self) -> RefMut<Vec<Rc<Window>>> {
        self.windows.borrow_mut()
    }
}
