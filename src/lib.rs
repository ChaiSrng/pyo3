use pyo3::prelude::*;
use std::collections::HashMap;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn word_counter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(count_words, m)?)?;
    Ok(())
}

#[pyfunction]
fn count_words(s: String) -> Py<PyAny>{
    let mut hm = HashMap::new();
    //iterate through all words in the string
    for sub_str in s.split(' '){

        //using hashmap entry api to get access to the count of a word 
        //or if the word doesnt exists create a new entry with the count set to 0
        let count:&mut i32 = hm 
            .entry(sub_str)
            .or_insert(0);
        //if entry already exists we will increment count by 1
        *count +=1;
    }

    //returning hashmap as a py object
    return Python::with_gil(|py|{
        hm.to_object(py)
    });
}
