use druid::{
    widget::{Button, Flex, Label, Scroll},
    Widget,
};

use crate::AppState;

fn build_item(article: mc::news::Article) -> impl Widget<AppState> {
    let label = Label::new(article.default_tile.title);

    let read_button = Button::new("Read")
        .on_click(move |_ctx, _data, _env| open::that(article.article_url.as_str()).unwrap());

    Flex::row()
        .with_child(label)
        .with_default_spacer()
        .with_child(read_button)
}

pub fn build_widget() -> impl Widget<AppState> {
    let news = mc::news::get_minecraft_news(None)
        .unwrap()
        .article_grid
        .into_iter()
        .collect::<Vec<mc::news::Article>>();

    let mut col = Flex::column();
    for article in news {
        col.add_child(build_item(article));
    }

    Scroll::new(col)
}
