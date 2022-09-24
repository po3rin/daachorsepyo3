use daachorse::CharwiseDoubleArrayAhoCorasick;
use pyo3::prelude::*;

// Formats the sum of two numbers as string.
#[pyfunction]
fn substring_match(text_list: Vec<String>, patterns: Vec<String>) -> PyResult<Vec<Vec<i32>>> {
    let result = find_iter_with_charwise(text_list, patterns);
    Ok(result)
}

// A Python module implemented in Rust. The name of this function must match
// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
// import the module.
#[pymodule]
fn daachorsepyo3(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(substring_match, m)?)?;
    Ok(())
}

fn find_iter_with_charwise(text_list: Vec<String>, patterns: Vec<String>) -> Vec<Vec<i32>> {
    let pma = CharwiseDoubleArrayAhoCorasick::new(patterns).unwrap();

    let mut result: Vec<Vec<i32>> = Vec::new();
    for text in text_list {
        let it = pma.find_iter(text);
        let vec: Vec<i32> = it.map(|i| i.value()).collect();
        result.push(vec);
    }
    result
}
