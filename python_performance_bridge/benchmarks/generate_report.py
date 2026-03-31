import json
from pathlib import Path

INPUT = Path("benchmarks/profile_result.json")
OUTPUT = Path("benchmarks/PROFILE_REPORT.md")


def main() -> None:
    if not INPUT.exists():
        raise SystemExit(f"Missing benchmark result: {INPUT}")

    data = json.loads(INPUT.read_text(encoding="utf-8"))

    report = f"""# Python-Rust Performance Profiling Report

## Environment
- Dataset size: {data['dataset_size']:,} rows
- Repeats: {data['repeat']}

## Result Summary
| Metric | Value |
|---|---:|
| Pure Python (sec) | {data['python_seconds']:.6f} |
| Rust Extension (sec) | {data['rust_seconds']:.6f} |
| Speedup | {data['speedup']:.2f}x |
| Target (>=10x) | {str(data['passed'])} |

## Conclusion
{('Target achieved: Rust preprocessing reached at least 10x speedup over pure Python.' if data['passed'] else 'Target not achieved: optimize regex, batching, and thread settings, then rerun benchmarks.')}
"""

    OUTPUT.write_text(report, encoding="utf-8")
    print(f"Wrote report: {OUTPUT}")


if __name__ == "__main__":
    main()
