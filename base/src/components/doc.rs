use maud::{Markup, html};

use crate::{components::Formatter, types::tiptap::TipTapNode};

pub fn doc(content: &TipTapNode, formatter: &Formatter) -> Markup {
    html! {
        div class="doc" {
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
