mod series;

use plotters::chart::ChartContext;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::Cartesian2d;
use plotters_piston::PistonBackend;

use serde::Deserialize;
pub use series::*;

pub struct Plots {
    plots: Vec<PlotType>,
}

impl Plots {
    pub fn new() -> Self {
        Self { plots: Vec::new() }
    }

    pub fn build(&mut self, options: PlotOptions, data: &[f64]) {
        self.plots.push(match options {
            PlotOptions::Series(options) => Series::build(options, data),
        })
    }

    pub fn draw(
        &self,
        chart: &mut ChartContext<PistonBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) {
        for plot in &self.plots {
            match plot {
                PlotType::Series(plot) => plot.draw(chart),
            }
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PlotOptions {
    Series(SeriesOptions),
}

pub enum PlotType {
    Series(Series),
}
