// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use anyhow::Result;
use druid::{
    im::Vector,
    widget::{Label, List, Scroll, Tabs},
    AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc,
};

#[macro_use]
extern crate anyhow;

#[derive(Data, Clone, Lens)]
struct AppState {
    news: Vector<String>,
}

fn main() -> Result<()> {
    let main_window = WindowDesc::new(build_root_widget()).title("TinyMinecraftLauncher");

    let news = mc::news::get_minecraft_news(None)?
        .article_grid
        .into_iter()
        .map(|article| article.default_tile.title)
        .collect::<Vec<String>>();

    let news: Vector<String> = Vector::from(news);
    let initial_state = AppState { news };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .map_err(|e| anyhow!(e))
}

fn build_root_widget() -> impl Widget<AppState> {
    let news_tab = Scroll::new(List::new(|| {
        Label::new(|item: &String, _env: &_| format!("{item}"))
    }))
    .lens(AppState::news);

    let about_tab = Label::new("GPLv3 Licensed | © 2022 Manuel Quarneti");

    Tabs::new()
        .with_tab("News", news_tab)
        .with_tab("About", about_tab)
}
