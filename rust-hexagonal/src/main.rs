mod adapters;
mod application;
mod cmd;
fn main() {
    let result = cmd::execute().expect("error main cli");
    println!("{}", result)
}
