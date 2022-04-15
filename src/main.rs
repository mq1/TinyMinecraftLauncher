use anyhow::Result;

slint::include_modules!();

fn main() -> Result<()> {
    let ui = AppWindow::new();

    let news = mc::news::get_minecraft_news(None)?;
    let titles = news.article_grid.into_iter().map(|article| slint::SharedString::from(article.default_tile.title)).collect::<Vec<slint::SharedString>>();
    let titles = std::rc::Rc::new(slint::VecModel::from(titles));
    ui.set_news(titles.clone().into());

    ui.run();
    Ok(())
}
