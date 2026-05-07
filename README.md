# Rust Projects

Rust 학습, 시스템 프로그래밍, 백엔드 서비스, AI 포트폴리오 실험을 모아 둔 독립 프로젝트 모음입니다. 루트는 Cargo workspace가 아니며, 각 하위 폴더가 개별 `Cargo.toml`을 가진 별도 프로젝트입니다.

<div align="center">
  <img src="https://github-readme-stats.vercel.app/api?username=onizuka-38&cache_seconds=21600" height="180" />
  <img src="https://github-readme-stats.vercel.app/api/top-langs/?username=onizuka-38&layout=compact&cache_seconds=21600" height="180" />
</div>

## 프로젝트 목록

| 폴더 | 설명 |
|---|---|
| `incident_commander_rs` | Mock, Ollama, vLLM/OpenAI-compatible 백엔드를 지원하는 기업형 AI 장애 분석 서비스 |
| `hello_rust` | Rust 모듈과 진입점을 확인하는 최소 예제 |
| `budget_tracker` | 로컬 JSON 파일 기반 CLI 가계부 |
| `fast_data_processor` | 병렬 로그 스캐너와 NDJSON 거래 통계 처리기 |
| `python_performance_bridge` | Python 텍스트 전처리를 가속하는 PyO3 브리지 |
| `legacy_c_ffi_bridge` | 레거시 C 행렬 연산을 감싼 safe Rust 래퍼와 Python 브리지 |
| `biofeedback_engine` | EDF 파싱과 EEG/EMG 신호 분석 파이프라인 |
| `cyberpunk_tui_dashboard` | `ratatui` 기반 터미널 시스템 모니터링 대시보드 |
| `realtime_collab_server` | WebSocket 채팅과 바이너리 화이트보드 협업 서버 |
| `rust_practice_notebook` | 문법, 표준 라이브러리, async, 테스트, 알고리즘 연습용 독립 Rust 파일 모음 |

## 사용 방법

각 프로젝트 폴더로 이동해서 Cargo 명령을 실행합니다.

```powershell
cd incident_commander_rs
cargo run -- --provider mock
```

벤치마크가 있는 프로젝트는 각 README와 `benches/` 폴더를 참고하세요.

```powershell
cargo bench
```

## 참고

- `target/`, Python 빌드 산출물, 생성된 벤치마크 데이터는 `.gitignore`에서 제외합니다.
- PyO3 프로젝트는 Python 3.9 이상과 `maturin`이 필요합니다.
- `legacy_c_ffi_bridge`는 C 툴체인과 `bindgen` 실행 환경이 필요합니다.
- `incident_commander_rs`는 mock LLM provider로 모델 없이 실행할 수 있고, 이후 Ollama 또는 vLLM/OpenAI-compatible endpoint에 연결할 수 있습니다.
