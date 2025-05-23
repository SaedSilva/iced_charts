use crate::get_max_value;
use iced::advanced::{Widget, renderer};
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::Geometry;
use iced::{Element, Length, Padding, Point, Rectangle, Renderer, Size, Theme};

pub struct BarChart {
    legends: Vec<String>,
    values: Vec<f32>,
    max_value: f32,
    width: Length,
    height: Length,
    internal_padding: Padding,
}

impl BarChart {
    pub fn new(legends: Vec<String>, values: Vec<f32>) -> Self {
        let max = get_max_value(&values);

        Self {
            legends,
            values,
            max_value: max,
            width: Length::Shrink,
            height: Length::Shrink,
            internal_padding: Padding::new(8.0),
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<Message> canvas::Program<Message> for BarChart {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let size = frame.size();
        let height = size.height;
        let width = size.width;
        let bar_width = width / self.values.len() as f32 - self.internal_padding.horizontal();
        let value_for_multiply = height / self.max_value;

        for (index, &value) in self.values.iter().enumerate() {
            let bar = canvas::Path::rectangle(
                Point {
                    x: (bar_width + self.internal_padding.horizontal()) * index as f32,
                    y: height - value * value_for_multiply,
                },
                Size {
                    height: value * value_for_multiply,
                    width: bar_width,
                },
            );
            frame.fill(&bar, theme.palette().primary)
        }

        // We create a `Path` representing a simple circle
        // let circle = canvas::Path::circle(frame.center(), 16f32);

        // And fill it with some color
        // frame.fill(&circle, Color::BLACK);

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

impl<'a, Message: 'a> From<BarChart> for Element<'a, Message>
where
    Renderer: renderer::Renderer,
{
    fn from(bar_chart: BarChart) -> Self {
        let width = bar_chart.width;
        let height = bar_chart.height;
        canvas(bar_chart).width(width).height(height).into()
    }
}
