use maud::html;

use crate::{
    components::build_components,
    db::PgPool,
    header::Header,
    models::Website,
    renderer::{PageResult, ThemeRenderer},
    types::tiptap::TipTapNode,
    utils::get_page,
};

pub fn get_default_theme() -> ThemeRenderer {
    ThemeRenderer {
        name: String::from("default"),
        build_content: default_content_builder,
    }
}

pub fn default_content_builder(
    pool: &PgPool,
    header: &mut Header,
    website: Website,
    path: &str,
) -> PageResult {
    // PageResult::Found(html!((path)))

    let page_content = get_page(pool, &website.id, path)
        .map(|result| result.page_content)
        .unwrap_or(None);

    let success = page_content.is_some();

    let (components, formatter) = match page_content {
        Some(page_content) => build_components(page_content.content, Some(website), None),
        None => build_components(
            TipTapNode {
                node_type: "doc".to_string(),
                content: Some(vec![TipTapNode {
                    node_type: "paragraph".to_string(),
                    content: Some(vec![TipTapNode {
                        node_type: "text".to_string(),
                        content: None,
                        attrs: None,
                        text: Some("Page not found! 404".to_string()),
                        marks: None,
                    }]),
                    attrs: None,
                    text: None,
                    marks: None,
                }]),
                attrs: None,
                text: None,
                marks: None,
            },
            Some(website),
            None,
        ),
    };

    let assets = formatter.collect_assets(Some("theme_a"));
    let title = header.title.clone().unwrap_or("".to_string());

    header.add_assets(assets);

    let content = html! {
        body class="bg-white text-gray-900 font-sans p-8" {
            header class="mb-6 border-b pb-4" {
                h1 class="text-4xl font-bold text-blue-600" { (title) }
                p class="text-sm text-gray-500 italic" { "Theme A" }
            }
            main class="prose lg:prose-lg" {
                (components)
            }
            footer class="mt-10 text-xs text-gray-500" {
                "This is Theme A Footer"
            }
        }
    };

    match success {
        true => PageResult::Found(content),
        false => PageResult::NotFound(content),
    }
}
