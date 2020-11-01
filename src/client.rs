use tokio::net::{ToSocketAddrs, TcpStream};
use crate::commands::Command;
use crate::enums::LatencyMode;
use crate::connection::Connection;
use futures::SinkExt;
use tokio::stream::{StreamExt, Stream};
use tokio::sync::mpsc;
use crate::events::stream_mapper::EventResult;
use tokio::sync::mpsc::{Sender, Receiver};
use async_stream::stream;


pub struct Client {
    /// The TCP connection decorated with the redis protocol encoder / decoder
    /// implemented using a buffered `TcpStream`.
    ///
    /// When `Listener` receives an inbound connection, the `TcpStream` is
    /// passed to `Connection::new`, which initializes the associated buffers.
    /// `Connection` allows the handler to operate at the "frame" level and keep
    /// the byte level protocol parsing details encapsulated in `Connection`.
    connection: Connection,
    tx: Sender<EventResult>,
    pub rx: Receiver<EventResult>
}


pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client, std::io::Error> {
    // The `addr` argument is passed directly to `TcpStream::connect`. This
    // performs any asynchronous DNS lookup and attempts to establish the TCP
    // connection. An error at either step returns an error, which is then
    // bubbled up to the caller of `mini_redis` connect.
    let socket = TcpStream::connect(addr).await?;

    // Initialize the connection state. This allocates read/write buffers to
    // perform redis protocol frame parsing.
    let connection = Connection::new(socket);

    let (tx,  rx) = mpsc::channel::<EventResult>(32);
    

    Ok(Client { connection, tx, rx })
}

// impl tokio::stream::Stream<Item = Result<EventResult, std::io::Error>> 

impl Client {
    pub async fn subscribe(&mut self, button: &str) {
        // Issue the subscribe command
        let create_conn_cmd = Command::CreateConnectionChannel {
            conn_id: 2,
            bd_addr: button.to_string(),
            latency_mode: LatencyMode::NormalLatency,
            auto_disconnect_time: 11111_i16,
        };

        self.connection.writer.send(create_conn_cmd).await;

        while let Some(e) = self.connection.reader.next().await {
            if let Ok(event) = e {
                println!("got {:?}", event);
                self.tx.send(event.clone());
            }
        }

        
        
        // // Update the set of subscribed channels.
        // self.subscribed_channels
        //     .extend(channels.iter().map(Clone::clone));
        
    }
}

