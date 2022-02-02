use plotters::prelude::*;
use plotters_piston::PistonBackend;

use serde::Deserialize;

use super::{ChartType, ChartTypeOptions, ElementType, SeriesType, ShapeColor, TextStyles};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChartBuild {
    pub options: Vec<ChartOptions>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ChartOptions {
    Margin(i32),
    Caption { caption: String, style: TextStyles },
    XLabelAreaSize(f64),
    YLabelAreaSize(f64),
    Cartesian2D(ChartTypeOptions),
    Mesh(Vec<Mesh>),
    SeriesLabel(Vec<SeriesLabel>),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Mesh {
    XLabels(usize),
    YLabels(usize),
    XDesc(String),
    YDesc(String),
    AxisDescStyle(TextStyles),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SeriesLabel {
    BackgroundStyle(ShapeColor),
    BorderStyle(ShapeColor),
}

pub struct PlotChart {
    pub options: Vec<ChartOptions>,
    pub elements: Vec<ElementType>,
    pub series: Vec<SeriesType>,
    pub chart_type: ChartTypeOptions,
}

macro_rules! configure_mesh {
    ($chart: expr, $mesh_options: expr) => {{
        let mut mesh = $chart.configure_mesh();
        if let Some(options) = $mesh_options {
            for option in options {
                match option {
                    Mesh::XLabels(labels) => mesh.x_labels(labels.clone()),
                    Mesh::YLabels(labels) => mesh.y_labels(labels.clone()),
                    Mesh::XDesc(desc) => mesh.x_desc(desc),
                    Mesh::YDesc(desc) => mesh.y_desc(desc),
                    Mesh::AxisDescStyle(style) => {
                        mesh.axis_desc_style((style.family.as_str(), style.size))
                    }
                };
            }
            mesh.draw().expect("Error: Could not draw mesh!");
        }
    };};
}

macro_rules! configure_series_labels {
    ($chart: expr, $label_options: expr) => {{
        let mut series_label = $chart.configure_series_labels();
        if let Some(options) = $label_options {
            for option in options {
                match option {
                    SeriesLabel::BackgroundStyle(style) => {
                        series_label.background_style(style.to_color())
                    }
                    SeriesLabel::BorderStyle(style) => series_label.border_style(style.to_color()),
                };
            }
            series_label.draw().expect("Error: Could not draw mesh!");
        }
    }};
}

impl PlotChart {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            elements: Vec::new(),
            series: Vec::new(),
            chart_type: ChartTypeOptions::None
        }
    }

    pub fn draw(&mut self, backend: PistonBackend) {
        let mut root = backend.into_drawing_area();
        root.fill(&WHITE).expect("Error: Could not fill window");
        {
            let mut builder = &mut ChartBuilder::on(&root);
            let mut chart_options = None;
            let mut mesh_options = None;
            let mut label_options = None;
            for option in &self.options {
                match option {
                    ChartOptions::Margin(margin) => builder = builder.margin(margin.clone()),
                    ChartOptions::Caption { caption, style } => {
                        builder = builder.caption(caption, (style.family.as_str(), style.size))
                    }
                    ChartOptions::XLabelAreaSize(size) => {
                        builder = builder.x_label_area_size(size.clone())
                    }
                    ChartOptions::YLabelAreaSize(size) => {
                        builder = builder.y_label_area_size(size.clone())
                    }
                    ChartOptions::Cartesian2D(options) => chart_options = Some(options),
                    ChartOptions::Mesh(mesh) => mesh_options = Some(mesh.clone()),
                    ChartOptions::SeriesLabel(label) => label_options = Some(label.clone()),
                }
            }
            let options = chart_options.unwrap();
            let mut chart = match options {
                ChartTypeOptions::Ranged { x_axis, y_axis } => {
                    let mut chart = builder
                        .build_cartesian_2d(x_axis.start..x_axis.end, y_axis.start..y_axis.end)
                        .expect("Error: Could not build chart!");
                    configure_mesh!(chart, &mesh_options);
                    ChartType::Ranged(chart)
                }
                ChartTypeOptions::SegmentedX { x_axis, y_axis } => {
                    let mut chart = builder
                        .build_cartesian_2d(
                            (x_axis.start..x_axis.end).into_segmented(),
                            y_axis.start..y_axis.end,
                        )
                        .expect("Error: Could not build chart!");
                    configure_mesh!(chart, &mesh_options);
                    ChartType::SegmentedX(chart)
                }
                ChartTypeOptions::SegmentedY { x_axis, y_axis } => {
                    let mut chart = builder
                        .build_cartesian_2d(
                            x_axis.start..x_axis.end,
                            (y_axis.start..y_axis.end).into_segmented(),
                        )
                        .expect("Error: Could not build chart!");
                    configure_mesh!(chart, &mesh_options);
                    ChartType::SegmentedY(chart)
                }
                ChartTypeOptions::ValuesX { x_axis, y_axis } => {
                    let mut chart = builder
                        .build_cartesian_2d(x_axis.into_segmented(), y_axis.start..y_axis.end)
                        .expect("Error: Could not build chart!");
                    configure_mesh!(chart, &mesh_options);
                    ChartType::ValuesX(chart)
                }
                ChartTypeOptions::ValuesY { x_axis, y_axis } => {
                    let mut chart = builder
                        .build_cartesian_2d(x_axis.start..x_axis.end, (y_axis).into_segmented())
                        .expect("Error: Could not build chart!");
                    configure_mesh!(chart, &mesh_options);
                    ChartType::ValuesY(chart)
                }
                _ => panic!("Error: Could not build chart!"),
            };
            
            for series in &self.series {
                series.draw(&mut chart)
            }
            match &mut chart {
                ChartType::Ranged(chart) => configure_series_labels!(chart, &label_options),
                ChartType::SegmentedX(chart) => configure_series_labels!(chart, &label_options),
                ChartType::SegmentedY(chart) => configure_series_labels!(chart, &label_options),
                ChartType::ValuesX(chart) => configure_series_labels!(chart, &label_options),
                ChartType::ValuesY(chart) => configure_series_labels!(chart, &label_options),
                _ => panic!("Error: Could not build chart!"),
            };
        }
        for elements in &self.elements {
            elements.draw(&mut root)
        }
    }
}
