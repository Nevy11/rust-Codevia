use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use std::ffi::CString;

pub fn run_python_code() -> String {
    Python::attach(|py| {
        let code = c_str!("'Hello from embedded Python!'");
        let result: String = py.eval(code, None, None).unwrap().extract().unwrap();
        result
    })
}

pub fn process_python_string(code: &str) -> i32 {
    Python::attach(|py| {
        // Locals for variable sharing
        let locals = [("input_str", "Hello from Rust!")]
            .into_py_dict(py)
            .unwrap();

        // Convert Rust string -> CString for safety
        let c_code = CString::new(code).unwrap();
        let c_code_str = c_code.as_c_str();

        // Run the Python block
        py.run(c_code_str, None, Some(&locals)).unwrap();

        // Extract the result into Rust
        let result: i32 = locals
            .get_item("result")
            .unwrap()
            .unwrap()
            .extract()
            .unwrap();

        result
    })
}
