pub mod not_found;
pub mod single;
pub mod spa;

use canopie_utils::header::Header;
use maud::{Markup, html};

use crate::{
    components::bordered::{BorderWidth, Counts, Edge, Props as BorderedProps, bordered_component},
    sections::section_home,
};

pub fn home(content: Markup, headers: &mut Header) -> Markup {
    headers.add_asset(html!(link rel="stylesheet" href="/_canopie/static/styles/index.css";));
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
