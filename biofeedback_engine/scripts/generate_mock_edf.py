#!/usr/bin/env python3
from pathlib import Path


def pad(s, n):
    b = s.encode('ascii', errors='ignore')[:n]
    return b + b' ' * (n - len(b))


def make_mock_edf(path: Path):
    num_signals = 2
    records = 5
    duration = 1
    samples_per_record = [256, 256]
    header_bytes = 256 + num_signals * 256

    h = bytearray()
    h += pad('0', 8)
    h += pad('TEST_PATIENT', 80)
    h += pad('TEST_RECORD', 80)
    h += pad('01.01.26', 8)
    h += pad('00.00.00', 8)
    h += pad(str(header_bytes), 8)
    h += pad('', 44)
    h += pad(str(records), 8)
    h += pad(str(duration), 8)
    h += pad(str(num_signals), 4)

    labels = ['EEG Fp1', 'EMG Chin']
    for x in labels:
        h += pad(x, 16)

    h += pad('transducer', 80 * num_signals)
    h += pad('uV', 8 * num_signals)
    h += pad('-32768', 8 * num_signals)
    h += pad('32767', 8 * num_signals)
    h += pad('-100', 8 * num_signals)
    h += pad('100', 8 * num_signals)
    h += pad('prefilter', 80 * num_signals)

    for s in samples_per_record:
        h += pad(str(s), 8)

    h += pad('', 32 * num_signals)

    assert len(h) == header_bytes

    payload = bytearray()
    for r in range(records):
        for ch in range(num_signals):
            n = samples_per_record[ch]
            for i in range(n):
                v = int(((i + r * 3 + ch * 5) % 200) - 100)
                payload += int(v).to_bytes(2, byteorder='little', signed=True)

    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(bytes(h) + bytes(payload))
    print(f'wrote {path}')


if __name__ == '__main__':
    make_mock_edf(Path('examples/sample.edf'))
