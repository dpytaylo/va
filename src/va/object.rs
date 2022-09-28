use crate::{graphics::render_data::RenderData, layer::Layer};

pub trait Object {
    fn add_child<T>(object: T)
        where T: Object
    {
        unimplemented!();
    }
}