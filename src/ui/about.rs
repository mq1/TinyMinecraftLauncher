use druid::{
    widget::{CrossAxisAlignment, Flex, Label},
    Widget,
};

use crate::AppState;

pub fn build_about_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new(format!(
            "TinyMinecraftLauncher version {}",
            env!("CARGO_PKG_VERSION")
        )))
        .with_child(Label::new("GPLv3 Licensed | © 2022 Manuel Quarneti"))
}
