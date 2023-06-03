mod error;
mod tera_test;
mod tera_reload;
mod portfolio;
mod static_page; 

pub use static_page::static_page;
pub use error::handle_rejection;
pub use tera_test::tera_test;
pub use tera_reload::tera_reload;
pub use portfolio::portfolio_page;
