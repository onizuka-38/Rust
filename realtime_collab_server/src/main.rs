mod hub;
mod protocol;

use anyhow::{Context, Result};
use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use hub::{spawn_hub, HubCommand, JoinResult};
use protocol::{
    validate_whiteboard_packet, wrap_whiteboard_broadcast, ClientTextFrame, RoomEvent, ServerTextFrame,
    WHITEBOARD_DRAW_SEGMENT,
};
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

#[derive(Parser, Debug)]
#[command(name = "realtime-collab-server")]
#[command(about = "Low-latency async WebSocket chat + binary whiteboard server")]
struct Cli {
    #[arg(long, default_value = "0.0.0.0:9001")]
    listen: String,

    #[arg(long, default_value_t = 2048)]
    room_broadcast_buffer: usize,

    #[arg(long, default_value_t = 512)]
    client_send_buffer: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let listener = TcpListener::bind(&cli.listen)
        .await
        .with_context(|| format!("failed to bind {}", cli.listen))?;

    let hub_tx = spawn_hub(cli.room_broadcast_buffer);

    println!("server listening on ws://{}", cli.listen);
    loop {
        let (stream, addr) = listener.accept().await?;
        let hub_tx = hub_tx.clone();
        let client_send_buffer = cli.client_send_buffer;

        tokio::spawn(async move {
            if let Err(err) = handle_connection(stream, addr, hub_tx, client_send_buffer).await {
                eprintln!("connection {} error: {err:#}", addr);
            }
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    hub_tx: mpsc::Sender<HubCommand>,
    client_send_buffer: usize,
) -> Result<()> {
    let ws_stream = accept_async(stream)
        .await
        .with_context(|| format!("websocket handshake failed for {addr}"))?;

    let (mut ws_write, mut ws_read) = ws_stream.split();
    let (out_tx, mut out_rx) = mpsc::channel::<Message>(client_send_buffer);

    let writer_task = tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            if ws_write.send(msg).await.is_err() {
                break;
            }
        }
    });

    send_text(
        &out_tx,
        &ServerTextFrame::Hello {
            protocol: "json-text + binary-whiteboard-v1",
            binary_whiteboard_opcode: WHITEBOARD_DRAW_SEGMENT,
        },
    )
    .await?;

    let join = await_join(&mut ws_read, &hub_tx, &out_tx).await?;
    send_text(
        &out_tx,
        &ServerTextFrame::Joined {
            room: join.room.clone(),
            peer_id: join.peer_id,
            members: join.members,
        },
    )
    .await?;

    let room = join.room.clone();
    let peer_id = join.peer_id;

    let out_tx_for_room = out_tx.clone();
    let room_forward_task = tokio::spawn(async move {
        let mut rx = join.room_rx;
        loop {
            match rx.recv().await {
                Ok(RoomEvent::MemberJoined { peer_id, name }) => {
                    if send_text(
                        &out_tx_for_room,
                        &ServerTextFrame::MemberJoined { peer_id, name },
                    )
                    .await
                    .is_err()
                    {
                        break;
                    }
                }
                Ok(RoomEvent::MemberLeft { peer_id }) => {
                    if send_text(&out_tx_for_room, &ServerTextFrame::MemberLeft { peer_id })
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Ok(RoomEvent::Chat {
                    peer_id,
                    name,
                    text,
                    ts_ms,
                }) => {
                    if send_text(
                        &out_tx_for_room,
                        &ServerTextFrame::Chat {
                            peer_id,
                            name,
                            text,
                            ts_ms,
                        },
                    )
                    .await
                    .is_err()
                    {
                        break;
                    }
                }
                Ok(RoomEvent::WhiteboardBinary { peer_id, payload }) => {
                    let packet = wrap_whiteboard_broadcast(peer_id, &payload);
                    if out_tx_for_room.send(Message::Binary(packet)).await.is_err() {
                        break;
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                    let _ = send_text(
                        &out_tx_for_room,
                        &ServerTextFrame::Error {
                            message: "You are lagging behind. Some room events were dropped.".to_string(),
                        },
                    )
                    .await;
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    while let Some(frame) = ws_read.next().await {
        match frame {
            Ok(Message::Text(text)) => {
                if let Ok(cmd) = serde_json::from_str::<ClientTextFrame>(&text) {
                    match cmd {
                        ClientTextFrame::Join { .. } => {
                            send_text(
                                &out_tx,
                                &ServerTextFrame::Error {
                                    message: "already joined".to_string(),
                                },
                            )
                            .await?;
                        }
                        ClientTextFrame::Chat { text } => {
                            hub_tx
                                .send(HubCommand::Chat {
                                    room: room.clone(),
                                    peer_id,
                                    text,
                                    ts_ms: now_ms(),
                                })
                                .await
                                .context("failed to send chat to hub")?;
                        }
                        ClientTextFrame::Ping { client_ts } => {
                            send_text(
                                &out_tx,
                                &ServerTextFrame::Pong {
                                    server_ts_ms: now_ms(),
                                    client_ts,
                                },
                            )
                            .await?;
                        }
                    }
                } else {
                    send_text(
                        &out_tx,
                        &ServerTextFrame::Error {
                            message: "invalid JSON command".to_string(),
                        },
                    )
                    .await?;
                }
            }
            Ok(Message::Binary(payload)) => {
                if let Err(msg) = validate_whiteboard_packet(&payload) {
                    send_text(&out_tx, &ServerTextFrame::Error { message: msg }).await?;
                    continue;
                }
                hub_tx
                    .send(HubCommand::Whiteboard {
                        room: room.clone(),
                        peer_id,
                        payload: bytes::Bytes::from(payload),
                    })
                    .await
                    .context("failed to send whiteboard payload to hub")?;
            }
            Ok(Message::Ping(v)) => {
                out_tx.send(Message::Pong(v)).await.ok();
            }
            Ok(Message::Pong(_)) => {}
            Ok(Message::Close(_)) => break,
            Err(err) => {
                eprintln!("socket {} read error: {err}", addr);
                break;
            }
            _ => {}
        }
    }

    hub_tx
        .send(HubCommand::Leave { room, peer_id })
        .await
        .ok();

    room_forward_task.abort();
    drop(out_tx);
    writer_task.abort();

    Ok(())
}

async fn await_join(
    ws_read: &mut futures_util::stream::SplitStream<WebSocketStream<TcpStream>>,
    hub_tx: &mpsc::Sender<HubCommand>,
    out_tx: &mpsc::Sender<Message>,
) -> Result<JoinResult> {
    while let Some(frame) = ws_read.next().await {
        match frame {
            Ok(Message::Text(text)) => {
                let parsed = match serde_json::from_str::<ClientTextFrame>(&text) {
                    Ok(v) => v,
                    Err(_) => {
                        send_text(
                            out_tx,
                            &ServerTextFrame::Error {
                                message: "join frame must be valid JSON command".to_string(),
                            },
                        )
                        .await?;
                        continue;
                    }
                };

                if let ClientTextFrame::Join { room, name } = parsed {
                    if room.trim().is_empty() || name.trim().is_empty() {
                        send_text(
                            out_tx,
                            &ServerTextFrame::Error {
                                message: "room and name must not be empty".to_string(),
                            },
                        )
                        .await?;
                        continue;
                    }

                    let (reply_tx, reply_rx) = oneshot::channel();
                    hub_tx
                        .send(HubCommand::Join {
                            room,
                            name,
                            responder: reply_tx,
                        })
                        .await
                        .context("failed to send join command to hub")?;

                    return reply_rx.await.context("hub dropped join response");
                }

                send_text(
                    out_tx,
                    &ServerTextFrame::Error {
                        message: "first command must be join".to_string(),
                    },
                )
                .await?;
            }
            Ok(Message::Binary(_)) => {
                send_text(
                    out_tx,
                    &ServerTextFrame::Error {
                        message: "binary frames are allowed only after join".to_string(),
                    },
                )
                .await?;
            }
            Ok(Message::Close(_)) => anyhow::bail!("client closed before join"),
            Ok(_) => {}
            Err(err) => return Err(err.into()),
        }
    }

    anyhow::bail!("client disconnected before join")
}

async fn send_text(out_tx: &mpsc::Sender<Message>, frame: &ServerTextFrame) -> Result<()> {
    let json = serde_json::to_string(frame)?;
    out_tx
        .send(Message::Text(json))
        .await
        .context("failed to queue outbound message")?;
    Ok(())
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
