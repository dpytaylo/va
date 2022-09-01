// use std::cell::Cell;
// use std::ops::{Index, IndexMut};
// use std::rc::Rc;
// use std::slice::Iter;

// struct RawHandle<T> {
//     value: Rc<T>,
//     id: Cell<usize>,
// }

// struct Handle<T> {
//     raw: Rc<RawHandle<T>>,
// }

// impl<T> Handle<T> {
//     fn value(&self) -> &T {
//         &*self.raw.value
//     }
    
//     fn id(&self) -> usize {
//         self.raw.id.get()
//     }
// }

// struct HandlerContainer<T: std::fmt::Debug> {
//     values: Vec<(Rc<T>, Vec<Rc<RawHandle<T>>>)>,
// }

// struct HandlerContainerIter<'a, T> {
//     iter: Iter<'a, (Rc<T>, Vec<Rc<RawHandle<T>>>)>,
// }

// impl<'a, T> Iterator for HandlerContainerIter<'a, T> {
//     type Item = &'a (Rc<T>, Vec<Rc<RawHandle<T>>>);

//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next()
//     }
// }

// impl<T: std::fmt::Debug> HandlerContainer<T> {
//     fn new() -> Self {
//         Self {
//             values: vec![],
//         }
//     }
    
//     fn push(&mut self, value: Rc<T>) -> Handle<T> {
//         let raw_handle = Rc::new(RawHandle {
//             value: Rc::clone(&value),
//             id: Cell::new(self.values.len()),
//         });
        
//         self.values.push((value, vec![Rc::clone(&raw_handle)]));
        
//         Handle {
//             raw: raw_handle,
//         }
//     }

//     fn iter(&self) -> HandlerContainerIter<'_, T> {
//         HandlerContainerIter {
//             iter: self.values.iter(),
//         }
//     }
    
//     fn remove(&mut self, handle: Handle<T>) -> Rc<T> {
//         let last_pair = match self.values.last_mut() {
//             Some(val) => val,
//             None => return self.values.remove(0).0,
//         };
        
//         for raw_handle in &mut last_pair.1 {
//             raw_handle.id.set(handle.id());
//         }
    
//         self.values.swap_remove(handle.id()).0
//     }
// }

// // fn main() {
// //     let mut container = HandlerContainer::new();
    
// //     let handle1 = container.push(Rc::new(1));
// //     let handle2 = container.push(Rc::new(2));
// //     let handle3 = container.push(Rc::new(3));
    
// //     // println!("{}: {}", handle1.id(), handle1.value());
// //     // println!("{}: {}", handle2.id(), handle2.value());
// //     // println!("{}: {}", handle3.id(), handle3.value());

// //     for value in container.iter() {
// //         println!("value: {}", value.0);
// //     }
    
// //     println!("Remove: {}", container.remove(handle1));
    
// //     println!("{}: {}", handle2.id(), handle2.value());
// //     println!("{}: {}", handle3.id(), handle3.value());
// // }