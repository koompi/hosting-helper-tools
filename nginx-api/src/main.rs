use actix_web::{middleware, App, HttpServer};
use libnginx_wrapper::init_migration;

mod actix_api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    libdatabase::read_dotenv();

    let hosting_addr = dotenv::var("HOSTING_ADDR").unwrap();
    let hosting_port = dotenv::var("HOSTING_PORT").unwrap();
    let hosting = format!("{hosting_addr}:{hosting_port}");

    init_migration(false).unwrap();

    let server = HttpServer::new(move || {
        let cors_allowed_addr = dotenv::var("CORS_ALLOWED_ADDR").unwrap();
        let production = dotenv::var("PRODUCTION").unwrap().parse::<bool>().unwrap();
        App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allowed_origin_fn(move |origin, _req_head| match production {
                        true => cors_allowed_addr
                            .split(",")
                            .any(|item| origin.as_bytes() == item.as_bytes()),
                        false => true,
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .supports_credentials(),
            )
            .wrap(middleware::Logger::default())
            .wrap(actix_web_lab::middleware::from_fn(
                actix_api::middleware::simple_auth,
            ))
            .service(actix_api::api::get_dns)
            .service(actix_api::api::get_nginx_list)
            .service(actix_api::api::post_add_nginx)
            .service(actix_api::api::post_force_cert)
            .service(actix_api::api::post_force_migration)
            .service(actix_api::api::delete_remove_nginx)
            .service(actix_api::api::put_update_target_site)
        // )
    })
    .bind(&hosting)?;
    println!("Server Running at {hosting}");
    server.run().await
}
