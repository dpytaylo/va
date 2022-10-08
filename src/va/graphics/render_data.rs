use std::rc::Rc;

use vulkano::buffer::BufferContents;

use super::mesh::Mesh;
use super::render_state::RenderState;

pub struct RenderData<T, U>
    where [T]: BufferContents,
          U: RenderState<T>,
{
    pub mesh: Rc<Mesh<T>>,
    pub render_state: Rc<U>,
}

impl<T, U> RenderData<T, U>
    where [T]: BufferContents,
          U: RenderState<T>,
{
    pub fn new(mesh: Rc<Mesh<T>>, render_state: Rc<U>) -> Self {
        Self {
            mesh,
            render_state,
        }
    }
}

impl<T, U> Clone for RenderData<T, U>
    where [T]: BufferContents,
          U: RenderState<T>,
{
    fn clone(&self) -> Self {
        Self {
            mesh: Rc::clone(&self.mesh),
            render_state: Rc::clone(&self.render_state),
        }
    }
}