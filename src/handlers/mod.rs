mod homepage;
mod error;
mod tera_test;
mod tera_reload;
pub mod portfolio;

pub use error::handle_rejection;
pub use homepage::home_page;
pub use tera_test::tera_test;
pub use tera_reload::tera_reload;
pub use portfolio::portfolio_page;
