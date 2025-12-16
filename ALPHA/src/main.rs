use canopie_client_utils::dev::{renderer::get_default_theme, run::dev_serve};

fn main() -> Result<(), std::io::Error> {
    let theme = get_default_theme();
    dev_serve(theme, None)
}
