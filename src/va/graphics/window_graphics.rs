// abcdefghijklmnopqrstuvwxyz
use std::cell::{Cell, RefCell};
use std::cmp::{max, min};
use std::sync::Arc;

use anyhow::{bail, Context};
use vulkano::device::{Device, Queue};
use vulkano::format::Format;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::swapchain::{Surface, Swapchain, SurfaceInfo, SwapchainCreateInfo};

pub struct WindowGraphics {
    surface: Arc<Surface<winit::window::Window>>,
    swapchain: RefCell<Arc<Swapchain<winit::window::Window>>>,
    images: RefCell<Option<Vec<Arc<SwapchainImage<winit::window::Window>>>>>,
    recreate_swapchain: Cell<bool>,
}

impl WindowGraphics {
    pub fn new(
        device: Arc<Device>,
        queue: Arc<Queue>,
        surface: Arc<Surface<winit::window::Window>>,
    ) -> anyhow::Result<Self> 
    {
        let (swapchain, images) =
            WindowGraphics::create_swapchain(device, queue, Arc::clone(&surface))?;

        Ok(Self {
            surface,
            swapchain: RefCell::new(swapchain),
            images: RefCell::new(Some(images)),
            recreate_swapchain: Cell::new(false),
        })
    }

    pub fn surface(&self) -> &Arc<Surface<winit::window::Window>> {
        &self.surface
    }

    pub fn set_swapchain(&self, swapchain: Arc<Swapchain<winit::window::Window>>) {
        *self.swapchain.borrow_mut() = swapchain;
    }

    pub fn swapchain(&self) -> Arc<Swapchain<winit::window::Window>> {
        Arc::clone(&self.swapchain.borrow())
    }

    pub fn take_images(&self) -> Vec<Arc<SwapchainImage<winit::window::Window>>> {
        self.images
            .borrow_mut()
            .take()
            .expect("no available images")
    }

    pub fn set_recreate_swapchain(&self, recreate_swapchain: bool) {
        self.recreate_swapchain.set(recreate_swapchain);
    }

    pub fn is_recreate_swapchain(&self) -> bool {
        self.recreate_swapchain.get()
    }

    pub fn create_swapchain(
        device: Arc<Device>,
        queue: Arc<Queue>,
        surface: Arc<Surface<winit::window::Window>>,
    ) -> anyhow::Result<(
        Arc<Swapchain<winit::window::Window>>,
        Vec<Arc<SwapchainImage<winit::window::Window>>>,
    )> 
    {
        let physical = device.physical_device();
        let caps = physical.surface_capabilities(&surface, SurfaceInfo::default())?;

        let supported_formats = physical.surface_formats(&surface, SurfaceInfo::default())
            .context("failed to create Swapchain")?;

        if supported_formats.is_empty() {
            bail!("no supported formats for Swapchain");
        }

        let image_format = supported_formats.iter()
            .find(|&val| val.0 == Format::B8G8R8A8_SRGB)
            .unwrap_or_else(|| &supported_formats[0]);

        Ok(Swapchain::new(
            Arc::clone(&device),
            Arc::clone(&surface),
            SwapchainCreateInfo {
                min_image_count: caps.min_image_count,
                image_format: Some(image_format.0),
                image_color_space: image_format.1,
                image_extent: surface.window().inner_size().into(),
                image_usage: ImageUsage::color_attachment(),
                composite_alpha: caps.supported_composite_alpha.iter().next().context("failed to create swapchain")?,
                ..Default::default()
            },
        ).context("failed to create swapchain")?)
    }

    // TODO
    // pub fn recreate_swapchain(&self)
    //     -> Result<(Arc<Swapchain<winit::window::Window>>, Vec<Arc<SwapchainImage<winit::window::Window>>>), GraphicsError>
    // {

    // }
}
