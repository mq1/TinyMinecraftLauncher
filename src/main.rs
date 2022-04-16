// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use anyhow::Result;
use druid::{
    im::Vector,
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll, Tabs},
    AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc,
};

#[macro_use]
extern crate anyhow;

#[derive(Data, Clone, Lens)]
struct AppState {
    news: Vector<(String, String)>,
}

fn main() -> Result<()> {
    let main_window = WindowDesc::new(build_root_widget()).title("TinyMinecraftLauncher");

    let news = mc::news::get_minecraft_news(None)?
        .article_grid
        .into_iter()
        .map(|article| (article.default_tile.title, article.article_url.to_string()))
        .collect::<Vec<(String, String)>>();

    let news: Vector<(String, String)> = Vector::from(news);
    let initial_state = AppState { news };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .map_err(|e| anyhow!(e))
}

fn build_root_widget() -> impl Widget<AppState> {
    let news_tab = Scroll::new(List::new(|| {
        Flex::row()
            .with_child(Label::new(|(title, _url): &(String, String), _env: &_| {
                title.to_owned()
            }))
            .with_child(
                Button::new("Read").on_click(|_ctx, (_title, url), _env| open::that(url).unwrap()),
            )
    }))
    .lens(AppState::news);

    let about_tab = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new(format!(
            "TinyMinecraftLauncher version {}",
            env!("CARGO_PKG_VERSION")
        )))
        .with_child(Label::new("GPLv3 Licensed | © 2022 Manuel Quarneti"));

    Tabs::new()
        .with_tab("News", news_tab)
        .with_tab("About", about_tab)
}
