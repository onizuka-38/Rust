# Python-Rust Performance Bridge

머신러닝 파이프라인의 대용량 텍스트 전처리 병목을 Rust로 가속하는 `PyO3` 네이티브 확장 프로젝트입니다.

## 문제 정의
기존 Python 텍스트 정제 단계(정규식 치환 + 토큰화)가 대용량 배치에서 병목이 발생하는 상황을 가정합니다.

## 해결 방식
- Python baseline: `python/python_performance_bridge/baseline.py`
- Rust port: `src/processing.rs`
- Python에서 즉시 import 가능한 네이티브 확장: `python_performance_bridge._core`
- 병렬 처리: `rayon` (`clean_texts(..., parallel=True)`)

## 설치/사용
```bash
cd python_performance_bridge
python -m pip install -U pip maturin
python -m pip install -e .
```

```python
from python_performance_bridge import clean_text, clean_texts, token_frequency

print(clean_text("BTC update! https://example.com now"))
rows = clean_texts(["trade 1 ...", "trade 2 ..."], parallel=True)
stats = token_frequency(["trade 1 ...", "trade 2 ..."], parallel=True)
```

## 데이터 변환 오버헤드 최소화 전략
- FFI API를 **배치 단위** (`List[str]`)로 설계해 호출 횟수를 줄였습니다.
- Rust 내부에서 정규식 객체를 `once_cell`로 재사용해 재컴파일 오버헤드를 제거했습니다.
- 병렬 분기(`parallel=True`)는 Rust 내부에서 처리하여 Python GIL 기반 루프 비용을 줄였습니다.

## 10x 성능 검증
### 1) Python vs Rust 프로파일링
```bash
python benchmarks/run_profile.py --size 300000 --repeat 5 --out benchmarks/profile_result.json
python benchmarks/generate_report.py
```

- 결과 JSON: `benchmarks/profile_result.json`
- 최종 리포트: `benchmarks/PROFILE_REPORT.md`
- 기준: `speedup >= 10.0`

### 2) Rust 내부 마이크로벤치
```bash
cargo bench --bench text_clean_bench
```

## 파이프라인 적용 예시
- 거래소 API 크롤러/수집기가 저장한 원시 텍스트를 Python에서 배치로 읽음
- 전처리 함수만 Rust 확장으로 교체
- 기존 ML 학습/추론 코드는 그대로 유지

## 파일 구조
- `src/lib.rs`: PyO3 바인딩 + 공개 함수
- `src/processing.rs`: 텍스트 정제 핵심 로직
- `python/python_performance_bridge/baseline.py`: 기존 Python 구현
- `benchmarks/run_profile.py`: Python vs Rust 비교 벤치
- `benches/text_clean_bench.rs`: criterion 벤치
