param(
  [int]$Size = 128,
  [int]$Repeat = 5
)

python -m pip install -U pip maturin
python -m pip install -e .
python benchmarks/measure_overhead.py --size $Size --repeat $Repeat --out benchmarks/ffi_overhead.json
python benchmarks/generate_overhead_report.py
cargo bench --bench wrapper_overhead
