use stylist::yew::{styled_component, Global};
use yew::prelude::*;

use crate::{
    component::inside::Inside,
    context::{theme_kind::ThemeKind, theme_provider::use_theme},
};

#[styled_component(App)]
pub fn app() -> Html {
    let theme = use_theme();

    let theme_str = match theme.kind() {
        ThemeKind::Light => "light theme",
        ThemeKind::Dark => "dark theme",
    };

    html! {
        <>
        <Global css={css!(
            r#"
             html, body {
                        font-family: sans-serif;

                        padding: 0;
                        margin: 0;

                        display: flex;
                        justify-content: center;
                        align-items: center;
                        min-height: 100vh;
                        flex-direction: column;

                        background-color: ${bg};
                        color: ${ft_color};
                    }
            "#,
            bg = theme.background_color.clone(),
            ft_color = theme.font_color.clone(),
            )}/>
            <h1>{"Yew Theming w/ Context"}</h1>
            <div class={css!(
                r#"
                    box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
                    height: 500px;
                    width: 500px;
                    border-radius: 5px;

                    display: flex;
                    justify-content: space-around;
                    align-items: center;

                    padding: 15px;
                    box-sizing: border-box;

                    flex-direction: column;
                    background-color: ${bg};
                "#,
                bg = theme.paper_color.clone()
            )} id="yew-sample-content">
                {"You are now using the "}{theme_str}{"!"}

            <Inside />
            </div>
        </>
    }
}
