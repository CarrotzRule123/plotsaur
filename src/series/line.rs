use plotters::chart::ChartContext;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use plotters_piston::PistonBackend;

use serde::Deserialize;

use super::SeriesType;
use super::super::{PlotChart, ShapeColor};

pub struct Line {
    pub color: ShapeColor,
    pub label: String,
    pub data: Vec<(f64, f64)>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LineOptions {
    pub color: ShapeColor,
    pub label: String,
}

impl Line {
    pub fn build(options: LineOptions, buf: &[f64]) -> SeriesType {
        let mut data = Vec::new();
        for i in (0..buf.len()).step_by(2) {
            data.push((buf[i], buf[i + 1]))
        }
        SeriesType::Line(Line {
            color: options.color,
            label: options.label,
            data,
        })
    }

    pub fn draw(
        &self,
        chart: &mut ChartContext<PistonBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) {
        let color = self.color.to_color();
        chart
            .draw_series(LineSeries::new(self.data.clone(), color.clone()))
            .expect("Error: Could not draw line!")
            .label(self.label.clone())
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.clone()));
    }
}
