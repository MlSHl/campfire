use worker::*;

mod model;
mod service;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env, // DB later
    _ctx: Context,
) -> Result<Response> {
    let path = req.uri().path().to_string();
    let method = req.method().clone();

    match (method, path.as_str()) {
        (http::Method::GET, "/") => Response::ok("Campfire Rust Wokrer Root!"),
        (http::Method::GET, "/api/ping") => Response::from_json(&service::get_ping()),
        _ => Response::error("Not Found", 404),
    }
}
