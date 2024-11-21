use pyo3::prelude::*;
use pyo3::types::{PyList, PyString};
use std::cmp::{max, min};

/// Calculates the Damerau-Levenshtein distance between two lists of PartialEq elements.
///
///
/// # Arguments
/// * `seq1` - The one list
/// * `seq2` - The other list
///
/// # Examples
/// ```
/// // Get the distance between the two lists
/// use pyrsdameraulevenshtein;
/// let seq1 = vec![1, 2, 3, 4];
/// let seq2 = vec![2, 1, 3, 4];
/// assert_eq!(1, distance_native(&seq1, &seq2))
/// ```
pub fn distance_native<'a, T: PartialEq>(seq1: &'a Vec<T>, seq2: &'a Vec<T>) -> usize {
    // Swap if len(seq1) < len(seq2)
    let mut seq1 = seq1;
    let mut seq2 = seq2;
    if seq2.len() < seq1.len() {
        (seq1, seq2) = (seq2, seq1);
    }
    // Shorten the list, if the start of the lists are equal
    let mut first_differing_index = 0;
    while first_differing_index < seq1.len()
        && first_differing_index < seq2.len()
        && seq1[first_differing_index] == seq2[first_differing_index]
    {
        first_differing_index += 1;
    }
    let seq1 = &seq1[first_differing_index..seq1.len()];
    let seq2 = &seq2[first_differing_index..seq2.len()];

    // Shortcut, return the len of a list if the len of the other list is zero
    if seq1.len() == 0 {
        return seq2.len();
    }
    if seq2.len() == 0 {
        return seq1.len();
    }

    let mut delete_cost: usize = 0;
    let mut add_cost: usize = 0;
    let mut subtract_cost: usize = 0;

    let offset = seq2.len() + 1;
    let mut storage: Vec<usize> = vec![0; 3 * offset];

    let two_ago: usize = 0;
    let one_ago: usize = 1;
    let this_row: usize = 2;

    for i in 1..offset {
        storage[this_row * offset + (i - 1)] = i
    }

    for i in 0..seq1.len() {
        // swap/initialize vectors
        for j in 0..offset {
            storage[two_ago * offset + j] = storage[one_ago * offset + j];
            storage[one_ago * offset + j] = storage[this_row * offset + j];
        }
        for j in 0..seq2.len() {
            storage[this_row * offset + j] = 0;
        }
        storage[this_row * offset + seq2.len()] = i + 1;

        // now compute costs
        for j in 0..seq2.len() {
            delete_cost = storage[one_ago * offset + j] + 1;
            if j > 0 {
                add_cost = storage[this_row * offset + (j - 1)] + 1;
                subtract_cost = storage[one_ago * offset + (j - 1)];
                if seq1[i] != seq2[j] {
                    subtract_cost += 1;
                }
            } else {
                add_cost = storage[this_row * offset + seq2.len()] + 1;
                subtract_cost = storage[one_ago * offset + seq2.len()];
                if seq1[i] != seq2[j] {
                    subtract_cost += 1;
                }
            }
            storage[this_row * offset + j] = min(min(delete_cost, add_cost), subtract_cost);
            if i > 0
                && j > 0
                && seq1[i] == seq2[j - 1]
                && seq1[i - 1] == seq2[j]
                && seq1[i] != seq2[j]
            {
                if j > 1 {
                    storage[this_row * offset + j] = min(
                        storage[this_row * offset + j],
                        storage[two_ago * offset + j - 2] + 1,
                    )
                } else {
                    storage[this_row * offset + j] =
                        min(storage[this_row * offset + j], storage[seq2.len()] + 1)
                }
            }
        }
    }
    storage[this_row * offset + (seq2.len() - 1)]
}

#[cfg(test)]
mod tests {
    use super::distance_native;

    #[test]
    fn test_distance_native_equal_lists() {
        let seq1 = vec![1, 2, 3, 4];
        let seq2 = vec![1, 2, 3, 4];
        assert_eq!(0, distance_native(&seq1, &seq2));
        assert_eq!(0, distance_native(&seq2, &seq1));
    }

    #[test]
    fn test_distance_native_one_change() {
        let seq1 = vec![1, 2, 3, 4];
        let seq2 = vec![2, 3, 4];
        assert_eq!(1, distance_native(&seq1, &seq2));
    }

    #[test]
    fn test_distance_native_one_swap() {
        let seq1 = vec![1, 2, 3, 4];
        let seq2 = vec![2, 1, 3, 4];
        assert_eq!(1, distance_native(&seq1, &seq2));
        assert_eq!(1, distance_native(&seq2, &seq1));
    }

    #[test]
    fn test_distance_native_one_empty_list() {
        let seq1 = vec![];
        let seq2 = vec![1, 2, 3, 4];
        assert_eq!(4, distance_native(&seq1, &seq2));
        assert_eq!(4, distance_native(&seq2, &seq1));
    }

    #[test]
    fn test_distance_native_one_missing() {
        let seq1 = vec![1, 2, 4];
        let seq2 = vec![1, 2, 3, 4];
        assert_eq!(1, distance_native(&seq1, &seq2));
        assert_eq!(1, distance_native(&seq2, &seq1));
    }

    #[test]
    fn test_distance_native_two_differnt_lists() {
        let seq1 = vec![1, 2, 3];
        let seq2 = vec![4, 5, 6];
        assert_eq!(3, distance_native(&seq1, &seq2));
        assert_eq!(3, distance_native(&seq2, &seq1));
    }

    #[test]
    fn test_str() {
        let seq1 = vec![String::from("A"), String::from("B"), String::from("C")];
        let seq2 = vec![String::from("A"), String::from("C"), String::from("E")];
        assert_eq!(2, distance_native(&seq1, &seq2))
    }
}

/// distance_int(seq1, seq2, /)
/// --
///
/// Calculates the Damerau-Levenshtein distance between two lists of integer values.
///
/// Example:
/// distance_int([1,2,3],[2,3])
/// -> 1
#[pyfunction]
fn distance_int<'a>(seq1: &Bound<'_, PyList>, seq2: &Bound<'_, PyList>) -> PyResult<usize> {
    let seq1: Vec<i32> = seq1.extract()?;
    let seq2: Vec<i32> = seq2.extract()?;
    Ok(distance_native(&seq1, &seq2))
}

/// normalized_distance_int(seq1, seq2, /)
/// --
///
/// Calculates the normalized Damerau-Levenshtein distance between two arrays of integer values.
///
/// Example:
/// normalized_distance_int([1,2,3],[2,3])
/// -> 0.33
#[pyfunction]
fn normalized_distance_int<'a>(
    seq1: &Bound<'_, PyList>,
    seq2: &Bound<'_, PyList>,
) -> PyResult<f64> {
    let _n = max(seq1.len(), seq2.len());
    Ok(distance_int(&seq1, &seq2)? as f64 / max(_n, 1) as f64)
}

/// similarity_int(seq1, seq2, /)
/// --
///
/// Calculates the similarity between two arrays of integer values based on the normalized Damerau-Levenshtein distance.
///
/// Example:
/// similarity_int([1,2,3],[2,3])
/// -> 0.66
///
#[pyfunction]
fn similarity_int<'a>(seq1: &Bound<'_, PyList>, seq2: &Bound<'_, PyList>) -> PyResult<f64> {
    Ok(1.0f64 - normalized_distance_int(&seq1, &seq2).unwrap())
}

/// distance_str(seq1, seq2, /)
/// --
///
/// Calculates the Damerau-Levenshtein distance between two lists of string values.
///
/// Example:
/// distance_str(["A","B","C"],["A","C"])
/// -> 1
#[pyfunction]
fn distance_str<'a>(seq1: &Bound<'_, PyList>, seq2: &Bound<'_, PyList>) -> PyResult<usize> {
    let seq1: Vec<String> = seq1.extract()?;
    let seq2: Vec<String> = seq2.extract()?;
    Ok(distance_native(&seq1, &seq2))
}

/// normalized_distance_str(seq1, seq2, /)
/// --
///
/// Calculates the normalized Damerau-Levenshtein distance between two arrays of string values.
///
/// Example:
/// normalized_distance_str(["A","B","C"],["A","C"])
/// -> 0.33
#[pyfunction]
fn normalized_distance_str<'a>(
    seq1: &Bound<'_, PyList>,
    seq2: &Bound<'_, PyList>,
) -> PyResult<f64> {
    let _n = max(seq1.len(), seq2.len());
    Ok(distance_str(&seq1, &seq2)? as f64 / max(_n, 1) as f64)
}

/// similarity_str(seq1, seq2, /)
/// --
///
/// Calculates the similarity between two arrays of string values based on the normalized Damerau-Levenshtein distance.
///
/// Example:
/// similarity_str(["A","B","C"],["A","C"])
/// -> 0.66
///
#[pyfunction]
fn similarity_str<'a>(seq1: &Bound<'_, PyList>, seq2: &Bound<'_, PyList>) -> PyResult<f64> {
    Ok(1.0 as f64 - normalized_distance_str(&seq1, &seq2)?)
}

/// distance_unicode(seq1, seq2, /)
/// --
///
/// Calculates the Damerau-Levenshtein distance between two strings.
///
/// Example:
/// distance_unicode("ABC","AC")
/// -> 1
#[pyfunction]
fn distance_unicode<'a>(seq1: &Bound<'_, PyString>, seq2: &Bound<'_, PyString>) -> PyResult<usize> {
    let seq1: Vec<char> = seq1.to_string().chars().collect();
    let seq2: Vec<char> = seq2.to_string().chars().collect();
    Ok(distance_native(&seq1, &seq2))
}

/// normalized_distance_unicode(seq1, seq2, /)
/// --
///
/// Calculates the normalized Damerau-Levenshtein distance between two strings.
///
/// Example:
/// normalized_distance_unicode("ABC","AC")
/// -> 0.33
#[pyfunction]
fn normalized_distance_unicode<'a>(
    seq1: &Bound<'_, PyString>,
    seq2: &Bound<'_, PyString>,
) -> PyResult<f64> {
    let _n = max(seq1.len()?, seq2.len()?);
    Ok(distance_unicode(&seq1, &seq2)? as f64 / max(_n, 1) as f64)
}

/// similarity_unicode(seq1, seq2, /)
/// --
///
/// Calculates the similarity between two strings based on the normalized Damerau-Levenshtein distance.
///
/// Example:
/// similarity_unicode("ABC","AC")
/// -> 0.66
///
#[pyfunction]
fn similarity_unicode(seq1: &Bound<'_, PyString>, seq2: &Bound<'_, PyString>) -> PyResult<f64> {
    Ok(1.0 as f64 - normalized_distance_unicode(&seq1, &seq2)?)
}

/// Damerau-Levenshtein distance implementation in rust for high-performance.
#[pymodule]
fn pyrsdameraulevenshtein(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(distance_int, m)?)?;
    m.add_function(wrap_pyfunction!(normalized_distance_int, m)?)?;
    m.add_function(wrap_pyfunction!(similarity_int, m)?)?;
    m.add_function(wrap_pyfunction!(distance_str, m)?)?;
    m.add_function(wrap_pyfunction!(normalized_distance_str, m)?)?;
    m.add_function(wrap_pyfunction!(similarity_str, m)?)?;
    m.add_function(wrap_pyfunction!(distance_unicode, m)?)?;
    m.add_function(wrap_pyfunction!(normalized_distance_unicode, m)?)?;
    m.add_function(wrap_pyfunction!(similarity_unicode, m)?)?;
    Ok(())
}
