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
    event_loop: RefCell<Option<EventLoop<()>>>,
    event_loop_window_target: Cell<Option<*const EventLoopWindowTarget<()>>>,

    pub time: Time,
    pub graphics: Rc<Graphics>,
    pub manager: Rc<Manager>,

    windows: RefCell<Vec<Rc<Window>>>,
}

impl Va {
    pub fn new() -> anyhow::Result<Rc<Self>> {
        env_logger::builder()
            .filter_level(LevelFilter::Info)
            .target(Target::Stdout)
            .init();

        // let event_loop = RefCell::new(Some(EventLoop::new()));
        // let graphics = Graphics::new()?;
        // let windows = RefCell::default();

        // let va = Rc::new({
        //     let mut uninit = MaybeUninit::<Self>::uninit();
        //     let ptr = uninit.as_mut_ptr();

        //     unsafe {
        //         addr_of_mut!((*ptr).event_loop).write(event_loop);

        //         // let tmp = (*addr_of_mut!((*ptr).event_loop)).borrow();
        //         // let event_loop_window_target = tmp.as_deref().unwrap();
        //         // addr_of_mut!((*ptr).event_loop_window_target).write(Cell::new(Some(event_loop_window_target)));

        //         addr_of_mut!((*ptr).event_loop_window_target).write(Cell::default());
        //         addr_of_mut!((*ptr).graphics).write(graphics);

        //         let manager = Manager::new(&*addr_of_mut!((*ptr).graphics));
        //         addr_of_mut!((*ptr).manager).write(manager);

        //         addr_of_mut!((*ptr).windows).write(windows);

        //         uninit.assume_init()
        //     }
        // });

        // va.event_loop_window_target.set(Some(va.event_loop.borrow().as_deref().unwrap() as *const EventLoopWindowTarget<()>));

        // dbg!(va.event_loop.borrow().as_ref().unwrap() as *const EventLoop<()>);
        // dbg!(va.event_loop.borrow().as_deref().unwrap() as *const EventLoopWindowTarget<()>);
        // dbg!(va.event_loop_window_target.get().unwrap());

        let event_loop = RefCell::new(Some(EventLoop::new()));

        let graphics = Graphics::new()?;
        let manager = Manager::new(Rc::clone(&graphics));

        let va = Rc::new(Self {
            event_loop,
            event_loop_window_target: Cell::default(),

            time: Time::new(),
            graphics,
            manager,

            windows: RefCell::default(),
        });

        va.event_loop_window_target.set(Some(
            va.event_loop.borrow().as_deref().unwrap() as *const EventLoopWindowTarget<()>
        ));
        va.graphics
            .setup_device_and_queues(va.event_loop_window_target())?;

        Ok(va)
    }

    pub fn take_event_loop(&self) -> EventLoop<()> {
        self.event_loop.borrow_mut().take().expect("no event loop")
    }

    pub fn set_event_loop_window_target(&self, event_loop: &EventLoopWindowTarget<()>) {
        self.event_loop_window_target.set(Some(event_loop));
    }

    pub fn event_loop_window_target(&self) -> &EventLoopWindowTarget<()> {
        unsafe {
            &*self
                .event_loop_window_target
                .get()
                .expect("invalid event loop window target")
        }
    }

    pub fn windows(&self) -> Ref<Vec<Rc<Window>>> {
        self.windows.borrow()
    }

    pub fn windows_mut(&self) -> RefMut<Vec<Rc<Window>>> {
        self.windows.borrow_mut()
    }
}
