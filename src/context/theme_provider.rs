use stylist::yew::styled_component;
use yew::prelude::*;

use crate::context::{theme_context::ThemeContext, theme_kind::ThemeKind};

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ThemeProviderProps {
    pub children: Children,
}

#[styled_component(ThemeProvider)]
pub(crate) fn theme_provider(props: &ThemeProviderProps) -> Html {
    let theme_kind = use_state(|| ThemeKind::Dark);

    let theme_ctx = ThemeContext::new(theme_kind);

    html! {
        <ContextProvider<ThemeContext> context={theme_ctx}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

#[hook]
pub(crate) fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
        .expect("The context should return an instance given that we are using a context provider")
}
