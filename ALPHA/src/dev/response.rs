use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};

use actix_web::{HttpRequest, HttpResponse, Responder, web};
use libvips::VipsApp;

use canopie_utils::image_transformations::{ImgParams, transform_image};
use canopie_utils::time::format_duration;
use mime_guess::from_path;

pub async fn build_dev_page_response(
    request: HttpRequest,
    route: web::Path<String>,
    query: web::Query<ImgParams>,
    vips: web::Data<Arc<VipsApp>>,
) -> impl Responder {
    let path = route.into_inner();
    HttpResponse::Ok().content_type("text/html").body(path)
}

pub async fn serve_static_file(
    route: web::Path<String>,
    query: web::Query<ImgParams>,
    vips: web::Data<Arc<VipsApp>>,
) -> impl Responder {
    let start_time = Instant::now();
    // Sanitize and construct file path
    let static_dir = Path::new("static");
    let requested_path: PathBuf = static_dir.join(route.as_str());

    // Prevent directory traversal
    if !requested_path.starts_with(static_dir) {
        println!(
            "/_canopie/static/{} [{}] -> {}",
            route.as_str(),
            401,
            format_duration(start_time.elapsed())
        );
        return HttpResponse::Forbidden().body("Forbidden");
    }

    // Check if file exists and is a file
    if !requested_path.exists() || !requested_path.is_file() {
        println!(
            "/_canopie/static/{} [{}] -> {}",
            route.as_str(),
            404,
            format_duration(start_time.elapsed())
        );
        return HttpResponse::NotFound().body("404 Not Found");
    }

    // Guess MIME type
    let mime_type = from_path(&requested_path).first_or_octet_stream();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(mime_type.to_string())

    // Read file contents
    // match File::open(&requested_path).await {
    //     Ok(mut file) => {
    //         let mut contents = Vec::new();
    //         if let Err(_) = file.read_to_end(&mut contents).await {
    //             return HttpResponse::InternalServerError().body("Error reading file");
    //         }
    //         println!(
    //             "/_canopie/static/{} [{}] -> {}",
    //             route.as_str(),
    //             200,
    //             format_duration(start_time.elapsed())
    //         );

    //         HttpResponse::Ok()
    //             .content_type(mime_type.as_ref())
    //             .append_header(("Content-Disposition", "inline"))
    //             .append_header((
    //                 "Cache-Control",
    //                 "public, max-age=2592000, stale-while-revalidate=86400",
    //             ))
    //             .body(contents)
    //     }
    //     Err(_) => HttpResponse::InternalServerError().body("Could not open file"),
    // }
}
