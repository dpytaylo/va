use crate::{graphics::render_data::RenderData, layer::Layer};

pub trait Object {
    fn process_layer(layer: &Layer);
}