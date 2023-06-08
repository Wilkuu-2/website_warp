use std::convert::Infallible;
use tera::Context;
use warp::{reply,Reply, Rejection};
use warp::http::StatusCode;
use crate::tera::{render, TemplateError};
use crate::error::RouteClosed;


pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible>{
    let mut ctx = Context::new();
    let mut code: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        ctx.insert("msg", "The content you are looking for is not found");
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
       ctx.insert("msg", "Method not supported");
    } else if let Some(route_closed) = err.find::<RouteClosed>(){
        code = StatusCode::GONE;
        ctx.insert("msg", &format!("Route: {} has been closed", route_closed.name));
    } else if let Some(templaterror) = err.find::<TemplateError>() {
        code = StatusCode::IM_A_TEAPOT;
        ctx.insert("msg", &format!("{}", templaterror));
    } else {
        ctx.insert("msg", "An unhandled exception found, contact the system administrator");
    } 

    ctx.insert("code", &code.as_u16());
    let payload = render("error.html", &ctx).await.unwrap();
    Ok(reply::with_status(reply::html(payload), code))
}

