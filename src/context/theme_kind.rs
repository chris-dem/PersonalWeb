use std::sync::Arc;

use yew::html::ImplicitClone;

use once_cell::sync::Lazy;

use super::theme::Theme;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ThemeKind {
    Dark,
    Light,
}

impl ImplicitClone for ThemeKind {}

impl ThemeKind {
    pub fn current(&self) -> &Theme {
        static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: Arc::from("black"),
            background_color: Arc::from("rgb(237, 244, 255)"),
            paper_color: Arc::from("white"),
        });

        static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: Arc::from("white"),
            background_color: Arc::from("blacl"),
            paper_color: Arc::from("rgb(50, 50, 50)"),
        });

        match self {
            ThemeKind::Dark => &DARK_THEME,
            _ => &LIGHT_THEME,
        }
    }
}
