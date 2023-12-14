use super::obj_response::ActixCustomResponse;
use actix_web::{body::MessageBody, dev, Error};

pub(crate) async fn simple_auth(
    req: dev::ServiceRequest,
    next: actix_web_lab::middleware::Next<impl MessageBody + 'static>,
) -> Result<dev::ServiceResponse<impl MessageBody>, Error> {
    let req_headermap = req.request().headers();

    let user = match req_headermap.get("X-Auth-User") {
        Some(header) => Ok(header.to_str().unwrap()),
        None => Err(ActixCustomResponse::new_text(
            400,
            String::from("Missing X-Auth-User"),
        )),
    }?;

    let pass = match req_headermap.get("X-Auth-Pass") {
        Some(header) => Ok(header.to_str().unwrap()),
        None => Err(ActixCustomResponse::new_text(
            400,
            String::from("Missing X-Auth-Pass"),
        )),
    }?;

    match user == dotenv::var("USERNAME").unwrap() && pass == dotenv::var("PASSWORD").unwrap() {
        true => next.call(req).await,
        false => Err({
            println!(
                "Failed Authentication from IP: \"{}\" or Proxy: \"{}\" with Username: \"{}\" and Password: \"{}\"",
                req.connection_info()
                    .peer_addr()
                    .unwrap_or("Unknowned"),
                req.connection_info()
                    .realip_remote_addr()
                    .unwrap_or("Unknowned"),
                user,
                pass
            );
            ActixCustomResponse::new_text(401, String::from("Incorrect Authenticaton")).into()
        }),
    }
}
