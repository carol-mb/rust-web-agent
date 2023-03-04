use rocket::launch;

mod routes;

use routes::get_all_routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", get_all_routes())
}
