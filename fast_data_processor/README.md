# Fast Data Processor

대용량 로그/NDJSON 데이터를 빠르게 처리하는 Rust CLI 도구입니다.

도메인 특화 포인트:
- 가상화폐 거래소 API 수집 데이터(한 줄 JSON = NDJSON) 분석
- 심볼별 체결 통계(거래량, 체결수, VWAP, Notional) 계산

## 핵심 기술
- `clap`: 서브커맨드 기반 CLI
- `serde`, `serde_json`: NDJSON 직렬화/역직렬화
- `regex`: 로그 필터링
- `rayon`: 병렬 집계 최적화
- `criterion`: 성능 벤치마크

## 명령어
### 1) 로그/텍스트 스캔
```bash
cargo run -- scan ./logs/app.log --include "error|timeout" --ignore-case
cargo run -- scan ./logs/app.log ./logs/app2.log --include "BTCUSDT" --exclude "heartbeat" --count-only
```

### 2) 거래소 NDJSON 통계
```bash
cargo run -- crypto-stats --input examples/sample_trades.ndjson --mode serial
cargo run -- crypto-stats --input examples/sample_trades.ndjson --mode parallel --batch-size 50000 --top 20
cargo run -- crypto-stats --input data/trades.ndjson --mode parallel --json
```

### 3) 벤치 입력용 대용량 샘플 생성
```bash
cargo run -- bench-input --output data/trades_1m.ndjson --lines 1000000 --symbols 6
```

## 벤치마크
### 실행
```bash
cargo bench --bench processing_bench
```

### 측정 항목
- `serial`: 단일 스레드 파싱/집계
- `parallel`: `rayon` 기반 병렬 파싱/집계
- 데이터 크기: 100,000 / 500,000 lines

### 자동 비교 스크립트(요약 수치 출력)
```powershell
./scripts/benchmark.ps1 -Input data/trades_1m.ndjson -BatchSize 50000
```

### 결과 템플릿
실제 측정 결과를 아래 표에 채워 팀에 공유하세요.

| Dataset | Serial (ms) | Parallel (ms) | Speedup |
|---|---:|---:|---:|
| 100k lines | - | - | - |
| 500k lines | - | - | - |

## 구현 포인트
- 파일 전체를 메모리에 올리지 않고 **배치 단위**로 읽어 처리합니다.
- 병렬 모드에서도 배치 단위 처리 후 집계 결과만 머지하여 메모리 사용량을 제어합니다.
- 파싱 실패 라인은 `invalid_lines`로 카운트해 데이터 품질을 함께 확인합니다.

