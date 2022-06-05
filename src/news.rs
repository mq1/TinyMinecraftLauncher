use druid::{
    piet::ImageBuf,
    widget::{Button, Flex, Image, Label, Scroll},
    Widget, WidgetExt,
};

use crate::AppState;

fn build_item(article: mc::news::Article) -> impl Widget<AppState> {
    let image_data = attohttpc::get(article.default_tile.image.image_url)
        .send()
        .unwrap();
    let image_buf = ImageBuf::from_data(&image_data.bytes().unwrap()).unwrap();
    let image = Image::new(image_buf);

    let label = Label::new(article.default_tile.title);

    let read_button = Button::new("Read")
        .on_click(move |_ctx, _data, _env| open::that(article.article_url.as_str()).unwrap());

    Flex::row()
        .with_child(image)
        .with_child(label)
        .with_flex_spacer(1.)
        .with_child(read_button)
        .fix_width(500.)
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

    Scroll::new(col).expand()
}
