use iced_wgpu::Renderer;
use iced_winit::{Align, Clipboard, Column, Command, Container, Element, Length, Program, Text};
use plotters::{coord::Shift, prelude::*};
use plotters_iced::plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget, DrawingArea};

use super::RESOURCES;

const TITLE_FONT_SIZE: u16 = 22;

#[allow(unused)]
#[derive(Debug)]
pub enum Message {
    Tick,
}

pub struct PlotProgram {}

impl PlotProgram {
    pub fn new() -> Self {
        Self {}
    }

    pub fn title(&self) -> String {
        "Split Chart Example".to_owned()
    }
}

impl Program for PlotProgram {
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
        RESOURCES.with(|cell| {
            let resources = cell.borrow();
            resources.widgets.view()
        })
    }
}

pub struct PlotWidgets {}

impl PlotWidgets {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<'static, Message, Renderer> {
        let content = Column::new()
            .spacing(20)
            .align_items(Align::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Text::new("Iced test chart").size(TITLE_FONT_SIZE));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }
}
