use plotters::coord::ranged1d::SegmentedCoord;
use plotters::coord::types::{RangedCoordf64, RangedCoordi32, RangedSlice};
use plotters::prelude::*;
use plotters::chart::MeshStyle;
use plotters_piston::PistonBackend;

use serde::Deserialize;
use super::Range;

pub enum ChartType<'a, 'b, 'c> {
    Ranged(ChartContext<'c, PistonBackend<'a, 'b>, Cartesian2d<RangedCoordf64, RangedCoordf64>>),
    SegmentedX(
        ChartContext<
            'c,
            PistonBackend<'a, 'b>,
            Cartesian2d<SegmentedCoord<RangedCoordi32>, RangedCoordf64>,
        >,
    ),
    SegmentedY(
        ChartContext<
            'c,
            PistonBackend<'a, 'b>,
            Cartesian2d<RangedCoordf64, SegmentedCoord<RangedCoordi32>>,
        >,
    ),
    ValuesX(
        ChartContext<
            'c,
            PistonBackend<'a, 'b>,
            Cartesian2d<SegmentedCoord<RangedSlice<'c, String>>, RangedCoordf64>,
        >,
    ),
    ValuesY(
        ChartContext<
            'c,
            PistonBackend<'a, 'b>,
            Cartesian2d<RangedCoordf64, SegmentedCoord<RangedSlice<'c, String>>>,
        >,
    ),
    None
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ChartTypeOptions {
    Ranged {
        x_axis: Range<f64>,
        y_axis: Range<f64>,
    },
    SegmentedX {
        x_axis: Range<i32>,
        y_axis: Range<f64>,
    },
    SegmentedY {
        x_axis: Range<f64>,
        y_axis: Range<i32>,
    },
    ValuesX {
        x_axis: Vec<String>,
        y_axis: Range<f64>,
    },
    ValuesY {
        x_axis: Range<f64>,
        y_axis: Vec<String>,
    },
    None,
}
