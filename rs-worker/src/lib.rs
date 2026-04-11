use worker::*;

use crate::model::{logbook::Log, user::UserDto};

mod model;
mod repository;
mod service;

#[event(fetch)]
async fn fetch(
    mut req: Request,
    env: Env, // for DB
    _ctx: Context,
) -> Result<Response> {
    let path = req.path().to_string();
    let method = req.method().clone();

    match (method, path.as_str()) {
        (Method::Get, "/") => Response::ok("Campfire Rust Worker Root!"),
        (Method::Get, "/api/ping") => Response::from_json(&service::general::ping_response()),
        (Method::Get, "/api/hello") => Response::from_json(&service::general::hello(
            get_param_value(req.url()?.query(), "name"),
        )),
        (Method::Post, "/api/echo") => {
            let body = req.text().await?;
            Response::from_json(&service::general::echo(body))
        }
        (Method::Post, "/api/logs") => {
            let log: Log = req.json().await?;
            Response::from_json(&service::logbook::insert_log(&env, log).await)
        }
        (Method::Post, "/api/user") => {
            let user: UserDto = req.json().await?;
            Response::from_json(&service::users::register_user(&env, user).await)
        }
        _ => Response::error("Not Found", 404),
    }
}

fn get_param_value(query: Option<&str>, key_name: &str) -> String {
    query
        .and_then(|q| {
            q.split("&").find_map(|pair| {
                let mut parts = pair.splitn(2, "=");
                let key = parts.next();
                let value = parts.next();

                if key == Some(key_name) {
                    Some(value?.to_string())
                } else {
                    None
                }
            })
        })
        .unwrap_or("stranger".to_string())
}
