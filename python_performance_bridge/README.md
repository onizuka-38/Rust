# Python-Rust Performance Bridge ()

�ӽŷ��� ������������ ��뷮 �ؽ�Ʈ ��ó�� ������ Rust�� �����ϴ� `PyO3` ����Ƽ�� Ȯ�� ������Ʈ�Դϴ�.

## ���� ����
���� Python �ؽ�Ʈ ���� �ܰ�(���Խ� ġȯ + ��ūȭ)�� ��뷮 ��ġ���� ������ �߻��ϴ� ��Ȳ�� �����մϴ�.

## �ذ� ���
- Python baseline: `python/python_performance_bridge/baseline.py`
- Rust port: `src/processing.rs`
- Python���� ��� import ������ ����Ƽ�� Ȯ��: `python_performance_bridge._core`
- ���� ó��: `rayon` (`clean_texts(..., parallel=True)`)

## ��ġ/���
```bash
cd python_performance_bridge
python -m pip install -U pip maturin
python -m pip install -e .
```

```python
from python_performance_bridge import clean_text, clean_texts, token_frequency

print(clean_text("BTC update! https://example.com now"))
rows = clean_texts(["trade 1 ...", "trade 2 ..."], parallel=True)
stats = token_frequency(["trade 1 ...", "trade 2 ..."], parallel=True)
```

## ������ ��ȯ ������� �ּ�ȭ ����
- FFI API�� **��ġ ����** (`List[str]`)�� ������ ȣ�� Ƚ���� �ٿ����ϴ�.
- Rust ���ο��� ���Խ� ��ü�� `once_cell`�� ������ �������� ������带 �����߽��ϴ�.
- ���� �б�(`parallel=True`)�� Rust ���ο��� ó���Ͽ� Python GIL ��� ���� ����� �ٿ����ϴ�.

## 10x ���� ����
### 1) Python vs Rust �������ϸ�
```bash
python benchmarks/run_profile.py --size 300000 --repeat 5 --out benchmarks/profile_result.json
python benchmarks/generate_report.py
```

- ��� JSON: `benchmarks/profile_result.json`
- ���� ����Ʈ: `benchmarks/PROFILE_REPORT.md`
- ����: `speedup >= 10.0`

### 2) Rust ���� ����ũ�κ�ġ
```bash
cargo bench --bench text_clean_bench
```

## ���������� ���� ����
- �ŷ��� API ũ�ѷ�/�����Ⱑ ������ ���� �ؽ�Ʈ�� Python���� ��ġ�� ����
- ��ó�� �Լ��� Rust Ȯ������ ��ü
- ���� ML �н�/�߷� �ڵ�� �״�� ����

## ���� ����
- `src/lib.rs`: PyO3 ���ε� + ���� �Լ�
- `src/processing.rs`: �ؽ�Ʈ ���� �ٽ� ����
- `python/python_performance_bridge/baseline.py`: ���� Python ����
- `benchmarks/run_profile.py`: Python vs Rust �� ��ġ
- `benches/text_clean_bench.rs`: criterion ��ġ
