use druid::{
    widget::{Button, Flex, Label, List, Scroll},
    Env, Widget, WidgetExt,
};

use crate::{AppState, Article};

fn build_item() -> impl Widget<Article> {
    let label = Label::new(|data: &Article, _: &Env| data.title.to_owned());

    let read_button = Button::new("Read")
        .on_click(|_ctx, article: &mut Article, _env| open::that(article.url.to_owned()).unwrap());

    Flex::row().with_child(label).with_child(read_button)
}

pub fn build_news_widget() -> impl Widget<AppState> {
    Scroll::new(List::new(build_item).lens(AppState::news))
}
