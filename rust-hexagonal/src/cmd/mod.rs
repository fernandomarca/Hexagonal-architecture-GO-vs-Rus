use error_stack::Report;
use rusqlite::Connection;
use structopt::StructOpt;

use crate::{
    adapters::{
        cli::cli_product::{self, CliError},
        db::product_db::ProductDb,
        web::server::web_server::WebServer,
    },
    application::product_service::ProductService,
};

use self::cmmd::CommandLineArgs;

mod cmmd;

pub fn connection_db_factory() -> ProductService {
    let db = Connection::open("sqlite.db").unwrap();
    let product_db_adapter = ProductDb::new(db);
    ProductService::new(Box::new(product_db_adapter))
}

pub async fn execute() -> error_stack::Result<String, CliError> {
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
        cmmd::Command::Http => {
            let server = WebServer::new(Box::new(connection_db_factory()));

            let server_started = server.serve().await;
            match server_started {
                Ok(server) => {
                    println!("Web server has been started in port: 9001");
                    let result = server.await;
                    match result {
                        Ok(_) => Ok("web server async method stopped".to_string()),
                        Err(e) => Err(Report::new(CliError(format!(
                            "web server internal error: {} {}",
                            e.kind(),
                            e
                        )))),
                    }
                }
                Err(e) => Err(e.change_context(CliError("web server bind addr error".to_owned()))),
            }
        }
    }
}
