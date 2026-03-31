import re
from collections import Counter

_URL_RE = re.compile(r"https?://\S+|www\.\S+")
_NON_WORD_RE = re.compile(r"[^a-z\s]+")
_MULTI_SPACE_RE = re.compile(r"\s+")


def clean_text(text: str) -> list[str]:
    lower = text.lower()
    no_url = _URL_RE.sub(" ", lower)
    alpha_only = _NON_WORD_RE.sub(" ", no_url)
    normalized = _MULTI_SPACE_RE.sub(" ", alpha_only)
    return [tok for tok in normalized.split() if len(tok) >= 2]


def clean_texts(texts: list[str]) -> list[list[str]]:
    return [clean_text(t) for t in texts]


def token_frequency(texts: list[str]) -> dict[str, int]:
    counter = Counter()
    for row in clean_texts(texts):
        counter.update(row)
    return dict(counter)
