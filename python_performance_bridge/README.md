# Python-Rust Performance Bridge

Python 텍스트 전처리 함수를 Rust/PyO3 확장으로 옮겨 대량 배치 처리 성능을 비교하는 프로젝트입니다. Python 패키지에서는 `python_performance_bridge._core` 네이티브 모듈을 통해 Rust 구현을 호출합니다.

## 기능

- URL 제거
- 영문 소문자 정규화
- 영문자와 공백 외 문자 제거
- 연속 공백 정리
- 길이 2 이상 토큰 추출
- 여러 텍스트의 병렬 전처리
- 토큰 빈도 집계

## 구조

- `src/lib.rs`: PyO3 바인딩과 Python 노출 함수
- `src/processing.rs`: Rust 텍스트 정제/토큰화 로직
- `python/python_performance_bridge/__init__.py`: Python 패키지 진입점
- `python/python_performance_bridge/baseline.py`: 순수 Python 기준 구현
- `benchmarks/run_profile.py`: Python 구현과 Rust 구현 비교 실행
- `benchmarks/generate_report.py`: 프로파일 결과 보고서 생성
- `benches/text_clean_bench.rs`: Rust criterion 벤치마크

## 설치

Python 3.9 이상과 `maturin`이 필요합니다.

```powershell
cd python_performance_bridge
python -m pip install -U pip maturin
python -m pip install -e .
```

## 사용 예시

```python
from python_performance_bridge import clean_text, clean_texts, token_frequency

print(clean_text("BTC update! https://example.com now"))
rows = clean_texts(["trade 1 ...", "trade 2 ..."], parallel=True)
stats = token_frequency(["trade 1 ...", "trade 2 ..."], parallel=True)
print(rows)
print(stats)
```

## 성능 비교

```powershell
python benchmarks/run_profile.py --size 300000 --repeat 5 --out benchmarks/profile_result.json
python benchmarks/generate_report.py
cargo bench --bench text_clean_bench
```

## 설계 포인트

- Python/Rust 경계를 여러 번 넘지 않도록 `List[str]` 배치 입력을 사용합니다.
- 정규식은 `once_cell`로 초기화해 반복 생성 비용을 줄입니다.
- `clean_texts(..., parallel=True)`는 Rust 내부에서 `rayon` 병렬 iterator를 사용합니다.
- 동일한 알고리즘을 Python baseline과 Rust 구현에 두어 성능 차이를 비교하기 쉽습니다.
