use stylist::yew::styled_component;
use yew::prelude::*;

use crate::{component::app::App, context::theme_provider::ThemeProvider};

#[styled_component(Root)]
pub fn root() -> Html {
    html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
    }
}
