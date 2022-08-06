use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    //Cli product
    Cli,
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "rust-hexagonal",
    about = "A command line rust-hexagonal project"
)]

pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub command: Command,

    //flags
    #[structopt(short = "a", long, default_value = "get")]
    pub action: String,

    #[structopt(short = "i", long, default_value = "")]
    pub product_id: String,

    #[structopt(short = "n", long, default_value = "")]
    pub product_name: String,

    #[structopt(short = "p", long, default_value = "0.0")]
    pub product_price: String,
}
