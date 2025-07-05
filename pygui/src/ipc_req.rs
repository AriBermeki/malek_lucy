use pyo3::{Py, PyAny, Python};
use wry::http::Request;

/// Create a Wry IPC request handler that delegates work to a Python callback.
///
/// This function takes a Python-callable object (`handler`) and returns a closure
/// suitable for use as a Wry HTTP request handler. When the closure is invoked,
/// it:
/// 1. Acquires the Python GIL.
/// 2. Calls your Rust-to-Python bridge (`crate::executpy::executer`) with:
///    - the current Python interpreter handle,
///    - a cloned reference to your Python handler,
///    - the raw request body as a `String`.
/// 3. Returns the Python-side result (`Py<PyAny>`) directly if it succeeds,
///    or prints an error and returns `None` on failure (so the Rust type stays `Py<PyAny>`).
///
/// # Parameters
/// - `handler: Py<PyAny>`  
///   A reference to a Python function or callable object. This will be cloned
///   for each incoming request so that Python code can be run safely.
///
/// # Returns
/// A `Fn(Request<String>) + 'static` closure.  
/// When invoked by Wry with an HTTP request, it extracts the request body,
/// passes it into the Python handler via `executer`, and yields the Python result.
///
/// # Panics & Errors
/// - Any panic inside the Python code will be caught and printed to stderr.
/// - On Python-side errors, the handler prints an error message (`Some Error: …`)
///   and returns Python’s `None` to keep the return type consistent.
///
/// # Example
/// ```rust,ignore
/// // Suppose you have a Python function `process_event` exposed via PyO3:
/// let py_handler = /* obtain Py<PyAny> for process_event */;
/// let ipc_handler = handle_ipc_req(py_handler);
///
/// // Register with Wry:
/// webview_builder
///     .with_ipc_handler(ipc_handler)
///     .build()?;
/// ```
pub fn handle_ipc_req(handler: Py<PyAny>) -> impl Fn(Request<String>) + 'static {
    move |_req: Request<String>| {
        Python::with_gil(|py| {
            // match expression *without* a trailing semicolon, so it is returned
            match crate::executpy::executer(py, handler.clone_ref(py), _req.body().to_string()) {
                Ok(res) => res,
                Err(error) => {
                    eprintln!("Some Error: {:?}", error);
                    // return Python `None` so the type is still `Py<PyAny>`
                    py.None()
                }
            }
        });
    }
}
