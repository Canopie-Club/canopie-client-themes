use canopie_macros::{ThemeConfig, theme_defaults};
use canopie_utils::themes;
use canopie_utils::{
    components::{Asset, AssetOptions, Formatter, build_components},
    db::PgPool,
    header::Header,
    models::{MenuItem, Website},
    renderer::{PageResponse, PageResult},
    theme_utils::{get_menus, get_page, get_page_from_id},
    themes::{GetThemeOverview, PageThemeOverview, ThemeOverview, ThemeRestriction, ThemeSchema},
    types::tiptap::{empty_tiptap_node, to_tiptap_node},
};

#[cfg(feature = "embed")]
use canopie_utils::{
    renderer::{GetThemeRenderer, ThemeRenderer},
    resource::embed::ThemeResource,
};
#[cfg(feature = "embed")]
use include_dir::Dir;
use serde::{Deserialize, Serialize};

use crate::templates::{not_found::atlas_not_found, page::page_layout};

mod templates;

#[derive(Debug, Clone)]
pub struct NavItem {
    pub label: String,
    pub href: String,
    pub active: bool,
    pub external: bool,
}

#[theme_defaults]
#[derive(Serialize, Deserialize, Debug, ThemeConfig)]
pub struct AtlasThemeConfig {
    #[theme(interface = "Input", default = "Atlas")]
    pub site_title: String,
    #[theme(
        interface = "InputText",
        default = "A flexible, multi-page theme for modern publishing."
    )]
    pub site_tagline: String,
    #[theme(interface = "FileImage")]
    pub logo: Option<String>,
    #[theme(interface = "Checkbox", default = true, width = "Half")]
    pub show_page_hero: bool,
    #[theme(interface = "Input", default = "Insights")]
    pub hero_overline: String,
    #[theme(
        interface = "InputText",
        default = "Thoughtful pages with clear hierarchy and generous spacing."
    )]
    pub hero_subtitle: String,
    #[theme(interface = "FileImage")]
    pub hero_image: Option<String>,
    #[theme(interface = "SelectDropdown", default = "wide", width = "Half")]
    pub layout_width: String,
    #[theme(interface = "SelectDropdown", default = "glass", width = "Half")]
    pub header_style: String,
    #[theme(interface = "SelectColor", default = "#f8fafc", width = "Half")]
    pub surface_color: String,
    #[theme(interface = "SelectColor", default = "#0f172a", width = "Half")]
    pub text_color: String,
    #[theme(interface = "SelectColor", default = "#0ea5e9", width = "Half")]
    pub accent_color: String,
    #[theme(interface = "SelectColor", default = "#64748b", width = "Half")]
    pub muted_color: String,
    #[theme(interface = "Checkbox", default = true, width = "Half")]
    pub show_footer: bool,
    #[theme(
        interface = "InputText",
        default = "Let’s build something calm and clear."
    )]
    pub footer_blurb: String,
    #[theme(interface = "Input", default = "hello@canopie.io")]
    pub footer_email: String,
    #[theme(interface = "Input", default = "Get Updates")]
    pub nav_cta_label: String,
    #[theme(interface = "Input", default = "/contact")]
    pub nav_cta_url: String,
}

impl Default for AtlasThemeConfig {
    fn default() -> Self {
        Self {
            site_icon: None,
            primary_color: "#0ea5e9".to_string(),
            site_title: "Atlas".to_string(),
            site_tagline: "A flexible, multi-page theme for modern publishing.".to_string(),
            logo: None,
            show_page_hero: true,
            hero_overline: "Insights".to_string(),
            hero_subtitle: "Thoughtful pages with clear hierarchy and generous spacing."
                .to_string(),
            hero_image: None,
            layout_width: "wide".to_string(),
            header_style: "glass".to_string(),
            surface_color: "#f8fafc".to_string(),
            text_color: "#0f172a".to_string(),
            accent_color: "#0ea5e9".to_string(),
            muted_color: "#64748b".to_string(),
            show_footer: true,
            footer_blurb: "Let’s build something calm and clear.".to_string(),
            footer_email: "hello@canopie.io".to_string(),
            nav_cta_label: "Get Updates".to_string(),
            nav_cta_url: "/contact".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ThemeConfig)]
pub struct AtlasPageConfig {
    #[theme(interface = "Checkbox", default = true, width = "Half")]
    pub show_hero: bool,
    #[theme(interface = "InputText", default = "")]
    pub hero_overline: String,
    #[theme(interface = "InputTextarea", default = "")]
    pub hero_subtitle: String,
    #[theme(interface = "FileImage")]
    pub hero_image: Option<String>,
    #[theme(interface = "SelectDropdown", default = "split", width = "Half")]
    pub hero_layout: String,
}

pub struct ThemeAtlas {}

impl GetThemeOverview for ThemeAtlas {
    fn get_theme_overview() -> ThemeOverview {
        ThemeOverview {
            id: "atlas".to_string(),
            name: "Atlas".to_string(),
            description: Some("A flexible, multi-page theme with strong hierarchy.".to_string()),
            restricted: ThemeRestriction::None,
            config: AtlasThemeConfig::schema(),
            page_themes: vec![PageThemeOverview {
                name: "Atlas Page".to_string(),
                config: AtlasPageConfig::schema(),
                default: true,
            }],
        }
    }
}

#[cfg(feature = "embed")]
impl ThemeResource for ThemeAtlas {
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
impl GetThemeRenderer for ThemeAtlas {
    fn get_theme_renderer() -> ThemeRenderer {
        ThemeRenderer {
            name: Self::get_theme_overview().id,
            build_content: atlas,
        }
    }
}

pub fn get_theme_overview() -> ThemeOverview {
    ThemeAtlas::get_theme_overview()
}

pub fn atlas(pool: &PgPool, headers: &mut Header, website: Website, path: &str) -> PageResult {
    let theme_config = resolve_theme_config(&website);

    let formatter = Formatter::default(Some(website.clone()));
    formatter.add_asset(Asset::Style(AssetOptions::new("tailwind")));
    formatter.add_asset(Asset::Style(AssetOptions::new("atlas")));

    let menus = get_menus(pool, &website.id);
    let main_menu = menus.iter().find(|menu| menu.0.name == "Main");
    let nav_items = main_menu
        .map(|menu| build_nav_items(pool, &menu.1, path))
        .unwrap_or_default();

    let page_result = get_page(pool, &website.id, path);

    if page_result.is_none() || page_result.as_ref().unwrap().page_content.is_none() {
        let response = atlas_not_found(&theme_config, &website, &nav_items, headers, formatter);
        let assets = response.formatter.collect_assets(Some("atlas"));
        headers.add_assets(assets);
        headers.set_title(response.title.as_str());
        return response.result;
    }

    let page_data = page_result.unwrap();
    let page = page_data.page;
    let page_content = page_data.page_content.unwrap();

    let content = match to_tiptap_node(page_content.content) {
        Ok(content) => content,
        Err(_error) => empty_tiptap_node(Some("Error parsing content")),
    };

    let (components, content_formatter) = build_components(content, None, Some(formatter.clone()));
    formatter.add_assets_from(&content_formatter);

    let body = page_layout(
        &theme_config,
        &website,
        path,
        page.title.as_str(),
        &nav_items,
        components,
        headers,
    );

    let response = PageResponse::new(page.title, body, formatter);
    let assets = response.formatter.collect_assets(Some("atlas"));
    headers.add_assets(assets);
    headers.set_title(response.title.as_str());

    response.result
}

fn resolve_theme_config(website: &Website) -> AtlasThemeConfig {
    if let Some(value) = website.theme_config.clone() {
        AtlasThemeConfig::from_json_value(value).unwrap_or_default()
    } else {
        AtlasThemeConfig::default()
    }
}

fn build_nav_items(pool: &PgPool, menu_items: &[MenuItem], current_path: &str) -> Vec<NavItem> {
    let mut sorted = menu_items.to_vec();
    sorted.sort_by_key(|item| item.order);

    sorted
        .iter()
        .filter_map(|item| {
            if let Some(url) = item.url.clone() {
                return Some(NavItem {
                    label: url.clone(),
                    href: url,
                    active: false,
                    external: true,
                });
            }

            if let Some(page_id) = &item.page_id {
                let page_fetch = get_page_from_id(pool, page_id);
                if let Some((page, _page_content)) = page_fetch {
                    let slug = if page.slug.is_empty() {
                        "/".to_string()
                    } else {
                        format!("/{}", page.slug)
                    };
                    let active = normalize_path(current_path) == normalize_path(&slug);
                    return Some(NavItem {
                        label: page.title,
                        href: slug,
                        active,
                        external: false,
                    });
                }
            }

            None
        })
        .collect()
}

fn normalize_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return "/".to_string();
    }
    let trimmed = trimmed.trim_end_matches('/');
    if trimmed.is_empty() {
        "/".to_string()
    } else {
        trimmed.to_string()
    }
}
