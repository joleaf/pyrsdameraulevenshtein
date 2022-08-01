# PyRsDamerauLevenshtein
Damerau Levenshtein Implementation in Rust as Python package.

Based on the [C implementation pyxDamerauLevenshtein](https://github.com/lanl/pyxDamerauLevenshtein).

## Get started

1. First, create a virtual python environment.
2. Install packages `pip install -r requirements.txt`
3. Create the Rust binary `maturin build`
4. Run the tests in `python tests/DamerauLevenshteinTest.py`


## NOTE
This project is WIP and does NOT perform as fast as the C implementation!