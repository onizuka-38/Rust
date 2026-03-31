# Biofeedback Engine (EEG/EMG)

EEG/EMG 등 고샘플링(1000Hz+) 생체 시계열을 파싱/분석하는 고속 코어 엔진입니다.

## 핵심 기술
- `nom`: EDF(European Data Format) 바이너리 헤더/페이로드 파싱
- `tokio`: 비동기 다채널 스트림 파이프라인
- `rustfft`: 실시간 FFT 기반 주파수 대역 분석

## 구현 범위
- EDF 파서: 헤더 + 신호 샘플(16-bit) 파싱
- 실시간 파이프라인: 1000Hz+ 다채널 스트림 시뮬레이션 수신
- 신호 처리:
  - artifact 제거: amplitude clipping + DC offset 제거
  - FFT band power: delta/theta/alpha/beta/gamma

## 명령어
### EDF 파싱
```bash
cargo run -- parse-edf --input ./examples/sample.edf --max-records 10 --json
```

### 실시간 처리 시뮬레이션
```bash
cargo run -- realtime-sim --channels 8 --sample-rate 1024 --chunk-size 256 --seconds 5
```

출력 지표:
- `processed_chunks`
- `end_to_end_elapsed_ms`
- `avg_fft_stage_latency_ms`

## 벤치마크
```bash
cargo bench --bench pipeline_latency
```

## C++ 레거시 비교 프로파일링
```bash
python scripts/profile_vs_legacy.py --project-root . --channels 8 --sample-rate 1024 --chunk-size 256 --seconds 5 --legacy-cpp-ms 1200 --out benchmarks/profiling_report.json
```

`benchmarks/PROFILE_REPORT.md`에 결과를 반영해 비교 보고서로 사용하세요.

## 의료 데이터 안전성 관점
- 파서 단계에서 길이/경계 검증을 선행하여 malformed payload 접근 차단
- processing 단계에서 채널별 고정 chunk 단위 처리로 메모리 폭주 방지
- Rust 타입 시스템 기반으로 파서/분석기 사이 unsafe 없는 경로 유지
