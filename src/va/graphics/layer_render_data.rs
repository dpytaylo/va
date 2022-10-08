// abcdefghijklmnopqrstuvwxyz
use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use thiserror::Error;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, BufferContents};
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::device::Device;
use vulkano::memory::DeviceMemoryAllocationError;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::render_pass::Framebuffer;

use crate::utils::iter::IteratorWithLen;

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
        graphics: &Rc<Graphics>,
        framebuffer: Arc<Framebuffer>,
        viewport: Viewport,
    ) -> anyhow::Result<PrimaryAutoCommandBuffer>;
}

pub struct LayerRenderData<T, U> 
    where T: Clone + 'static,
          [T]: BufferContents,
          U: RenderState<T>,
{
    device: Arc<Device>,

    handles: RefCell<Vec<Rc<RawLayerRenderDataHandle>>>,
    meshes: RefCell<Vec<Rc<Mesh<T>>>>,

    vertex_buffer: RefCell<Arc<CpuAccessibleBuffer<[T]>>>,
    render_state: Rc<U>,
}

#[derive(Error, Debug)]
pub enum LayerRenderDataError {
    #[error("failed to find the mesh for remove")]
    FailedToRemoveMesh,
}

impl<T, U> LayerRenderData<T, U> 
    where T: Clone + 'static,
          [T]: BufferContents,
          U: RenderState<T> + Clone,
{
    pub fn new(
        device: Arc<Device>,
        layer_render_data_index: usize,
        render_data: RenderData<T, U>,
    ) -> Result<(Self, LayerRenderDataHandle<T, U>), DeviceMemoryAllocationError> 
    {
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            Arc::clone(&device),
            BufferUsage::all(),
            false,
            render_data.mesh.vertices().iter().map(|val| val.clone()),
        )?;

        let raw_handle = RawLayerRenderDataHandle::new(layer_render_data_index, 0);

        Ok((
            Self {
                device,

                handles: RefCell::new(vec![Rc::clone(&raw_handle)]),
                meshes: RefCell::new(vec![render_data.mesh]),

                vertex_buffer: RefCell::new(vertex_buffer),
                render_state: render_data.render_state,
            }, 
            LayerRenderDataHandle::new(raw_handle)
        ))
    }

    pub fn remove_mesh(&self, index: usize) -> Result<Rc<Mesh<T>>, LayerRenderDataError> {
        if self.meshes.borrow().is_empty() {
            return Err(LayerRenderDataError::FailedToRemoveMesh);
        }

        if self.meshes.borrow().len() - 1 == index {
            return Ok(self.meshes.borrow_mut().remove(index));
        }

        let value = self.meshes.borrow_mut().swap_remove(index);
        self.handles.borrow()[index].set_mesh_index(index);

        Ok(value)
    }

    pub fn update_vertex_buffer(&self) -> Result<(), DeviceMemoryAllocationError> {
        let mut counter = 0;

        let meshes = self.meshes.borrow();
        for mesh in meshes.iter() {
            counter += mesh.vertices().len();
        }

        let iter = meshes
            .iter()
            .flat_map(|val| {
                val.vertices().iter().cloned()
            });

        // let iter = self
        //     .meshes
        //     .borrow()
        //     .iter()
        //     .flat_map(|val| val.vertices().iter().cloned());

        // *self.vertex_buffer.borrow_mut() = CpuAccessibleBuffer::from_iter(
        //     Arc::clone(&self.device),
        //     BufferUsage::all(),
        //     false,
        //     data,
        // )?;

        *self.vertex_buffer.borrow_mut() = CpuAccessibleBuffer::from_iter(
            Arc::clone(&self.device),
            BufferUsage::all(),
            false,
            IteratorWithLen::new(iter, counter),
        )?;

        Ok(())
    }
}

impl<T, U> AbstractLayerRenderData for LayerRenderData<T, U> 
    where T: Clone,
          [T]: BufferContents,
          U: RenderState<T>,
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
        graphics: &Rc<Graphics>,
        framebuffer: Arc<Framebuffer>,
        viewport: Viewport,
    ) -> anyhow::Result<PrimaryAutoCommandBuffer> 
    {
        self.render_state.command_buffer(
            graphics,
            self.vertex_buffer.borrow(),
            framebuffer,
            viewport,
        )
    }
}