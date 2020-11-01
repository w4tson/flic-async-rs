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
use flic_client::connection::Connection;
use flic_client::client::connect;
use tokio::stream::{StreamExt};
// use futures::{StreamExt, TryFutureExt};
use async_stream::stream;
use async_stream::try_stream;

use futures::stream::Stream;
use futures::pin_mut;
use tokio::join;

use std::io;
use tokio::net::{TcpStream, TcpListener};


const BUTTON : &str = "80:e4:da:71:64:d6";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse what address we're going to connect to
    let addr = "192.168.1.149:5551";
    let addr = addr.parse::<SocketAddr>()?;

    let mut client = connect(addr).await?;
    
    println!("**********");

    let incoming = foo().await;

    // let mut listener = TcpListener::bind("127.0.0.1:4567").await.unwrap();
    // 
    // let incoming = stream! {
    //     loop {
    //         let (socket, _) = listener.accept().await.unwrap();
    //         yield socket;
    //     }
    // };


    let f1 = tokio::spawn(async {
        pin_mut!(incoming);
        while let Some(v) = incoming.next().await {
            println!("handle = {:?}", v);
        }
    });
    
    let f2 = client.subscribe(BUTTON);

    join!(f1, f2);
    
    println!("done");
    
    Ok(())
}



async fn  foo() -> impl Stream<Item = Result<TcpStream, ()>> {
    let mut listener = TcpListener::bind("127.0.0.1:4567").await.unwrap();

    let incoming = try_stream! {
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            yield socket;
        }
    };
    incoming
}