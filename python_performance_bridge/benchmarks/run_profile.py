import argparse
import json
import os
import random
import string
import time
from pathlib import Path

from python_performance_bridge import clean_texts as rust_clean_texts
from python_performance_bridge.baseline import clean_texts as py_clean_texts


def make_dataset(size: int) -> list[str]:
    rows = []
    for i in range(size):
        noise = "".join(random.choices(string.ascii_letters + string.digits, k=24))
        rows.append(
            f"Trade {i} executed on BTCUSDT at 67123.10 qty=0.01. url=https://exchange.example/t/{i} ref={noise}"
        )
    return rows


def measure(fn, data: list[str], repeat: int) -> float:
    times = []
    for _ in range(repeat):
        start = time.perf_counter()
        fn(data)
        end = time.perf_counter()
        times.append(end - start)
    return min(times)


def run(size: int, repeat: int, out: Path) -> dict:
    data = make_dataset(size)

    py_sec = measure(py_clean_texts, data, repeat)
    rust_sec = measure(lambda d: rust_clean_texts(d, True), data, repeat)

    speedup = py_sec / rust_sec if rust_sec > 0 else float("inf")
    passed = speedup >= 10.0

    result = {
        "dataset_size": size,
        "repeat": repeat,
        "python_seconds": py_sec,
        "rust_seconds": rust_sec,
        "speedup": speedup,
        "target_speedup": 10.0,
        "passed": passed,
    }

    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(result, indent=2), encoding="utf-8")
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description="Python vs Rust preprocessing benchmark")
    parser.add_argument("--size", type=int, default=300_000)
    parser.add_argument("--repeat", type=int, default=5)
    parser.add_argument("--out", type=Path, default=Path("benchmarks/profile_result.json"))
    args = parser.parse_args()

    result = run(args.size, args.repeat, args.out)

    print("Benchmark result")
    print(f"dataset_size={result['dataset_size']}")
    print(f"python_seconds={result['python_seconds']:.6f}")
    print(f"rust_seconds={result['rust_seconds']:.6f}")
    print(f"speedup={result['speedup']:.2f}x")
    print(f"target_10x_passed={result['passed']}")

    if not result["passed"]:
        raise SystemExit("Speedup target (10x) not met")


if __name__ == "__main__":
    random.seed(42)
    os.environ.setdefault("PYTHONHASHSEED", "0")
    main()
