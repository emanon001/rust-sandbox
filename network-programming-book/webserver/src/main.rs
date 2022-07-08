use log::error;
use std::env;
use structopt::StructOpt;
use webserver::server::WebServer;

#[derive(StructOpt)]
struct Opt {
    #[structopt(help = "[addr:port]")]
    address: String,
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let opt = Opt::from_args();
    let mut server = WebServer::new(&opt.address).unwrap_or_else(|e| {
        error!("{}", e);
        panic!();
    });
    server.run().unwrap_or_else(|e| {
        error!("{}", e);
        panic!();
    });
}
