mod validate;

use std::{convert::Infallible, net::SocketAddr};

use warp::Filter;
use crate::validate::Validator;

#[tokio::main]
async fn main() {

    let path = std::path::Path::new(".env");
    dotenv::from_path(path).ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let validator = Validator::new();

    let hello = warp::path!("hello" / String)
        .and(with_validator(validator))
        .and(warp::addr::remote())
        .and_then(response);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn with_validator(validator: Validator) -> impl Filter<Extract = (Validator,), Error = Infallible> + Clone {
    warp::any().map(move || validator.clone())
}

async fn response(name: String, validator: Validator, remote: Option<SocketAddr>) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(addr) = remote {
        let ip_string = addr.ip().to_string();
        if validator.is_valid(ip_string.clone()).await {
            Ok(format!("Hello, {} from {}!", name, &ip_string))
        } else {
            Ok(format!("No key for {}!", &ip_string))
        }
    } else {
        Ok("No remote address!".to_string())
    }
}
