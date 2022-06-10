use iced::{Column, Element, Text};

use crate::Message;

pub struct News;

impl News {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new().push(Text::new("News")).into()
    }
}
