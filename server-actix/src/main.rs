use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};

extern crate fs_utilities;

pub async fn folder_handler(request: web::Json<fs_utilities::Request>) -> web::Json<fs_utilities::Folder> {
    web::Json(fs_utilities::get_folder_contents(request.0))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5555")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    ,
            )
            .wrap(Logger::default())
            .service(web::resource("/folder").route(web::post().to(folder_handler)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
