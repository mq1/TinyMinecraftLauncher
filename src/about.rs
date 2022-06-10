use const_format::formatcp;
use iced::{Column, Element, Text};

use crate::Message;

const APP_VERSION: &str = formatcp!(
    "TinyMinecraftLauncher version {}",
    env!("CARGO_PKG_VERSION")
);
const LICENSE: &str = "GPLv3 Licensed | © 2022 Manuel Quarneti";

pub struct About;

impl About {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(Text::new(APP_VERSION))
            .push(Text::new(LICENSE))
            .into()
    }
}
