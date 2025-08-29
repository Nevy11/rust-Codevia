use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

#[allow(dead_code)]
pub fn convert_case() -> PyResult<()> {
    Python::with_gil(|py| {
        // create locals (unwrap Result into real dict)
        let locals = [("output", py.None())]
            .into_py_dict(py)
            .expect("failed to create dict");

        // Python code to capture stdout
        #[allow(unused_variables)]
        let code = r#"
import io, sys
from contextlib import redirect_stdout

f = io.StringIO()
with redirect_stdout(f):
    exec(user_code)

output = f.getvalue()
"#;

        // insert user code
        locals.set_item("user_code", "print('Hello world')")?;

        // run Python code (use run_bound instead of run)
        // py.run_bound(code, None, Some(&locals))?;

        // extract result
        let result: String = locals
            .get_item("output")? // PyResult<Option<Bound>>
            .unwrap() // unwrap Option
            .extract()?; // convert to String

        println!("Captured Output: {}", result);

        Ok(())
    })
}
