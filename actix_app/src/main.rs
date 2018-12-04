extern crate actix_web;
extern crate futures;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use actix_web::{
    http, server, App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse, Json, Result,
};
use futures::Future;

fn index(_req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

#[derive(Serialize, Deserialize, Debug)]
struct Resp {
    data: String,
    code: i32,
}

fn json(_req: &HttpRequest) -> Result<HttpResponse> {
    let resp = Resp {
        data: String::from("Rust"),
        code: 200,
    };
    let json = serde_json::to_string(&resp)?;
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("application/json")
        .body(&json))
}

// TODO
fn create(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
        .from_err()
        .and_then(|val: Resp| {
            println!("model: {:?}", val);
            Ok(HttpResponse::Ok().json(val)) // <- send response
        }).responder()
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
            .resource("/json", |r| r.f(json))
            .resource("/create", |r| r.f(create))
    }).bind("127.0.0.1:8088")
    .unwrap()
    .run();
}
