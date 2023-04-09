use tera::Tera;
use tera::Context;
use tokio::sync::RwLock;
use core::fmt;
use std::error::Error;
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
    pub static ref TERA: RwLock<Tera> = {
        RwLock::new(init_tera())
    };
}

fn init_tera() -> Tera {
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
                              crate::handlers::portfolio::portfolio_page_wrapper).unwrap();
    
        tera.autoescape_on(vec![".html",".sql"]);
        tera
}

#[derive(Debug)]
pub enum TemplateError {   
    TeraError(tera::Error),
}
impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::TeraError(err)  => write!(f,"Tera had a fuckie wuckie: {}", err),
        }
    }
} 
impl Error for TemplateError{}
impl Reject for TemplateError {}
impl From<tera::Error> for TemplateError {
    fn from(value: tera::Error) -> Self {
        TemplateError::TeraError(value)
    }
}


pub async fn reload_tera() -> Result<(), TemplateError> {
        let mut tera = TERA.write().await; 
        tera.full_reload()?;
        let paths = SnippetPaths::from_folder("templates/portfolio/", 20, true);
        snippet_template_page(& mut tera,
                              "portfolio.done",
                              "portfolio.html",
                              &paths, 
                              crate::handlers::portfolio::portfolio_page_wrapper)?;
        
        Ok(())
}


pub async fn render(name: &str, ctx: &Context) -> Result<String, Rejection> {
    let tera = TERA.read().await; 
    match tera.render(name, &ctx) {
        Ok(page) => Ok(page),
        Err(err) => Err(custom(TemplateError::from(err))),
    }
}

pub fn snippet_template_page( tera: & mut Tera,
                                template_name: &str,
                                template_path: &str,
                                   paths: &SnippetPaths,
                                   wrapper: SnippetWrapper) -> Result<(), TemplateError> {
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
    
    tera.add_raw_template(&template_name, &page).map_err(|err| {TemplateError::from(err)})

}
