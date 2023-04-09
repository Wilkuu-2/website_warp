//
// Routes
//
//
//
use crate::handlers::*;
use warp::{Filter, filters::BoxedFilter, Reply};

pub fn routes() -> BoxedFilter<(impl Reply, )>{
    // All files 
    let public_files = warp::fs::dir("./public/");
    // Home page
    let home_page = warp::path::end().and_then(home_page);
    let portfolio_page = warp::path("portfolio").and_then(portfolio_page);
    // Tera template tester 
    let tera_test = warp::get()
            .and(warp::path("tera"))
            .and(warp::path::param::<String>())
            .and(warp::path::param::<String>())
            .boxed().and_then(crate::handlers::tera_test);

    let tera_reload = warp::get().and(warp::path("tera_reload")).boxed().and_then(tera_reload);    

    
    // Put all of the routes together
    home_page
        .or(public_files)
        .or(tera_test)
        .or(portfolio_page)
        .or(tera_reload)
        .recover(handle_rejection).boxed()
}
