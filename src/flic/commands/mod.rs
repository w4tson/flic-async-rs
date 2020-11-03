use bytes::{BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use super::commands::stream_mapper::CommandToByteMapper;
use super::events::Event;
use super::events::stream_mapper::{ByteToEventMapper, EventResult};

use super::enums::LatencyMode;

pub mod stream_mapper;

pub struct FlicCodec {}

impl Decoder for FlicCodec {
    type Item = EventResult;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut event_mapper = ByteToEventMapper::new();
        let mut result : Option<Self::Item> = None;
        for b in src.iter() {
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
        Ok(result)
    }
    
}

impl Encoder<Command> for FlicCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Command, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut mapper = CommandToByteMapper::new();
        let drain = mapper.map(item);
        dst.extend(drain);
        Ok(())
    }
}

/// Commands
#[derive(Debug)]
pub enum Command {
    GetInfo,
    CreateScanner {
        scan_id: u32,
    },
    RemoveScanner {
        scan_id: u32,
    },
    CreateConnectionChannel {
        conn_id: u32,
        bd_addr: String,
        latency_mode: LatencyMode,
        auto_disconnect_time: i16,
    },
    RemoveConnectionChannel {
        conn_id: u32,
    },
    ForceDisconnect {
        bd_addr: String,
    },
    ChangeModeParameters {
        conn_id: u32,
        latency_mode: LatencyMode,
        auto_disconnect_time: i16,
    },
    Ping {
        ping_id: u32,
    },
    GetButtonInfo {
        bd_addr: String,
    },
    CreateScanWizard {
        scan_wizard_id: u32,
    },
    CancelScanWizard {
        scan_wizard_id: u32,
    },
    DeleteButton {
        bd_addr: String,
    },
    CreateBatteryStatusListener {
        listener_id: u32,
        bd_addr: String,
    },
    RemoveBatteryStatusListener {
        listener_id: u32,
    },
}

impl Command {
    pub fn opcode(&self) -> u8 {
        match self {
            Self::GetInfo { .. } => 0,
            Self::CreateScanner { .. } => 1,
            Self::RemoveScanner { .. } => 2,
            Self::CreateConnectionChannel { .. } => 3,
            Self::RemoveConnectionChannel { .. } => 4,
            Self::ForceDisconnect { .. } => 5,
            Self::ChangeModeParameters { .. } => 6,
            Self::Ping { .. } => 7,
            Self::GetButtonInfo { .. } => 8,
            Self::CreateScanWizard { .. } => 9,
            Self::CancelScanWizard { .. } => 10,
            Self::DeleteButton { .. } => 11,
            Self::CreateBatteryStatusListener { .. } => 12,
            Self::RemoveBatteryStatusListener { .. } => 13,
        }
    }
}
