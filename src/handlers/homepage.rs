use warp::{reply,Reply,Rejection};
use tera::Context;
use crate::tera::render;

pub async fn home_page() -> Result<Box<dyn Reply>, Rejection> {
    let payload = render("index.html", &Context::new())?;
    Ok(Box::new(reply::html(payload)))
}

