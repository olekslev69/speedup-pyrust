// rustimport:pyo3
use pyo3::prelude::*;

#[pyfunction]
fn revert_string(input_str: String) -> String {
    let mut s = String::new();
    for c in input_str.chars().rev() { 
        s.push(c)
    }
    s
}