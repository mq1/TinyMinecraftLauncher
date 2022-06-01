// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

mod about;
mod accounts;
mod instances;
mod news;
mod settings;

use anyhow::Result;
use druid::{im::Vector, widget::Tabs, AppLauncher, Data, Lens, Widget, WindowDesc};

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
        .with_tab("Instances", instances::build_widget())
        .with_tab("Accounts", accounts::build_widget())
        .with_tab("News", news::build_widget())
        .with_tab("Settings", settings::build_widget())
        .with_tab("About", about::build_widget())
}
