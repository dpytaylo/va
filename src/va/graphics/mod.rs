pub mod font;
pub mod framerate_counter;
pub mod glyph_render;
pub mod image;
pub mod layer_render_data_handle;
pub mod layer_render_data;
pub mod layer_render_state;
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

use vulkano::device::{DeviceCreateInfo, QueueCreateInfo};
use vulkano::device::physical::PhysicalDeviceType;
use vulkano::device::{
    physical::PhysicalDevice, physical::QueueFamily, Device, DeviceCreationError, DeviceExtensions,
    Features, Queue,
};
use vulkano::instance::InstanceCreateInfo;
use vulkano::instance::debug::{DebugUtilsMessenger, DebugUtilsMessengerCreateInfo};
use vulkano::instance::{self, Instance, LayerProperties};
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
    debug_utils_messenger: DebugUtilsMessenger,

    device: RefCell<Option<Arc<Device>>>,
    queue: RefCell<Option<Arc<Queue>>>,

    futures: RefCell<Vec<Box<dyn GpuFuture>>>,
}

impl Graphics {
    pub fn new() -> anyhow::Result<Rc<Self>> {
        let (instance, debug_utils_messenger) = Graphics::create_instance()?;

        Ok(Rc::new(Self {
            instance,
            debug_utils_messenger,

            device: RefCell::default(),
            queue: RefCell::default(),

            futures: RefCell::default(),
        }))
    }

    fn create_instance() -> anyhow::Result<(Arc<Instance>, DebugUtilsMessenger)> {
        let mut request_extensions = vulkano_win::required_extensions();

        // Debug
        request_extensions.ext_debug_utils = true;
        
        //#[cfg(not(target_os = "macos"))]
        //let validation_layers = vec!["VK_LAYER_LUNARG_standard_validation".to_owned()];

        //#[cfg(target_os = "macos")]
        let validation_layers = vec!["VK_LAYER_KHRONOS_validation".to_owned()];

        let layers: Vec<LayerProperties> = instance::layers_list().context("failed to get Instance layers list")?.collect();

        // Debug information
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

        for validation_layer in &validation_layers {
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

        let create_info = InstanceCreateInfo {
            enabled_extensions: request_extensions,
            enabled_layers: validation_layers,
            // Enable enumerating devices that use non-conformant vulkan implementations. (ex. MoltenVK)
            enumerate_portability: true,
            ..InstanceCreateInfo::application_from_cargo_toml()
        };

        let instance = Instance::new(create_info)?;

        let debug = unsafe {
            DebugUtilsMessenger::new(
                Arc::clone(&instance),
                DebugUtilsMessengerCreateInfo::user_callback(Arc::new(|msg| {
                    error!("Vulkan debug: {:?}", msg.description);
                })),
            ).context("failed to create DebugUtilsMessenger")?
        };

        Ok((instance, debug))
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
        let (physical, queue_family) = PhysicalDevice::enumerate(instance)
            .filter(|&p| {
                p.supported_extensions().is_superset_of(&REQUIRED_EXTENSIONS)
                && p.supported_features().is_superset_of(&REQUIRED_FEATURES)
            })
            .filter_map(|p| {
                p.queue_families()
                    .find(|&q| {
                        q.supports_graphics() 
                        && match q.supports_surface(surface) {
                            Ok(val) => val,
                            Err(err) => panic!("PhysicalDevice enumerate error: {:?}", err),
                        }
                    })
                    .map(|q| (p, q))
            })
            .min_by_key(|(p, _)| {
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                }
        }).context("no supporting physical devices")?;

        Ok((physical, queue_family))
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
    ) -> Result<(Arc<Device>, impl ExactSizeIterator<Item = Arc<Queue>>), DeviceCreationError> 
    {
        let create_info = DeviceCreateInfo {
            enabled_extensions: REQUIRED_EXTENSIONS,
            enabled_features: REQUIRED_FEATURES,
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            ..DeviceCreateInfo::default()
        };

        Device::new(physical, create_info)
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
