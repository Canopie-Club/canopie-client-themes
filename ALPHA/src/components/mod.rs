use std::{cell::RefCell, collections::HashMap, default, sync::Arc};

use maud::{Markup, html};

use crate::{models::Website, types::tiptap::TipTapNode};

pub mod doc;
pub mod image_selector;
pub mod prose;
pub mod video;

#[derive(Clone, Debug)]
pub struct AssetOptions {
    pub path: String,
    pub defer: bool,
    pub _async: bool,
}

impl Default for AssetOptions {
    fn default() -> Self {
        Self {
            path: String::new(),
            defer: false,
            _async: false,
        }
    }
}

impl AssetOptions {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            defer: false,
            _async: false,
        }
    }

    pub fn with_defer(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            defer: true,
            _async: false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Asset {
    Script(AssetOptions),
    Style(AssetOptions),
    Link(AssetOptions),
    Image(AssetOptions),
}

impl Asset {
    pub fn get_options(&self) -> &AssetOptions {
        match self {
            Asset::Script(options) => options,
            Asset::Style(options) => options,
            Asset::Link(options) => options,
            Asset::Image(options) => options,
        }
    }
}

#[derive(Clone)]
pub struct Formatter {
    inner_map: HashMap<String, fn(&TipTapNode, &Formatter) -> Markup>,
    website: Option<Website>,
    assets: RefCell<HashMap<String, Asset>>,
}

impl Formatter {
    pub fn new(website: Option<Website>) -> Self {
        Self {
            inner_map: HashMap::new(),
            website,
            assets: RefCell::new(HashMap::new()),
        }
    }

    pub fn default(website: Option<Website>) -> Self {
        let mut formatter = Self::new(website);
        formatter.safe_insert("doc", doc::doc);
        formatter.safe_insert("paragraph", prose::paragraph);
        formatter.safe_insert("text", prose::text);
        formatter.safe_insert("heading", prose::heading);
        formatter.safe_insert("imageSelector", image_selector::image_selector);
        formatter.safe_insert("horizontalRule".to_string(), prose::horizontal_rule);
        formatter.safe_insert("bulletList".to_string(), prose::unordered_list);
        formatter.safe_insert("unorderedList".to_string(), prose::unordered_list);
        formatter.safe_insert("orderedList".to_string(), prose::ordered_list);
        formatter.safe_insert("listItem".to_string(), prose::list_item);
        formatter.safe_insert("blockquote".to_string(), prose::blockquote);
        formatter
    }

    pub fn insert(&mut self, key: impl Into<String>, f: fn(&TipTapNode, &Formatter) -> Markup) {
        self.inner_map.insert(key.into(), f);
    }

    pub fn safe_insert(
        &mut self,
        key: impl Into<String>,
        f: fn(&TipTapNode, &Formatter) -> Markup,
    ) {
        self.inner_map.entry(key.into()).or_insert(f);
    }

    pub fn add_asset(&self, asset: Asset) {
        let path = match asset.clone() {
            Asset::Script(options) => options.path,
            Asset::Style(options) => options.path,
            Asset::Link(options) => options.path,
            Asset::Image(options) => options.path,
        };

        self.assets.borrow_mut().insert(path, asset);
    }

    pub fn list_assets(&self) -> Vec<Asset> {
        self.assets.borrow().values().cloned().collect()
    }

    pub fn add_assets_from(&self, formatter: &Formatter) {
        for asset in formatter.list_assets() {
            self.add_asset(asset);
        }
    }

    pub fn collect_assets(&self, theme_key: Option<impl Into<String>>) -> Vec<Markup> {
        let assets: Vec<Asset> = self.assets.borrow().values().cloned().collect();

        let theme_key = match theme_key {
            Some(key) => key.into(),
            None => "_generic".to_string(),
        };

        let mut markup = Vec::new();

        for asset in assets {
            let mut path = asset.get_options().path.clone();

            match asset {
                Asset::Script(options) => {
                    markup.push(html! {
                        script src=(format!("/_canopie/static/javascript/{}.js", path)) {
                        }
                    });
                }
                Asset::Style(options) => {
                    markup.push(html! {
                        link rel="stylesheet" href=(format!("/_canopie/static/styles/{}.css", path));
                    });
                }
                Asset::Link(options) => {
                    markup.push(html! {
                        link rel="preload" href=(path);
                    });
                }
                Asset::Image(options) => {
                    markup.push(html! {
                        img src=(path) {
                        }
                    });
                }
            }
        }

        markup
    }

    pub fn get(&self, key: &str) -> Option<&fn(&TipTapNode, &Formatter) -> Markup> {
        let tiptap_formatter = self.inner_map.get(key);
        if tiptap_formatter.is_none() {
            println!("Formatter not found for key: {}", key);
            return None;
        }
        tiptap_formatter
    }
}

pub fn build_components(
    tiptap: TipTapNode,
    website: Option<Website>,
    formatter: Option<Formatter>,
) -> (Markup, Formatter) {
    let mut formatter = formatter.unwrap_or(Formatter::new(website));
    formatter.safe_insert("doc", doc::doc);
    formatter.safe_insert("paragraph", prose::paragraph);
    formatter.safe_insert("text", prose::text);
    formatter.safe_insert("heading", prose::heading);
    formatter.safe_insert("imageSelector", image_selector::image_selector);
    formatter.safe_insert("horizontalRule".to_string(), prose::horizontal_rule);
    formatter.safe_insert("bulletList".to_string(), prose::unordered_list);
    formatter.safe_insert("unorderedList".to_string(), prose::unordered_list);
    formatter.safe_insert("orderedList".to_string(), prose::ordered_list);
    formatter.safe_insert("listItem".to_string(), prose::list_item);
    formatter.safe_insert("youtube".to_string(), video::youtube);
    // formatter.insert("blockquote".to_string(), prose::blockquote);
    // formatter.insert("code_block".to_string(), prose::code_block);
    // formatter.insert("image".to_string(), prose::image);
    // formatter.insert("link".to_string(), prose::link);
    // formatter.insert("code".to_string(), prose::code);
    // formatter.insert("em".to_string(), prose::em);
    // formatter.insert("strong".to_string(), prose::strong);
    // formatter.insert("strike".to_string(), prose::strike);
    // formatter.insert("sub".to_string(), prose::sub);
    // formatter.insert("sup".to_string(), prose::sup);

    let html = html! {
        div class="" {
            @match tiptap.content.as_ref() {
                Some(inner_content) => {
                    @for node in inner_content {
                        @match formatter.get(node.node_type.as_str()) {
                            Some(formatter_function) => {
                                (formatter_function(&node, &formatter))
                            }
                            None => {

                            }
                        }
                    }
                }
                None => {

                }
            }
        }
    };

    (html, formatter)
}
