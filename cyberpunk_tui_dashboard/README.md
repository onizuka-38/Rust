# Cyberpunk TUI Dashboard

서버 리소스를 실시간으로 확인하는 터미널 대시보드입니다. `ratatui`와 `crossterm`으로 화면을 그리고, `tokio` task가 시스템 메트릭을 주기적으로 수집합니다.

## 기능

- CPU 코어별 사용률 표시
- 메모리 사용량과 사용률 표시
- 네트워크 RX/TX 처리량 표시
- CPU 기준 상위 프로세스 테이블
- NVIDIA NVML 사용 가능 시 GPU VRAM, 온도, 사용률 표시
- 글리치 스타일 타이틀과 네온 색상 테마
- 입력 처리와 메트릭 수집을 분리해 UI 갱신 지연을 줄임

## 기술 스택

- `ratatui`: 터미널 UI
- `crossterm`: 터미널 제어와 키 입력
- `tokio`: 비동기 루프
- `sysinfo`: CPU/메모리/네트워크/프로세스 메트릭
- `nvml-wrapper`: NVIDIA GPU 메트릭

## 실행

```powershell
cd cyberpunk_tui_dashboard
cargo run
```

종료 키:

- `q`
- `Esc`

## 아키텍처

- Collector task가 일정 주기로 시스템 상태를 수집합니다.
- 최신 메트릭은 `tokio::sync::watch` 채널로 UI loop에 전달됩니다.
- UI loop는 고정 tick마다 렌더링하고, 키 입력 이벤트를 별도 스레드에서 받습니다.
- NVML이 없으면 GPU 패널은 사용 불가 상태로 동작하고 나머지 대시보드는 계속 실행됩니다.
