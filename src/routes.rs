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
    let home_page = warp::path::end().and_then(
        || { static_page("index.html")});
    // Portfolio
    let portfolio_page = warp::path!("portfolio").and_then(
        portfolio_page);

    // Profdev Challange
    let profdev = warp::path!("profdev").and_then(
        || { static_page("profdev.html")});

    

    // Tera template tester 
    let tera_test = warp::get()
            .and(warp::path!("tera" / String / String))
            .boxed().and_then(crate::handlers::tera_test);

    let tera_reload = warp::get().and(warp::path("tera_reload")).boxed().and_then(tera_reload);    

    
    // Put all of the routes together
    home_page
        .or(public_files)
        .or(tera_test)
        .or(portfolio_page)
        .or(tera_reload)
        .or(profdev)
        .recover(handle_rejection).boxed()
}
