// abcdefghijklmnopqrstuvwxyz
use std::cell::{Cell, RefCell};
use std::cmp::{max, min};
use std::sync::Arc;

use anyhow::bail;
use vulkano::device::{Device, Queue};
use vulkano::format::Format;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::swapchain::{Surface, Swapchain};

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
        let caps = surface.capabilities(device.physical_device())?;

        // NOTE:
        // On some drivers the swapchain dimensions are specified by `caps.current_extent` and the
        // swapchain size must use these dimensions.
        // These dimensions are always the same as the window dimensions.
        let dimensions: [u32; 2] = surface.window().inner_size().into();

        let buffer_count = match caps.max_image_count {
            Some(limit) => min(max(2, caps.min_image_count), limit),
            None => max(2, caps.min_image_count),
        };

        let transform = caps.current_transform;

        if caps.supported_formats.is_empty() {
            bail!("no supported formats for swapchain");
        }

        let (format, color_space) = *caps.supported_formats.iter().find(|&val| val.0 == Format::B8G8R8A8_SRGB)
            .unwrap_or_else(|| &caps.supported_formats[0]);

        let usage = ImageUsage::color_attachment(); // Only color attachment
        let composite_alpha = caps
            .supported_composite_alpha
            .iter()
            .next()
            .expect("no supported composite alpha is supported");

        Ok(Swapchain::start(device, surface)
            .num_images(buffer_count)
            .format(format)
            .color_space(color_space)
            .dimensions(dimensions)
            .usage(usage)
            .sharing_mode(&queue)
            .transform(transform)
            .composite_alpha(composite_alpha)
            .build()?
        )
    }

    // TODO
    // pub fn recreate_swapchain(&self)
    //     -> Result<(Arc<Swapchain<winit::window::Window>>, Vec<Arc<SwapchainImage<winit::window::Window>>>), GraphicsError>
    // {

    // }
}
