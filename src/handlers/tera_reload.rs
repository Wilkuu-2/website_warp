use warp::{reply,Reply,Rejection};
use tera::Context;
use crate::tera::{reload_tera,render};
use crate::error::RouteClosed;

pub async fn tera_reload() -> Result<Box<dyn Reply>, Rejection> {
    let ctx = Context::new();
    if !cfg!(feature="debug") {

        return Err(warp::reject::custom(RouteClosed { name: "Tera Template Test".to_string() }))
    } 
    reload_tera().await?;
    Ok(Box::new(reply::html(render("tera_reset.html", &ctx).await?)))
}
