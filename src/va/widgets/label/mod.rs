pub mod label_render_state;

use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use vulkano::device::Device;
use vulkano::image::view::ImageView;
use vulkano::render_pass::RenderPass;

use crate::graphics::font::Font;
use crate::graphics::mesh::Mesh;
use crate::layer::Layer;
use crate::manager::Manager;
use crate::object::Object;
use crate::global::Va;
use crate::utils::cast::Cast;
use crate::utils::math::geometry::rect::Rect;
use crate::utils::math::vector::vector2::Vec2;

use self::label_render_state::{LabelRenderState, LabelRenderStateVertex};

pub struct Label {
    text: String,
    font: Rc<Font>,

    mesh: RefCell<Option<Rc<Mesh<LabelRenderStateVertex>>>>,
    render_state: Rc<LabelRenderState>,
}

impl Label {
    pub fn new(manager: &Rc<Manager>, device: Arc<Device>, render_pass: Arc<RenderPass>, font: Rc<Font>, text: &str) 
        -> anyhow::Result<Self> 
    {
        let mut vertices = Vec::with_capacity(text.len() * 4);

        for char in text.chars() {
            let bbox: Rect<f32> = font.chars_info[&char].bounding_box.cast();

            vertices.push(LabelRenderStateVertex {
                position: [0.0, 0.0],
                tex_coords: bbox.p1.into(),
            });

            vertices.push(LabelRenderStateVertex {
                position: [0.0, bbox.p2.y - bbox.p1.y],
                tex_coords: [bbox.p1.x, bbox.p2.y],
            });

            vertices.push(LabelRenderStateVertex {
                position: (bbox.p2 - bbox.p1).into(),
                tex_coords: bbox.p2.into(),
            });

            vertices.push(LabelRenderStateVertex {
                position: [bbox.p2.x - bbox.p1.x, bbox.p1.y],
                tex_coords: [bbox.p2.x, bbox.p1.y],
            });
        }

        let label = Self { 
            text: text.to_string(),
            font: Rc::clone(&font),

            mesh: RefCell::new(Some(manager.create_mesh(vertices))),
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

    fn add_in_layer(&self, _: &Va, layer: &Layer) -> anyhow::Result<()> {
        layer.add_render_data(self.mesh.borrow_mut().take().unwrap(), Rc::clone(&self.render_state));
        Ok(())
    }

    fn remove_from_layer(&self, va: &Va, layer: &Layer) -> anyhow::Result<()> {
        unimplemented!();
    }
}