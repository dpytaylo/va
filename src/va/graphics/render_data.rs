use std::rc::Rc;

use vulkano::buffer::BufferContents;

use super::mesh::Mesh;
use super::render_state::RenderState;

#[derive(Clone)]
pub struct RenderData<T, U> 
    where [T]: BufferContents,
          U: RenderState<T> + Clone,
{
    pub mesh: Rc<Mesh<T>>,
    pub render_state: Rc<U>,
}

impl<T, U> RenderData<T, U> 
    where [T]: BufferContents,
          U: RenderState<T> + Clone,
{
    pub fn new(mesh: Rc<Mesh<T>>, render_state: Rc<U>) -> Self {
        Self {
            mesh,
            render_state,
        }
    }
}