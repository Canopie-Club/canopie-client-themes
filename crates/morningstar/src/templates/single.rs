use canopie_macros::ThemeConfig;
use canopie_utils::header::Header;
use maud::{Markup, html};
use serde::{Deserialize, Serialize};

use crate::components::bordered::{
    BorderWidth, Counts, Edge, Props as BorderedProps, bordered_component,
};

#[derive(Serialize, Deserialize, Debug, ThemeConfig)]
pub struct SinglePageConfig {}

pub fn single_page(path: &str, content: Markup, headers: &mut Header) -> Markup {
    let mut bordered_props = BorderedProps::default();
    bordered_props.seed = path.to_string();
    // bordered_props.intensity = (0.1, 0.2);
    bordered_props.intensity = (0.03, 0.06);
    bordered_props.goo = 1.0;
    bordered_props.id_card = "bigger".to_string();
    bordered_props.color = "#e5aaa7".to_string();
    bordered_props.counts = Some(Counts::XY(20, 30));
    bordered_props.border_width = BorderWidth::WidthHeight(0.06, 0.01);
    bordered_props.edges = vec![Edge::Top, Edge::Bottom, Edge::Left, Edge::Right];
    bordered_props.width_variance = (0.8, 0.8);

    let bordered_content = bordered_component(
        1000.0,
        3000.0,
        None,
        html! {

            div class="inner"{
                // <!-- <div class="frosted-glass"></div> -->
                div class="inner-content p-8"{
                    (content)
                }
            }
        },
        Some(bordered_props),
    );

    headers.add_asset(
        html!{
            link rel="preconnect" href="https://fonts.googleapis.com" crossorigin="";
            link rel="preconnect" href="https://fonts.gstatic.com";
            link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Cabin+Sketch:wght@400;700&display=swap";
            link rel="stylesheet" href="/_canopie/static/styles/index.css";
            link rel="stylesheet" href="/_canopie/static/styles/single-page.css";
            @if path == "/liminal-zone" {
                link rel="stylesheet" href="/_canopie/static/styles/liminal-zone.css";
            }
        },
    );

    html! {
        body class="text-gray-900 font-sans" {
            div class="layout-epk"{
                div class="background-image"{}
                div class="header"{
                    div class="header-content"{
                        a href="/" { ("Morningstar Music Club") }
                    }
                }
                div class="layout-content"{
                    (bordered_content)
                    // <DecoPageBorder
                    //     :seed="route.path"
                    //     :intensity="[0.03, 0.06]"
                    //     :edges="['top', 'bottom', 'left', 'right']"
                    //     color="#e5aaa7"
                    //     :counts="counts"
                    //     :border-width="borderWidth"
                    //     :width-variance="[0.5, 1.12]"
                    // >
                    // </DecoPageBorder>
                }
            }
        }
    }
}
