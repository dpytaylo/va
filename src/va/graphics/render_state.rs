// abcdefghijklmnopqrstuvwxyz
use std::rc::Rc;
use std::sync::Arc;

use vulkano::buffer::{CpuAccessibleBuffer, BufferContents};
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::render_pass::Framebuffer;

use super::Graphics;

pub trait RenderState<T> 
    where [T]: BufferContents,
{
    fn command_buffer(
        &self,
        graphics: &Rc<Graphics>,
        buffer: &Arc<CpuAccessibleBuffer<[T]>>,
        framebuffer: Arc<Framebuffer>,
        viewport: Viewport,
    ) -> anyhow::Result<PrimaryAutoCommandBuffer>;
}

// pub struct StandartRenderStateBuilder {
//     vertex_shader: Arc<ShaderModule>,
//     fragment_shader: Arc<ShaderModule>,
//     render_pass: Arc<RenderPass>,
//     descriptor_sets: Vec<Arc<PersistentDescriptorSet>>,
// }

// impl StandartRenderStateBuilder {
//     pub fn start(vertex_shader: Arc<ShaderModule>, fragment_shader: Arc<ShaderModule>, render_pass: Arc<RenderPass>) -> Self {
//         Self {
//             vertex_shader,
//             fragment_shader,
//             render_pass,
//             ..Default::default()
//         }
//     }

//     pub fn with_constants(self, index: usize) -> Result<Self, DescriptorSetError> {

//     }

//     pub fn with_descriptor_set(self, index: usize) -> Self {
//         let layout = Arc::clone(
//             graphics_pipeline.layout().descriptor_set_layouts().get(0).context("invalid descriptor set layout")?
//         );

//         let mut builder = PersistentDescriptorSet::start(layout);
//         builder.add_sampled_image(Arc::clone(&self.image_view), Arc::clone(&sampler))?;
//         let descriptor_set = builder.build()?;

//     }

//     pub fn build(self) -> StandartRenderState {

//     }
// }

// pub struct StandartRenderState {
//     vertex_shader: Arc<ShaderModule>,
//     fragment_shader: Arc<ShaderModule>,
//     graphics_pipeline: Arc<GraphicsPipeline>,
//     render_pass: Arc<RenderPass>,
//     image_view: Arc<dyn ImageViewAbstract>,
//     sampler: Arc<Sampler>,
//     descriptor_set: Arc<PersistentDescriptorSet>,
// }

// impl StandartRenderState {
//     /// # Panics
//     ///
//     /// Function will panic if not setup device
//     pub fn new(
//         graphics: &Rc<Graphics>,
//         vertex_shader: Arc<ShaderModule>,
//         fragment_shader: Arc<ShaderModule>,
//         render_pass: Arc<RenderPass>,
//         image_view: Arc<dyn ImageViewAbstract>,
//         sampler: Arc<Sampler>
//     ) -> anyhow::Result<Arc<Self>>
//     {
//         let device = graphics.device().expect("no available device");

//         let subpass = Subpass::from(Arc::clone(&render_pass), 0).unwrap();
//         let graphics_pipeline = GraphicsPipeline::start()
//             .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
//             .vertex_shader(vertex_shader.entry_point("main").expect("no shader entry point"), ())
//             .input_assembly_state(InputAssemblyState::new())
//             .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
//             .fragment_shader(fragment_shader.entry_point("main").expect("no shader entry point"), ())
//             .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend_alpha())
//             .render_pass(subpass)
//             .build(device)?; // TODO: build_with_cache ?

//         let layout = Arc::clone(
//             graphics_pipeline.layout().descriptor_set_layouts().get(0).context("invalid descriptor set layout")?
//         );

//         let mut builder = PersistentDescriptorSet::start(layout);
//         builder.add_sampled_image(Arc::clone(&image_view), Arc::clone(&sampler))?;
//         let descriptor_set = builder.build()?;

//         Ok(Arc::new(Self {
//             vertex_shader,
//             fragment_shader,
//             graphics_pipeline,
//             render_pass,
//             image_view,
//             sampler,
//             descriptor_set,
//         }))
//     }
// }

// impl RenderState for StandartRenderState {
//     fn create_command_buffer(
//         &self,
//         va: &Arc<Va>,
//         framebuffer: Arc<Framebuffer>,
//         viewport: Viewport
//     ) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandPoolBuilder>
//     {
//         //.push_constants(layout, 0, constants)
//         //.push_descriptor_set(PipelineBindPoint::Graphics, layout, 0, descriptor_set)
//         //.bind_descriptor_sets(PipelineBindPoint::Graphics, layout, 0, descriptor_set)
//         //.bind_vertex_buffers(0, Arc::clone(&mesh.vertex_buffer()))
//         //.draw(6, 1, 0, 0)?
//     }
// }
