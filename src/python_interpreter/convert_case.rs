use pyo3::ffi::c_str;
use pyo3::prelude::*;

pub fn run_python_code() -> String {
    Python::attach(|py| {
        let code = c_str!("'Hello from embedded Python!'");
        let result: String = py.eval(code, None, None).unwrap().extract().unwrap();
        result
    })
}
