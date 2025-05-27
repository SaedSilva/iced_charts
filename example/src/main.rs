use iced::widget::{column, container, text};
use iced::{Color, Element, Task};
use iced_charts::bar_chart::VerticalBarChart;
use iced_charts::pie_chart::PieChart;

fn main() -> iced::Result {
    iced::application("A example app", update, view).run()
}

#[derive(Default)]
struct State;

#[derive(Debug)]
enum Message {}

fn view(state: &State) -> Element<Message> {
    let vertical_bars = VerticalBarChart::new(
        vec![
        ],
        vec![],
    )
    // .max(20.0)
    .lines(15)
    .width(200)
    .height(200);

    let pie_chart = PieChart::new(vec![5.0, 10.0, 5.0, 7.5], COLORS.to_vec()).width(200).height(200);

    container(column![vertical_bars, pie_chart].spacing(16))
        .padding(16)
        .into()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    Task::none()
}

const COLORS: &[Color] = &[
    Color::from_rgb(0.1, 0.2, 0.3),
    Color::from_rgb(0.4, 0.5, 0.6),
    Color::from_rgb(0.7, 0.8, 0.9),
    Color::from_rgb(1.0, 0.5, 0.5),
    Color::from_rgb(0.5, 1.0, 0.5),
];
