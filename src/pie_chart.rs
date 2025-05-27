use iced::advanced::{renderer, Widget};
use iced::widget::canvas;
use iced::widget::canvas::path::Arc;
use iced::widget::canvas::{Path, Stroke};
use iced::{color, mouse, Color, Element, Length, Radians, Rectangle, Renderer, Theme};
use std::f32::consts::PI;

#[derive(Debug)]
pub struct PieChart {
    width: Length,
    height: Length,
    values: Vec<f32>,
    colors: Vec<Color>,
    total: f32,
}

impl PieChart {
    pub fn new(values: Vec<f32>, colors: Vec<Color>) -> Self {
        let mut total = 0.0;
        for value in values.iter() {
            total += value;
        }
        Self {
            width: Length::Shrink,
            height: Length::Shrink,
            values,
            colors,
            total,
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

// Then, we implement the `Program` trait
impl<Message> canvas::Program<Message> for PieChart {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let center = frame.center();
        let min = frame.width().min(frame.height());
        let radius = min / 2.25;

        let mut percent_acumuled = 0.0;
        
        let arcs: Vec<Arc> = self
            .values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                
                let index = index as f32;
                let percent = value / self.total;
                let total_area = PI * 2.0;
                
                let start_angle = Radians(percent_acumuled * total_area);
                percent_acumuled += percent;
                let end_angle = Radians(percent_acumuled * total_area);
                
                
                println!(
                    "[{}] - Value: {}, StartAngle: {}, EndAngle: {}, Percent: {}",
                    index, value, start_angle, end_angle, percent
                );
                Arc {
                    center,
                    radius,
                    start_angle,
                    end_angle,
                }
            })
            .collect();

        // Define o arco: um semicírculo (0 a PI radianos = 0 a 180 graus)
        /* let simple_arc_data = canvas::path::Arc {
            center,                       // Centro do arco
            radius,                       // Raio do arco
            start_angle: Radians(0.0),    // Começa em 0 radianos (direita)
            end_angle: Radians(PI * 2.0), // Termina em PI radianos (esquerda, 180 graus)
        };*/

        let stroke_style = Stroke {
            style: canvas::Style::Solid(theme.palette().primary), // Cor primária do tema
            width: 5.0,
            ..Stroke::default()
        };

        for (index, arc) in arcs.iter().enumerate() {
            let arc_path = Path::new(|builder| {
                builder.arc(*arc);
                builder.line_to(frame.center());
            });
            frame.fill(
                &arc_path,
                *self
                    .colors
                    .clone()
                    .get(index)
                    .unwrap_or_else(|| &Color::BLACK),
            );
        }

        // Cria o caminho (Path) com o arco

        // Define o estilo do traço (cor e espessura)
        

        // Desenha o contorno do arco
        // frame.stroke(&arc_path, stroke_style);

        vec![frame.into_geometry()]
    }
}

impl<'a, Message: 'a> From<PieChart> for Element<'a, Message>
where
    Renderer: renderer::Renderer,
{
    fn from(pie_chart: PieChart) -> Self {
        let width = pie_chart.width;
        let height = pie_chart.height;

        canvas(pie_chart).width(width).height(height).into()
    }
}
