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

pub const TEMPLATES_PATH: &str = "templates/";

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
        Ok(())
}


pub async fn render(name: &str, ctx: &Context) -> Result<String, Rejection> {
    let tera = TERA.read().await; 
    match tera.render(name, &ctx) {
        Ok(page) => Ok(page),
        Err(err) => Err(custom(TemplateError::from(err))),
    }
}

async fn render_one_off(page: &str, ctx: &Context) -> Result<String, Rejection>{
    let mut tera = TERA.write().await;
    match tera.render_str(page, &ctx) {
        Ok(page) => Ok(page),
        Err(err) => Err(custom(TemplateError::from(err))),
    }
}

pub async fn snippet_template_page(super_path: &str,
                                   snippets_path: &str,
                                   wrapper: SnippetWrapper,
                                   context: &Context) 
    -> Result<String, Rejection> {
    
    let paths: Vec<_> = SnippetPaths::_from_folder_async(snippets_path, 30, true).await.paths.clone();
    let mut builder = string_builder::Builder::new(20 * paths.len());

    for (id, path) in paths.iter().enumerate() {
        builder.append(wrapper(id,&path.replace(TEMPLATES_PATH, "")));
    } 
    let page: String = format!(
        "
        {{% extends \"{}\" %}}
        {{% block snippet_in %}}
            {}
        {{% endblock snippet_in %}}
        ", 
        super_path, 
        builder.string().unwrap_or(String::from("<div> ERROR Parsing snippet template failed </div>")));
        
    render_one_off(&page, &context).await    
}
