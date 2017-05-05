#![plugin(rocket_codegen)]


#[get("/")]
pub fn index() -> &'static str {
    "posts index"
}

#[get("/<name>")]
pub fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}
