# Realtime Collab Server

`tokio + tokio-tungstenite + mpsc + broadcast` 기반의 고동시성 실시간 채팅/화이트보드 서버입니다.

## 핵심 포인트
- 방(Room) 단위 `broadcast` 채널: O(1)에 가까운 fan-out 경로로 브로드캐스트 지연 최소화
- 연결(Connection) 단위 `mpsc` 채널: 소켓 write를 분리해 느린 클라이언트가 전체 방 전파를 막지 않게 설계
- 텍스트(JSON) + 바이너리(화이트보드) 이중 프로토콜
- `broadcast` 버퍼 기반 역압(backpressure) 처리: 느린 수신자는 이벤트 유실을 명시적으로 통지

## 빠른 시작
```bash
cd realtime_collab_server
cargo run -- --listen 0.0.0.0:9001 --room-broadcast-buffer 2048 --client-send-buffer 512
```

브라우저 예제 클라이언트:
- `examples/whiteboard_client.html` 파일을 브라우저로 열고 `ws://127.0.0.1:9001` 접속

## 텍스트 프로토콜(JSON)
클라이언트 -> 서버
```json
{"type":"join","room":"team-a","name":"alice"}
{"type":"chat","text":"hello"}
{"type":"ping","client_ts":1710000000000}
```

서버 -> 클라이언트
- `hello`, `joined`, `member_joined`, `member_left`, `chat`, `pong`, `error`

## 바이너리 화이트보드 프로토콜
클라이언트 -> 서버 payload:
- Draw Segment: `0x01 + x1(f32 LE) + y1(f32 LE) + x2(f32 LE) + y2(f32 LE) + rgba(u32 LE) + width(f32 LE)`
- Clear Canvas: `0x02`

서버 -> 클라이언트 payload:
- `0x7F + peer_id(u64 LE) + 원본 payload`

## 지연 최소화 설계
- 방별 중앙 fan-out은 `broadcast`로 처리해 N명의 개별 락 획득/반복 전송 비용을 줄였습니다.
- 각 연결은 별도 writer task(`mpsc` 소비)로 동작해 read path와 write path를 분리했습니다.
- 바이너리 이벤트는 JSON 직렬화를 거치지 않아 CPU 부하를 줄입니다.
- 대용량/고동시성에서 느린 소비자 문제를 `Lagged` 이벤트로 감지해 시스템 전체 지연 급증을 방지합니다.

## 운영 팁
- 코어 수가 많은 환경: `TOKIO_WORKER_THREADS`를 CPU 코어와 맞춰 튜닝
- 방 이벤트 폭주 시: `--room-broadcast-buffer`를 트래픽 피크에 맞춰 확장
- 느린 모바일 클라이언트가 많으면: `--client-send-buffer`를 다소 늘리고, 클라이언트에서 프레임 샘플링 적용
