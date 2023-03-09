use tera::Tera;
use tera::Context;
use lazy_static::lazy_static; 
use warp::{
    self,
    reject::{custom, Reject},
    Rejection,
};

use crate::snippets::SnippetPaths;
use crate::snippets::SnippetWrapper;

const TEMPLATES_PATH: &str = "templates/";

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = match Tera::new(&format!("{}**/*.html", TEMPLATES_PATH)) {
            Ok(t) => t,
            Err(e) => {
                println!("TERA ERROR: {}", e);
                ::std::process::exit(1);
            }
        };
    
        let paths = SnippetPaths::from_folder("templates/portfolio/", 20, true);
        snippet_template_page(& mut tera,
                              "portfolio.done",
                              "portfolio.html",
                              &paths, 
                              crate::handlers::portfolio::portfolio_page_wrapper);

        tera.autoescape_on(vec![".html",".sql"]);
        tera
    };
}

#[derive(Debug)]
struct TemplateError;
impl Reject for TemplateError {}

pub fn render(name: &str, ctx: &Context) -> Result<String, Rejection> {
    TERA.render(name, &ctx).or(Err(custom(TemplateError)))
}


pub fn snippet_template_page( tera: & mut Tera,
                                template_name: &str,
                                template_path: &str,
                                   paths: &SnippetPaths,
                                   wrapper: SnippetWrapper) {
    let paths: Vec<_> = paths.paths.clone();
    
    
    let mut builder = string_builder::Builder::new(20 * paths.len());

    for (id, path) in paths.iter().enumerate() {
        builder.append(wrapper(id,&path.replace(TEMPLATES_PATH, "")));
    } 
    let page: String = format!("
        {{% extends \"{}\" %}}
        {{% block snippet_in %}}
            {}
        {{% endblock snippet_in %}}
        ", template_path, 
        builder.string().unwrap_or(String::from("<div> ERROR Parsing snippet template failed </div>")));
    
    tera.add_raw_template(&template_name, &page).unwrap()

}
