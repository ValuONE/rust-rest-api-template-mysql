use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    if validate_token(credentials.token()) {
        log::info!("{} called {} endpoint", req.peer_addr().map_or("Unknown ip".to_string(), |addr| addr.to_string()), req.uri());

        return Ok(req);
    }

    log::info!("{} tried to call {} endpoint with wrong bearer token", req.peer_addr().map_or("Unknown ip".to_string(), |addr| addr.to_string()), req.uri());

    Err((ErrorUnauthorized("No valid token found"), req))
}

fn validate_token(str: &str) -> bool {
    let token: Vec<&str> = vec!["1234"];

    token.contains(&str)
}