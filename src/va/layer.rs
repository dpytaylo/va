use std::any::TypeId;
use std::cell::{Ref, RefCell};
use std::mem;
use std::rc::Rc;
use std::sync::Arc;

use thiserror::Error;
use vulkano::buffer::BufferContents;
use vulkano::device::Device;
use vulkano::memory::DeviceMemoryAllocationError;

use crate::graphics::mesh::Mesh;
use crate::graphics::layer_render_data::{LayerRenderData, AbstractLayerRenderData};
use crate::graphics::layer_render_data_handle::LayerRenderDataHandle;
use crate::graphics::render_data::RenderData;
use crate::graphics::render_state::RenderState;
use crate::object::Object;

pub struct Layer {
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
        device: Arc<Device>,
        objects: Vec<Rc<dyn Object>>,
        mut render_data: Vec<Option<Box<dyn AbstractLayerRenderData>>>,
    ) -> Rc<Self> 
    {
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

        Rc::new(Self {
            device,
            objects: RefCell::new(objects),
            render_data: RefCell::new(render_data),
        })
    }

    pub fn add_render_data<T, U>(&self, mesh: Rc<Mesh<T>>, render_state: Rc<U>) -> Result<LayerRenderDataHandle<T, U>, DeviceMemoryAllocationError>
        where T: Clone + 'static,
              [T]: BufferContents,
              U: RenderState<T> + 'static,
    {
        let ref_render_data = self.render_data.borrow();
        for i in 0..ref_render_data.len() {
            if let Some(render_data) = &ref_render_data[i] {
                if render_data.type_id() == (TypeId::of::<T>(), TypeId::of::<U>()) {
                    drop(ref_render_data);

                    let layer_render_data = self.render_data.borrow_mut()[i].take().unwrap();
                    let layer_render_data = Self::transmute_layer_render_data::<T, U>(layer_render_data);
                    
                    return Ok(layer_render_data.add_mesh(mesh));
                }
            }
        }

        drop(ref_render_data);

        let layer_render_data = LayerRenderData::new(
            Arc::clone(&self.device),
            self.render_data.borrow().len(),
            render_state,
        );
        
        let handle = layer_render_data.add_mesh(mesh);
        self.render_data.borrow_mut().push(Some(Box::new(layer_render_data)));

        Ok(handle)
    }

    pub fn remove_render_data<T, U>(&self, handle: LayerRenderDataHandle<T, U>) -> Rc<Mesh<T>> 
        where T: Clone + 'static,
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
                || self.render_data.borrow().len() == handle.layer_render_data_index() + 1 {
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
        where T: Clone + 'static,
             [T]: BufferContents,
             U: RenderState<T> + 'static,
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

    // pub fn create_render_object_(
    //     layer: Rc<Self>,
    //     mesh: Rc<Mesh<f32>>,
    //     render_state: Rc<dyn RenderState<f32>>,
    // ) -> Result<RenderObject, DeviceMemoryAllocError> 
    // {
    //     let (data_render_state_pointer, _) = unsafe { 
    //         mem::transmute::<_, (*const usize, *const usize)>(Rc::clone(&render_state))
    //     };

    //     let render_data = {
    //         let ref_render_data = layer.render_data.borrow();
    //         let render_data = ref_render_data
    //             .iter()
    //             .find(|val| ptr::eq(Rc::as_ptr(val.render_state()) as *const usize, data_render_state_pointer));

    //         render_data.map(|val| Rc::clone(val))
    //     };

    //     let render_data = match render_data {
    //         Some(val) => val,
    //         None => {
    //             let render_data = LayerRenderData::new(
    //                 Rc::clone(&layer.graphics),
    //                 Rc::clone(&layer),
    //                 vec![],
    //                 render_state,
    //             )?;

    //             layer.render_data.borrow_mut().push(Rc::clone(&render_data));
    //             render_data
    //         }
    //     };

    //     render_data.add_mesh(Rc::clone(&mesh));
    //     render_data.update_vertex_buffer()?;
    //     let render_object = RenderObject::new(mesh,Rc::clone(&render_data));

    //     Ok(render_object)
    // }

    // pub unsafe fn remove_render_data(&self, render_data: &LayerRenderData) -> Result<(), ()> {
    //     let index = self
    //         .render_data
    //         .borrow()
    //         .iter()
    //         .position(|val| ptr::eq(Rc::as_ref(val), render_data));

    //     if index.is_none() {
    //         return Err(());
    //     }

    //     self.render_data.borrow_mut().swap_remove(index.unwrap());
    //     Ok(())
    // }

    pub fn objects(&self) -> Ref<Vec<Rc<dyn Object>>> {
        self.objects.borrow()
    }

    pub fn render_data(&self) -> Ref<Vec<Option<Box<dyn AbstractLayerRenderData>>>> {
        self.render_data.borrow()
    }
}