# Legacy C FFI Bridge

포인터와 수동 메모리 관리를 사용하는 레거시 C 행렬 연산 코드를 Rust safe API로 감싸고, PyO3로 Python 패키지까지 노출하는 예제입니다.

## 목표

- C FFI의 `unsafe` 영역을 작은 모듈에 격리
- Rust `Context`와 `Matrix` 타입이 C 리소스 수명 관리
- shape 검증을 Rust 쪽에서 먼저 수행해 C단 오류 가능성 축소
- C 에러코드를 Rust `Result`와 Python 예외로 변환

## 구조

- `c_legacy/legacy_math.c`: 레거시 C 행렬 연산 구현
- `c_legacy/legacy_math.h`: C API 헤더
- `build.rs`: `cc`로 C 코드를 컴파일하고 `bindgen` 바인딩 생성
- `src/ffi.rs`: 생성된 C 바인딩 include
- `src/safe.rs`: safe Rust 래퍼
- `src/lib.rs`: PyO3 함수와 Rust 벤치용 함수
- `benches/wrapper_overhead.rs`: 래퍼 오버헤드 벤치마크

## 제공 API

Python에 노출되는 주요 함수:

- `matmul(...)`
- `affine_sigmoid(...)`
- `rust_only_matmul(...)`
- `ping(iterations)`

## 설치

Python 3.9 이상, `maturin`, C 컴파일러, `bindgen` 실행 환경이 필요합니다.

```powershell
cd legacy_c_ffi_bridge
python -m pip install -U pip maturin
python -m pip install -e .
```

## 사용 예시

```python
import legacy_c_ffi_bridge as bridge

out = bridge.matmul(
    2, 3, [1, 2, 3, 4, 5, 6],
    3, 2, [1, 2, 3, 4, 5, 6],
)
print(out)

y = bridge.affine_sigmoid(
    1, 3, [0.1, 0.2, 0.3],
    3, 2, [0.5, 0.1, 0.2, 0.3, 0.4, 0.7],
    1, 2, [0.01, -0.02],
)
print(y)
```

## 검증

```powershell
cargo check
cargo bench --bench wrapper_overhead
python benchmarks/measure_overhead.py --size 128 --repeat 5 --out benchmarks/ffi_overhead.json
python benchmarks/generate_overhead_report.py
```

Linux에서 메모리 안전성을 더 확인하려면 Valgrind를 사용할 수 있습니다.

```bash
cargo build --release
valgrind --leak-check=full --show-leak-kinds=all \
  python -c "import legacy_c_ffi_bridge as b; print(b.matmul(2,2,[1,2,3,4],2,2,[1,0,0,1]))"
```
