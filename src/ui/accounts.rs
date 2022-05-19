use druid::{widget::Label, Widget};

use crate::AppState;

pub fn build_accounts_widget() -> impl Widget<AppState> {
    Label::new("Accounts")
}
