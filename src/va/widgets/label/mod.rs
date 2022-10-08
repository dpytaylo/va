pub mod label_render_state;

use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use vulkano::device::Device;
use vulkano::image::view::ImageView;
use vulkano::render_pass::RenderPass;

use crate::graphics::font::Font;
use crate::graphics::mesh::Mesh;
use crate::layer::Layer;
use crate::manager::Manager;
use crate::object::Object;
use crate::global::Va;
use crate::utils::math::vector::vector2::Vec2;

use self::label_render_state::LabelRenderState;

pub struct Label {
    text: String,
    font: Rc<Font>,

    mesh: RefCell<Option<Rc<Mesh<Vec2<f32>>>>>,
    render_state: Rc<LabelRenderState>,
}

impl Label {
    pub fn new(manager: &Rc<Manager>, device: Arc<Device>, render_pass: Arc<RenderPass>, font: Rc<Font>, text: &str) 
        -> anyhow::Result<Self> 
    {
        let label = Self { 
            text: text.to_string(),
            font: Rc::clone(&font),

            mesh: Default::default(),
            render_state: LabelRenderState::new(
                manager, 
                device, 
                render_pass, 
                ImageView::new_default(Arc::clone(&font.image))?,
            )?,
        };

        Ok(label)
    }
}

impl Object for Label {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Label>()
    }

    fn add_in_layer(&self, _: &Va, layer: &Rc<Layer>) -> anyhow::Result<()> {
        layer.add_render_data(self.mesh.borrow_mut().take().unwrap(), Rc::clone(&self.render_state));
        Ok(())
    }

    fn remove_from_layer(&self, va: &Va, layer: &Rc<Layer>) -> anyhow::Result<()> {
        unimplemented!();
    }
}