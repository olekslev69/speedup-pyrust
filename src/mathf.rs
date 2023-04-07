// rustimport:pyo3

use pyo3::prelude::*;

#[pyfunction]
fn square(x: i32) -> i32 {
    x * x
}

#[pyfunction]
fn sum_as_string(a: i32, b: i32) -> i32 {
    a + b
}
