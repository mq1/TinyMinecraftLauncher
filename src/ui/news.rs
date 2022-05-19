use druid::{
    widget::{Button, Flex, Label, List, Scroll},
    Widget, WidgetExt,
};

use crate::AppState;

pub fn build_news_widget() -> impl Widget<AppState> {
    Scroll::new(List::new(|| {
        Flex::row()
            .with_child(Label::new(|(title, _url): &(String, String), _env: &_| {
                title.to_owned()
            }))
            .with_child(
                Button::new("Read").on_click(|_ctx, (_title, url), _env| open::that(url).unwrap()),
            )
    }))
    .lens(AppState::news)
}
