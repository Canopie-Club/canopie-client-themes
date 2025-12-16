use std::{path::PathBuf, sync::Arc};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger, web};
use canopie_utils::encre::create_encre_styles;
use canopie_utils::image_transformations::ImgParams;
use dotenvy::from_path;
use libvips::VipsApp;
use maud::PreEscaped;
use rust_embed::Embed;

use crate::{
    db::{self, PgPool},
    dev::response::build_dev_page_response,
    header::Header,
    renderer::{PageResponse, PageResult, ThemeRenderer},
    resource::Resource,
    utils::{get_website, get_website_from_project_id},
};

#[actix_web::main]
pub async fn dev_serve(
    renderer: ThemeRenderer,
    resources: Option<impl Embed>,
) -> std::io::Result<()> {
    println!("Starting server");

    // Initialize libvips globally
    let vips =
        Arc::new(VipsApp::new("canopie-client-rs", false).expect("Failed to initialize libvips"));

    // Optionally configure libvips
    vips.concurrency_set(4);

    dotenvy::dotenv().ok();

    let env_path = std::env::var("ENV_PATH").unwrap_or_else(|_| "../ALPHA/.env".into());

    from_path(PathBuf::from(env_path)).ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let dev = true;

    let pool = db::init_pool();

    // let

    // let read_only = db::ensure_read_only(&pool);

    println!("Project cache initialized");
    println!("Starting server on 0.0.0.0:{}", port);
    println!("Development mode: {}", dev);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::new("%r %s %T"))
            .app_data(web::Data::new(vips.clone()))
            .app_data(web::Data::new(renderer.clone()))
            .app_data(web::Data::new(pool.clone()))
            // .configure(api::config)
            // .configure(routes::init)
            .service(Files::new("/static", "static").use_last_modified(true))
            .service(Files::new("/_canopie/static", "static").use_last_modified(true))
            .route("/{tail:.*}", web::get().to(dev_page_response))
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}

async fn dev_page_response(
    request: HttpRequest,
    route: web::Path<String>,
    query: web::Query<ImgParams>,
    vips: web::Data<Arc<VipsApp>>,
    renderer: web::Data<ThemeRenderer>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let path = route.into_inner();

    let website = get_website_from_project_id(&pool, "morningstar".to_string()).unwrap();

    let mut header = Header::new_with_title(&website.title);

    let render_result = (renderer.build_content)(&pool, &mut header, website, path.as_str());

    match render_result {
        PageResult::Found(content) => {
            let styles = create_encre_styles(&content);
            header.add_style_tag(PreEscaped(styles));
            let styled_content = header.render(&content);
            HttpResponse::Ok()
                .content_type("text/html")
                .body(styled_content.into_string())
        }
        PageResult::NotFound(content) => {
            let styles = create_encre_styles(&content);
            header.add_style_tag(PreEscaped(styles));
            let styled_content = header.render(&content);
            HttpResponse::NotFound()
                .content_type("text/html")
                .body(styled_content.into_string())
        }
    }

    // // Ensure the favicon is up to date by setting the favicon version
    // let website_updated_at = website.updated_at.clone();
    // let formatted_updated_at = website_updated_at.format("%Y-%m-%d_%H:%M:%S").to_string();
    // header.set_favicon_version(formatted_updated_at.as_str());

    // (success, styled_content)
    // HttpResponse::Ok().content_type("text/html").body(path)
}
