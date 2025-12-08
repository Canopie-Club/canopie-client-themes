use maud::{Markup, html};

use crate::{
    components::{Asset, AssetOptions, Formatter},
    types::tiptap::TipTapNode,
};

pub fn youtube(content: &TipTapNode, formatter: &Formatter) -> Markup {
    let src_id: Option<&str> = content
        .attrs
        .as_ref() // Option<&Value>
        .and_then(|v| v.as_object()) // Option<&Map<String, Value>>
        .and_then(|map| map.get("videoId")) // Option<&Value>
        .and_then(|v| v.as_str());

    formatter.add_asset(Asset::Script(AssetOptions::new("youtube")));
    println!("YouTube video: {:?}", content);

    if src_id.is_none() {
        return html! {
            // div class="relative w-full h-0 pb-[56.25%] overflow-hidden rounded-xl shadow-lg group cursor-pointer"
            //     data-video-id=(src_id)
            //     onclick="loadYoutubeIframe(this)"
            // {
            //     // Thumbnail
            //     img src=("https://img.youtube.com/vi/invalid/hqdefault.jpg")
            //         class="absolute top-0 left-0 w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
            //         alt="YouTube thumbnail";
            // }
        };
    };

    let video_id = src_id.unwrap_or("invalid");

    // TODO: Implement thumbnail loading logic (Smaller thumbnail for smaller screens)
    // let thumbnail = format!("https://img.youtube.com/vi/{}/hqdefault.jpg", video_id);
    let thumbnail = format!("https://img.youtube.com/vi/{}/maxresdefault.jpg", video_id);

    html! {
        div class="relative w-full h-0 pb-[56.25%] overflow-hidden rounded shadow-lg group cursor-pointer"
            data-video-id=(video_id)
            onclick="loadYoutubeIframe(this)"
        {
            // Thumbnail
            img src=(thumbnail)
                class="absolute top-0 left-0 w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
                alt="YouTube thumbnail" {}

            // Overlay gradient
            div class="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-colors duration-300" {}

            // Play button
            div class="youtube-icon grayscale group-hover:grayscale-0" {}
        }
    }
}
