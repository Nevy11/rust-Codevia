use pyo3::prelude::*;
use rust_codevia::python_interpreter::convert_case::run_python_code;
fn main() -> PyResult<()> {
    let result: String = run_python_code();
    println!("{}", result);
    Ok(())
}
