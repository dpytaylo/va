use std::cell::{Ref, RefCell};
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

use thiserror::Error;
use vulkano_win::VkSurfaceBuild;

use crate::va::layer::Layer;

use super::graphics::window_graphics::WindowGraphics;
use super::graphics::window_render::WindowRender;

use super::global::Va;

pub struct WindowBuilder<'a> {
    width: usize,
    height: usize,
    title: &'a str,
}

impl<'a> WindowBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_inner_size(mut self, width: usize, height: usize) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn build(self, va: &Va) -> Result<Rc<Window>, WindowCreationError> {
        Window::new(va, self.width, self.height, self.title)
    }
}

impl<'a> Default for WindowBuilder<'a> {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Default Window Title",
        }
    }
}

#[derive(Debug, Error)]
pub enum WindowCreationError {
    #[error("surface creation error")]
    SurfaceCreationError(#[from] vulkano_win::CreationError),

    #[error("no device availaible")]
    NoDeviceAvailaible,

    #[error("no queue availaible")]
    NoQueueAvailable,

    #[error("other error")]
    Other(#[from] anyhow::Error),
}

pub struct Window {
    graphics: WindowGraphics,
    render: WindowRender,
    layers: RefCell<Vec<Rc<Layer>>>,
}

impl Window {
    fn new(
        va: &Va,
        width: usize,
        height: usize,
        title: &str,
    ) -> Result<Rc<Self>, WindowCreationError> {
        let surface = winit::window::WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(width as f64, height as f64))
            .with_title(title)
            .build_vk_surface(
                va.event_loop_window_target(),
                Arc::clone(va.graphics.instance()),
            )?;

        let device = match va.graphics.device() {
            Some(val) => Arc::clone(&val),
            None => return Err(WindowCreationError::NoDeviceAvailaible),
        };

        let graphics = WindowGraphics::new(Arc::clone(&device), surface)?;
        let render = WindowRender::new(device, &graphics)?;

        let window = Rc::new(Self {
            graphics,
            render,
            layers: RefCell::default(),
        });

        va.windows_mut().push(Rc::clone(&window));
        Ok(window)
    }

    pub fn width(&self) -> usize {
        self.graphics.surface().window().inner_size().width as usize
    }

    pub fn height(&self) -> usize {
        self.graphics.surface().window().inner_size().height as usize
    }

    pub fn set_title(&self, title: &str) {
        self.graphics.surface().window().set_title(title);
    }

    pub fn add_layer(&self, layer: Rc<Layer>) {
        self.layers.borrow_mut().push(layer);
    }

    pub fn winit_window(&self) -> &winit::window::Window {
        self.graphics.surface().window()
    }

    pub fn graphics(&self) -> &WindowGraphics {
        &self.graphics
    }

    pub fn render(&self) -> &WindowRender {
        &self.render
    }

    pub fn layers(&self) -> Ref<Vec<Rc<Layer>>> {
        self.layers.borrow()
    }
}

// pub trait WindowAddWidget<T> {
//     fn add_widget(&mut self, widget: T);
// }

// impl WindowAddWidget<WidgetData> for Window {
//     fn add_widget(&mut self, widget: WidgetData) {
//         self.widgets.push(WidgetKind::Widget(widget));
//     }
// }

// impl WindowAddWidget<Label> for Window {
//     fn add_widget(&mut self, widget: Label) {
//         self.widgets.push(WidgetKind::Label(widget));
//     }
// }
