# PyRsDamerauLevenshtein

[Damerau-Levenshtein](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance) Implementation in Rust as
Python package.

Based on the [C implementation pyxDamerauLevenshtein](https://github.com/lanl/pyxDamerauLevenshtein).

## Install

```shell
pip install pyrsdameraulevenshtein
```
## Use
```python
import pyrsdameraulevenshtein
distance =  pyrsdameraulevenshtein.distance_int([1,2,3], [1,3])
# distance = 1
distance =  pyrsdameraulevenshtein.distance_unicode("ABC", "AC")
# distance = 1
```

## Get started

1. First, create a virtual python environment.
2. Install packages `pip install -r requirements.txt`
3. Create the Rust binary
    1. Full performance: `maturin build --release` and `pip install target/wheels/*.whl`
    2. Develop version:  `maturin develop`
4. Run the tests `python tests/DamerauLevenshteinTest.py`

## Performance

Speed comparison with the [C implementation pyxDamerauLevenshtein](https://github.com/lanl/pyxDamerauLevenshtein) results in 4 times faster performance.

```python
import random
import time
import pyrsdameraulevenshtein
from pyxdameraulevenshtein import damerau_levenshtein_distance

n = 100000
x = 10
a_lists = [random.sample(list(range(x)), k=x, counts=[x for i in range(x)]) for i in range(n)]
b_lists = [random.sample(list(range(x)), k=x, counts=[x for i in range(x)]) for i in range(n)]

tic = time.perf_counter()
for a, b in zip(a_lists, b_lists):
    result = pyrsdameraulevenshtein.distance_int(a, b)
toc = time.perf_counter()
print(f"{toc - tic:0.4f} seconds, RUST implementation")
# 0.0864 seconds, RUST implementation

tic = time.perf_counter()
for a, b in zip(a_lists, b_lists):
    result = damerau_levenshtein_distance(a, b)
toc = time.perf_counter()
print(f"{toc - tic:0.4f} seconds, Gold standard - pyxdameraulevenshtein implementation")
# 0.3195 seconds, Gold standard - pyxdameraulevenshtein implementation
````
