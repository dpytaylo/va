use std::ops::{Index, IndexMut};

pub struct IdVector<T> {
    vector: Vec<T>,
    empty_cells: Vec<usize>,
}

type IdVectorId = usize;

impl<T> IdVector<T> {
    pub fn new() -> Self {
        Self {
            vector: Vec::new(),
            empty_cells: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) -> IdVectorId {
        if let Some(pos) = self.empty_cells.pop() {
            self.vector[pos] = value;
            return pos;
        }

        self.vector.push(value);
        self.vector.len() - 1
    }

    pub fn erase(&mut self, id: usize) {
        self.empty_cells.push(id);
    }
}

impl<T> Index<usize> for IdVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vector[index]
    }
}

impl<T> IndexMut<usize> for IdVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vector[index]
    }
}
