use std::{error::Error, net::SocketAddr};

use futures::{future, SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

use crate::commands::{Command, FlicCodec};
use crate::enums::{LatencyMode};
use crate::events::Event;
use crate::events::stream_mapper::{ByteToEventMapper, EventResult};

mod commands;
mod events;
mod enums;

const BUTTON : &str = "YOUR_BUTTON";


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

    let mut event_mapper = ByteToEventMapper::new();

    let (tx,  _rx) = mpsc::channel::<EventResult>(32);
    
    let mut stream = FramedRead::new(r, BytesCodec::new())
        .filter_map(|i| match i {
            //BytesMut into Bytes
            Ok(i) => {
                // println!("{:#?}",i);
                future::ready(Some(i.freeze()))},
            Err(e) => {
                println!("failed to read from socket; error={}", e);
                future::ready(None)
            }
        })
        .filter_map(|bytes| {
            let mut result : Option<EventResult> = None;
            for b in bytes.iter() {
                match event_mapper.map(*b) {
                    EventResult::None => {}
                    EventResult::Some(Event::NoOp) => {}
                    EventResult::Some(event) => {
                        // eprintln!("event = {:#?}", event); 
                        result = Some(EventResult::Some(event));
                    }
                    _ => {
                    }
                }
            }
            future::ready(result)
        });
        //this does not work
        // .forward(tx);
    
    
    while let Some(e) = stream.next().await {
        let event = e.clone();
        tx.send(event);
        println!("got {:?}", e);
    }

    Ok(())
}
