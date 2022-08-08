mod adapters;
mod application;
mod cmd;

#[actix_web::main]
async fn main() {
    let result = cmd::execute().await.expect("error main cli");
    println!("{}", result);
}
