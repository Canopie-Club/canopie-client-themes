use maud::{Markup, html};
use regex::Regex;

use crate::{components::Formatter, types::tiptap::TipTapNode};

pub fn image_selector(content: &TipTapNode, formatter: &Formatter) -> Markup {
    let src = content
        .attrs
        .as_ref() // Option<&Value>
        .and_then(|v| v.as_object()) // Option<&Map<String, Value>>
        .and_then(|map| map.get("src")) // Option<&Value>
        .and_then(|v| v.as_str()) // Option<&str>
        .unwrap_or("");

    let classes = content
        .attrs
        .as_ref() // Option<&Value>
        .and_then(|v| v.as_object()) // Option<&Map<String, Value>>
        .and_then(|map| map.get("class")) // Option<&Value>
        .and_then(|v| v.as_str()) // Option<&str>
        .unwrap_or("my-4");

    let re = Regex::new(r"^/_f/[^/]+").unwrap();
    let src_path = re.replace(src, "/_f");

    html! {
        img src=(src_path) class=(classes) {
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
