use std::rc::Rc;

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
