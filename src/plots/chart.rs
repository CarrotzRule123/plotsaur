use piston_window::*;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_piston::PistonBackend;

use serde::Deserialize;

use super::FontStyle;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ChartOptions {
    Margin(u32),
    Caption { caption: String, style: FontStyle },
}

pub struct PlotChart {
    pub options: Vec<ChartOptions>,
}

impl PlotChart {
    pub fn new(options: Vec<ChartOptions>) -> Self {
        Self { options }
    }

    pub fn draw(&self, root: &DrawingArea<PistonBackend, Shift>) {
        let mut builder = &mut ChartBuilder::on(root);
        for option in &self.options {
            match option {
                ChartOptions::Margin(margin) => builder = builder.margin(margin.clone()),
                ChartOptions::Caption { caption, style } => {
                    builder = builder.caption(caption, (style.family.as_str(), style.size.clone()))
                }
            }
        }
        let mut cc = builder
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0..50 as u32, 0f32..1f32)
            .expect("aaaa");
        cc.configure_mesh()
            .x_labels(15)
            .y_labels(5)
            .x_desc("Seconds")
            .y_desc("% Busy")
            .axis_desc_style(("sans-serif", 15))
            .draw()
            .expect("aaaa");
        cc.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .expect("aaaa");
    }
}
