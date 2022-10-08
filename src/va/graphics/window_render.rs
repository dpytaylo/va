// abcdefghijklmnopqrstuvwxyz
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::sync::Arc;

use anyhow::{bail, Context};

use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::image::{view::ImageView, ImageAccess, ImageDimensions, SwapchainImage};
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::render_pass::{Framebuffer, RenderPass, RenderPassCreationError, FramebufferCreateInfo};
use vulkano::swapchain::{self, AcquireError, SwapchainCreationError, SwapchainCreateInfo};
use vulkano::sync::{self, FlushError, GpuFuture};

use crate::global::Va;
use crate::va::layer::Layer;

use super::Graphics;
use super::framerate_counter::FramerateCounter;
use super::window_graphics::WindowGraphics;

pub struct WindowRender {
    viewport: RefCell<Viewport>,
    framebuffers: RefCell<Vec<Arc<Framebuffer>>>,
    render_pass: Arc<RenderPass>,
    previous_frame_end: RefCell<Option<Box<dyn GpuFuture>>>,
    framerate_counter: FramerateCounter,
}

impl WindowRender {
    pub fn new(device: Arc<Device>, window_graphics: &WindowGraphics) -> anyhow::Result<Self> {
        let render_pass =
            Self::create_render_pass(Arc::clone(&device), window_graphics.swapchain().image_format())?;
        let (viewport, framebuffers) = Self::create_viewport_and_framebuffers(
            window_graphics.take_images(),
            Arc::clone(&render_pass),
        )?;

        Ok(Self {
            viewport: RefCell::new(viewport),
            framebuffers: RefCell::new(framebuffers),
            render_pass,
            previous_frame_end: RefCell::new(Some(sync::now(device).boxed())),
            framerate_counter: FramerateCounter::new(),
        })
    }

    fn create_render_pass(
        device: Arc<Device>,
        format: Format,
    ) -> Result<Arc<RenderPass>, RenderPassCreationError> {
        let render_pass = vulkano::single_pass_renderpass!(device,
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: format,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )?;

        Ok(render_pass)
    }

    fn create_viewport_and_framebuffers(
        images: Vec<Arc<SwapchainImage<winit::window::Window>>>,
        render_pass: Arc<RenderPass>,
    ) -> anyhow::Result<(Viewport, Vec<Arc<Framebuffer>>)> {
        let dimensions = images[0].dimensions();

        let (width, height) = match dimensions {
            ImageDimensions::Dim1d { .. } => panic!("invalid image dimensions"),
            ImageDimensions::Dim2d { width, height, .. }
            | ImageDimensions::Dim3d { width, height, .. } => (width, height),
        };

        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [width as f32, height as f32],
            depth_range: 0.0..1.0,
        };

        let mut framebuffers = Vec::with_capacity(images.len());
        for image in images {
            let view =
                ImageView::new_default(image).context("failed to create image view for the framebuffer")?;

            let framebuffer_create_info = FramebufferCreateInfo {
                attachments: vec![view],
                ..Default::default()
            };

            let framebuffer = Framebuffer::new(
                Arc::clone(&render_pass),
                framebuffer_create_info,
            )?;

            framebuffers.push(framebuffer);
        }

        Ok((viewport, framebuffers))
    }

    pub fn render_pass(&self) -> &Arc<RenderPass> {
        &self.render_pass
    }

    pub fn framerate_counter(&self) -> &FramerateCounter {
        &self.framerate_counter
    }

    pub fn draw(
        &self,
        graphics: &Rc<Graphics>,
        window_graphics: &WindowGraphics,
        layers: Ref<Vec<Rc<Layer>>>,
    ) -> anyhow::Result<()> 
    {
        self.framerate_counter.update();

        let mut previous_frame_end = self.previous_frame_end.borrow_mut().take().unwrap();
        previous_frame_end.cleanup_finished();
        
        if window_graphics.is_recreate_swapchain() {
            self.recreate_swapchain(window_graphics)?;
        }

        let device = graphics.device().expect("no available device");
        let queue = graphics.queue().expect("no available queue");

        let swapchain = window_graphics.swapchain();

        let (image_num, suboptimal, acquire_future) 
            = match swapchain::acquire_next_image(Arc::clone(&swapchain), None)
        {
            Ok(val) => val,
            Err(AcquireError::OutOfDate) => {
                window_graphics.set_recreate_swapchain(true);
                return Ok(());
            }
            Err(err) => bail!(err),
        };

        if suboptimal {
            window_graphics.set_recreate_swapchain(true);
            return Ok(()); // TODO ???
        }

        for layer in layers.iter() {
            layer.update_layer_render_data();
        }

        let layer = layers.iter().next().unwrap();
        let render_data = layer.render_data();
        let rdata = render_data.iter().next().unwrap().as_ref().unwrap();

        let command_buffer = rdata.command_buffer(
            graphics,
            Arc::clone(&self.framebuffers.borrow()[image_num]), 
            self.viewport.borrow().clone()
        )?;

        let command_buffer = if let Some(val) = command_buffer {
            val
        }
        else {
            return Ok(());
        };

        let future = previous_frame_end
            .join(acquire_future)
            .then_execute(Arc::clone(&queue), command_buffer)
            .unwrap()
            .then_swapchain_present(queue, swapchain, image_num)
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                *self.previous_frame_end.borrow_mut() = Some(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                window_graphics.set_recreate_swapchain(true);
                *self.previous_frame_end.borrow_mut() = Some(sync::now(device).boxed());
            }
            Err(err) => {
                *self.previous_frame_end.borrow_mut() = Some(sync::now(device).boxed());
                bail!(err);
            }
        }

        Ok(())
    }

    fn recreate_swapchain(&self, window_graphics: &WindowGraphics) -> anyhow::Result<()> {
        let dimensions = window_graphics.surface().window().inner_size().into();
        let swapchain = window_graphics.swapchain();

        let (swapchain, images) = match swapchain
            .recreate(SwapchainCreateInfo {
                image_extent: dimensions,
                ..swapchain.create_info()
            })
        {
            Ok(val) => val,
            // This error tends to happen when the user is manually resizing the window.
            // Simply restarting the loop is the easiest way to fix this issue.
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return Ok(()),
            Err(err) => bail!(err),
        };

        let (viewport, framebuffers) =
            Self::create_viewport_and_framebuffers(images, Arc::clone(&self.render_pass))?;

        window_graphics.set_swapchain(swapchain);
        *self.viewport.borrow_mut() = viewport;
        *self.framebuffers.borrow_mut() = framebuffers;
        window_graphics.set_recreate_swapchain(false);

        Ok(())
    }
}
