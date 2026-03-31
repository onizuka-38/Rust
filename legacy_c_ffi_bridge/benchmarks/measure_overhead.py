import argparse
import json
import random
import time
from pathlib import Path

import legacy_c_ffi_bridge as bridge


def py_matmul(a_rows, a_cols, a_data, b_rows, b_cols, b_data):
    out = [0.0] * (a_rows * b_cols)
    for i in range(a_rows):
        for k in range(a_cols):
            aik = a_data[i * a_cols + k]
            row_base = i * b_cols
            b_base = k * b_cols
            for j in range(b_cols):
                out[row_base + j] += aik * b_data[b_base + j]
    return out


def measure(fn, repeat=5):
    best = float("inf")
    for _ in range(repeat):
        t0 = time.perf_counter()
        fn()
        t1 = time.perf_counter()
        best = min(best, t1 - t0)
    return best


def make_matrix(rows, cols):
    return [random.random() for _ in range(rows * cols)]


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--size", type=int, default=128)
    p.add_argument("--repeat", type=int, default=5)
    p.add_argument("--out", type=Path, default=Path("benchmarks/ffi_overhead.json"))
    args = p.parse_args()

    n = args.size
    a = make_matrix(n, n)
    b = make_matrix(n, n)

    py_s = measure(lambda: py_matmul(n, n, a, n, n, b), args.repeat)
    rust_s = measure(lambda: bridge.matmul(n, n, a, n, n, b), args.repeat)

    # pure call overhead (Python -> Rust) without heavy payload
    ping_s = measure(lambda: bridge.ping(10_000), args.repeat)

    speedup = py_s / rust_s if rust_s > 0 else float("inf")

    result = {
        "matrix_size": n,
        "repeat": args.repeat,
        "python_seconds": py_s,
        "rust_bridge_seconds": rust_s,
        "speedup_py_over_bridge": speedup,
        "python_to_rust_call_overhead_seconds": ping_s,
    }

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(result, indent=2), encoding="utf-8")

    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    random.seed(42)
    main()
