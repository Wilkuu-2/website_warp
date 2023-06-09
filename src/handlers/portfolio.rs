use tera::Context;
use warp::{reply::{self, Html},Rejection};
use crate::{tera::snippet_template_page};
use minify_html::{Cfg,minify};
use lazy_static::lazy_static;

// TODO: Config redundance
lazy_static!{
    static ref MCONFIG: Cfg = {
        let mut cfg = Cfg::new();
        
        cfg.keep_closing_tags=true;

        cfg
    }; 
}

pub fn portfolio_page_wrapper(id: usize, snippet_path: &String) -> String {
    format!("<article id=\"A{}\" class=\"card border-primary row col-11 mb-4 pb-3 justify-content-around\">
            {{% include \"{}\" %}}
            </article>", id, snippet_path)
}

pub async fn portfolio_page() -> Result<Box<Html<String>>,Rejection> {
    let ctx = Context::new(); 
    let page = snippet_template_page("portfolio.html",
                                     "templates/portfolio/", 
                                      portfolio_page_wrapper,
                                      &ctx).await?;
   let page_minified: String = 
       String::from_utf8_lossy(
           &minify(&page.as_bytes(),
                   &MCONFIG))
       .to_string(); 

    Ok(Box::new(reply::html(page_minified)))
    
}
