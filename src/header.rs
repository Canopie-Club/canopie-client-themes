use maud::{DOCTYPE, Markup, PreEscaped, html};

pub struct Header {
    pub title: Option<String>,
    style_tags: Vec<PreEscaped<String>>,
    assets: Vec<PreEscaped<String>>,
    favicon_version: Option<String>,
}

impl Header {
    pub fn new() -> Self {
        Header {
            title: None,
            style_tags: Vec::new(),
            assets: Vec::new(),
            favicon_version: None,
        }
    }

    pub fn new_with_title(title: &str) -> Self {
        Header {
            title: Some(title.to_string()),
            style_tags: Vec::new(),
            assets: Vec::new(),
            favicon_version: None,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_string());
    }

    pub fn add_style_tag(&mut self, tag: PreEscaped<String>) {
        self.style_tags.push(tag);
    }

    pub fn add_asset(&mut self, asset: PreEscaped<String>) {
        self.assets.push(asset);
    }

    pub fn add_assets(&mut self, assets: Vec<PreEscaped<String>>) {
        self.assets.extend(assets);
    }

    pub fn set_favicon_version(&mut self, version: &str) {
        self.favicon_version = Some(version.to_string());
    }

    pub fn render(&self, content: &Markup) -> Markup {
        let favicon = if let Some(version) = &self.favicon_version {
            format!("/favicon.ico?v={}", version)
        } else {
            "/favicon.ico".to_string()
        };

        html! {
            (DOCTYPE)
            head {
                meta charset="utf-8";
                @if let Some(title) = &self.title {
                    title { (title) }
                }
                link rel="icon" type="image/x-icon" href=(favicon);
                // link rel="stylesheet" href="/static/css/tailwind.css";
                @for tag in &self.style_tags {
                    style {(tag)}
                }
                @for asset in &self.assets {
                    (asset)
                }
            }
            (content)
        }
    }
}
