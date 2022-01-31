use plotters::chart::ChartContext;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_piston::PistonBackend;

use serde::Deserialize;

use super::super::{PlotChart, Point, ShapeColor};
use super::ElementType;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RectShape {
    points: [Point<i32>; 2],
    style: ShapeColor,
    filled: bool,
}

impl RectShape {
    pub fn draw(&self, root: &mut DrawingArea<PistonBackend, Shift>) {
        let color = self.style.to_color();
        let coords = self.points.clone().map(|point| (point.x, point.y));
        let mut style: ShapeStyle = self.style.to_color().into();
        if self.filled {
            style = style.filled();
        };
        root.draw(&Rectangle::new(coords, style))
            .expect("Error: Could not draw rectangle!")
    }
}
