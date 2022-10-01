use std::rc::Rc;

use crate::object::Object;
use crate::layer::Layer;

pub struct ObjectData<T> 
    where T: Object,
{
    layer: Rc<Layer>,
    object: T,
}