# Legacy C FFI Bridge (C -> Rust Safe -> Python)

기존 레거시 C 연산 코드를 Rust의 안전 API로 감싸고, 최종적으로 PyO3로 Python 패키지화한 프로젝트입니다.

## 라이브러리 선정
- 대상: BLAS 계열 C API 스타일의 레거시 행렬 연산 루틴 (`c_legacy/legacy_math.c`)
- 성격: 포인터/수동 메모리 관리 중심의 불안전 C API (OpenBLAS/유사 C 수치 라이브러리 사용 패턴을 재현)
- 목적: unsafe FFI 영역을 Rust safe 추상화로 격리

## 기술 스택
- `bindgen`: C 헤더(`legacy_math.h`)로부터 Rust 바인딩 자동 생성
- `cc`: C 소스 컴파일 및 Rust 링크
- `PyO3` + `maturin`: Python 네이티브 확장 배포

## 구조
- `c_legacy/`: 기존 C 코드
- `build.rs`: `cc` 컴파일 + `bindgen` 생성
- `src/ffi.rs`: 생성된 unsafe 바인딩 include
- `src/safe.rs`: 안전 래퍼(Context/Matrix)
- `src/lib.rs`: Python에 노출할 API

## Safe API 설계 포인트
- `Context`/`Matrix`가 raw pointer를 소유하고 Drop에서 안전 해제
- shape 검증(행렬 차원) 선검사로 C단 segfault 가능성 차단
- C 에러코드 + `lm_last_error`를 Rust `Result`로 변환
- `to_vec()`에서 길이 검증 후 안전 복사

## 설치 (Python)
```bash
cd legacy_c_ffi_bridge
python -m pip install -U pip maturin
python -m pip install -e .
```

## 사용 예시
```python
import legacy_c_ffi_bridge as bridge

# 2x3 · 3x2
out = bridge.matmul(
    2, 3, [1,2,3,4,5,6],
    3, 2, [1,2,3,4,5,6]
)
print(out)

# affine + sigmoid
y = bridge.affine_sigmoid(
    1, 3, [0.1, 0.2, 0.3],
    3, 2, [0.5,0.1, 0.2,0.3, 0.4,0.7],
    1, 2, [0.01, -0.02]
)
print(y)
```

## 오버헤드 측정
```bash
python benchmarks/measure_overhead.py --size 128 --repeat 5 --out benchmarks/ffi_overhead.json
python benchmarks/generate_overhead_report.py
```

## Valgrind 메모리 안전성 검증
Linux 환경에서:
```bash
cargo build --release
valgrind --leak-check=full --show-leak-kinds=all \
  python -c "import legacy_c_ffi_bridge as b; print(b.matmul(2,2,[1,2,3,4],2,2,[1,0,0,1]))"
```

검증 리포트 템플릿:
- `benchmarks/VALGRIND_REPORT.md`

## Rust 마이크로벤치
```bash
cargo bench --bench wrapper_overhead
```

