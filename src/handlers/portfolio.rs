use tera::Context;
use warp::{reply::{self, Html},Rejection};
use crate::tera::snippet_template_page;
use minify_html::minify;
use crate::common::MINIFY_CONFIG;

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
                   &MINIFY_CONFIG))
       .to_string(); 

    Ok(Box::new(reply::html(page_minified)))
    
}
