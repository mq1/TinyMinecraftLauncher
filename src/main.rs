use iced::pure::{button, column, container, row, text, Element, Sandbox};
use iced::{Length, Settings};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

enum App {
    News,
    About,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    GoToNews,
    GoToAbout,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::News
    }

    fn title(&self) -> String {
        String::from("TinyMinecraftLauncher")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::GoToNews => *self = App::News,
            Message::GoToAbout => *self = App::About,
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self {
            App::News => text("News"),
            App::About => {
                text("TinyMinecraftLauncher - GPLv3 Licensed - Copyright (c) 2022 Manuel Quarneti")
            }
        };

        let nav = column()
            .padding(20)
            .push(button("News").on_press(Message::GoToNews))
            .push(button("About").on_press(Message::GoToAbout));

        let main = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        row().padding(20).push(nav).push(main).into()
    }
}
