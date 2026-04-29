# Rust Projects

Rust 학습과 실무형 실험을 모아 둔 프로젝트 묶음입니다. 루트는 Cargo workspace가 아니며, 각 하위 폴더가 독립적인 Cargo 프로젝트로 동작합니다.

<div align="center">
  <img src="https://github-readme-stats.vercel.app/api?username=onizuka-38&cache_seconds=21600" height="180" />
  <img src="https://github-readme-stats.vercel.app/api/top-langs/?username=onizuka-38&layout=compact&cache_seconds=21600" height="180" />
</div>

## 프로젝트 목록

| 폴더 | 설명 |
|---|---|
| `hello_rust` | Rust 기본 실행 구조를 확인하는 가장 작은 예제 |
| `budget_tracker` | JSON 파일에 수입/지출을 저장하는 CLI 가계부 |
| `fast_data_processor` | 로그 스캔과 거래 NDJSON 통계를 처리하는 병렬 CLI |
| `python_performance_bridge` | Python 텍스트 전처리를 Rust/PyO3 확장으로 가속하는 예제 |
| `legacy_c_ffi_bridge` | 레거시 C 행렬 연산을 Rust safe API와 Python 확장으로 감싼 예제 |
| `biofeedback_engine` | EDF 파싱과 EEG/EMG 주파수 대역 분석 파이프라인 |
| `cyberpunk_tui_dashboard` | `ratatui` 기반 터미널 시스템 모니터링 대시보드 |
| `realtime_collab_server` | WebSocket 채팅/화이트보드 실시간 협업 서버 |
| `rust_practice_notebook` | Rust 문법, 표준 라이브러리, 알고리즘 연습 파일 50개 |

## 사용 방법

각 프로젝트 폴더로 이동해서 Cargo 명령을 실행합니다.

```powershell
cd fast_data_processor
cargo run -- --help
```

벤치마크가 있는 프로젝트는 `benches/`와 README의 명령을 참고하세요.

```powershell
cargo bench
```

## 환경 메모

- `target/`, Python 빌드 산출물, 벤치마크 생성 데이터는 `.gitignore`에서 제외합니다.
- PyO3 프로젝트는 Python 3.9 이상과 `maturin`이 필요합니다.
- `legacy_c_ffi_bridge`는 C 빌드 도구와 `bindgen` 실행 환경이 필요합니다.
- 현재 폴더는 독립 프로젝트 모음이므로 루트에서 `cargo check --workspace`를 실행하는 구조가 아닙니다.
