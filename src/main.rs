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
use std::sync::Arc;

use structopt::StructOpt;
use tokio::stream::StreamExt;
use tokio::sync::Mutex;

use flic_async_rs::flic::client::connect;
use flic_async_rs::lights_controller::LightController;

const OK: &'static str = "\x1b[0;92m[Ok]\x1b[0m";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::from_args();

    print!("Connecting to Hue bridge...\n");
    let lights = LightController::new(&options.username).await;
    lights.list_all();
    let lights_mutex = Arc::new(Mutex::new(lights));
    println!("{}", &OK);

    print!("Connecting to Flic Server...");
    let addr = format!("{}:5551", options.server).parse::<SocketAddr>()?;
    let mut client = connect(addr).await?;
    client.subscribe(&options.button).await;
    println!("{}", &OK);

    let splash = include_str!("splash.txt");
    
    let splash = format!("\x1b[0;92m{}", splash)
        .replace("#TITLE#", "\x1b[31;1mFlicFun\x1b[0;92m")
        .replace("#SUBTITLE#", "\x1b[34;1mRust + \x1b[36;1mFlic + \x1b[35;1mHue\x1b[0;92m");
    println!("{}\x1b[0m",splash);

    while let Some(e) = client.connection.reader.next().await {
        if let Ok(event) = e {
            let lights_mutex = Arc::clone(&lights_mutex);

            tokio::spawn(async move {
                let lights_mutex = Arc::clone(&lights_mutex);
                let light_controller = lights_mutex.lock().await;
                light_controller.process_event_result(event).await;
            });
        }
    }

    println!("\x1b[0;92m[Done]\x1b[0m");
    
    Ok(())
}


#[derive(Debug, StructOpt)]
#[structopt(name = "flicfun", about = "Hacking on the flic button")]
pub struct Options {
    #[structopt(short = "s", long = "server", env = "FLIC_SERVER")]
    /// the hostname of the flicd server
    pub server: String,

    #[structopt(short = "b", long = "button", env = "FLIC_BUTTON")]
    /// the mac address of the flic button to connect to 
    pub button: String,

    #[structopt(short = "u", long = "hue-user", env = "HUE_USERNAME")]
    /// the hostname of the hue bridge
    pub username: String,
}
