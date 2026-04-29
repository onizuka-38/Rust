# Biofeedback Engine

EEG/EMG 같은 고샘플링 생체 시계열을 파싱하고 분석하는 Rust 코어 엔진입니다. EDF 헤더/샘플 파싱, 실시간 스트림 시뮬레이션, FFT 기반 주파수 대역 분석을 포함합니다.

## 기능

- EDF(European Data Format) 헤더와 16-bit 샘플 파싱
- 채널별 샘플 chunk 처리
- artifact clipping
- DC offset 제거
- RMS 계산
- FFT band power 계산: delta, theta, alpha, beta, gamma
- `tokio::mpsc` 기반 실시간 처리 파이프라인 시뮬레이션

## 기술 스택

- `nom`: 바이너리/ASCII 필드 파싱
- `tokio`: 비동기 파이프라인
- `rustfft`: FFT 계산
- `serde`, `serde_json`: 결과 리포트 출력
- `criterion`: 지연 시간 벤치마크

## 명령어

EDF 파싱:

```powershell
cd biofeedback_engine
cargo run -- parse-edf --input .\examples\sample.edf --max-records 10 --json
```

실시간 처리 시뮬레이션:

```powershell
cargo run -- realtime-sim --channels 8 --sample-rate 1024 --chunk-size 256 --seconds 5
```

출력 지표:

- `processed_chunks`
- `end_to_end_elapsed_ms`
- `avg_fft_stage_latency_ms`

## 벤치마크와 프로파일링

```powershell
cargo bench --bench pipeline_latency
python scripts/profile_vs_legacy.py --project-root . --channels 8 --sample-rate 1024 --chunk-size 256 --seconds 5 --legacy-cpp-ms 1200 --out benchmarks/profiling_report.json
```

`benchmarks/PROFILE_REPORT.md`는 비교 결과를 정리하는 보고서 템플릿입니다.

## 안전성 메모

- EDF 파일 길이가 선언된 헤더보다 짧으면 즉시 오류를 반환합니다.
- 샘플 읽기 중 EOF가 발생하면 malformed payload로 처리합니다.
- 분석 파이프라인은 safe Rust 경로로 구성되어 있습니다.
