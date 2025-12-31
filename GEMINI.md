# Creating a New Canopie Theme

This document outlines the process for creating a new theme for the Canopie platform. Themes are Rust projects located in the `crates/` directory and follow a specific naming convention and structure.

## Theme Location and Naming

All themes must be placed directly within the `crates/` directory. Each theme project should be named `canopie-themes-{THEME_NAME}`, where `{THEME_NAME}` is the same as the directory name (e.g., `crates/my-new-theme` would have a package name of `canopie-themes-my-new-theme`).

Example structure:

```
themes/
├── crates/
│   ├── morningstar/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── my-new-theme/ # Your new theme here
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
└── Cargo.toml
```

## `Cargo.toml` Configuration

Each theme project requires its own `Cargo.toml` file. Below is an example of the essential dependencies and configuration. You should adapt the `name` and `version` fields for your theme.

```toml
[package]
name = "canopie-themes-{YOUR_THEME_NAME}" # IMPORTANT: Update this
version = "0.1.0" # Start with 0.1.0 and update as needed
edition = "2024"
publish = ["canopie"]

[dependencies]
canopie-macros = { path = "../../../canopie-utils/crates/macros", registry = "canopie", version = "0.1.3" }
canopie-utils = { path = "../../../canopie-utils", registry = "canopie", version = "0.1.12" }
include_dir = "0.7.4"
maud = "0.27.0"
regex = "1.12.2"
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.148"

[features]
default=[]
embed = ["canopie-utils/embed"]
dev = ["embed", "canopie-utils/dev"]
```

## Theme Implementation

Themes typically consist of Rust code that defines the visual and functional aspects of your Canopie site.

### `ThemeSchema` Derive Macro

Derives the `ThemeSchema` trait for a struct, turning it into a theme configuration.

This macro generates the implementation of `ThemeSchema`, which includes:
- A `schema()` method that provides a vector of `PropertySchema` for introspection.
- A `from_json()` method that safely deserializes a JSON string into the struct,
  applying default values and performing validations.

It relies on the `#[theme(...)]` attribute on each field to generate the schema
and validation logic.

#### The `#[theme]` attribute

The `#[theme]` attribute accepts the following key-value pairs:
- `interface`: The UI component to use (e.g., "SelectColor", "FileImage").
  - available components are currently Input, InputCode, InputText, InputTextarea, InputTags, InputString, Slug, Checkbox, Decimal, Float, Hash, Integer, SelectDropdown, SelectColor, SelectIcon, SelectRadioGroup, File, FileImage, Datetime, EditorBlock, Csv
- `width`: The width of the input field ("Full" or "Half").
- `default`: The default value for the field (e.g., `"hello"`, `true`, `123`).
- `min_length`: The minimum length for a string value.
- `max_length`: The maximum length for a string value.
- `regex`: A regex pattern that a string value must match.

#### Compile-Time Checks

- This macro will produce a compile error if the struct does not contain `favicon`
  and `primary_color` fields. Use the `#[theme_defaults]` macro to add them automatically.
- It will also error if a non-optional field does not have a `default` attribute.

#### Example

```ignore
use canopie_macros::{theme_defaults, ThemeConfig};
use serde::Deserialize;

#[theme_defaults]
#[derive(ThemeConfig, Deserialize)]
pub struct MyTheme {
    #[theme(interface = "Text", default = "My Awesome Site")]
    pub site_title: String,

    #[theme(interface = "TextArea", max_length = 500)]
    pub site_description: Option<String>,
}
```

### `#[theme_defaults]` Attribute

An attribute macro that adds default fields to a theme configuration struct.

This macro inspects the struct it's applied to and adds the following fields
if they are not already present:
- `favicon`: An `Option<String>` for the site's favicon.
- `primary_color`: A `String` for the theme's primary color.

This is useful to ensure that all themes have a common set of required fields.

#### Example

```ignore
use canopie_macros::theme_defaults;
use serde::Deserialize;

#[theme_defaults]
#[derive(Deserialize)]
struct MyTheme {
    // other fields
}
```
The `MyTheme` struct will now effectively have `favicon` and `primary_color` fields
with default attributes.

## Basic `src/main.rs` Structure

Your `src/main.rs` file will typically contain the core logic for your theme, utilizing the `canopie-macros` and `canopie-utils` crates to interact with the Canopie platform.

```rust
use canopie_macros::{theme, theme_defaults, ThemeConfig};
use canopie_utils::prelude::*;
use canopie_utils::theme::ThemeSchema;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};

// Define a configuration schema for each page template in
// the `templates` subdirectory
#[derive(Debug, Clone, Serialize, Deserialize, ThemeConfig)]
pub struct MyPageConfig {
    // Example config fields
    #[theme(
        interface = "SelectColor",
        width = "Half",
        default = "#ff0000",
        regex = "^#([A-Fa-f0-9]{6})$"
    )]
    pub background_color: String,

    #[theme(width = "Half", default = true)]
    pub show_sidebar: bool,
}

// Define your theme's configuration schema
// in the `lib.rs` file
#[theme_defaults]
#[derive(Debug, Clone, Serialize, Deserialize, ThemeConfig)]
pub struct MyThemeConfig {
    // Example config fields
    #[theme(width = "Half", default = "My Awesome Canopie Site")]
    pub site_title: String,

    #[theme(
        interface = "SelectColor",
        width = "Half",
        default = "#ff0000",
        regex = "^#([A-Fa-f0-9]{6})$"
    )]
    pub primary_color: String,
}


// Create theme struct
pub struct ThemeMyTheme {}

// Implement `GetThemeOverview` for your theme
impl GetThemeOverview for ThemeMyTheme {
    fn get_id() -> String {
        "{YOUR_THEME_NAME}".to_string()
    }
    fn get_name() -> String {
        "My Theme Full Text Name".to_string()
    }
    fn get_theme_overview() -> ThemeOverview {
        ThemeOverview {
            id: Self::get_id(),
            name: Self::get_name(),
            config: MyThemeConfig::schema(),
            page_themes: vec![
                PageThemeOverview {
                    name: "My Page Template".to_string(),
                    config: MyPageConfig::schema(),
                    default: true,
                },
                // Other page themes...
            ],
        }
    }
}

// Define your theme's main entry point
#[theme]
pub fn my_theme(
    pool: &PgPool,
    headers: &mut Header,
    website: Website,
    path: &str,
) -> PageResult {

    let menus = get_menus(pool, &website.id);
    
    let main_menu = menus.iter().find(|menu| menu.0.name == "Main");
    let mut formatter = Formatter::default(Some(website.clone()));

    let page_response = html! {
        div {
            // This `build_menu_header` function can be defined in a subdirectory
            (build_menu_header(pool, path, &website.id, main_menu, formatter, headers))
        }
        // Styling should be done with tailwind style classes
        div class="flex flex-col items-center justify-center h-screen bg-gray-100">
            // This `build_page_response` function can be defined in a subdirectory
            // Or you can define a different function for each page template
            (build_page_response(pool, path, &website.id, main_menu, formatter, headers))
        }
    }

    let assets = formatter.collect_assets(Some("my_theme"));

    headers.add_assets(assets);
    // This can be set inside the `build_page_response` function
    headers.set_title("My Page Title");

    // Use the `PageResult::Found` variant to indicate that the page was found
    // For 404 errors, use `PageResult::NotFound`
    PageResult::Found(page_response)
}

// Expose the entry point by implementing `GetThemeRenderer`
#[cfg(feature = "embed")]
impl GetThemeRenderer for ThemeMyTheme {
    fn get_theme_renderer() -> ThemeRenderer {
        ThemeRenderer {
            name: Self::get_id(),
            build_content: my_theme,
        }
    }
}
```

Remember to replace placeholders like `{YOUR_THEME_NAME}` and fill in the detailed documentation for the `ThemeSchema`, `#[theme]`, and `#[theme_defaults]` attributes.

When in doubt, refer to the Morning Star theme at `./crates/morningstar` or ask for clarification. For reference, the database schema is provided at `./reference/schema.rs` and available models from `./reference/models.rs`. These can be accessed from the `canopie_utils` crate.


ps - the `./scripts` directory contains outdated scripts and should be ignored.
