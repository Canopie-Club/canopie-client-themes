use maud::{Markup, html};

use crate::{
    db::PgPool,
    lookup::theme::{get_menus, get_page, get_page_from_id},
    models::{Menu, MenuItem, Website},
    templates::{
        components::{Asset, AssetOptions, Formatter, build_components},
        headers::Header,
        themes::{
            PageResponse, PageResult,
            morningstar::{
                components::{album_cover::album_cover, video::youtube},
                sections::render_section,
                templates::{home, home_section, single_page},
            },
        },
    },
};

mod components;
mod sections;
mod templates;

impl PageResponse {
    pub fn morningstar_not_found(formatter: Formatter, headers: &mut Header) -> Self {
        Self {
            title: "404 Page not found".to_string(),
            result: PageResult::NotFound(single_page(
                "/404",
                html! { ("404 Page not found!!") },
                headers,
            )),
            formatter,
        }
    }
}

fn build_content_for_menu_pages(
    pool: &PgPool,
    path: &str,
    website_id: &String,
    menu_input: &(Menu, Vec<MenuItem>),
    formatter: Formatter,
    headers: &mut Header,
) -> PageResponse {
    let (menu, menu_items) = menu_input;

    let page_result = get_page(pool, website_id, path);
    let page_content = page_result
        .clone()
        .map(|result| result.page_content)
        .unwrap_or(None);

    if page_result.is_none() || page_content.is_none() {
        return PageResponse::morningstar_not_found(formatter, headers);
    }

    let page = page_result.unwrap().page;

    let page_content = page_content.unwrap();
    let given_page_id = page.id.to_string();

    let page_in_menu = menu_items
        .iter()
        .any(|item| item.page_id.clone().unwrap_or("NO ID".to_string()) == given_page_id);

    if !page_in_menu {
        let (components, formatter) =
            build_components(page_content.content, None, Some(formatter.clone()));
        return PageResponse::new(
            page.title,
            single_page(path, components, headers),
            formatter,
        );
    }

    let mut component_list: Vec<Markup> = vec![];

    for item in menu_items {
        let item_id = item.page_id.clone();

        // TODO: Maybe not the most elegant way of doing this:
        let item_details = if item_id.unwrap_or("NO ID".to_string()) == given_page_id {
            Some((page.clone(), page_content.clone()))
        } else if let Some(page_id) = item.page_id.clone() {
            let page_fetch = get_page_from_id(pool, &page_id);
            let (item_page, item_page_content) = if page_fetch.is_none() {
                println!("Content not found for page ID: {}", page_id);
                (page.clone(), None)
            } else {
                page_fetch.unwrap()
            };

            match item_page_content {
                Some(item_page_content) => Some((item_page, item_page_content)),
                None => {
                    println!("Content not found for page: /{}", item_page.slug);
                    None
                }
            }
        } else {
            None
        };

        let (components, new_formatter) = match item_details {
            Some((item_page, item_page_content)) => {
                let (components, formatter) =
                    build_components(item_page_content.content, None, Some(formatter.clone()));
                let path = item_page.slug.as_str();

                let title = item_page.title.clone();
                (
                    home_section(path, title, render_section(path, components)),
                    formatter,
                )
            }
            None => (html! {}, formatter.clone()),
        };

        component_list.push(components);

        formatter.add_assets_from(&new_formatter);
    }

    formatter.add_asset(Asset::Style(AssetOptions::new("home")));
    formatter.add_asset(Asset::Script(AssetOptions::new("spa")));
    formatter.add_asset(Asset::Script(AssetOptions::new("header-rotation")));

    PageResponse::new(
        page.title,
        home(
            html! {
                @for component in component_list {
                    (component)
                }
            },
            headers,
        ),
        formatter,
    )
}

pub fn morningstar(
    pool: &PgPool,
    headers: &mut Header,
    website: Website,
    path: &str,
) -> (bool, Markup) {
    let menus = get_menus(pool, &website.id);

    let main_menu = menus.iter().find(|menu| menu.0.name == "Main");
    let mut formatter = Formatter::default(Some(website.clone()));

    formatter.add_asset(Asset::Style(AssetOptions::new("index")));

    formatter.insert("youtube", youtube);
    formatter.insert("albumCover", album_cover);

    let page_response = match main_menu {
        Some(menu) => {
            build_content_for_menu_pages(pool, path, &website.id, menu, formatter, headers)
        }
        None => PageResponse::morningstar_not_found(formatter, headers),
    };

    let assets = page_response.formatter.collect_assets(Some("morningstar"));

    let (success, content) = match page_response.result {
        PageResult::Found(content) => (true, content),
        PageResult::NotFound(content) => (false, content),
    };

    headers.add_assets(assets);
    headers.set_title(page_response.title.as_str());

    (success, content)
}
