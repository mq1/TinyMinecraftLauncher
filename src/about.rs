use druid::{
    widget::{CrossAxisAlignment, Flex, Label},
    Widget,
};
use const_format::formatcp;

use crate::AppState;

const APP_VERSION: &str = formatcp!("TinyMinecraftLauncher version {}", env!("CARGO_PKG_VERSION"));
const LICENSE: &str = "GPLv3 Licensed | © 2022 Manuel Quarneti";

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new(APP_VERSION))
        .with_child(Label::new(LICENSE))
}
