use warp::{reply,Reply,Rejection};
use tera::Context;
use crate::tera::render;
use crate::error::RouteClosed;

pub async fn tera_test(template_name: String, other_str: String) -> Result<Box<dyn Reply>, Rejection> {
    let mut ctx = Context::new();
    ctx.insert("in_str", &format!("<i>{}</i>", other_str));
    let payload = render(&template_name, &ctx)?;
    if cfg!(feature="debug") {
        Ok(Box::new(reply::html(payload)))
    } else {
        Err(warp::reject::custom(RouteClosed { name: "Tera Template Test".to_string() }))
    }
}
