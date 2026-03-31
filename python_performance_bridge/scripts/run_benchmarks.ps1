param(
  [int]$Size = 300000,
  [int]$Repeat = 5
)

python -m pip install -U pip maturin
python -m pip install -e .
python benchmarks/run_profile.py --size $Size --repeat $Repeat --out benchmarks/profile_result.json
python benchmarks/generate_report.py
cargo bench --bench text_clean_bench
