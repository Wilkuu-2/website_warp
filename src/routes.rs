//
// Routes
//
//
//

#[macro_export]
macro_rules! routes  {
    () => {
        {
        // All files 
        let public_files = warp::fs::dir("./public/");
        // Home page
        let home_page = warp::path::end().and_then(handlers::home_page);
        let portfolio_page = warp::path("portfolio").and_then(handlers::portfolio_page);
        // Tera template tester 
        let tera_test = warp::get()
                .and(warp::path("tera"))
                .and(warp::path::param::<String>())
                .and(warp::path::param::<String>())
                .boxed().and_then(handlers::tera_test);

        

        
        // Put all of the routes together
        home_page
            .or(public_files)
            .or(tera_test)
            .or(portfolio_page)
            .recover(handlers::handle_rejection).boxed()
        }
            
    };
} 
