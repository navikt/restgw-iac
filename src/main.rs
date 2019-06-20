extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod fasit;

fn main() {
    println!("Hello, world!");
    let payload = json! ({
    "hei": "sup"
    });

    println!("{}", serde_json::to_string(&payload).unwrap())
}
