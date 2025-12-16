use std::io::Error;

use canopie_client_theme_morningstar::morningstar;
use canopie_client_utils::{dev::run::dev_serve, renderer::ThemeRenderer};

fn main() -> Result<(), Error> {
    // Asset::get(file_path);

    let renderer = ThemeRenderer {
        name: String::from("morningstar"),
        build_content: morningstar,
    };

    dev_serve(renderer)
}
