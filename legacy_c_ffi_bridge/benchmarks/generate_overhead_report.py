import json
from pathlib import Path

SRC = Path("benchmarks/ffi_overhead.json")
DST = Path("benchmarks/OVERHEAD_REPORT.md")


def main() -> None:
    if not SRC.exists():
        raise SystemExit(f"missing {SRC}")

    d = json.loads(SRC.read_text(encoding="utf-8"))
    md = f"""# C -> Rust -> Python Overhead Report

| Metric | Value |
|---|---:|
| Matrix size | {d['matrix_size']}x{d['matrix_size']} |
| Python baseline (sec) | {d['python_seconds']:.6f} |
| Rust bridge (sec) | {d['rust_bridge_seconds']:.6f} |
| Speedup | {d['speedup_py_over_bridge']:.2f}x |
| Python->Rust call overhead (sec) | {d['python_to_rust_call_overhead_seconds']:.6f} |
"""
    DST.write_text(md, encoding="utf-8")
    print(f"wrote {DST}")


if __name__ == "__main__":
    main()
