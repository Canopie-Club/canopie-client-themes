use canopie_utils::{
    components::Formatter,
    header::Header,
    models::Website,
    renderer::PageResponse,
};
use maud::html;

use crate::{AtlasThemeConfig, NavItem};
use crate::templates::page::page_layout;

pub fn atlas_not_found(
    config: &AtlasThemeConfig,
    website: &Website,
    nav_items: &[NavItem],
    headers: &mut Header,
    formatter: Formatter,
) -> PageResponse {
    let content = html! {
        div class="atlas-card p-8" {
            p class="text-sm uppercase tracking-[0.2em] atlas-muted" { "404" }
            h2 class="mt-3 text-2xl font-semibold" { "Page not found" }
            p class="mt-3 atlas-muted" { "That route is missing or unpublished. Try another page from the navigation." }
            a class="mt-6 inline-flex items-center gap-2 rounded-full bg-slate-900 px-5 py-2.5 text-sm font-semibold text-white" href="/" {
                "Back home"
                span aria-hidden="true" { "→" }
            }
        }
    };

    let body = page_layout(
        config,
        website,
        "/404",
        "Page not found",
        nav_items,
        content,
        headers,
    );

    PageResponse::not_found("Page not found".to_string(), body, formatter)
}
