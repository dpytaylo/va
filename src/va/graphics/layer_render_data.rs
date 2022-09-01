// abcdefghijklmnopqrstuvwxyz
use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use thiserror::Error;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::memory::DeviceMemoryAllocError;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::render_pass::Framebuffer;

use super::mesh::Mesh;
use super::layer_render_data_handle::{LayerRenderDataHandle, RawLayerRenderDataHandle};
use super::render_data::RenderData;
use super::render_state::RenderState;
use super::Graphics;

pub trait AbstractLayerRenderData {
    fn type_id(&self) -> TypeId;
    fn has_owners(&self) -> bool;
    fn update_layer_render_data_index(&self, index: usize);
    fn command_buffer(
        &self,
        framebuffer: Arc<Framebuffer>,
        viewport: Viewport,
    ) -> anyhow::Result<PrimaryAutoCommandBuffer>;
}

pub struct LayerRenderData<T> 
    where T: Clone + 'static,
{
    graphics: Rc<Graphics>,

    handles: RefCell<Vec<Rc<RawLayerRenderDataHandle>>>,
    meshes: RefCell<Vec<Rc<Mesh<T>>>>,

    vertex_buffer: RefCell<Arc<CpuAccessibleBuffer<[T]>>>,
    render_state: Rc<dyn RenderState<T>>,
}

#[derive(Error, Debug)]
pub enum LayerRenderDataError {
    #[error("failed to find the mesh for remove")]
    FailedToRemoveMesh,
}

impl<T> LayerRenderData<T> 
    where T: Clone + 'static,
{
    pub fn new(
        graphics: Rc<Graphics>,
        layer_render_data_index: usize,
        render_data: RenderData<T>,
    ) -> Result<(Self, LayerRenderDataHandle<T>), DeviceMemoryAllocError> 
    {
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            graphics.device().expect("no available device"),
            BufferUsage::all(),
            false,
            render_data.mesh.vertices().iter().map(|val| val.clone()),
        )?;

        let raw_handle = RawLayerRenderDataHandle::new(layer_render_data_index, 0);

        Ok((
            Self {
                graphics,

                handles: RefCell::new(vec![Rc::clone(&raw_handle)]),
                meshes: RefCell::new(vec![render_data.mesh]),

                vertex_buffer: RefCell::new(vertex_buffer),
                render_state: render_data.render_state,
            }, 
            LayerRenderDataHandle::new(raw_handle)
        ))
    }

    pub fn update_vertex_buffer(&self) -> Result<(), DeviceMemoryAllocError> {
        let data: Vec<_> = self
            .meshes
            .borrow()
            .iter()
            .flat_map(|val| val.vertices().iter().cloned())
            .collect();

        *self.vertex_buffer.borrow_mut() = CpuAccessibleBuffer::from_iter(
            self.graphics.device().expect("no available device"),
            BufferUsage::all(),
            false,
            data,
        )?;

        Ok(())
    }

    pub fn remove_mesh(&self, index: usize) -> Result<Rc<Mesh<T>>, LayerRenderDataError> {
        if self.meshes.borrow().len() < 2 || self.meshes.borrow().len() == index + 1 {
            return Ok(self.meshes.borrow_mut().remove(index));
        }

        let value = self.meshes.borrow_mut().swap_remove(index);
        self.handles.borrow()[index].set_mesh_index(index);

        Ok(value)
    }
}

impl<T> AbstractLayerRenderData for LayerRenderData<T> 
    where T: Clone,
{
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn has_owners(&self) -> bool {
        if self.handles.borrow().len() > 0 {
            return true;
        }

        false
    }

    fn update_layer_render_data_index(&self, index: usize) {
        for handle in self.handles.borrow().iter() {
            handle.set_layer_render_data_index(index);
        }
    }

    fn command_buffer(
        &self,
        framebuffer: Arc<Framebuffer>,
        viewport: Viewport,
    ) -> anyhow::Result<PrimaryAutoCommandBuffer> 
    {
        self.render_state.command_buffer(
            &self.graphics,
            self.vertex_buffer.borrow(),
            framebuffer,
            viewport,
        )
    }
}