use std::rc::Rc;
use std::cell::Ref;
use std::sync::Arc;

use anyhow::Context;
use bytemuck::{Zeroable, Pod};
use vulkano::buffer::CpuAccessibleBuffer; 
use vulkano::command_buffer::{PrimaryAutoCommandBuffer, RenderPassBeginInfo, AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::device::Device;
use vulkano::format::ClearValue;
use vulkano::image::ImmutableImage;
use vulkano::image::view::ImageView;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::{GraphicsPipeline, PipelineBindPoint};
use vulkano::pipeline::graphics::color_blend::ColorBlendState;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::viewport::{ViewportState, Viewport};
use vulkano::render_pass::{Framebuffer, Subpass, RenderPass};
use vulkano::sampler::{SamplerCreateInfo, Sampler};

use crate::graphics::Graphics;
use crate::graphics::render_state::RenderState;
use crate::utils::math::vector::vector2::Vec2;
use crate::manager::Manager;

struct LabelRenderState {
    graphics_pipeline: Arc<GraphicsPipeline>,
    descriptor_set: Arc<PersistentDescriptorSet>,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Zeroable, Pod)]
pub struct LabelRenderStateVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
vulkano::impl_vertex!(LabelRenderStateVertex, position, tex_coords);

impl LabelRenderState {
    fn new(manager: &Rc<Manager>, device: Arc<Device>, render_pass: Arc<RenderPass>, image_view: Arc<ImageView<ImmutableImage>>) 
        -> anyhow::Result<Rc<Self>>
    {
        let graphics_pipeline = manager.load_graphics_pipeline::<LabelRenderState, _>( 
            |device, manager| 
        {
            let vs = manager.load_shader("shaders/spv/texture2d/vert.spv")?;
            let fs = manager.load_shader("shaders/spv/texture2d/frag.spv")?;
            
            let subpass = Subpass::from(render_pass, 0).unwrap();
            let graphics_pipeline = GraphicsPipeline::start()
                .vertex_input_state(
                    BuffersDefinition::new()
                        .vertex::<LabelRenderStateVertex>(),
                )
                .vertex_shader(
                    vs
                        .entry_point("main")
                        .expect("no shader entry point"),
                    (),
                )
                .input_assembly_state(InputAssemblyState::new())
                .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
                .fragment_shader(
                    fs
                        .entry_point("main")
                        .expect("no shader entry point"),
                    (),
                )
                .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend_alpha())
                .render_pass(subpass)
                .build(device)?; // TODO: build_with_cache ?

            Ok(graphics_pipeline)
        })?;
            
        // TODO
        let layout = Arc::clone(
            graphics_pipeline
                .layout()
                .set_layouts()
                .get(0)
                .context("invalid descriptor set layout")?,
        );

        let sampler = Sampler::new(
            device,
            SamplerCreateInfo::simple_repeat_linear(),
        )?;

        let descriptor_set = PersistentDescriptorSet::new(
            layout,
            vec![WriteDescriptorSet::image_view_sampler(0, image_view, sampler)].into_iter()
        )?;

        Ok(Rc::new(Self {
            graphics_pipeline,
            descriptor_set,
        }))
    }
}

impl RenderState<Vec2<f32>> for LabelRenderState {
    fn command_buffer(
        &self,
        graphics: &Rc<Graphics>,
        buffer: Ref<Arc<CpuAccessibleBuffer<[Vec2<f32>]>>>,
        framebuffer: Arc<Framebuffer>,
        viewport: Viewport,
    ) -> anyhow::Result<PrimaryAutoCommandBuffer> 
    {
        let device = graphics.device().expect("no available device");
        let queue = graphics.queue().expect("no available queue");

        let render_pass_begin_info = RenderPassBeginInfo {
            clear_values: vec![Some(ClearValue::Float([0.0, 0.0, 0.0, 1.0]))],
            ..RenderPassBeginInfo::framebuffer(framebuffer)
        };

        let mut builder = AutoCommandBufferBuilder::primary(
            device,
            queue.family(),
            CommandBufferUsage::SimultaneousUse,
        )?;
        builder
            .begin_render_pass(render_pass_begin_info, SubpassContents::Inline)?
            .set_viewport(0, [viewport])
            .bind_pipeline_graphics(Arc::clone(&self.graphics_pipeline))
            //.push_constants(layout, 0, constants)
            //.push_descriptor_set(PipelineBindPoint::Graphics, layout, 0, descriptor_set)
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                Arc::clone(self.graphics_pipeline.layout()),
                0,
                Arc::clone(&self.descriptor_set),
            )
            .bind_vertex_buffers(0, Arc::clone(&buffer))
            .draw(buffer.len() as u32, 1, 0, 0)?
            .end_render_pass()?;

        Ok(builder.build()?)
    }
}