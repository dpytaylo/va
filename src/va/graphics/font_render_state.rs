use std::{rc::Rc, cell::Ref, sync::Arc};

use bytemuck::{Zeroable, Pod};
use vulkano::{buffer::CpuAccessibleBuffer, render_pass::{Framebuffer, Subpass}, pipeline::{graphics::{viewport::{Viewport, ViewportState}, color_blend::ColorBlendState, input_assembly::InputAssemblyState, vertex_input::BuffersDefinition}, GraphicsPipeline}, command_buffer::PrimaryAutoCommandBuffer, descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet}, image::ImmutableImage};

use crate::utils::math::vector::vector2::Vec2;

use super::{render_state::RenderState, font::Font, Graphics};

struct FontRenderState {
    image: Arc<ImmutableImage>,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Zeroable, Pod)]
pub struct FontRenderStateVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
vulkano::impl_vertex!(FontRenderStateVertex, position, tex_coords);

impl FontRenderState {
    fn new(graphics_pipeline: Arc<GraphicsPipeline>, image: Arc<ImmutableImage>) -> Self {
        let layout = Arc::clone(
            graphics_pipeline
                .layout()
                .set_layouts()
                .get(0)
                .context("invalid descriptor set layout")?,
        );

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

impl RenderState<Vec2<f32>> for FontRenderState {
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