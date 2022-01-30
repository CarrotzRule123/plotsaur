use plotters::chart::ChartContext;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use plotters_piston::PistonBackend;

use serde::Deserialize;

use super::PlotType;
use super::super::{PlotChart, ShapeColor};

pub struct Series {
    pub color: ShapeColor,
    pub label: String,
    pub data: Vec<(f64, f64)>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeriesOptions {
    pub color: ShapeColor,
    pub label: String,
}

impl Series {
    pub fn build(options: SeriesOptions, buf: &[f64]) -> PlotType {
        let mut data = Vec::new();
        for i in (0..buf.len()).step_by(2) {
            data.push((buf[i], buf[i + 1]))
        }
        PlotType::Series(Series {
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
            .expect("Error: Could not draw series!")
            .label(self.label.clone())
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.clone()));
    }
}
