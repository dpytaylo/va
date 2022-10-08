pub mod label_render_state;

use std::rc::Rc;
use std::sync::Arc;

use vulkano::device::Device;
use vulkano::image::view::ImageView;
use vulkano::render_pass::RenderPass;

use crate::graphics::font::Font;
use crate::graphics::mesh::Mesh;
use crate::graphics::render_data::RenderData;
use crate::graphics::render_state::RenderState;
use crate::layer::Layer;
use crate::manager::Manager;
use crate::object::Object;
use crate::global::Va;
use crate::utils::math::vector::vector2::Vec2;

use self::label_render_state::LabelRenderState;

pub struct Label {
    text: String,
    font: Rc<Font>,
    render_data: RenderData<Vec2<f32>, LabelRenderState>,
}

impl Label {
    pub fn new(manager: &Rc<Manager>, device: Arc<Device>, render_pass: Arc<RenderPass>, font: Rc<Font>, text: &str) 
        -> anyhow::Result<Self> 
    {
        let mut label = Self { 
            text: text.to_string(),
            font: Rc::clone(&font),
            render_data: RenderData::new(Default::default(), LabelRenderState::new(
                manager, 
                device, 
                render_pass, 
                ImageView::new_default(Arc::clone(&font.image))?,
            )?),
        };

        Ok(label)
    }
}

impl Object for Label {
    fn type_id(&self) -> std::any::TypeId {
        unimplemented!();
    }

    fn add_in_layer(&self, _: &Va, layer: &Rc<Layer>) -> anyhow::Result<()> {
        let cloned = self.render_data.clone();
        layer.add_render_data(cloned)?;
        Ok(())
    }

    fn remove_from_layer(&self, va: &Va, layer: &Rc<Layer>) -> anyhow::Result<()> {
        unimplemented!();
    }
}