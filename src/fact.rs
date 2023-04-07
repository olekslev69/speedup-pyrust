//rustimport:pyo3

use pyo3::prelude::*;

#[pyfunction]
fn factorial(x: i32) -> i32 {
    if x == 1 {
        1
    }
    else{
        x*factorial(x-1)
    }  
}