use tera::Context;
use warp::Rejection;
use warp::reply::{Html,html}; 
use crate::tera::render;


pub async fn static_page(template_name: &str) 
    -> Result<Box<Html<String>>, Rejection>
{
   let ctx = Context::new(); 
   Ok(Box::new(html(render(template_name, &ctx).await?)))
}


