use maud::Markup;

use crate::{components::Formatter, db::PgPool, header::Header, models::Website};

pub type ContentBuilder = fn(&PgPool, &mut Header, Website, &str) -> PageResult;

#[derive(Debug, Clone)]
pub struct ThemeRenderer {
    pub name: String,
    pub build_content: ContentBuilder,
}

impl ThemeRenderer {
    pub fn new(name: String, build_content: ContentBuilder) -> Self {
        Self {
            name,
            build_content,
        }
    }
}

pub enum PageResult {
    NotFound(Markup),
    Found(Markup),
}

pub struct PageResponse {
    pub title: String,
    pub result: PageResult,
    pub formatter: Formatter,
}

impl PageResponse {
    pub fn new(title: String, content: Markup, formatter: Formatter) -> Self {
        Self {
            title,
            result: PageResult::Found(content),
            formatter,
        }
    }

    pub fn not_found(title: String, content: Markup, formatter: Formatter) -> Self {
        Self {
            title,
            result: PageResult::NotFound(content),
            formatter,
        }
    }
}
