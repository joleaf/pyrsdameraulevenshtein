"""Type stubs for the pyrsdameraulevenshtein module."""

def distance_int(seq1: list[int], seq2: list[int]) -> int:
    """Damerau–Levenshtein distance between two lists of integers."""
    ...

def normalized_distance_int(seq1: list[int], seq2: list[int]) -> float:
    """Normalized distance (in [0, 1]) between two lists of integers."""
    ...

def similarity_int(seq1: list[int], seq2: list[int]) -> float:
    """Similarity (in [0, 1]) between two lists of integers."""
    ...

def distance_str(seq1: list[str], seq2: list[str]) -> int:
    """Damerau–Levenshtein distance between two lists of strings."""
    ...

def normalized_distance_str(seq1: list[str], seq2: list[str]) -> float:
    """Normalized distance (in [0, 1]) between two lists of strings."""
    ...

def similarity_str(seq1: list[str], seq2: list[str]) -> float:
    """Similarity (in [0, 1]) between two lists of strings."""
    ...

def distance_unicode(seq1: str, seq2: str) -> int:
    """Damerau–Levenshtein distance between two strings."""
    ...

def normalized_distance_unicode(seq1: str, seq2: str) -> float:
    """Normalized distance (in [0, 1]) between two strings."""
    ...

def similarity_unicode(seq1: str, seq2: str) -> float:
    """Similarity (in [0, 1]) between two strings."""
    ...
