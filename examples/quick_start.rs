use iced::{Element, Length, Task, Theme};
use iced::widget::{column, container, text};
use iced_mouse_layer::mouse_layer;

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct App;

impl App {
    fn title(&self) -> String {
        "iced-mouse-layer quick start".into()
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let base = container(text("Move your mouse.")).padding(16);

        let ghost = mouse_layer(
            container(text("👻 MouseLayer"))
                .padding(10)
        )
        .offset(15.0, 15.0);

        column![base, ghost]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(App::title)
        .theme(Theme::Dark)
        .run()
}