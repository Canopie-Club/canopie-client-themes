use canopie_utils::{
    components::{Asset, AssetOptions, Formatter},
    db::PgPool,
    header::Header,
    models::Website,
    renderer::PageResult,
    theme_utils::get_menus,
};

use crate::{
    components::{album_cover::album_cover, video::youtube},
    templates::{not_found::morningstar_not_found, spa::build_content_for_menu_pages},
};

mod components;
mod sections;
mod templates;

pub fn morningstar(
    pool: &PgPool,
    headers: &mut Header,
    website: Website,
    path: &str,
) -> PageResult {
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
        None => morningstar_not_found(formatter, headers),
    };

    let assets = page_response.formatter.collect_assets(Some("morningstar"));

    headers.add_assets(assets);
    headers.set_title(page_response.title.as_str());

    page_response.result
}
