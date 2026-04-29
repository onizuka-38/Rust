# Fast Data Processor

로그 파일 스캔과 거래소 NDJSON 통계를 처리하는 Rust CLI입니다. 큰 입력을 한 번에 메모리에 올리지 않고 배치 단위로 읽고, 통계 집계는 `rayon`으로 병렬화할 수 있습니다.

## 주요 기능

- 정규식 기반 로그/텍스트 스캔
- include/exclude 필터와 대소문자 무시 옵션
- 거래 이벤트 NDJSON 집계
- 심볼별 체결 수, 매수/매도 수, 총 수량, Notional, VWAP, 최저/최고가 계산
- 벤치마크용 대량 샘플 데이터 생성
- JSON 또는 표 형태 출력

## 기술 스택

- `clap`: CLI 서브커맨드
- `serde`, `serde_json`: NDJSON 파싱
- `regex`: 텍스트 필터링
- `rayon`: 병렬 집계
- `criterion`: 벤치마크

## 명령어

로그 스캔:

```powershell
cd fast_data_processor
cargo run -- scan .\logs\app.log --include "error|timeout" --ignore-case
cargo run -- scan .\logs\app.log .\logs\app2.log --include "BTCUSDT" --exclude "heartbeat" --count-only
```

거래 NDJSON 통계:

```powershell
cargo run -- crypto-stats --input examples/sample_trades.ndjson --mode serial
cargo run -- crypto-stats --input examples/sample_trades.ndjson --mode parallel --batch-size 50000 --top 20
cargo run -- crypto-stats --input data/trades.ndjson --mode parallel --json
```

벤치 입력 생성:

```powershell
cargo run -- bench-input --output data/trades_1m.ndjson --lines 1000000 --symbols 6
```

## 벤치마크

```powershell
cargo bench --bench processing_bench
.\scripts\benchmark.ps1 -Input data/trades_1m.ndjson -BatchSize 50000
```

측정 대상은 단일 스레드 집계와 `rayon` 기반 병렬 집계입니다.

## 구현 메모

- 잘못된 JSON 라인은 `invalid_lines`로 집계합니다.
- 병렬 모드에서도 배치별 결과만 합쳐 메모리 사용량을 제어합니다.
- `SymbolStats::merge_from`으로 partial result를 안전하게 병합합니다.
