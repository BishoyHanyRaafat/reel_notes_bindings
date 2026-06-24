use pyo3::prelude::*;

#[pyfunction]
pub fn double(x: usize) -> usize {
    x * 2
}
