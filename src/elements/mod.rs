mod shapes;

use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_piston::PistonBackend;

use serde::Deserialize;
pub use shapes::*;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ElementType {
    Rect(RectShape),
}

impl ElementType {
    pub fn draw(&self, root: &mut DrawingArea<PistonBackend, Shift>) {
        match self {
            ElementType::Rect(rect) => rect.draw(root),
        }
    }
}
