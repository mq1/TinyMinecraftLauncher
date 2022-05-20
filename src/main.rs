// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

pub mod ui;

use anyhow::Result;
use druid::{im::Vector, widget::Tabs, AppLauncher, Data, Lens, Widget, WindowDesc};
use ui::{
    about::build_about_widget, accounts::build_accounts_widget, instances::build_instances_widget,
    news::build_news_widget, settings::build_settings_widget,
};

#[macro_use]
extern crate anyhow;

#[derive(Data, Clone, Lens)]
pub struct Article {
    image_url: String,
    title: String,
    url: String
}

#[derive(Data, Clone, Lens)]
pub struct AppState {
    news: Vector<Article>,
}

fn main() -> Result<()> {
    let main_window = WindowDesc::new(build_root_widget()).title("TinyMinecraftLauncher");

    let news = mc::news::get_minecraft_news(None)?
        .article_grid
        .into_iter()
        .map(|article| Article { image_url: article.default_tile.image.image_url.to_string(), title: article.default_tile.title, url: article.article_url.to_string() })
        .collect::<Vec<Article>>();

    let news: Vector<Article> = Vector::from(news);
    let initial_state = AppState { news };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .map_err(|e| anyhow!(e))
}

fn build_root_widget() -> impl Widget<AppState> {
    Tabs::new()
        .with_tab("Instances", build_instances_widget())
        .with_tab("Accounts", build_accounts_widget())
        .with_tab("News", build_news_widget())
        .with_tab("Settings", build_settings_widget())
        .with_tab("About", build_about_widget())
}
