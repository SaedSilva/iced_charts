use crate::{get_max_value, is_valid};
use iced::advanced::{renderer, Widget};
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::{Geometry, Stroke};
use iced::{Element, Length, Padding, Pixels, Point, Rectangle, Renderer, Size, Theme};
use iced::alignment::{Horizontal, Vertical};

pub struct VerticalBarChart {
    legends: Vec<String>,
    values: Vec<f32>,
    pub bar_max_value: f32,
    max_value: f32,
    width: Length,
    height: Length,
    internal_padding: Padding,
    lines: u32,
    font_size: Pixels,
}

impl VerticalBarChart {
    pub fn new(legends: Vec<String>, values: Vec<f32>) -> Self {
        let max = get_max_value(&values);
        let _ = is_valid(&values, &legends).expect("Legends and values must have the same length");

        Self {
            legends,
            values,
            bar_max_value: max,
            max_value: max,
            width: Length::Shrink,
            height: Length::Shrink,
            internal_padding: Padding::new(8.0),
            lines: 4,
            font_size: Pixels::from(16.0),
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

    pub fn max_multiply(mut self, max_multiply: f32) -> Self {
        self.max_value = self.max_value * max_multiply;
        self
    }
    
    pub fn lines(mut self, lines: u32) -> Self {
        self.lines = lines;
        self
    }
    
    pub fn font_size(mut self, font_size: impl Into<Pixels>) -> Self {
        self.font_size = font_size.into();
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
        let width = size.width;
        let height = size.height;
        let len_bars = self.values.len() as f32;
        
        if len_bars <= 0.0 {
            return vec![frame.into_geometry()]
        }
        
        let bar_max_height = height - self.internal_padding.vertical();
        let bar_width = (width - self.internal_padding.horizontal() * (len_bars + 1.0)) / len_bars;
        let value_for_multiply = bar_max_height / self.max_value;
        
        let blank_space = height - value_for_multiply * self.max_value;
        let line_space = value_for_multiply * self.max_value / self.lines as f32;

        println!("line_space: {}", line_space);
        println!("blank_space: {}", blank_space);
        println!("width: {}", width);
        println!("bar_width: {}", bar_width);

        for index in 0..=self.lines {
            let line = canvas::Path::line(
                Point {
                    x: 0.0,
                    y: index as f32 * line_space + blank_space,
                },
                Point {
                    x: width,
                    y: index as f32 * line_space + blank_space,
                }
            );

            frame.stroke(&line, Stroke::default().with_color(theme.extended_palette().secondary.weak.color));
        }

        for (index, &value) in self.values.iter().enumerate() {
            let top_left_x = self.internal_padding.horizontal() + (bar_width + self.internal_padding.horizontal()) * index as f32;
            let top_left_x_mid = self.internal_padding.horizontal() + (bar_width + self.internal_padding.horizontal()) * index as f32 + bar_width / 2.0;
            let top_left_y = bar_max_height - value * value_for_multiply + self.internal_padding.vertical();

            let height = value * value_for_multiply;

            let bar = canvas::Path::rectangle(
                Point {
                    x: top_left_x,
                    y: top_left_y,
                },
                Size {
                    height,
                    width: bar_width,
                },
            );

            let text = canvas::Text {
                content: value.to_string(),
                position: Point {
                    x: top_left_x_mid,
                    y: top_left_y - (self.font_size.0 / 2.0)
                },
                color: theme.palette().text,
                size: self.font_size,
                line_height: Default::default(),
                font: Default::default(),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                shaping: Default::default(),
            };

            frame.fill_text(text);
            frame.fill(&bar, theme.palette().primary);
        }
        
        frame.stroke_rectangle(
            Point { x: 2.0, y: 2.0 },
            Size {
                height: bounds.size().height - 2.0,
                width: bounds.size().width - 2.0,
            },
            Stroke::default().with_color(theme.palette().text),
        );

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
