use std::rc::Rc;

use crate::{graphics::render_data::RenderData, layer::Layer};

pub trait Object {
    // fn add_child<T>(&self, object: T)
    //     where T: Object
    // {
    //     unimplemented!();
    // }

    // fn children(&self) -> Vec<Rc<dyn Object>> {
    //     unimplemented!();
    // }

    fn update(&self) {

    }
}