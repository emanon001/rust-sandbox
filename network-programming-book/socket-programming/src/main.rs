use log::error;
use std::env;
use std::str::FromStr;
use structopt::StructOpt;

use socket_programming::{tcp_client, tcp_server, udp_client, udp_server};

enum Protocol {
    Tcp,
    Udp,
}

impl FromStr for Protocol {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tcp" => Ok(Protocol::Tcp),
            "udp" => Ok(Protocol::Udp),
            _ => Err("Please specify tcp or udp".into()),
        }
    }
}

enum Role {
    Server,
    Client,
}

impl FromStr for Role {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "server" => Ok(Role::Server),
            "client" => Ok(Role::Client),
            _ => Err("Please specify server or client".into()),
        }
    }
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(help = "[tcp|udp]")]
    protocol: Protocol,
    #[structopt(help = "[server|client]")]
    role: Role,
    #[structopt(help = "[addr:port]")]
    address: String,
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let opt = Opt::from_args();
    let protocol = opt.protocol;
    let role = opt.role;
    let address = &opt.address;
    match protocol {
        Protocol::Tcp => match role {
            Role::Server => {
                tcp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            Role::Client => {
                tcp_client::connect(address).unwrap_or_else(|e| error!("{}", e));
            }
        },
        Protocol::Udp => match role {
            Role::Server => {
                udp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            Role::Client => {
                udp_client::communicate(address).unwrap_or_else(|e| error!("{}", e));
            }
        },
    }
}
