pub struct IteratorWithLen<T> 
    where T: Iterator,
{
    iter: T,
    len: usize,
}

impl<T> IteratorWithLen<T> 
    where T: Iterator,
{
    pub fn new(iter: T, len: usize) -> Self {
        Self {
            iter,
            len,
        }
    }
}

impl<T> Iterator for IteratorWithLen<T>
    where T: Iterator,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let value = match self.iter.next() {
            Some(val) => {
                val
            }
            None => return None,
        };
        
        if self.len != 0 {
            self.len -= 1;
        }
        
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> ExactSizeIterator for IteratorWithLen<T> where T: Iterator {}