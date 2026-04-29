# Python-Rust Performance Profiling Report

`benchmarks/run_profile.py` 실행 후 Python baseline과 Rust/PyO3 구현의 성능 차이를 기록하는 보고서입니다.

## 실행 예시

```bash
python benchmarks/run_profile.py --size 300000 --repeat 5 --out benchmarks/profile_result.json
python benchmarks/generate_report.py
```

## 결과

| Metric | Value |
|---|---:|
| Dataset Size | - |
| Repeat | - |
| Python Baseline (ms) | - |
| Rust Serial (ms) | - |
| Rust Parallel (ms) | - |
| Best Speedup | - |

## 결론

- [ ] 목표 성능 달성
- [ ] 추가 최적화 필요
