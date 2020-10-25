//! An example of hooking up stdin/stdout to either a TCP or UDP stream.
//!
//! This example will connect to a socket address specified in the argument list
//! and then forward all data read on stdin to the server, printing out all data
//! received on stdout. An optional `--udp` argument can be passed to specify
//! that the connection should be made over UDP instead of TCP, translating each
//! line entered on stdin to a UDP packet to be sent to the remote address.
//!
//! Note that this is not currently optimized for performance, especially
//! around buffer management. Rather it's intended to show an example of
//! working with a client.
//!
//! This example can be quite useful when interacting with the other examples in
//! this repository! Many of them recommend running this as a simple "hook up
//! stdin/stdout to a server" to get up and running.

#![warn(rust_2018_idioms)]



use std::error::Error;
use std::net::SocketAddr;
use flic_client::connect;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse what address we're going to connect to
    let addr = "x.x.x.x:5551";
    let addr = addr.parse::<SocketAddr>()?;

    connect(&addr).await?;
    
    Ok(())
}
