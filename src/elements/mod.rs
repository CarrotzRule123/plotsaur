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
    Circle(CircleShape),
    Polygon(PolygonShape),
    Text(TextShape),
    Path(PathShape),
}

impl ElementType {
    pub fn draw(&self, root: &mut DrawingArea<PistonBackend, Shift>) {
        match self {
            ElementType::Rect(rect) => rect.draw(root),
            ElementType::Circle(circle) => circle.draw(root),
            ElementType::Polygon(polygon) => polygon.draw(root),
            ElementType::Text(text) => text.draw(root),
            ElementType::Path(path) => path.draw(root),
        }
    }
}
