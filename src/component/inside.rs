use stylist::yew::styled_component;
use yew::prelude::*;

use crate::context::{theme_kind::ThemeKind, theme_provider::use_theme};

#[styled_component(Inside)]
pub fn inside() -> Html {
    let theme = use_theme();

    let theme_str = match theme.kind() {
        ThemeKind::Light => "Dark Theme",
        ThemeKind::Dark => "Light Theme",
    };

    let other_theme = match theme.kind() {
        ThemeKind::Light => ThemeKind::Dark,
        ThemeKind::Dark => ThemeKind::Light,
    };

    let switch_theme = { move |_| theme.set(other_theme.clone()) };
    html! {
        <div>
            <button class={css!(r#"color: white;
                height: 50px;
                width: 300px;
                font-size: 20px;
                background-color: rgb(88, 164, 255);
                border-radius: 5px;
                border: none;
            "#)} onclick={switch_theme} id="yew-sample-button">{"Switch to "}{theme_str}</button>
        </div>
    }
}
