use tera::Context;
use warp::{reply,Reply,Rejection};
use crate::tera::render;

pub fn portfolio_page_wrapper(id: usize, snippet_path: &String) -> String {
    format!("<article id=\"A{}\" class=\"card border-primary row col-11 mb-4 pb-3 justify-content-around\">
            {{% include \"{}\" %}}
            </article>", id, snippet_path)
}

pub async fn portfolio_page() -> Result<Box<dyn Reply>, Rejection> {
    let ctx = Context::new();
    let payload = render("portfolio.done",&ctx).await?;
    Ok(Box::new(reply::html(payload)))
} 
