#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[derive(FromForm)]
struct MyStruct {
    pub query_param: u32,
}

// Call this like so: curl -X GET http://localhost:8000/?query_param=101
#[get("/?<param_name>")]
fn root(param_name: MyStruct) -> String {
    println!("{}", param_name.query_param);
    String::from("Ok\r\n")
}

fn main() {
    rocket::ignite().mount("/", routes![root]).launch();
}