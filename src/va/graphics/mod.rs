pub mod framerate_counter;
pub mod glyph_render;
pub mod image;
pub mod layer_render_data_handle;
pub mod layer_render_data;
pub mod buffer;
pub mod mesh;
pub mod rasterizate;
pub mod render_data;
pub mod render;
pub mod render_state;
pub mod shaders;
pub mod window_graphics;
pub mod window_render;

// abcdefghijklmnopqrstuvwxyz
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use anyhow::{bail, Context};
use log::{error, info};

use vulkano::device::{
    physical::PhysicalDevice, physical::QueueFamily, Device, DeviceCreationError, DeviceExtensions,
    Features, Queue, QueuesIter,
};
use vulkano::instance::{self, debug::DebugCallback, Instance, LayerProperties, Version};
use vulkano::swapchain::Surface;
use vulkano::sync::GpuFuture;

use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoopWindowTarget;

const REQUIRED_EXTENSIONS: DeviceExtensions = DeviceExtensions {
    khr_swapchain: true,
    ..DeviceExtensions::none()
};

const REQUIRED_FEATURES: Features = Features {
    //shader_float64: true, TODO
    ..Features::none()
};

pub struct Graphics {
    instance: Arc<Instance>,

    #[allow(dead_code)]
    debug_callback: DebugCallback,

    device: RefCell<Option<Arc<Device>>>,
    queue: RefCell<Option<Arc<Queue>>>,

    futures: RefCell<Vec<Box<dyn GpuFuture>>>,
}

impl Graphics {
    pub fn new() -> anyhow::Result<Rc<Self>> {
        let (instance, debug_callback) = Graphics::create_instance()?;

        Ok(Rc::new(Self {
            instance,
            debug_callback,

            device: RefCell::default(),
            queue: RefCell::default(),

            futures: RefCell::default(),
        }))
    }

    fn create_instance() -> anyhow::Result<(Arc<Instance>, DebugCallback)> {
        let mut request_extensions = vulkano_win::required_extensions();

        // Debug
        request_extensions.ext_debug_utils = true;
        let validation_layers = ["VK_LAYER_KHRONOS_validation"];

        let layers: Vec<LayerProperties> = instance::layers_list().unwrap().collect();

        let mut layers_str = String::with_capacity(layers.len() * 10);
        let mut first_time = true;

        for layer in &layers {
            if first_time {
                layers_str.push_str(layer.name());
                first_time = false;
            } else {
                layers_str.push_str(&format!("\n{}", layer.name()));
            }
        }
        info!("Available layers:\n{}", layers_str);

        for validation_layer in validation_layers {
            let mut found = false;

            for layer in &layers {
                if validation_layer == layer.name() {
                    found = true;
                    break;
                }
            }

            if !found {
                bail!("validation layers not found")
            }
        }

        let instance = Instance::new(None, Version::V1_2, &request_extensions, validation_layers)?;
        let debug_callback = DebugCallback::errors_and_warnings(&instance, |msg| {
            error!("Vulkan debug: {:?}", msg.description);
        })?;

        Ok((instance, debug_callback))
    }

    pub fn setup_device_and_queues(
        &self,
        event_loop: &EventLoopWindowTarget<()>,
    ) -> anyhow::Result<()> {
        let test_surface = winit::window::WindowBuilder::new()
            .with_visible(false)
            .build_vk_surface(event_loop, Arc::clone(&self.instance))
            .context("failed to build surface")?;

        let (physical, queue_family) =
            Graphics::get_physical_device(&self.instance, &test_surface)?;
        let (device, mut queues) = Graphics::create_device(physical, queue_family)?;

        let properies = device.physical_device().properties();
        info!(
            "Device: {} ({:?})",
            properies.device_name, properies.device_type
        );

        *self.device.borrow_mut() = Some(device); // TODO ?
        *self.queue.borrow_mut() = Some(queues.next().unwrap());

        Ok(())
    }

    fn get_physical_device<'a>(
        instance: &'a Arc<Instance>,
        surface: &'a Arc<Surface<winit::window::Window>>,
    ) -> anyhow::Result<(PhysicalDevice<'a>, QueueFamily<'a>)> 
    {
        match PhysicalDevice::enumerate(instance)
            .filter(|physical| {
                physical.supported_extensions().is_superset_of(&REQUIRED_EXTENSIONS)
                && physical.supported_features().is_superset_of(&REQUIRED_FEATURES)
                && physical.queue_families().find(|&queue_family| queue_family.supports_graphics()
                    && surface.is_supported(queue_family).is_ok()).is_some()
            }).next()
        {
            Some(physical) 
                => Ok((physical, physical.queue_families().find(|q| q.supports_graphics()).unwrap())),
            None => bail!("no supporting physical devices"),
        }
    }

    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    pub fn device(&self) -> Option<Arc<Device>> {
        (*self.device.borrow()).as_ref().map(Arc::clone)
    }

    pub fn queue(&self) -> Option<Arc<Queue>> {
        (*self.queue.borrow()).as_ref().map(Arc::clone)
    }

    fn create_device(
        physical: PhysicalDevice,
        queue_family: QueueFamily,
    ) -> Result<(Arc<Device>, QueuesIter), DeviceCreationError> 
    {
        Device::new(
            physical,
            &REQUIRED_FEATURES,
            &REQUIRED_EXTENSIONS,
            [(queue_family, 0.5)].iter().cloned(),
        )
    }

    pub fn new_future(&self, future: Box<dyn GpuFuture>) {
        self.futures.borrow_mut().push(future);
    }

    pub fn update(&self) {
        while let Some(mut future) = self.futures.borrow_mut().pop() {
            future.cleanup_finished(); // TODO ???
        }
    }
}
