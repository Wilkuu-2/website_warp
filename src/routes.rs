//
// Routes
//
//
//
use crate::handlers::*;
use warp::{Filter, filters::BoxedFilter, Reply};

pub fn api() -> BoxedFilter<(impl Reply, )>{
    // Tera related API
    let tera_test = warp::get()
            .and(warp::path("tera"))
            .and(warp::path::param::<String>())
            .and(warp::path::param::<String>())
            .boxed().and_then(crate::handlers::tera_test);
    let tera_reload = warp::get().and(warp::path("tera_reload")).boxed().and_then(tera_reload);    

    tera_test
        .or(tera_reload).boxed() 
}

pub fn pages() -> BoxedFilter<(impl Reply, )>{
    // Home page
    let home_page = warp::path::end().and_then(home_page);
    let portfolio_page = warp::path("portfolio").and_then(portfolio_page);

    home_page 
        .or(portfolio_page).boxed()

} 

pub fn routes() -> BoxedFilter<(impl Reply, )>{
    // All files 
    let public_files = warp::fs::dir("./public/");
    // Put all of the routes together
    public_files
        .or(api())
        .or(pages())
        .recover(handle_rejection)
        .boxed()
    
}
