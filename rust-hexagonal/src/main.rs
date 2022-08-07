mod adapters;
mod application;
mod cmd;

use std::ops::Add;
fn main() {
    let result = cmd::execute().expect("error main cli");
    println!("{}", result);
}
