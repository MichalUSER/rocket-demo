use rocket::fs::FileServer;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Person {
    pub name: String,
    pub age: u64,
}

#[post("/", format = "json", data = "<p>")]
fn api(p: Json<Person>) -> Value {
    json!({
        "greeting": format!("Hello {}!", p.name),
        "age": p.age
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("web"))
        .mount("/api", routes![api])
}
