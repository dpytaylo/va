use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct RawLayerRenderDataHandle {
    layer_render_data_index: Cell<usize>,
    mesh_index: Cell<usize>,
}

impl RawLayerRenderDataHandle {
    pub fn new(layer_render_data_index: usize, mesh_index: usize) 
        -> Rc<Self> 
    {
        Rc::new(Self {
            layer_render_data_index: Cell::new(layer_render_data_index),
            mesh_index: Cell::new(mesh_index),
        })
    }

    pub fn set_layer_render_data_index(&self, index: usize) {
        self.layer_render_data_index.set(index);
    }

    pub fn set_mesh_index(&self, index: usize) {
        self.mesh_index.set(index);
    }
}

pub struct LayerRenderDataHandle<T> {
    phantom: PhantomData<T>,
    raw: Rc<RawLayerRenderDataHandle>,
}

impl<T> LayerRenderDataHandle<T> {
    pub fn new(raw_layer_render_data_handle: Rc<RawLayerRenderDataHandle>) -> Self {
        Self {
            phantom: PhantomData,
            raw: raw_layer_render_data_handle,
        }
    }

    pub fn layer_render_data_index(&self) -> usize {
        self.raw.layer_render_data_index.get()
    }

    pub fn mesh_index(&self) -> usize {
        self.raw.mesh_index.get()
    }
}