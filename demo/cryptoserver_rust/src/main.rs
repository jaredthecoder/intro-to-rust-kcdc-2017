#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;

use rand::Rng;

const BASE62: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

#[derive(FromForm)]
struct IDSpec {
    size: usize
}

#[get("/")]
fn index() -> &'static str {
    "
        Welcome to CryptoServer, where you can generate random, secure, and unique IDs!

        USAGE:

            GET /generate/<size>

                returns a random, secure, and unique ID with the given size of characters generated on-the-fly
    "
}

fn get_secure_id(size: usize) -> String {
    let mut id = String::with_capacity(size);
    let mut rng = rand::thread_rng();
    for _ in 0..size {
        id.push(BASE62[rng.gen::<usize>() % 62] as char);
    }

    id
}

#[get("/generate?<idspec>")]
fn generate(idspec: IDSpec) -> String {
    get_secure_id(idspec.size)
}

fn main() {
    rocket::ignite().mount("/", routes![index, generate]).launch();
}
