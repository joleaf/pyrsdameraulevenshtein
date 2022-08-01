use std::cmp::{max, min};
use pyo3::prelude::*;
use pyo3::types::PyList;


pub fn distance_native<'a, T: PartialEq>(seq1: &'a Vec<T>, seq2: &'a Vec<T>) -> usize {
    //println!("seq1: {}; seq2 {}", seq1, seq2);
    // Swap if len(seq1) < len(seq2)
    let mut seq1 = seq1;
    let mut seq2 = seq2;
    if seq2.len() < seq1.len() {
        (seq1, seq2) = (seq2, seq1);
    }
    //println!("seq1: {}; seq2 {}", seq1, seq2);
    // Shorten the list, if the start of the lists are equal
    let mut first_differing_index = 0;
    while first_differing_index < seq1.len()
        && first_differing_index < seq2.len()
        && seq1[first_differing_index] == seq2[first_differing_index] {
        first_differing_index += 1;
    }
    //println!("first_differing_index: {}", first_differing_index);
    let seq1 = &seq1[first_differing_index..seq1.len()];
    let seq2 = &seq2[first_differing_index..seq2.len()];
    //println!("seq1: {}; seq2 {}", seq1, seq2);

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
    //let mut storage: Vec<usize> = Vec::new()

    let two_ago: usize = 0;
    let one_ago: usize = 1;
    let this_row: usize = 2;

    for i in 1..offset {
        storage[this_row * offset + (i - 1)] = i
    }
    //println!("storage {:?}", storage);

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
            if i > 0 && j > 0
                && seq1[i] == seq2[j - 1]
                && seq1[i - 1] == seq2[j]
                && seq1[i] != seq2[j] {
                if j > 1 {
                    storage[this_row * offset + j] = min(storage[this_row * offset + j],
                                                         storage[two_ago * offset + j - 2] + 1)
                } else {
                    storage[this_row * offset + j] = min(storage[this_row * offset + j],
                                                         storage[seq2.len()] + 1)
                }
            }
            //println!("{:?}", storage);
        }
    }
    //println!("{:?}", storage);
    storage[this_row * offset + (seq2.len() - 1)]
}


/// Calculates the Damerau Levenshtein distance between two int lists
#[pyfunction]
fn int_distance<'a>(seq1: &'a PyList, seq2: &'a PyList) -> PyResult<usize> {
    let seq1: Vec<i32> = seq1.extract().unwrap();
    let seq2: Vec<i32> = seq2.extract().unwrap();
    Ok(distance_native(&seq1, &seq2))
}

/// Calculates the normalized Damerau Levenshtein distance between two arrays
#[pyfunction]
fn int_normalized_distance<'a>(seq1: &'a PyList, seq2: &'a PyList) -> PyResult<f64> {
    let _n = max(seq1.len(), seq2.len());
    Ok(int_distance(&seq1, &seq2).unwrap() as f64 / max(_n, 1) as f64)
}

/// Calculates the similarity between two arrays based on the normalized Damerau Levenshtein distance
#[pyfunction]
fn int_similarity<'a>(seq1: &'a PyList, seq2: &'a PyList) -> PyResult<f64> {
    Ok(1.0 as f64 - int_normalized_distance(&seq1, &seq2).unwrap())
}

// /// Calculates the Damerau Levenshtein distance between two strings
//#[pyfunction]
//unsafe fn str_distance<'a>(seq1: &'a PyUnicode, seq2: &'a PyUnicode) -> PyResult<usize> {
//    let seq1 = Python::with_gil(|py| {
//        PyList::new(py, seq1.to_string().chars().collect())
//    });
//    let seq2 = Python::with_gil(|py| {
//        PyList::new(py, seq2.to_string().chars().collect())
//    });
//    distance(seq1, seq2)
//}

// /// Calculates the normalized Damerau Levenshtein distance between two strings
//#[pyfunction]
//unsafe fn str_normalized_distance<'a>(seq1: &'a PyUnicode, seq2: &'a PyUnicode) -> PyResult<f64> {
//    let n = max(seq1.len(), seq2.len());
//    Ok(str_distance(&seq1, &seq2).unwrap() as f64 / max(n, 1) as f64)
//}

// /// Calculates the similarity between two strings based on the normalized Damerau Levenshtein distance
//#[pyfunction]
//unsafe fn str_similarity<'a>(seq1: &'a PyUnicode, seq2: &'a PyUnicode) -> PyResult<f64> {
//    let n = max(seq1.len(), seq2.len());
//    Ok(1.0 as f64 - str_normalized_distance(&seq1, &seq2).unwrap())
//}

/// A Python module implemented in Rust.
#[pymodule]
fn pyrsdameraulevenshtein(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(int_distance, m)?)?;
    m.add_function(wrap_pyfunction!(int_normalized_distance, m)?)?;
    m.add_function(wrap_pyfunction!(int_similarity, m)?)?;
    //m.add_function(wrap_pyfunction!(str_distance, m)?)?;
    Ok(())
}