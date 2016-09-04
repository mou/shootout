extern crate iron;
extern crate params;

use iron::prelude::*;
use iron::status;
use params::{Value, Params};

fn generate_response(x: i64, y: i64) -> IronResult<Response> {

    Ok(Response::with((status::Ok, "Response")))
}

fn shootout(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();
    let x = map.find(&["x"]);
    let y = map.find(&["y"]);
    match (x, y) {
        (Some(&Value::I64(x)), Some(&Value::I64(y))) => { generate_response(x, y) }
        (Some(&Value::String(ref x)), Some(&Value::String(ref y))) => {
            match (x.parse::<i64>(), x.parse::<i64>()) {
                (Ok(x), Ok(y)) => { generate_response(x, y) }
                _ => Ok(Response::with((status::BadRequest, "Invalid arguments. Not numbers")))
            }}
        (x, y) => { println!("{:?} {:?}", x, y); Ok(Response::with((status::BadRequest, "Not enought arguments")))}
    }
}

fn main() {

    Iron::new(shootout).http("localhost:3000").unwrap();
    println!("On 3000");
}