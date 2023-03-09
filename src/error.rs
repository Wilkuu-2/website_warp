#[derive(Debug)]
pub struct RouteClosed {
    pub name: String
}
impl warp::reject::Reject for RouteClosed {} 

