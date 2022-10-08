use std::rc::Rc;

#[derive(Default)]
pub struct Mesh<T> {
    vertices: Vec<T>,
}

impl<T> Mesh<T> {
    pub fn new(vertices: Vec<T>) -> Rc<Self> {
        Rc::new(Self { vertices })
    }

    pub fn vertices(&self) -> &Vec<T> {
        &self.vertices
    }
}
