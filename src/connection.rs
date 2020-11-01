use tokio::net::TcpStream;
use tokio_util::codec::{FramedWrite, FramedRead};
use crate::commands::FlicCodec;
use tokio::net::tcp::{WriteHalf, OwnedWriteHalf, OwnedReadHalf};
use crate::events::EventCodec;


pub struct Connection {
    pub writer: FramedWrite<OwnedWriteHalf, FlicCodec>,
    pub reader: FramedRead<OwnedReadHalf, EventCodec>
}

impl  Connection {
    pub  fn new(socket: TcpStream) -> Connection {
        let (r, w) = socket.into_split();

        Connection {
            writer: FramedWrite::new(w, FlicCodec {}),
            reader: FramedRead::new(r, EventCodec::new())
        }
    }
    
    
}