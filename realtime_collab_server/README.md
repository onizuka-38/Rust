# Realtime Collab Server

`tokio`, `tokio-tungstenite`, `mpsc`, `broadcast`를 사용한 실시간 채팅/화이트보드 WebSocket 서버입니다. 텍스트 명령은 JSON으로, 화이트보드 이벤트는 바이너리 프레임으로 처리합니다.

## 기능

- 방(Room) 단위 참여
- 참여자 입장/퇴장 이벤트 브로드캐스트
- 채팅 메시지 브로드캐스트
- ping/pong 지연 측정용 프레임
- 바이너리 화이트보드 draw/clear 이벤트 전달
- 느린 클라이언트가 방 전체 전파를 막지 않도록 연결별 writer task 분리
- `broadcast` lag 감지 시 클라이언트에 오류 메시지 전달

## 실행

```powershell
cd realtime_collab_server
cargo run -- --listen 0.0.0.0:9001 --room-broadcast-buffer 2048 --client-send-buffer 512
```

브라우저 예제:

- `examples/whiteboard_client.html` 파일을 열고 `ws://127.0.0.1:9001`에 접속합니다.

## 텍스트 프로토콜

클라이언트에서 서버로 보내는 JSON:

```json
{"type":"join","room":"team-a","name":"alice"}
{"type":"chat","text":"hello"}
{"type":"ping","client_ts":1710000000000}
```

서버에서 클라이언트로 보내는 이벤트:

- `hello`
- `joined`
- `member_joined`
- `member_left`
- `chat`
- `pong`
- `error`

## 바이너리 화이트보드 프로토콜

클라이언트에서 서버로 보내는 payload:

- Draw Segment: `0x01 + x1(f32 LE) + y1(f32 LE) + x2(f32 LE) + y2(f32 LE) + rgba(u32 LE) + width(f32 LE)`
- Clear Canvas: `0x02`

서버에서 클라이언트로 보내는 payload:

- `0x7F + peer_id(u64 LE) + 원본 payload`

## 설계 메모

- 방별 중앙 fan-out은 `broadcast` 채널로 처리합니다.
- 연결별 `mpsc` writer task를 두어 read path와 write path를 분리합니다.
- 바이너리 화이트보드 이벤트는 JSON 직렬화 비용을 피합니다.
- 방 이벤트 폭주에 대비해 `--room-broadcast-buffer`와 `--client-send-buffer`를 조정할 수 있습니다.
