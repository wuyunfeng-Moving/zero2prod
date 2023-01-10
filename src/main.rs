use std::net::TcpListener;

use ztp::startup::run;
use ztp::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
let configuration = get_configuration().expect("fail to read configuration!");

    let address = format!("127.0.0.1:{}",configuration.application_port);
    let tcplistener = TcpListener::bind(address)?;
    
    println!("{:?}",tcplistener);
    run(tcplistener)?.await
}
