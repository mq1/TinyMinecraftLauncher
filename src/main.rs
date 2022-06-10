// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

mod about;
mod accounts;
mod instances;
mod news;
mod settings;

use anyhow::Result;
use druid::{widget::{Tabs, Axis}, AppLauncher, Data, Lens, Widget, WindowDesc};

#[macro_use]
extern crate anyhow;

#[derive(Data, Clone, Lens)]
pub struct AppState {}

fn main() -> Result<()> {
    let main_window = WindowDesc::new(build_root_widget())
        .title("TinyMinecraftLauncher")
        .window_size((525., 400.));

    let initial_state = AppState {};

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .map_err(|e| anyhow!(e))
}

fn build_root_widget() -> impl Widget<AppState> {
    Tabs::new()
        .with_axis(Axis::Vertical)
        .with_tab("Instances", instances::build_widget())
        .with_tab("Accounts", accounts::build_widget())
        .with_tab("News", news::build_widget())
        .with_tab("Settings", settings::build_widget())
        .with_tab("About", about::build_widget())
}
