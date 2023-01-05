use std::net::TcpListener;

use ztp::run;

#[tokio::main]
async fn main()->Result<(),std::io::Error>{
    let tcplistener = TcpListener::bind("127.0.0.1:0")?;

    run(tcplistener)?.await
}