use std::any::TypeId;
use std::cell::{Ref, RefCell};
use std::mem;
use std::rc::Rc;
use std::sync::Arc;

use anyhow::{Context, bail};
use thiserror::Error;
use vulkano::buffer::BufferContents;
use vulkano::device::Device;
use vulkano::memory::DeviceMemoryAllocationError;

use crate::global::Va;
use crate::graphics::mesh::Mesh;
use crate::graphics::layer_render_data::{LayerRenderData, AbstractLayerRenderData};
use crate::graphics::layer_render_data_handle::LayerRenderDataHandle;
use crate::graphics::render_state::RenderState;
use crate::object::Object;

pub struct Layer {
    va: Rc<Va>,
    device: Arc<Device>,
    objects: RefCell<Vec<Rc<dyn Object>>>,
    render_data: RefCell<Vec<Option<Box<dyn AbstractLayerRenderData>>>>,
}

#[derive(Error, Debug)]
pub enum LayerError {
    #[error("failed to remove render data")]
    FailedToRemoveRenderData,
}

impl Layer {
    pub fn new(
        va: Rc<Va>,
        objects: Vec<Rc<dyn Object>>,
        mut render_data: Vec<Option<Box<dyn AbstractLayerRenderData>>>,
    ) -> anyhow::Result<Rc<Self>> 
    {
        let device = va.graphics.device().context("no device")?;

        let mut indices = Vec::new();
        for i in 0..render_data.len() {
            match render_data[i].as_ref() {
                Some(val) => {
                    if !val.has_owners() {
                        indices.push(i);
                    }
                }
                None => indices.push(i),
            }
        }

        for i in indices.into_iter().rev() {
            render_data.swap_remove(i);
        }
        
        for i in 0..render_data.len() {
            render_data[i].as_ref().unwrap().update_layer_render_data_index(i);
        }

        let layer = Rc::new(Self {
            va: va,
            device,
            objects: RefCell::new(objects),
            render_data: RefCell::new(render_data),
        });

        for object in layer.objects.borrow().iter() {
            object.add_in_layer(&layer.va, &layer)?;
        }

        Ok(layer)
    }

    pub fn add_object<T>(&self, object: Rc<dyn Object>) -> anyhow::Result<()> {
        self.objects.borrow_mut().push(Rc::clone(&object));
        object.add_in_layer(&self.va, &self)?;
        Ok(())
    }

    pub fn objects(&self) -> Ref<Vec<Rc<dyn Object>>> {
        self.objects.borrow()
    }

    pub fn remove_all_objects(&self) -> anyhow::Result<Vec<Rc<dyn Object>>> {
        let objects = self.objects.borrow_mut();

        let mut errors = Vec::new();
        for object in objects.iter() {
            let _ = object.remove_from_layer(&self.va, &self).map_err(|err| errors.push(err));
        }

        if !errors.is_empty() {
            let mut buffer = String::new();
            
            for error in errors {
                buffer = format!("{buffer}; {}", error.to_string());
            }

            bail!("{buffer}");
        }

        Ok(mem::replace(&mut self.objects.borrow_mut(), Vec::new()))
    }

    pub fn add_render_data<T, U>(&self, mesh: Rc<Mesh<T>>, render_state: Rc<U>) -> LayerRenderDataHandle<T, U>
        where T: Clone,
              [T]: BufferContents,
              U: RenderState<T> + 'static,
    {
        let layer_render_data = LayerRenderData::new(
            Arc::clone(&self.device),
            self.render_data.borrow().len(),
            render_state,
        );
        
        let handle = layer_render_data.add_mesh(mesh);
        self.render_data.borrow_mut().push(Some(Box::new(layer_render_data)));

        handle
    }

    pub fn add_render_data_by_index<T, U>(
        &self, 
        index: usize,
        mesh: Rc<Mesh<T>>,
    ) -> LayerRenderDataHandle<T, U>
        where T: Clone,
              [T]: BufferContents,
              U: RenderState<T> + 'static,
    {
        let layer_render_data = self.render_data.borrow_mut()[index]
            .take()
            .expect("invalid layer render data index");

        if (*layer_render_data).type_id() != (TypeId::of::<T>(), TypeId::of::<U>()) {
            panic!("invalid handle type");
        }

        let layer_render_data = Self::transmute_layer_render_data::<T, U>(layer_render_data);
        let handle = layer_render_data.add_mesh(mesh);

        self.render_data.borrow_mut()[index] = Some(layer_render_data);
        handle
    }

    pub fn remove_render_data<T, U>(&self, handle: LayerRenderDataHandle<T, U>) -> Rc<Mesh<T>> 
        where T: Clone,
              [T]: BufferContents,
              U: RenderState<T> + 'static,
    {
        let layer_render_data = self.render_data.borrow_mut()[handle.layer_render_data_index()]
            .take()
            .expect("invalid layer render data index");

        if (*layer_render_data).type_id() != (TypeId::of::<T>(), TypeId::of::<U>()) {
            panic!("invalid handle type");
        }

        let layer_render_data = Self::transmute_layer_render_data::<T, U>(layer_render_data);

        let mesh = layer_render_data.remove_mesh(handle.layer_render_data_index()).expect("invalid index");
        
        if !layer_render_data.has_owners() {
            if self.render_data.borrow().len() < 2 
                || self.render_data.borrow().len() == handle.layer_render_data_index() + 1
            {
                self.render_data.borrow_mut().remove(handle.layer_render_data_index());
                return mesh;
            }
            
            self.render_data.borrow_mut().swap_remove(handle.layer_render_data_index());
            self.render_data.borrow_mut().remove(self.render_data.borrow().len() - 1);

            self.render_data.borrow()[handle.layer_render_data_index()].as_ref().unwrap()
                .update_layer_render_data_index(handle.layer_render_data_index());
        }

        mesh
    }

    fn transmute_layer_render_data<T, U>(layer_render_data: Box<dyn AbstractLayerRenderData>) -> Box<LayerRenderData<T, U>> 
        where T: Clone,
              [T]: BufferContents,
              U: RenderState<T>,
    {
        unsafe {
            let (val, _) = mem::transmute::<_, (*mut LayerRenderData<T, U>, usize)>(Box::into_raw(layer_render_data));
            Box::from_raw(val)
        }
    }

    pub fn update_layer_render_data(&self) -> Result<(), DeviceMemoryAllocationError> {
        for layer_render_data in self.render_data.borrow().iter() {
            if let Some(val) = layer_render_data {
                val.update_vertex_buffer()?;
            }
        }

        Ok(())
    }

    pub fn render_data(&self) -> Ref<Vec<Option<Box<dyn AbstractLayerRenderData>>>> {
        self.render_data.borrow()
    }
}

impl Drop for Layer {
    fn drop(&mut self) {
        for object in self.objects().iter() {
            object.remove_from_layer(&self.va, &self).unwrap(); // TODO
        }
    }
}