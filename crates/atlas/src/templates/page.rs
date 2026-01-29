use canopie_utils::header::Header;
use maud::{Markup, html};

use crate::{AtlasThemeConfig, NavItem};

pub fn page_layout(
    config: &AtlasThemeConfig,
    website: &canopie_utils::models::Website,
    path: &str,
    page_title: &str,
    nav_items: &[NavItem],
    content: Markup,
    headers: &mut Header,
) -> Markup {
    headers.add_asset(html! {
        link rel="preconnect" href="https://fonts.googleapis.com" crossorigin="";
        link rel="preconnect" href="https://fonts.gstatic.com";
        link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=DM+Sans:wght@400;500;600;700&display=swap";
    });

    let container_class = container_width_class(config.layout_width.as_str());
    let header_class = header_style_class(config.header_style.as_str());
    let style_vars = format!(
        "--atlas-surface: {}; --atlas-text: {}; --atlas-accent: {}; --atlas-muted: {};",
        config.surface_color, config.text_color, config.accent_color, config.muted_color
    );

    let site_title = if config.site_title.trim().is_empty() {
        website.title.clone()
    } else {
        config.site_title.clone()
    };

    html! {
        body class="atlas atlas-shell" style=(style_vars) {
            header class=(format!("atlas-header atlas-border {}", header_class)) {
                div class=(format!("atlas-container atlas-header-inner {}", container_class)) {
                    a href="/" class="atlas-brand" {
                        @if let Some(logo) = &config.logo {
                            img class="atlas-logo" src=(logo) alt="Site logo";
                        }
                        span { (site_title) }
                    }
                    nav class="atlas-nav" {
                        @for item in nav_items {
                            @if item.external {
                                a class=(nav_link_class(item.active)) href=(item.href) target="_blank" rel="noopener noreferrer" { (item.label) }
                            } @else {
                                a class=(nav_link_class(item.active)) href=(item.href) { (item.label) }
                            }
                        }
                    }
                    @if !config.nav_cta_label.trim().is_empty() && !config.nav_cta_url.trim().is_empty() {
                        a class="atlas-cta" href=(config.nav_cta_url) { (config.nav_cta_label) }
                    }
                }
            }
            main class="atlas-main" {
                @if config.show_page_hero {
                    section class="atlas-hero atlas-border" {
                        div class=(format!("atlas-container atlas-hero-inner {}", container_class)) {
                            div class="atlas-hero-content" {
                                @if !config.hero_overline.trim().is_empty() {
                                    span class="atlas-pill" { (config.hero_overline) }
                                }
                                h1 class="atlas-hero-title" { (page_title) }
                                @if !config.site_tagline.trim().is_empty() {
                                    p class="atlas-hero-lede atlas-muted" { (config.site_tagline) }
                                }
                                @if !config.hero_subtitle.trim().is_empty() {
                                    p class="atlas-hero-subtitle atlas-muted" { (config.hero_subtitle) }
                                }
                                @if !config.nav_cta_label.trim().is_empty() && !config.nav_cta_url.trim().is_empty() {
                                    a class="atlas-cta atlas-cta-strong" href=(config.nav_cta_url) {
                                        (config.nav_cta_label)
                                        span aria-hidden="true" { "→" }
                                    }
                                }
                            }
                            div class="atlas-card atlas-hero-card" {
                                @if let Some(hero_image) = &config.hero_image {
                                    img class="atlas-hero-image" src=(hero_image) alt="Hero image";
                                } @else {
                                    div class="atlas-card-stack" {
                                        div class="atlas-card-meta" {
                                            span class="atlas-overline atlas-muted" { (path) }
                                            span class="atlas-overline atlas-muted" { "Atlas" }
                                        }
                                        div class="atlas-card-copy" {
                                            h3 { "Page overview" }
                                            p class="atlas-muted" { "Each route renders independently for fast, focused navigation." }
                                        }
                                        div class="atlas-card-grid" {
                                            div class="atlas-card-mini atlas-border" {
                                                p class="atlas-card-title" { "Readable content" }
                                                p class="atlas-muted" { "Balanced line lengths and generous spacing." }
                                            }
                                            div class="atlas-card-mini atlas-border" {
                                                p class="atlas-card-title" { "Tailwind-ready" }
                                                p class="atlas-muted" { "Utility classes guide layout and rhythm." }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                section class=(format!("atlas-container atlas-content {}", container_class)) {
                    div class="content-stack" { (content) }
                }
            }
            @if config.show_footer {
                footer class="atlas-footer atlas-border" {
                    div class=(format!("atlas-container atlas-footer-inner {}", container_class)) {
                        div class="atlas-footer-copy" {
                            p class="atlas-footer-title" { (site_title) }
                            p class="atlas-muted" { (config.footer_blurb) }
                        }
                        div class="atlas-footer-contact" {
                            p class="atlas-muted" { "Contact" }
                            a class="atlas-footer-link" href=(format!("mailto:{}", config.footer_email)) { (config.footer_email) }
                        }
                    }
                }
            }
        }
    }
}

fn container_width_class(value: &str) -> &'static str {
    match value {
        "narrow" => "atlas-container-narrow",
        "wide" => "atlas-container-wide",
        "full" => "atlas-container-full",
        _ => "atlas-container-wide",
    }
}

fn header_style_class(value: &str) -> &'static str {
    match value {
        "solid" => "atlas-header-solid",
        "glass" => "atlas-glass",
        _ => "atlas-header-solid",
    }
}

fn nav_link_class(active: bool) -> &'static str {
    if active {
        "atlas-nav-link is-active"
    } else {
        "atlas-nav-link"
    }
}
