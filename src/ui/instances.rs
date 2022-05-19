use druid::{widget::Label, Widget};

use crate::AppState;

pub fn build_instances_widget() -> impl Widget<AppState> {
    Label::new("Instances")
}
