// abcdefghijklmnopqrstuvwxyz
use std::any::TypeId;
use std::cell::{RefCell, Cell};
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
use super::render_state::RenderState;
use super::Graphics;

pub trait AbstractLayerRenderData {
    fn type_id(&self) -> (TypeId, TypeId);
    fn has_owners(&self) -> bool;
    fn update_layer_render_data_index(&self, index: usize);
    fn update_vertex_buffer(&self) -> Result<(), DeviceMemoryAllocationError>;
    fn command_buffer(
        &self,
        graphics: &Rc<Graphics>,
        framebuffer: Arc<Framebuffer>,
        viewport: Viewport,
    ) -> anyhow::Result<Option<PrimaryAutoCommandBuffer>>;
}

pub struct LayerRenderData<T, U> 
    where T: Clone + 'static,
          [T]: BufferContents,
          U: RenderState<T>,
{
    device: Arc<Device>,

    layer_render_data_index: Cell<usize>,

    meshes: RefCell<Vec<Rc<Mesh<T>>>>,
    handles: RefCell<Vec<Rc<RawLayerRenderDataHandle>>>,

    was_edited: Cell<bool>,

    vertex_buffer: RefCell<Option<Arc<CpuAccessibleBuffer<[T]>>>>,
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
          U: RenderState<T>,
{
    pub fn new(
        device: Arc<Device>,
        layer_render_data_index: usize,
        render_state: Rc<U>,
    ) -> Self
    {
        // let vertex_buffer = CpuAccessibleBuffer::from_iter(
        //     Arc::clone(&device),
        //     BufferUsage::all(),
        //     false,
        //     mesh.vertices().iter().map(|val| val.clone()),
        // )?;

        // let raw_handle = RawLayerRenderDataHandle::new(layer_render_data_index, 0);

        Self {
            device,

            layer_render_data_index: Cell::new(layer_render_data_index),

            meshes: Default::default(),
            handles: Default::default(),

            was_edited: Default::default(),

            vertex_buffer: Default::default(),
            render_state,
        }
    }

    pub fn add_mesh(&self, mesh: Rc<Mesh<T>>) -> LayerRenderDataHandle<T, U> {
        self.meshes.borrow_mut().push(mesh);

        let raw_handle = RawLayerRenderDataHandle::new(
            self.layer_render_data_index.get(), 
            self.meshes.borrow().len(),
        );

        self.handles.borrow_mut().push(Rc::clone(&raw_handle));

        self.was_edited.set(true);
        LayerRenderDataHandle::new(raw_handle)
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

        self.was_edited.set(true);
        Ok(value)
    }
}

impl<T, U> AbstractLayerRenderData for LayerRenderData<T, U> 
    where T: Clone,
          [T]: BufferContents,
          U: RenderState<T> + 'static,
{
    fn type_id(&self) -> (TypeId, TypeId) {
        (TypeId::of::<T>(), TypeId::of::<U>())
    }

    fn has_owners(&self) -> bool {
        self.handles.borrow().len() > 0
    }

    fn update_layer_render_data_index(&self, index: usize) {
        self.layer_render_data_index.set(index);

        for handle in self.handles.borrow().iter() {
            handle.set_layer_render_data_index(index);
        }
    }

    fn update_vertex_buffer(&self) -> Result<(), DeviceMemoryAllocationError> {
        if !self.was_edited.get() {
            return Ok(());
        }
        self.was_edited.set(false);

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

        *self.vertex_buffer.borrow_mut() = Some(CpuAccessibleBuffer::from_iter(
            Arc::clone(&self.device),
            BufferUsage::all(),
            false,
            IteratorWithLen::new(iter, counter),
        )?);

        Ok(())
    }

    fn command_buffer(
        &self,
        graphics: &Rc<Graphics>,
        framebuffer: Arc<Framebuffer>,
        viewport: Viewport,
    ) -> anyhow::Result<Option<PrimaryAutoCommandBuffer>>
    {
        if self.vertex_buffer.borrow().is_none() {
            return Ok(None);
        }

        Ok(Some(self.render_state.command_buffer(
            graphics,
            self.vertex_buffer.borrow().as_ref().unwrap(),
            framebuffer,
            viewport,
        )?))
    }
}