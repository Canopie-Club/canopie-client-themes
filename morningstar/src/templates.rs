use canopie_client_theme_base::header::Header;
use maud::{Markup, html};

use crate::{
    components::bordered::{BorderWidth, Counts, Edge, Props as BorderedProps, bordered_component},
    sections::section_home,
};

pub fn home(content: Markup, headers: &mut Header) -> Markup {
    headers.add_asset(
        html!(link rel="stylesheet" href="/_canopie/static/styles/theme/morningstar/index.css";),
    );
    html! {
        body class="text-gray-900 font-sans" {
            main class="prose lg:prose-lg" {
                (section_home())
                (content)
            }
            (contact_section())
        }
    }
}

pub fn home_section(path: &str, title: String, content: Markup) -> Markup {
    let id = path.replace("/", "_");
    html! {
        section id=(id) page-title=(title) {
            (content)
        }
    }
}

pub fn contact_section() -> Markup {
    let mut bordered_props = BorderedProps::default();
    bordered_props.seed = "contact".to_string();
    bordered_props.intensity = (0.75, 0.75);
    bordered_props.goo = 1.0;
    bordered_props.id_card = "bigger".to_string();
    bordered_props.color = "#e5aaa7".to_string();
    bordered_props.counts = Some(Counts::XY(30, 15));
    bordered_props.border_width = BorderWidth::Single(0.8);
    bordered_props.edges = vec![Edge::Top];
    bordered_props.width_variance = (4.3, 1.2);

    let bordered_content = bordered_component(
        1200.0,
        76.0,
        None,
        html! {
            div class="page-background" {
                div class="page-content" {
                    div class="page-title" {}
                    p {
                        ("For booking, press enquiries and collaborations please contact: ")
                        a href="mailto:info@morningstarmusic.club"{
                            ("info@morningstarmusic.club")
                        }
                    }
                }
            }
        },
        Some(bordered_props),
    );

    html! {
        div class="contact-container -mt-8" {
            div class="page-container" {
                (bordered_content)
            }
        }
    }
}

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
            link rel="stylesheet" href="/_canopie/static/styles/theme/morningstar/index.css";
            link rel="stylesheet" href="/_canopie/static/styles/theme/morningstar/single-page.css";
            @if path == "/liminal-zone" {
                link rel="stylesheet" href="/_canopie/static/styles/theme/morningstar/liminal-zone.css";
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
