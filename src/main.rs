mod about;
mod news;

use iced::{button, Button, Column, Container, Element, Length, Row, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

enum Router {
    News,
    About,
}

struct App {
    router: Router,

    news_button: button::State,
    about_button: button::State,

    news: news::News,
    about: about::About,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    GoToNews,
    GoToAbout,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            router: Router::News,
            news_button: button::State::new(),
            about_button: button::State::new(),
            news: news::News::new(),
            about: about::About::new(),
        }
    }

    fn title(&self) -> String {
        String::from("TinyMinecraftLauncher")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::GoToNews => (*self).router = Router::News,
            Message::GoToAbout => (*self).router = Router::About,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content: Element<Message> = match self.router {
            Router::News => self.news.view(),
            Router::About => self.about.view(),
        };

        let nav = Column::new()
            .padding(20)
            .push(Button::new(&mut self.news_button, Text::new("News")).on_press(Message::GoToNews))
            .push(
                Button::new(&mut self.about_button, Text::new("About"))
                    .on_press(Message::GoToAbout),
            );

        let main = Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        Row::new().padding(20).push(nav).push(main).into()
    }
}
