use actix_web::{middleware, App, HttpServer};
use libdeploy_wrapper::init_migration as deployment_migration;

mod actix_api;

#[derive(Clone)]
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
    let hosting_port = dotenv::var("HOSTINGJS_PORT").unwrap();
    let hosting = format!("{hosting_addr}:{hosting_port}");
    let depl_mig = tokio::spawn(deployment_migration());
    depl_mig.await.unwrap().unwrap();

    let server = HttpServer::new(move || {
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
            .service(actix_api::api::put_update_git_pull)
            .service(actix_api::api::get_hosting_log)
            .service(actix_api::api::get_server_port)
            .service(actix_api::api::put_hosting_update)
            .service(actix_api::api::delete_hosting)
    })
    .bind(&hosting)?;
    println!("HostingJS Server Running at {hosting}");
    server.run().await
}
