use std::rc::Rc;

use super::mesh::Mesh;
use super::render_state::RenderState;

pub struct RenderData<T> {
    pub mesh: Rc<Mesh<T>>,
    pub render_state: Rc<dyn RenderState<T>>,
}

impl<T> RenderData<T> {
    pub fn new(mesh: Rc<Mesh<T>>, render_state: Rc<dyn RenderState<T>>) -> Self {
        Self {
            mesh,
            render_state,
        }
    }
}