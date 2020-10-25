use std::{error::Error, net::SocketAddr};

use futures::{future, SinkExt, StreamExt, TryStreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

use crate::commands::{Command, FlicCodec};
use crate::enums::{LatencyMode};
use crate::events::{Event, EventCodec};
use crate::events::stream_mapper::{ByteToEventMapper, EventResult};

mod commands;
mod events;
mod enums;

const BUTTON : &str = "BUTTON";


pub async fn connect(addr: &SocketAddr) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(addr).await?;
    let (r, w) = stream.split();
    let mut sink = FramedWrite::new(w, FlicCodec{});

    let create_conn = Command::CreateConnectionChannel {
        conn_id: 2,
        bd_addr: BUTTON.to_string(),
        latency_mode: LatencyMode::NormalLatency,
        auto_disconnect_time: 11111_i16,
    };

    sink.send(create_conn).await;

    let (tx,  _rx) = mpsc::channel::<EventResult>(32);
    
    let mut stream = FramedRead::new(r, EventCodec::new()).map_err(|e| eprintln!("asdf"));
    
    while let Some(e) = stream.next().await {
        if let Ok(event) = e {
            println!("got {:?}", event);
            // tx.send(event.clone());
        }
    }

    Ok(())
}
