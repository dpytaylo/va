use std::rc::Rc;

use crate::layer::Layer;
use crate::global::Va;

pub trait Object {
    fn create(&self, va: &Va) -> anyhow::Result<()> {
        Ok(())
    }

    fn add_in_layer(&self, va: &Va, layer: &Rc<Layer>) -> anyhow::Result<()> {
        Ok(())
    }

    fn remove_from_layer(&self, va: &Va, layer: &Rc<Layer>) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&self, va: &Va, layer: &Rc<Layer>) -> anyhow::Result<()> {
        Ok(())
    }

    fn add_child(&self, object: Rc<dyn Object>) {
        unimplemented!();
    }

    fn children(&self) -> Vec<Rc<dyn Object>> {
        unimplemented!();
    }
}