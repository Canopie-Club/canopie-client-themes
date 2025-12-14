use canopie_client_utils::dev::{renderer::get_default_theme, run::dev_serve};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let theme = get_default_theme();
    dev_serve(theme).await
}
