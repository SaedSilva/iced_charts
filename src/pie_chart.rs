use iced::{mouse, Color, Element, Rectangle, Renderer, Theme};
use iced::advanced::widget::tree::State;
use iced::widget::canvas;

#[derive(Debug)]
struct PieChart {
    radius: f32,
}

// Then, we implement the `Program` trait
impl<Message> canvas::Program<Message> for PieChart {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // We create a `Path` representing a simple circle
        let circle = canvas::Path::circle(frame.center(), self.radius);

        // And fill it with some color
        frame.fill(&circle, Color::BLACK);

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

// Finally, we simply use our `Circle` to create the `Canvas`!
fn view<'a, Message: 'a>(_state: &'a State) -> Element<'a, Message> {
    canvas(PieChart { radius: 50.0 }).into()
}