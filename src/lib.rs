use canopie_themes_atlas::ThemeAtlas;
use canopie_themes_morningstar::ThemeMorningStar;
use canopie_utils::themes::{GetThemeOverview, ThemeOverview, ThemeReference};
#[cfg(feature = "embed")]
use canopie_utils::{renderer::ThemeRenderer, resource::embed::Resources};

#[cfg(feature = "embed")]
pub fn get_resources() -> Result<Resources, String> {
    use canopie_themes_morningstar::ThemeMorningStar;
    use canopie_utils::resource::embed::ThemeResource;

    let mut resources = Resources::new();

    ThemeMorningStar::attach_theme_resources(&mut resources)?;
    ThemeAtlas::attach_theme_resources(&mut resources)?;

    Ok(resources)
}

pub fn get_themes() -> Vec<ThemeReference> {
    vec![
        ThemeMorningStar::get_reference(),
        ThemeAtlas::get_reference(),
    ]
}

pub fn get_theme_overview(theme_id: String) -> Option<ThemeOverview> {
    match theme_id.as_str() {
        "morningstar" => Some(ThemeMorningStar::get_theme_overview()),
        "atlas" => Some(ThemeAtlas::get_theme_overview()),
        _ => None,
    }
}

#[cfg(feature = "embed")]
pub fn get_theme(theme_id: String) -> ThemeRenderer {
    use canopie_utils::renderer::GetThemeRenderer;

    match theme_id.as_str() {
        "morningstar" => ThemeMorningStar::get_theme_renderer(),
        "atlas" => ThemeAtlas::get_theme_renderer(),
        // TODO: Create default theme
        _ => ThemeMorningStar::get_theme_renderer(),
    }
}
