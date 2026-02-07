use folio_lib::db_pool;

/// import the module.
#[pyo3::pymodule]
mod folio_backend {
  use pyo3::prelude::*;

  /// Formats the sum of two numbers as string.
  #[pyfunction]
  fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
  }


}
