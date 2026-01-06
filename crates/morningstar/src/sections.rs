use maud::{Markup, html};

use crate::components::bordered::{
    BorderWidth, Counts, Edge, Props as BorderedProps, bordered_component,
};

fn main_menu() -> Markup {
    html! {
        div class="menu" {
          div class="menu-bg" {}
          div class="menu-content"{
            a href="/band" page-title="Band" {
              div class="link band" { ("Band") }
            }
            a href="/vids" page-title="Vids" {
              div class="link vids" { ("Vids") }
            }
            a href="/music" page-title="Music" {
              div class="link music" { ("Music") }
            }
            // <!-- <a @click="scrollToSection('live')">
            //   <div class="link live">Live</div>
            // </a> -->
            a href="/contact" page-title="Contact" {
              div class="link contact"{ ("Contact") }
            }
          }
        }
    }
}

pub fn section_home() -> Markup {
    let mut bordered_props = BorderedProps::default();
    bordered_props.seed = "homepage".to_string();
    bordered_props.intensity = (0.1, 0.2);
    bordered_props.goo = 1.0;
    bordered_props.id_card = "bigger".to_string();
    bordered_props.color = "#e5aaa7".to_string();
    bordered_props.counts = Some(Counts::XY(20, 30));
    bordered_props.border_width = BorderWidth::Single(0.08);
    bordered_props.edges = vec![Edge::Bottom, Edge::Right];
    bordered_props.width_variance = (0.8, 0.8);

    let bordered_menu = bordered_component(
        640.0,
        960.0,
        None,
        html! {
            (main_menu())

            div class="sm:h-40 md:h-80 flex items-center justify-center"{
              img src="/_canopie/static/images/names.webp" {}
            }
        },
        Some(bordered_props),
    );

    html! {
        div class="home-container relative" {
            div class="banner-container"{
                div class="banner morning-star" {}
                div class="banner music-club" {}
            }
            div class="hidden sm:block sm:max-w-md md:max-w-screen-sm absolute z-10" {
                //DECOBORDER
                (bordered_menu)

                // div class="sm:h-40 md:h-80 flex items-center justify-center"{
                //   img src="/_canopie/static/images/names.webp" {}
                // }
            }
            div class="block sm:hidden absolute top-0 left-0 z-10 w-full" {
                (main_menu())
            }
            div class="jesse h-full w-full" {
                div class="jesse-image" {}
            }
        }
    }
}

pub fn render_section(section: &str, content: Markup) -> Markup {
    match section {
        "music" => section_music(content),
        "band" => section_band(content),
        "vids" => section_vids(content),
        _ => html! {
            div class="section-default" {
                div class="section-default-content" {
                    (content)
                }
            }
        },
    }
}

fn section_band(content: Markup) -> Markup {
    html! {
        div class="band-header" {}
        div class="section-band" {
            div class="big-block" {}
            div class="section-band-content text-white container m-auto max-w-screen-md" {
                (content)
            }
        }
    }
}

fn section_vids(content: Markup) -> Markup {
    html! {
        div class="section-vids" {
            div class="section-vids-content text-white container m-auto max-w-screen-md" {
                div class="page-title" {}
                (content)
            }
        }
    }
}

fn section_music(content: Markup) -> Markup {
    html! {
        div class="section-music" {
            div class="section-about-content text-white container m-auto max-w-screen-md" {
                div class="page-title" {}
                (content)
            }
        }
    }
}
