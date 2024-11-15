use silent::prelude::Route;

pub use services::BPM_ENGINE;

pub fn get_routes() -> Route {
    Route::new("api")
}
