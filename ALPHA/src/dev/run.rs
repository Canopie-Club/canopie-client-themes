use anyhow::Error;
use std::time::Instant;
use std::{path::PathBuf, sync::Arc};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger, web};
use canopie_utils::image_transformations::ImgParams;
use canopie_utils::time::format_duration;
use canopie_utils::{encre::create_encre_styles, image_transformations::transform_image};
use dotenvy::from_path;
use libvips::VipsApp;
use maud::PreEscaped;

#[cfg(feature = "embed")]
use include_dir::{Dir, File, include_dir};

use crate::{
    db::{self, PgPool},
    dev::response::build_dev_page_response,
    header::Header,
    renderer::{PageResponse, PageResult, ThemeRenderer},
    resource::embed::Resources,
    utils::{get_website, get_website_from_project_id},
};

#[derive(Clone)]
pub struct DevServer {
    renderer: ThemeRenderer,
    resources: Resources,
    vips: Arc<VipsApp>,
    pool: PgPool,
}

impl DevServer {
    pub fn new(
        renderer: ThemeRenderer,
        resources: Resources,
        vips: Arc<VipsApp>,
        pool: PgPool,
    ) -> Self {
        DevServer {
            renderer,
            resources,
            vips,
            pool,
        }
    }
}

#[actix_web::main]
pub async fn dev_serve(
    renderer: ThemeRenderer,
    theme_resources: Option<Dir<'static>>,
    s3_resources: Option<Dir<'static>>,
) -> std::io::Result<()> {
    println!("Starting server");

    let mut resources = Resources::new();

    if let Some(theme_resources) = theme_resources {
        resources.add_dir("theme", theme_resources);
    }

    if let Some(s3_resources) = s3_resources {
        resources.add_dir("s3", s3_resources);
    }

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

    let state = DevServer {
        renderer: renderer.clone(),
        resources: resources.clone(),
        vips: vips.clone(),
        pool: pool.clone(),
    };

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::new("%r %s %T"))
            .app_data(web::Data::new(state.clone()))
            // .configure(api::config)
            // .configure(routes::init)
            .service(Files::new("/static", "static").use_last_modified(true))
            .route(
                "/_canopie/static/{tail:.*}",
                web::get().to(dev_static_response),
            )
            .route("/_f/{tail:.*}", web::get().to(dev_s3_response))
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
    state: web::Data<DevServer>,
) -> impl Responder {
    let path = route.into_inner();

    let website = get_website_from_project_id(&state.pool, "morningstar".to_string()).unwrap();

    let mut header = Header::new_with_title(&website.title);

    let render_result =
        (state.renderer.build_content)(&state.pool, &mut header, website, path.as_str());

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

async fn dev_s3_response(
    request: HttpRequest,
    route: web::Path<String>,
    query: web::Query<ImgParams>,
    state: web::Data<DevServer>,
    accept: web::Header<actix_web::http::header::Accept>,
) -> impl Responder {
    file_response(request, route, query, state, accept, Some("s3")).await
}

async fn dev_static_response(
    request: HttpRequest,
    route: web::Path<String>,
    query: web::Query<ImgParams>,
    state: web::Data<DevServer>,
    accept: web::Header<actix_web::http::header::Accept>,
) -> impl Responder {
    file_response(request, route, query, state, accept, Some("theme")).await
}

async fn file_response(
    request: HttpRequest,
    route: web::Path<String>,
    query: web::Query<ImgParams>,
    state: web::Data<DevServer>,
    accept: web::Header<actix_web::http::header::Accept>,
    resource_id: Option<&str>,
) -> impl Responder {
    let start_time = Instant::now();
    let path = route.into_inner();

    let resource = state.resources.get_resource(&path, resource_id);

    match resource {
        Some(resource) => {
            let bytes = resource.contents().to_vec();

            let mime_type = mime_guess::from_path(&path)
                .first_or_octet_stream()
                .essence_str()
                .to_string();

            let is_image = mime_type.starts_with("image/");

            let (final_bytes, final_mime) = if is_image {
                let bytes_clone = bytes.clone();
                let thread_vips = state.vips.clone();
                let result = tokio::task::spawn_blocking(move || {
                    let accept_header: Option<String> = Some(accept.to_string());
                    let accept_header_ref = accept_header.as_deref(); // Option<&str>
                    transform_image(&bytes_clone, &query, accept_header_ref, thread_vips)
                })
                .await;

                if let Ok((buf, mime)) = result.unwrap_or(Err(Error::msg("error"))) {
                    (buf, mime)
                } else {
                    (bytes.clone(), mime_type.clone())
                }
            } else {
                (bytes.clone(), mime_type.clone())
            };

            println!(
                "/_f/{} [{}] -> {} (fetched from S3)",
                path,
                200,
                format_duration(start_time.elapsed())
            );

            HttpResponse::Ok()
                .content_type(final_mime)
                .append_header(("Content-Disposition", "inline"))
                .append_header((
                    "Cache-Control",
                    "public, max-age=2592000, stale-while-revalidate=86400",
                ))
                .body(final_bytes)
        }
        None => HttpResponse::NotFound()
            .content_type("text/html")
            .body(format!("{} not found", path)),
    }
}
