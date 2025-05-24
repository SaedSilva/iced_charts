use crate::{get_max_value, is_valid};
use iced::advanced::{renderer, Widget};
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::{Fill, Geometry, Stroke};
use iced::{Color, Element, Length, Padding, Point, Rectangle, Renderer, Size, Theme};

pub struct VerticalBarChart {
    legends: Vec<String>,
    values: Vec<f32>,
    max_value: f32,
    width: Length,
    height: Length,
    internal_padding: Padding,
}

impl VerticalBarChart {
    pub fn new(legends: Vec<String>, values: Vec<f32>) -> Self {
        let max = get_max_value(&values);
        let _ = is_valid(&values, &legends).expect("Legends and values must have the same length");

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

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.internal_padding = padding.into();
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max_value = max;
        self
    }
}

impl<Message> canvas::Program<Message> for VerticalBarChart {
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
        let len_bars = self.values.len() as f32;
        let bar_width = (width - self.internal_padding.horizontal() * (len_bars - 1.0)) / len_bars;
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

        let rectangle = canvas::Path::rectangle(
            Point {
                x: 1.0,
                y: 1.0,
            },
            Size {
                height: bounds.size().height - 1.0,
                width: bounds.size().width - 1.0,
            }
        );
        frame.stroke(&rectangle, Stroke::default().with_color(theme.palette().text));

        vec![frame.into_geometry()]
    }
}

impl<'a, Message: 'a> From<VerticalBarChart> for Element<'a, Message>
where
    Renderer: renderer::Renderer,
{
    fn from(bar_chart: VerticalBarChart) -> Self {
        let width = bar_chart.width;
        let height = bar_chart.height;
        canvas(bar_chart).width(width).height(height).into()
    }
}
