mod actix_api;
use actix_web::{middleware, App, HttpServer};
use libnginx_wrapper::init_migration;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct EnvData {
    hosting_addr: String,
    hosting_port: String,
    production: bool,
    cors_allowed_addr: Vec<String>,
}

impl EnvData {
    fn get_hosting_addr(&self) -> &str {
        &self.hosting_addr
    }
    fn get_hosting_port(&self) -> &str {
        &self.hosting_port
    }
    fn get_cors_allowed_addr(&self) -> &Vec<String> {
        &self.cors_allowed_addr
    }
    fn get_production(&self) -> &bool {
        &self.production
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("./settings.toml"))
        .build()
        .unwrap()
        .try_deserialize::<EnvData>()
        .unwrap();

    let hosting = format!(
        "{}:{}",
        settings.get_hosting_addr(),
        settings.get_hosting_port()
    );

    init_migration(false);

    let server = HttpServer::new(move || {
        let cors_allowed_addr = settings.get_cors_allowed_addr().to_owned();
        let production = settings.get_production().to_owned();
        App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allowed_origin_fn(move |origin, _req_head| match production {
                        true => cors_allowed_addr
                            .iter()
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
            .service(actix_api::api::get_nginx_list)
            .service(actix_api::api::post_add_nginx)
            .service(actix_api::api::post_force_cert)
            .service(actix_api::api::post_force_migration)
            .service(actix_api::api::delete_remove_nginx)
    })
    .bind(&hosting)?;
    println!("Server Running at {hosting}");
    server.run().await
}
