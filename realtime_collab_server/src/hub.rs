use crate::protocol::{MemberView, PeerId, RoomEvent};
use bytes::Bytes;
use std::collections::HashMap;
use tokio::sync::{broadcast, mpsc, oneshot};

#[derive(Debug)]
pub enum HubCommand {
    Join {
        room: String,
        name: String,
        responder: oneshot::Sender<JoinResult>,
    },
    Leave {
        room: String,
        peer_id: PeerId,
    },
    Chat {
        room: String,
        peer_id: PeerId,
        text: String,
        ts_ms: u64,
    },
    Whiteboard {
        room: String,
        peer_id: PeerId,
        payload: Bytes,
    },
}

pub struct JoinResult {
    pub room: String,
    pub peer_id: PeerId,
    pub room_rx: broadcast::Receiver<RoomEvent>,
    pub members: Vec<MemberView>,
}

#[derive(Debug, Clone)]
struct Member {
    name: String,
}

#[derive(Debug)]
struct RoomState {
    events_tx: broadcast::Sender<RoomEvent>,
    members: HashMap<PeerId, Member>,
}

pub fn spawn_hub(room_buffer: usize) -> mpsc::Sender<HubCommand> {
    let (tx, mut rx) = mpsc::channel::<HubCommand>(4096);

    tokio::spawn(async move {
        let mut rooms: HashMap<String, RoomState> = HashMap::new();
        let mut next_peer_id: PeerId = 1;

        while let Some(cmd) = rx.recv().await {
            match cmd {
                HubCommand::Join {
                    room,
                    name,
                    responder,
                } => {
                    let room_state = rooms.entry(room.clone()).or_insert_with(|| {
                        let (events_tx, _) = broadcast::channel(room_buffer);
                        RoomState {
                            events_tx,
                            members: HashMap::new(),
                        }
                    });

                    let peer_id = next_peer_id;
                    next_peer_id = next_peer_id.saturating_add(1);

                    let members = room_state
                        .members
                        .iter()
                        .map(|(pid, m)| MemberView {
                            peer_id: *pid,
                            name: m.name.clone(),
                        })
                        .collect::<Vec<_>>();

                    room_state
                        .members
                        .insert(peer_id, Member { name: name.clone() });

                    let room_rx = room_state.events_tx.subscribe();
                    let _ = room_state
                        .events_tx
                        .send(RoomEvent::MemberJoined { peer_id, name });

                    let _ = responder.send(JoinResult {
                        room,
                        peer_id,
                        room_rx,
                        members,
                    });
                }
                HubCommand::Leave { room, peer_id } => {
                    if let Some(room_state) = rooms.get_mut(&room) {
                        if room_state.members.remove(&peer_id).is_some() {
                            let _ = room_state.events_tx.send(RoomEvent::MemberLeft { peer_id });
                        }
                        if room_state.members.is_empty() {
                            rooms.remove(&room);
                        }
                    }
                }
                HubCommand::Chat {
                    room,
                    peer_id,
                    text,
                    ts_ms,
                } => {
                    if let Some(room_state) = rooms.get(&room) {
                        if let Some(member) = room_state.members.get(&peer_id) {
                            let _ = room_state.events_tx.send(RoomEvent::Chat {
                                peer_id,
                                name: member.name.clone(),
                                text,
                                ts_ms,
                            });
                        }
                    }
                }
                HubCommand::Whiteboard {
                    room,
                    peer_id,
                    payload,
                } => {
                    if let Some(room_state) = rooms.get(&room) {
                        let _ = room_state
                            .events_tx
                            .send(RoomEvent::WhiteboardBinary { peer_id, payload });
                    }
                }
            }
        }
    });

    tx
}

