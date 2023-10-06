use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::{Authority, FromRequest, TokenSigner};
use actix_web::{
    middleware,
    web::scope,
    App, HttpServer,
};
use ed25519_compact::{KeyPair, Seed};
use jwt_compact::alg::Ed25519;
use libnginx_wrapper::init_migration;
use serde::{Deserialize, Serialize};

mod actix_api;

#[derive(Serialize, Deserialize, Clone, FromRequest)]
struct User {
    username: String,
    password: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::from_path(format!(
        "{}/.env",
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
    )).unwrap();

    let key_pair = KeyPair::from_seed(Seed::generate());
    let hosting_addr = dotenv::var("HOSTING_ADDR").unwrap();
    let hosting_port = dotenv::var("HOSTING_PORT").unwrap();
    let hosting = format!("{hosting_addr}:{hosting_port}");

    init_migration(false).unwrap();

    let server = HttpServer::new(move || {
        let cors_allowed_addr = dotenv::var("CORS_ALLOWED_ADDR").unwrap();
        let production = dotenv::var("PRODUCTION").unwrap().parse::<bool>().unwrap();
        let authority = Authority::<User, Ed25519, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(key_pair.sk.clone())
                    .algorithm(Ed25519)
                    .build()
                    .expect(""),
            ))
            .enable_header_tokens(true)
            .enable_cookie_tokens(false)
            .enable_query_tokens(false)
            .renew_refresh_token_automatically(true)
            .access_token_name("access_token")
            .refresh_token_name("refresh_token")
            .verifying_key(key_pair.pk.clone())
            .build()
            .expect("");
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
            .service(actix_api::api::login)
            .use_jwt(
                authority,
                scope("")
                    .service(actix_api::api::get_nginx_list)
                    .service(actix_api::api::post_add_nginx)
                    .service(actix_api::api::post_force_cert)
                    .service(actix_api::api::post_force_migration)
                    .service(actix_api::api::delete_remove_nginx)
                    .service(actix_api::api::put_update_target_site),
            )
    })
    .bind(&hosting)?;
    println!("Server Running at {hosting}");
    server.run().await
}
