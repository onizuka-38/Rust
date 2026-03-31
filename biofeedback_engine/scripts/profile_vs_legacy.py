#!/usr/bin/env python3
import argparse
import json
import subprocess
from pathlib import Path


def run(cmd, cwd):
    p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    if p.returncode != 0:
        raise RuntimeError(f"command failed: {' '.join(cmd)}\n{p.stderr}")
    return p.stdout


def parse_metric(text, key):
    for line in text.splitlines():
        if line.startswith(key + "="):
            return float(line.split("=", 1)[1].strip())
    return None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--project-root", default=".")
    ap.add_argument("--channels", type=int, default=8)
    ap.add_argument("--sample-rate", type=int, default=1024)
    ap.add_argument("--chunk-size", type=int, default=256)
    ap.add_argument("--seconds", type=int, default=5)
    ap.add_argument("--legacy-cpp-ms", type=float, default=0.0)
    ap.add_argument("--out", default="benchmarks/profiling_report.json")
    args = ap.parse_args()

    root = Path(args.project_root)
    out = run([
        "cargo", "run", "--quiet", "--",
        "realtime-sim",
        "--channels", str(args.channels),
        "--sample-rate", str(args.sample_rate),
        "--chunk-size", str(args.chunk_size),
        "--seconds", str(args.seconds),
    ], cwd=root)

    rust_ms = parse_metric(out, "end_to_end_elapsed_ms")
    fft_stage_ms = parse_metric(out, "avg_fft_stage_latency_ms")

    legacy = args.legacy_cpp_ms
    rel = None
    if legacy > 0 and rust_ms is not None:
        rel = legacy / rust_ms

    report = {
        "channels": args.channels,
        "sample_rate": args.sample_rate,
        "chunk_size": args.chunk_size,
        "seconds": args.seconds,
        "rust_end_to_end_ms": rust_ms,
        "rust_avg_fft_stage_ms": fft_stage_ms,
        "legacy_cpp_end_to_end_ms": legacy,
        "legacy_vs_rust_ratio": rel,
        "raw_output": out,
    }

    out_path = root / args.out
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
