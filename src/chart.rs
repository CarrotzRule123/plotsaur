use iced_winit::{
    Align, Clipboard, Column, Command, Container, Element, Length, Program, Text
};
use iced_wgpu::Renderer;
use plotters::{coord::Shift, prelude::*};
use plotters_iced::plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget, DrawingArea};

const TITLE_FONT_SIZE: u16 = 22;

#[allow(unused)]
#[derive(Debug)]
pub enum Message {
    Tick,
}

pub struct ChartProgram {
    chart: MyChart,
}

impl ChartProgram {
    pub fn new() -> Self {
        Self {
            chart: MyChart::new(),
        }
    }

    pub fn title(&self) -> String {
        "Split Chart Example".to_owned()
    }
}

impl Program for ChartProgram {
    type Renderer = Renderer;
    type Message = Message;
    type Clipboard = Clipboard;

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message, Renderer> {
        let content = Column::new()
            .spacing(20)
            .align_items(Align::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Text::new("Iced test chart").size(TITLE_FONT_SIZE))
            .push(self.chart.view());

        Container::new(content)
            //.style(style::Container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }
}

#[allow(unused)]
struct MyChart {
    width: u16,  //wasm32 backend requires fixed size
    height: u16, //wasm32 backend requires fixed size
}

impl MyChart {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }

    fn view(&mut self) -> Element<Message, Renderer> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let chart = ChartWidget::new(self)
                .width(Length::Fill)
                .height(Length::Fill);

            chart.into()
        }
        #[cfg(target_arch = "wasm32")]
        {
            let width = self.width;
            let height = self.height;
            let chart = ChartWidget::new(self)
                .width(Length::Units(width))
                .height(Length::Units(height));

            chart.into()
        }
    }
}

impl Chart<Message> for MyChart {
    // leave it empty
    fn build_chart<DB: DrawingBackend>(&self, _builder: ChartBuilder<DB>) {}

    fn draw_chart<DB: DrawingBackend>(&self, root: DrawingArea<DB, Shift>) {
        let builder = ChartBuilder::on(&root);
        draw_chart(builder, 2);
    }
}

fn draw_chart<DB: DrawingBackend>(mut chart: ChartBuilder<DB>, power: usize) {
    let mut chart = chart
        .margin(30)
        // .caption(format!("y=x^{}", power), ("sans-serif", 22))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f32..10f32, -10f32..10f32)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        // .y_label_style(
        //     ("sans-serif", 15)
        //         .into_font()
        //         .color(&plotters::style::colors::BLACK.mix(0.8))
        //         .transform(FontTransform::RotateAngle(30.0)),
        // )
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (-500..=500)
                .map(|x| x as f32 / 50.0)
                .map(|x| (x, x.powf(power as f32))),
            &RED,
        ))
        .unwrap();
}
