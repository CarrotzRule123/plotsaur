mod window;
mod chart;
use chart::ChartProgram;
use window::window;

pub fn main() {
    let chart = ChartProgram::new();
    window(chart)
}