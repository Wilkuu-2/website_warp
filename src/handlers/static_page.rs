use tera::Context;
use warp::Rejection;
use warp::reply::{Html,html}; 
use crate::tera::render;
use minify_html::{Cfg,minify};
use lazy_static::lazy_static;

lazy_static!{
    static ref MCONFIG: Cfg = {
        let mut cfg = Cfg::new();
        
        cfg.keep_closing_tags=true;

        cfg
    }; 
}

pub async fn static_page(template_name: &str) 
    -> Result<Box<Html<String>>, Rejection>
{
   let ctx = Context::new();
   let page_minified: String = 
       String::from_utf8_lossy(
           &minify(&render(template_name, &ctx).await?.as_bytes(),
                   &MCONFIG))
       .to_string(); 
   
   Ok(Box::new(html(page_minified)))
}


