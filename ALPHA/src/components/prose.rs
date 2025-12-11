use maud::{Markup, html};

use crate::{
    components::{Asset, AssetOptions, Formatter},
    types::tiptap::TipTapNode,
};

pub fn paragraph(content: &TipTapNode, formatter: &Formatter) -> Markup {
    formatter.add_asset(Asset::Script(AssetOptions::new("fish")));
    formatter.add_asset(Asset::Script(AssetOptions::new("hello")));
    html! {
        p class="mb-4" {
            @if content.text.is_some() {
                (content.text.as_ref().unwrap())
            }

            @match content.content.as_ref() {
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
    }
}

pub fn text(content: &TipTapNode, formatter: &Formatter) -> Markup {
    html! {
        span class="" {
            @if content.text.is_some() {
                (content.text.as_ref().unwrap())
            }

            @match content.content.as_ref() {
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
    }
}

pub fn heading(content: &TipTapNode, formatter: &Formatter) -> Markup {
    let level: i64 = content
        .attrs
        .as_ref() // Option<&Value>
        .and_then(|v| v.as_object()) // Option<&Map<String, Value>>
        .and_then(|map| map.get("level")) // Option<&Value>
        .and_then(|v| v.as_i64()) // Option<i64>
        .unwrap_or(1);

    let inner = || {
        html! {
            @if let Some(text) = &content.text {
                (text)
            }

            @if let Some(inner_content) = &content.content {
                @for node in inner_content {
                    @if let Some(formatter_function) = formatter.get(node.node_type.as_str()) {
                        (formatter_function(&node, &formatter))
                    }
                }
            }
        }
    };

    html! {
        @match level {
            1 => h1 class="text-2xl font-bold" { (inner()) },
            2 => h2 class="text-xl font-bold" { (inner()) },
            3 => h3 class="text-lg font-bold" { (inner()) },
            4 => h4 class="text-base font-bold" { (inner()) },
            5 => h5 class="text-sm font-bold" { (inner()) },
            6 => h6 class="text-xs font-bold" { (inner()) },
            _ => div class="text-base font-bold" { (inner()) },
        }
    }
}

pub fn horizontal_rule(content: &TipTapNode, formatter: &Formatter) -> Markup {
    html! {
        hr class="my-4" {}

        @match content.content.as_ref() {
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
}

pub fn ordered_list(content: &TipTapNode, formatter: &Formatter) -> Markup {
    html! {
        ol class="list-decimal pl-4" {
            @if content.text.is_some() {
                (content.text.as_ref().unwrap())
            }

            @match content.content.as_ref() {
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

    }
}

pub fn unordered_list(content: &TipTapNode, formatter: &Formatter) -> Markup {
    html! {
        ul class="list-disc pl-4" {

            @if content.text.is_some() {
                (content.text.as_ref().unwrap())
            }

            @match content.content.as_ref() {
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
    }
}

pub fn list_item(content: &TipTapNode, formatter: &Formatter) -> Markup {
    html! {
        li {
            @if content.text.is_some() {
                (content.text.as_ref().unwrap())
            }

            @match content.content.as_ref() {
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

    }
}

pub fn blockquote(content: &TipTapNode, formatter: &Formatter) -> Markup {
    html! {
        blockquote {
            @if content.text.is_some() {
                (content.text.as_ref().unwrap())
            }

            @match content.content.as_ref() {
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

    }
}
