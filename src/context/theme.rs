use std::sync::Arc;

type ImString = Arc<str>;

#[derive(Debug, Clone)]
pub(crate) struct Theme {
    pub font_color: ImString,
    pub background_color: ImString,
    pub paper_color: ImString,
}
