
extern crate iron;

use iron::prelude::*;

fn hello_world(_: &mut Request) -> IronResult<Response> {
	let x: String = fib(15).to_string();
    Ok(Response::with((iron::status::Ok, x)))
}

fn main() {
    println!("==>> booting http server on 127.0.0.1:3000");

    let chain = Chain::new(hello_world);
    Iron::new(chain).http("127.0.0.1:3000").unwrap();
}

fn fib(term: usize) -> u32 {
    match term {
        0 =>  0,
        1 =>  1,
        _ => fib(term - 1) + fib(term - 2),
    }
}
