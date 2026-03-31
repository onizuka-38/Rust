use bytes::Bytes;
use serde::{Deserialize, Serialize};

pub type PeerId = u64;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientTextFrame {
    Join { room: String, name: String },
    Chat { text: String },
    Ping { client_ts: Option<u64> },
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerTextFrame {
    Hello {
        protocol: &'static str,
        binary_whiteboard_opcode: u8,
    },
    Joined {
        room: String,
        peer_id: PeerId,
        members: Vec<MemberView>,
    },
    MemberJoined {
        peer_id: PeerId,
        name: String,
    },
    MemberLeft {
        peer_id: PeerId,
    },
    Chat {
        peer_id: PeerId,
        name: String,
        text: String,
        ts_ms: u64,
    },
    Pong {
        server_ts_ms: u64,
        client_ts: Option<u64>,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Serialize, Clone)]
pub struct MemberView {
    pub peer_id: PeerId,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum RoomEvent {
    MemberJoined { peer_id: PeerId, name: String },
    MemberLeft { peer_id: PeerId },
    Chat {
        peer_id: PeerId,
        name: String,
        text: String,
        ts_ms: u64,
    },
    WhiteboardBinary {
        peer_id: PeerId,
        payload: Bytes,
    },
}

pub const WHITEBOARD_DRAW_SEGMENT: u8 = 0x01;
pub const WHITEBOARD_CLEAR_CANVAS: u8 = 0x02;
pub const WHITEBOARD_BROADCAST_ENVELOPE: u8 = 0x7F;

pub fn validate_whiteboard_packet(payload: &[u8]) -> Result<(), String> {
    if payload.is_empty() {
        return Err("binary whiteboard payload must not be empty".to_string());
    }
    let opcode = payload[0];
    if opcode != WHITEBOARD_DRAW_SEGMENT && opcode != WHITEBOARD_CLEAR_CANVAS {
        return Err(format!("unsupported whiteboard opcode: {opcode}"));
    }
    Ok(())
}

pub fn wrap_whiteboard_broadcast(peer_id: PeerId, payload: &Bytes) -> Vec<u8> {
    let mut out = Vec::with_capacity(1 + 8 + payload.len());
    out.push(WHITEBOARD_BROADCAST_ENVELOPE);
    out.extend_from_slice(&peer_id.to_le_bytes());
    out.extend_from_slice(payload.as_ref());
    out
}
