use pyo3::prelude::*;

/// return the result of `a / b`
#[pyfunction]
fn div_numbers(a: f64, b: f64) -> PyResult<f64> {
    Ok(schedulerclient::div_numbers(a, b))
}

/// scheduler-client python module impl in rust
#[pymodule]
mod schedulerclient_py {
    #[pymodule_export]
    use super::div_numbers;
}
