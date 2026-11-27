use iced::widget::{column, container, text};
use iced::{Element, Length, Theme};

fn main() -> iced::Result {
    iced::application("ORBYN", update, view)
        .theme(|_| Theme::Dark)
        .run()
}

#[derive(Debug, Clone)]
enum Message {}

fn update(_state: &mut App, _message: Message) {}

fn view(_state: &App) -> Element<'_, Message> {
    let content = column![
        text("ORBYN").size(38),
        text("Plataforma de colaboración humana y agentes").size(18),
        text("Rust nativo · Ejecución híbrida · Orquestaciones visuales y declarativas"),
        text("Fundación del producto: el espacio de trabajo y el editor de grafos se desarrollarán en hitos posteriores."),
    ]
    .spacing(14)
    .padding(32);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

#[derive(Default)]
struct App;
