# pyrsdameraulevenshtein

[![PyPI version](https://img.shields.io/pypi/v/pyrsdameraulevenshtein?color=blue&label=PyPI%20version)](https://pypi.org/project/pyrsdameraulevenshtein/)
[![PyPI downloads](https://img.shields.io/pypi/dm/pyrsdameraulevenshtein?label=downloads)](https://pypi.org/project/pyrsdameraulevenshtein/)
[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-green)](LICENSE)
[![CI](https://github.com/joleaf/pyrsdameraulevenshtein/actions/workflows/CI.yml/badge.svg)](https://github.com/joleaf/pyrsdameraulevenshtein/actions/workflows/CI.yml)

A fast [Damerau–Levenshtein](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance) distance, implemented in **Rust** and exposed to Python.

The Damerau–Levenshtein distance counts the minimum number of **insertions, deletions, substitutions, and adjacent transpositions (swaps)** needed to turn one sequence into another. This library returns that distance, its normalized form, and a similarity score — tuned for computing many distances quickly.

It works over three kinds of sequences from a single, consistent API:

- **Lists of integers** — `distance_int([1, 2, 3], [1, 3])`
- **Lists of strings** — `distance_str(["A", "B", "C"], ["A", "C"])`
- **Plain strings** (Unicode) — `distance_unicode("ABC", "AC")`

Most string-distance libraries only handle the last case. If plain strings are all you need, [editdistance](https://github.com/roy-ht/editdistance), [jellyfish](https://github.com/jamesturk/jellyfish), and [textdistance](https://github.com/amitao/textdistance) are excellent choices. This package is for when you also need lists of integers or strings, or want one fast, uniform API for all three.

## Install

```shell
pip install pyrsdameraulevenshtein
```

Supports Python 3.11, 3.12, 3.13, and 3.14. Prebuilt wheels are provided for Linux (x86_64, i686, aarch64, armv7, s390x, ppc64le), macOS (Apple silicon), and Windows (x64, x86); an sdist is also available.

## Usage

```python
import pyrsdameraulevenshtein as dl

# Lists of integers
dl.distance_int([1, 2, 3], [1, 3])              # 1
dl.normalized_distance_int([1, 2, 3], [1, 3])    # 0.33…
dl.similarity_int([1, 2, 3], [1, 3])             # 0.66…

# Lists of strings
dl.distance_str(["A", "B", "C"], ["A", "C"])              # 1
dl.normalized_distance_str(["A", "B", "C"], ["A", "C"])    # 0.33…
dl.similarity_str(["A", "B", "C"], ["A", "C"])             # 0.66…

# Plain strings (Unicode)
dl.distance_unicode("ABC", "AC")              # 1
dl.normalized_distance_unicode("ABC", "AC")    # 0.33…
dl.similarity_unicode("ABC", "AC")             # 0.66…
```

## API

All functions take `(seq1, seq2)`. There is one family per input type:

| | Lists of integers | Lists of strings | Strings |
|---|---|---|---|
| **Distance** | `distance_int` | `distance_str` | `distance_unicode` |
| **Normalized distance** | `normalized_distance_int` | `normalized_distance_str` | `normalized_distance_unicode` |
| **Similarity** | `similarity_int` | `similarity_str` | `similarity_unicode` |

- `distance_*` — the raw Damerau–Levenshtein distance (number of edits). Returns `int`.
- `normalized_distance_*` — the distance divided by the length of the longer sequence. Returns `float` in `[0.0, 1.0]`.
- `similarity_*` — `1.0 - normalized_distance_*`. Returns `float` in `[0.0, 1.0]`; higher means more similar.

## Development

1. Create a virtual environment.
2. Install the test dependencies: `pip install -r requirements.txt`
3. Build the Rust extension into the environment:
   - Development build: `maturin develop`
   - Release wheel: `maturin build --release`
   - Install wheel (to not run with dev mode): `pip install target/wheels/....whl`
4. Run the tests: `python tests/DamerauLevenshteinTest.py`

## Performance

The numbers below compare this implementation against other libraries on 100,000 random sequences of length 10 (lower is faster). The full benchmark lives in [`tests/DamerauLevenshteinTest.py`](tests/DamerauLevenshteinTest.py) — run it on your own hardware, since results depend heavily on the workload and platform.

**Lists of integers** (seconds)

| Implementation | Time |
|---|---|
| **pyrsdameraulevenshtein** | **0.085** |
| fastDamerauLevenshtein | 0.126 |
| pyxdameraulevenshtein | 0.307 |

**Strings** (seconds)

| Implementation | Time |
|---|---|
| textdistance | 0.019 |
| jellyfish | 0.055 |
| **pyrsdameraulevenshtein** | **0.076** |
| fastDamerauLevenshtein | 0.128 |
| pyxdameraulevenshtein | 0.393 |

Two takeaways:

- For **lists of integers and lists of strings**, this implementation is the fastest of the group — and it is the only one that handles both in a single package.
- For **plain strings**, dedicated string libraries (textdistance, jellyfish) can be faster. They simply don't support lists, which is where this library earns its place.

_Representative numbers measured on an Apple M1 Mac with Python 3.10. Your mileage will vary._

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
