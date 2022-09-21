use std::{rc::Rc, cell::Ref, sync::Arc};

use vulkano::{buffer::CpuAccessibleBuffer, render_pass::Framebuffer, pipeline::graphics::viewport::Viewport, command_buffer::PrimaryAutoCommandBuffer, descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet}};

use crate::utils::math::vector::vector2::Vec2;

use super::{render_state::RenderState, font::Font, Graphics};

struct FontRenderState {
    font: Rc<Font>,
    text: String,
}

// impl FontRenderState {
//     fn new(graphics: &Rc<Graphics>, font: Rc<Font>, text: String) -> Self {
//         let device = graphics.device().expect("no available device");

//         let subpass = Subpass::from(render_pass, 0).unwrap();
//         let graphics_pipeline = GraphicsPipeline::start()
//             .vertex_input_state(
//                 BuffersDefinition::new()
//                     //.vertex::<Position>()
//                     //.vertex::<TextureCoordinates>()
//                     .vertex::<Vertex>(),
//             )
//             .vertex_shader(
//                 vertex_shader
//                     .entry_point("main")
//                     .expect("no shader entry point"),
//                 (),
//             )
//             .input_assembly_state(InputAssemblyState::new())
//             .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
//             .fragment_shader(
//                 fragment_shader
//                     .entry_point("main")
//                     .expect("no shader entry point"),
//                 (),
//             )
//             .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend_alpha())
//             .render_pass(subpass)
//             .build(device)?; // TODO: build_with_cache ?

//         let layout = Arc::clone(
//             graphics_pipeline
//                 .layout()
//                 .set_layouts()
//                 .get(0)
//                 .context("invalid descriptor set layout")?,
//         );

//         let descriptor_set = PersistentDescriptorSet::new(
//             layout,
//             vec![WriteDescriptorSet::image_view_sampler(0, image_view, sampler)].into_iter()
//         )?;

//         Ok(Rc::new(Self {
//             graphics_pipeline,
//             descriptor_set,
//         }))
//     }
// }

// impl RenderState<Vec2<f32>> for FontRenderState {
//     fn command_buffer(
//         &self,
//         graphics: &Rc<Graphics>,
//         buffer: Ref<Arc<CpuAccessibleBuffer<[Vec2<f32>]>>>,
//         framebuffer: Arc<Framebuffer>,
//         viewport: Viewport,
//     ) -> anyhow::Result<PrimaryAutoCommandBuffer> 
//     {
//         let device = graphics.device().expect("no available device");

//         let subpass = Subpass::from(render_pass, 0).unwrap();
//         let graphics_pipeline = GraphicsPipeline::start()
//             .vertex_input_state(
//                 BuffersDefinition::new()
//                     //.vertex::<Position>()
//                     //.vertex::<TextureCoordinates>()
//                     .vertex::<Vertex>(),
//             )
//             .vertex_shader(
//                 vertex_shader
//                     .entry_point("main")
//                     .expect("no shader entry point"),
//                 (),
//             )
//             .input_assembly_state(InputAssemblyState::new())
//             .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
//             .fragment_shader(
//                 fragment_shader
//                     .entry_point("main")
//                     .expect("no shader entry point"),
//                 (),
//             )
//             .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend_alpha())
//             .render_pass(subpass)
//             .build(device)?; // TODO: build_with_cache ?

//         let layout = Arc::clone(
//             graphics_pipeline
//                 .layout()
//                 .set_layouts()
//                 .get(0)
//                 .context("invalid descriptor set layout")?,
//         );

//         let descriptor_set = PersistentDescriptorSet::new(
//             layout,
//             vec![WriteDescriptorSet::image_view_sampler(0, image_view, sampler)].into_iter()
//         )?;

//         unimplemented!();
//     }
// }