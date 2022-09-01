use std::rc::Rc;
use crate::{global::Va, window::Window};

pub struct Render;

impl Render {
    /// # Panics
    ///
    /// Panics if not setup the device or the queue
    pub fn draw(va: &Rc<Va>, window: &Rc<Window>) -> anyhow::Result<()> {
        window
            .render()
            .draw(va, window.graphics(), window.layers())?;

        Ok(())
    }
}
