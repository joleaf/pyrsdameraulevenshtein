# Rust implementation of the Damerau-Levenshtein distance

[Damerau-Levenshtein](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance) implementation in Rust as
Python package.
You should use this package if you need to calculate a distance metric for lists of integers or strings, and you need
high-performance.
If you only need to check the distance between two strings
checkout [editdistance](https://github.com/roy-ht/editdistance) or [jellyfish](https://github.com/jamesturk/jellyfish).

## Install

```shell
pip install pyrsdameraulevenshtein
```

## Use

```python
import pyrsdameraulevenshtein as dl

distance = dl.distance_int([1, 2, 3], [1, 3])
# distance = 1
normalized_distance = dl.normalized_distance_int([1, 2, 3], [1, 3])
# normalized_distance = 0.33
similarity = dl.similarity_int([1, 2, 3], [1, 3])
# similarity = 0.66
distance = dl.distance_str(["A", "B", "C"], ["A", "C"])
# distance = 1
normalized_distance = dl.normalized_distance_str(["A", "B", "C"], ["A", "C"])
# normalized_distance = 0.33
similarity = dl.similarity_str(["A", "B", "C"], ["A", "C"])
# similarity = 0.66
distance = dl.distance_unicode("ABC", "AC")
# distance = 1
normalized_distance = dl.normalized_distance_unicode("ABC", "AC")
# normalized_distance = 0.33
similarity = dl.similarity_unicode("ABC", "AC")
# similarity = 0.66
```

## Get started

1. First, create a virtual python environment.
2. Install packages `pip install -r requirements.txt`
3. Create the Rust binary
    1. Full performance: `maturin build --release` and `pip install target/wheels/*.whl`
    2. Develop version:  `maturin develop`
4. Run the tests `python tests/DamerauLevenshteinTest.py`

## Performance

Tests are executed on a Mac Mini with M1 chip with Python 3.10.
Redo these tests in [tests/DamerauLevenshteinTest.py](tests/DamerauLevenshteinTest.py).

#### List comparisons

```python
import random
import time
import pyrsdameraulevenshtein
from fastDamerauLevenshtein import damerauLevenshtein
from pyxdameraulevenshtein import damerau_levenshtein_distance

n = 100000
x = 10

print("Int lists:")
a_lists = [random.sample(list(range(x)), k=x, counts=[x for i in range(x)]) for i in range(n)]
b_lists = [random.sample(list(range(x)), k=x, counts=[x for i in range(x)]) for i in range(n)]
tic = time.perf_counter()
for a, b in zip(a_lists, b_lists):
    result = pyrsdameraulevenshtein.distance_int(a, b)
toc = time.perf_counter()
print(f"{toc - tic:0.4f} seconds, THIS implementation")
# 0.0847 seconds, THIS implementation <<< BEST PERFORMANCE
tic = time.perf_counter()
for a, b in zip(a_lists, b_lists):
    result = damerau_levenshtein_distance(a, b)
toc = time.perf_counter()
print(f"{toc - tic:0.4f} seconds, pyxdameraulevenshtein")
# 0.3073 seconds, pyxdameraulevenshtein
tic = time.perf_counter()
for a, b in zip(a_lists, b_lists):
    result = damerauLevenshtein(a, b, similarity=False)
toc = time.perf_counter()
print(f"{toc - tic:0.4f} seconds, fastDamerauLevenshtein")
# 0.1257 seconds, fastDamerauLevenshtein
```

#### String comparisons

```python
import random
import time
import jellyfish
import textdistance
import pyrsdameraulevenshtein
from fastDamerauLevenshtein import damerauLevenshtein
from pyxdameraulevenshtein import damerau_levenshtein_distance

n = 100000
x = 10

print("Strings:")
a_strings = [
    "".join(random.sample(list(chr(ord("A") + i) for i in range(x)), k=x, counts=[x for i in range(x)]))
    for y in range(n)]
b_strings = [
    "".join(random.sample(list(chr(ord("A") + i) for i in range(x)), k=x, counts=[x for i in range(x)]))
    for y in range(n)]
tic = time.perf_counter()
for a, b in zip(a_strings, b_strings):
    result = pyrsdameraulevenshtein.distance_unicode(a, b)
toc = time.perf_counter()
print(f"{toc - tic:0.4f} seconds, THIS implementation")
# 0.0764 seconds, THIS implementation
tic = time.perf_counter()
for a, b in zip(a_strings, b_strings):
    result = damerau_levenshtein_distance(a, b)
toc = time.perf_counter()
print(f"{toc - tic:0.4f} seconds, pyxdameraulevenshtein")
# 0.3925 seconds, pyxdameraulevenshtein
tic = time.perf_counter()
for a, b in zip(a_strings, b_strings):
    result = damerauLevenshtein(a, b, similarity=False)
toc = time.perf_counter()
print(f"{toc - tic:0.4f} seconds, fastDamerauLevenshtein")
# 0.1275 seconds, fastDamerauLevenshtein
tic = time.perf_counter()
for a, b in zip(a_strings, b_strings):
    result = jellyfish.damerau_levenshtein_distance(a, b)
toc = time.perf_counter()
print(f"{toc - tic:0.4f} seconds, jellyfish.damerau_levenshtein_distance")
# 0.0546 seconds, jellyfish.damerau_levenshtein_distance
tic = time.perf_counter()
for a, b in zip(a_strings, b_strings):
    result = textdistance.DamerauLevenshtein(a, b)
toc = time.perf_counter()
print(f"{toc - tic:0.4f} seconds, textdistance.DamerauLevenshtein")
# 0.0191 seconds, textdistance.DamerauLevenshtein <<< BEST PERFORMANCE
```
