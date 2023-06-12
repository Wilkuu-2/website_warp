use tera::Context;
use warp::Rejection;
use warp::reply::{Html,html}; 
use crate::tera::render;
use minify_html::minify;
use crate::common::MINIFY_CONFIG;


pub async fn static_page(template_name: &str) 
    -> Result<Box<Html<String>>, Rejection>
{
   let ctx = Context::new();
   let page_minified: String = 
       String::from_utf8_lossy(
           &minify(&render(template_name, &ctx).await?.as_bytes(),
                   &MINIFY_CONFIG))
       .to_string(); 
   
   Ok(Box::new(html(page_minified)))
}


