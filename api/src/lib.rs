mod route;
mod system;
// mod test;

use silent::prelude::Route;

pub fn get_routes() -> Route {
    route::get_routes()
}
