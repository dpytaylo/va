use std::rc::Rc;

use crate::graphics::mesh::Mesh;
use crate::graphics::render_data::RenderData;
use crate::graphics::render_state::RenderState;
use crate::layer::Layer;
use crate::object::Object;
use crate::global::Va;
use crate::utils::math::vector::vector2::Vec2;

pub mod label_render_state;

#[derive(Default)]
pub struct Label {
    text: String,
    render_data: RenderData<Vec2<f32>>,
}

impl Label {
    pub fn new(text: &str) -> Self {
        let mut label = Self { 
            text: text.to_string(),
        };

        

        label
    }
}

impl Object for Label {
    fn add_in_layer(&self, _: &Va, layer: &Rc<Layer>) -> anyhow::Result<()> {
        layer.add_render_data(RenderData::new(mesh, render_state))?;
        Ok(())
    }

    fn remove_from_layer(&self, va: &Va, layer: &Rc<Layer>) -> anyhow::Result<()> {
        
    }
}