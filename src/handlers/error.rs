use std::convert::Infallible;
use tera::Context;
use warp::{reply,Reply, Rejection};
use warp::http::StatusCode;
use crate::tera::render;
use crate::error::RouteClosed;


pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible>{
    let mut ctx = Context::new();
    let route_closed = err.find::<RouteClosed>();
    if err.is_not_found() {
        ctx.insert("code", &StatusCode::NOT_FOUND.as_u16());
        ctx.insert("msg", "The content you are looking for is not found");
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
       ctx.insert("code", &StatusCode::METHOD_NOT_ALLOWED.as_u16());
       ctx.insert("msg", "Method not supported");
    } else if let Some(_) = route_closed{
        ctx.insert("code", &StatusCode::GONE.as_u16());
        ctx.insert("msg", &format!("Route: {} has been closed", route_closed.unwrap().name));
    } else {
        ctx.insert("code", &StatusCode::INTERNAL_SERVER_ERROR.as_u16());
        ctx.insert("msg", "An unhandled exception found, contact the system administrator");
    } 
    let payload = render("error.html", &ctx).unwrap();
    Ok(reply::html(payload))
}

