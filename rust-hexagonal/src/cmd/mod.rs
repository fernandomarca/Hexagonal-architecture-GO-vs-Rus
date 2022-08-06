use rusqlite::Connection;
use structopt::StructOpt;

use crate::{
    adapters::{
        cli::cli_product::{self, CliError},
        db::product_db::ProductDb,
    },
    application::product_service::ProductService,
};

use self::cmmd::CommandLineArgs;

mod cmmd;

fn connection_db_factory() -> ProductService {
    let db = Connection::open("sqlite.db").unwrap();
    let product_db_adapter = ProductDb::new(db);
    ProductService::new(Box::new(product_db_adapter))
}

pub fn execute() -> error_stack::Result<String, CliError> {
    let CommandLineArgs {
        command,
        action,
        product_id,
        product_name,
        product_price,
    } = CommandLineArgs::from_args();

    match command {
        cmmd::Command::Cli => {
            let service = connection_db_factory();
            let price = product_price
                .parse::<f64>()
                .expect("price should be number");

            Ok(cli_product::run(
                Box::new(service),
                &action,
                &product_name,
                price,
                &product_id,
            )?)
        }
    }
}
