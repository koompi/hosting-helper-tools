mod actix_api;
use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::{Authority, FromRequest, TokenSigner};
use actix_web::{middleware, web::{scope, Data}, App, HttpServer};
use ed25519_compact::{KeyPair, Seed};
use jwt_compact::alg::Ed25519;
use libnginx_wrapper::init_migration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, FromRequest)]
struct User {
    username: String,
    password: String,
}

#[derive(Deserialize, Clone)]
struct EnvData {
    hosting_addr: String,
    hosting_port: String,
    production: bool,
    cors_allowed_addr: Vec<String>,
    login_accounts: Vec<User>,
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
    fn get_login_accounts(&self) -> &Vec<User> {
        &self.login_accounts
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key_pair = KeyPair::from_seed(Seed::generate());
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
        let cors_allowed_addr = settings.get_cors_allowed_addr().clone();
        let production = *settings.get_production();
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
            .app_data(Data::new(settings.get_login_accounts().to_owned()))
            .wrap(middleware::Logger::default())
            .service(actix_api::api::login)
            .use_jwt(
                authority,
                scope("")
                    .service(actix_api::api::get_nginx_list)
                    .service(actix_api::api::post_add_nginx)
                    .service(actix_api::api::post_force_cert)
                    .service(actix_api::api::post_force_migration)
                    .service(actix_api::api::delete_remove_nginx),
            )
            // .default_service(actix_api::api::default_route)
    })
    .bind(&hosting)?;
    println!("Server Running at {hosting}");
    server.run().await
}
