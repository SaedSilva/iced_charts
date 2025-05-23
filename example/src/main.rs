use iced::{Element, Length, Task};
use iced::widget::container;
use iced_charts::bar_chart::BarChart;

fn main() -> iced::Result {
    iced::application("A example app", update, view).run()
}

#[derive(Default)]
struct State;

#[derive(Debug)]
enum Message {}

fn view(state: &State) -> Element<Message> {
    container(BarChart::new(
        vec![
            "Teste".to_string(),
            "Teste2".to_string(),
            "Teste3".to_string(),
        ],
        vec![15.0, 10.0, 5.0, 6.0, 9.0],
    ).width(Length::Fill).height(Length::Fill)).padding(16).into()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    Task::none()
}
