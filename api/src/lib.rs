mod route;
mod system;
mod monitor;
// mod test;

use silent::prelude::Route;

pub fn get_routes() -> Route {
    route::get_routes()
}
