use maud::Markup;

use crate::{db::PgPool, header::Header, models::Website};

pub struct ThemeRenderer {
    pub name: String,
    pub build_content: fn(&PgPool, &mut Header, Website, &str) -> (bool, Markup),
}

impl ThemeRenderer {
    const fn new(
        name: String,
        build_content: fn(&PgPool, &mut Header, Website, &str) -> (bool, Markup),
    ) -> Self {
        Self {
            name,
            build_content,
        }
    }
}
