use canopie_macros::{ThemeConfig, theme_defaults};
use canopie_utils::{
    components::{Asset, AssetOptions, Formatter},
    db::PgPool,
    header::Header,
    models::Website,
    renderer::PageResult,
    theme_utils::get_menus,
    themes::{
        self, GetThemeOverview, PageThemeOverview, ThemeOverview, ThemeRestriction, ThemeSchema,
    },
};
#[cfg(feature = "embed")]
use canopie_utils::{
    renderer::{GetThemeRenderer, ThemeRenderer},
    resource::embed::ThemeResource,
};
#[cfg(feature = "embed")]
use include_dir::Dir;
use serde::{Deserialize, Serialize};

use crate::{
    components::{album_cover::album_cover, video::youtube},
    templates::{
        not_found::morningstar_not_found,
        single::SinglePageConfig,
        spa::{SpaPageConfig, build_content_for_menu_pages},
    },
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

#[theme_defaults]
#[derive(Serialize, Deserialize, Debug, ThemeConfig)]
pub struct MorningStarConfig {}

pub struct ThemeMorningStar {}

impl GetThemeOverview for ThemeMorningStar {
    fn get_theme_overview() -> ThemeOverview {
        ThemeOverview {
            id: "morningstar".to_string(),
            name: "Morning Star Music Club".to_string(),
            description: Some("Morning Star Music Club".to_string()),
            restricted: ThemeRestriction::Restricted(vec!["morningstar".to_string()]),
            config: MorningStarConfig::schema(),
            page_themes: vec![
                PageThemeOverview {
                    name: "Home Segment".to_string(),
                    config: SpaPageConfig::schema(),
                    default: false,
                },
                PageThemeOverview {
                    name: "Full Page".to_string(),
                    config: SinglePageConfig::schema(),
                    default: true,
                },
            ],
        }
    }
}

#[cfg(feature = "embed")]
impl ThemeResource for ThemeMorningStar {
    fn get_theme_resources() -> Dir<'static> {
        use include_dir::include_dir;

        let theme_dir: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/static");

        theme_dir
    }
    fn attach_theme_resources(
        resources: &mut canopie_utils::resource::embed::Resources,
    ) -> Result<(), String> {
        let id = Self::get_theme_overview().id;
        resources.add_dir(id.as_str(), Self::get_theme_resources());
        Ok(())
    }
}

#[cfg(feature = "embed")]
impl GetThemeRenderer for ThemeMorningStar {
    fn get_theme_renderer() -> ThemeRenderer {
        ThemeRenderer {
            name: Self::get_theme_overview().id,
            build_content: morningstar,
        }
    }
}
