use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_piston::PistonBackend;

use serde::Deserialize;

use super::super::{Point, ShapeColor, TextStyles};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RectShape {
    points: [Point<i32>; 2],
    style: ShapeColor,
    filled: bool,
}

impl RectShape {
    pub fn draw(&self, root: &mut DrawingArea<PistonBackend, Shift>) {
        let coords = self.points.clone().map(|point| (point.x, point.y));
        let mut style: ShapeStyle = self.style.to_color().into();
        if self.filled {
            style = style.filled();
        };
        root.draw(&Rectangle::new(coords, style))
            .expect("Error: Could not draw rectangle!")
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CircleShape {
    points: Point<i32>,
    style: ShapeColor,
    size: f64,
    filled: bool,
}

impl CircleShape {
    pub fn draw(&self, root: &mut DrawingArea<PistonBackend, Shift>) {
        let mut style: ShapeStyle = self.style.to_color().into();
        if self.filled {
            style = style.filled();
        };
        root.draw(&Circle::new(
            (self.points.x, self.points.y),
            self.size,
            style,
        ))
        .expect("Error: Could not draw circle!")
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PolygonShape {
    points: Vec<Point<i32>>,
    style: ShapeColor,
    filled: bool,
}

impl PolygonShape {
    pub fn draw(&self, root: &mut DrawingArea<PistonBackend, Shift>) {
        let coords: Vec<(i32, i32)> = self.points.iter().map(|point| (point.x, point.y)).collect();
        let mut style: ShapeStyle = self.style.to_color().into();
        if self.filled {
            style = style.filled();
        };
        root.draw(&Polygon::new(coords, style))
            .expect("Error: Could not draw polygon!")
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextShape {
    points: Point<i32>,
    style: TextStyles,
    text: String,
    color: ShapeColor,
}

impl TextShape {
    pub fn draw(&self, root: &mut DrawingArea<PistonBackend, Shift>) {
        let color = self.color.to_color();
        let style = TextStyle::from((self.style.family.as_str(), self.style.size).into_font())
            .color(&color);
        root.draw(&Text::new(
            self.text.clone(),
            (self.points.x, self.points.y),
            style,
        ))
        .expect("Error: Could not draw text!")
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PathShape {
    points: Vec<Point<i32>>,
    style: ShapeColor,
}

impl PathShape {
    pub fn draw(&self, root: &mut DrawingArea<PistonBackend, Shift>) {
        let coords: Vec<(i32, i32)> = self.points.iter().map(|point| (point.x, point.y)).collect();
        let style: ShapeStyle = self.style.to_color().into();
        root.draw(&PathElement::new(coords, style))
            .expect("Error: Could not draw path!")
    }
}
