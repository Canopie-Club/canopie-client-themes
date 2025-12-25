use canopie_utils::{
    components::{Asset, AssetOptions, Formatter},
    types::tiptap::TipTapNode,
};
use maud::{Markup, html};
use regex::Regex;

pub fn album_cover(content: &TipTapNode, formatter: &Formatter) -> Markup {
    let src: Option<&str> = content
        .attrs
        .as_ref() // Option<&Value>
        .and_then(|v| v.as_object()) // Option<&Map<String, Value>>
        .and_then(|map| map.get("src")) // Option<&Value>
        .and_then(|v| v.as_str());

    formatter.add_asset(Asset::Style(AssetOptions::new("deco-cd")));

    let src_str = src.unwrap_or("");

    println!("Album cover source: {}", src_str);

    let re = Regex::new(r"^/_f/[^/]+").unwrap();
    let src_path = re.replace(src_str, "/_f");

    html! {
        div class="deco cd" {
            div class="cd-front" style={"background-image: url('"(src_path)"'"} {}
            div class="cd-case" {}

        }
    }
}
