use std::io::Error;

use canopie_themes_morningstar::morningstar;
use canopie_utils::renderer::ThemeRenderer;
#[cfg(feature = "embed")]
use canopie_utils::theme_dev::run::run::dev_serve;
#[cfg(feature = "embed")]
use include_dir::{Dir, include_dir};

fn main() -> Result<(), Error> {
    #[cfg(feature = "embed")]
    {
        let theme_dir: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/static");
        let s3_dir: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/s3");

        let has_names = theme_dir.contains("images/names.webp");

        println!("Has names: {}", has_names);

        let renderer = ThemeRenderer {
            name: String::from("morningstar"),
            build_content: morningstar,
        };

        dev_serve(renderer, Some(theme_dir), Some(s3_dir))
    }

    #[cfg(not(feature = "embed"))]
    {
        println!("Embed feature is not enabled");
        Ok(())
    }
}
