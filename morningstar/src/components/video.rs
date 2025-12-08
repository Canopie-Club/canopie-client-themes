use canopie_client_theme_base::{
    components::{Asset, AssetOptions, Formatter},
    types::tiptap::TipTapNode,
};
use maud::{Markup, html};

use crate::components::bordered::{BorderWidth, Counts, Props, bordered_component};

pub fn youtube(content: &TipTapNode, formatter: &Formatter) -> Markup {
    let src_id: Option<&str> = content
        .attrs
        .as_ref() // Option<&Value>
        .and_then(|v| v.as_object()) // Option<&Map<String, Value>>
        .and_then(|map| map.get("videoId")) // Option<&Value>
        .and_then(|v| v.as_str());

    formatter.add_asset(Asset::Script(AssetOptions::new("youtube")));

    let mut bordered_props = Props::default();
    bordered_props.seed = "src_id".to_string();
    bordered_props.intensity = (0.05, 0.05);
    bordered_props.goo = 1.0;
    bordered_props.id_card = "bigger".to_string();
    bordered_props.color = "#e5aaa7".to_string();
    bordered_props.counts = Some(Counts::XY(30, 15));
    bordered_props.border_width = BorderWidth::Single(0.2);
    bordered_props.width_variance = (1.3, 1.2);

    if src_id.is_none() {
        return bordered_component(
            520.0,
            292.5,
            None,
            html! {
            // div class="relative w-full h-0 pb-[56.25%] overflow-hidden rounded-xl shadow-lg group cursor-pointer"
            //     data-video-id=(src_id)
            //     onclick="loadYoutubeIframe(this)"
            // {
            //     // Thumbnail
            //     img src=("https://img.youtube.com/vi/invalid/hqdefault.jpg")
            //         class="absolute top-0 left-0 w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
            //         alt="YouTube thumbnail";
            // }
            },
            Some(bordered_props),
        );
    };

    let video_id = src_id.unwrap_or("invalid");

    bordered_props.seed = video_id.to_string();

    // TODO: Implement thumbnail loading logic (Smaller thumbnail for smaller screens)
    // let thumbnail = format!("https://img.youtube.com/vi/{}/hqdefault.jpg", video_id);
    let thumbnail = format!("https://img.youtube.com/vi/{}/maxresdefault.jpg", video_id);

    html! {
        div class="video-container" {
            (bordered_component(
                520.0,
                292.5,
                None,
                html! {
                    div class="w-[450px] md:w-[520px]" {
                        div class="relative w-full aspect-video max-w-[450px] md:max-w-[520px] overflow-hidden rounded shadow-lg group cursor-pointer"
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
                },
                Some(bordered_props),
            ))
        }
    }
}
