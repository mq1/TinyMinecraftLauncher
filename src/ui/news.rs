use druid::{
    widget::{Button, Flex, Label, List, Scroll},
    Env, Widget, WidgetExt,
};

use crate::{AppState, Article};

fn build_item() -> impl Widget<Article> {
    Flex::row()
        .with_child(Label::new(|data: &Article, _: &Env| data.title.to_owned()))
        .with_child(
            Button::new("Read").on_click(|_ctx, article: &mut Article, _env| {
                open::that(article.url.to_owned()).unwrap()
            }),
        )
}

pub fn build_news_widget() -> impl Widget<AppState> {
    Scroll::new(List::new(build_item).lens(AppState::news))
}
