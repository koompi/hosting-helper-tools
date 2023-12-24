use actix_web::{middleware, App, HttpServer};
use libdeploy_wrapper::init_migration as deployment_migration;

mod actix_api;

struct EnvData {
    basepath: String,
    git_key: String,
    projroot: String,
    themepath: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    libdatabase::read_dotenv();

    let hosting_addr = dotenv::var("HOSTING_ADDR").unwrap();
    let hosting_port = dotenv::var("HOSTING_PORT")
        .unwrap()
        .as_str()
        .parse::<u16>()
        .unwrap()
        + 1;
    let hosting = format!("{hosting_addr}:{hosting_port}");
    let depl_mig = tokio::spawn(deployment_migration(false));
    depl_mig.await.unwrap().unwrap();

    let server = HttpServer::new(move || {
        let client = actix_web::web::Data::new(libcloudflare_wrapper::get_client());
        let headers = actix_web::web::Data::new(libcloudflare_wrapper::get_headers());
        let cors_allowed_addr = dotenv::var("CORS_ALLOWED_ADDR").unwrap();
        let production = dotenv::var("PRODUCTION").unwrap().parse::<bool>().unwrap();
        let cors = match production {
            true => actix_cors::Cors::default()
                .allowed_origin_fn(move |origin, _req_head| {
                    cors_allowed_addr
                        .split(",")
                        .any(|item| origin.as_bytes() == item.as_bytes())
                })
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allow_any_header()
                .allow_any_method()
                .supports_credentials(),
            false => actix_cors::Cors::permissive(),
        };
        App::new()
            .app_data(client)
            .app_data(headers)
            .app_data(actix_web::web::Data::new(EnvData {
                basepath: std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                git_key: dotenv::var("THEME_GIT_KEY").unwrap(),
                projroot: dotenv::var("PROJROOT").unwrap(),
                themepath: dotenv::var("THEME_PATH").unwrap(),
            }))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(actix_web_lab::middleware::from_fn(
                actix_api::middleware::simple_auth,
            ))
            .service(actix_api::api::post_hosting_add)
            .service(actix_api::api::post_hosting_update)
    })
    .bind(&hosting)?;
    println!("HostingJS Server Running at {hosting}");
    server.run().await
}
