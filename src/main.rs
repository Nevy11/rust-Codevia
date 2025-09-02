use pyo3::prelude::*;
use rust_codevia::python_interpreter::convert_case::{process_python_string, run_python_code};
fn main() -> PyResult<()> {
    let result: String = run_python_code();
    println!("{}", result);

    let code = r#"
print("hello from rust")
a = 5
b = 10
c = a + b
print(c)
result = c
"#;

    let output = process_python_string(code);
    println!("Result from Python: {}", output);
    Ok(())
}
