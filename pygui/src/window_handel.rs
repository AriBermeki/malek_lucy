use crate::runtime::send_window_event;
use pyo3::prelude::*;

#[pyclass]
/// A handle to control a window's title via async Python coroutines.
pub struct WindowHandle {}

#[pymethods]
impl WindowHandle {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(Self {})
    }

    /// Asynchronously set the window title. Returns a Python awaitable (coroutine).
    pub fn set_title(&self, py: Python, new_title: String) -> PyResult<PyObject> {
        // Capture the current Tokio loop and Python context
        let locals = pyo3_async_runtimes::TaskLocals::with_running_loop(py)?.copy_context(py)?;
        let (tx, rx) = tokio::sync::oneshot::channel::<String>();

        // Define an async block that sends the event and awaits the response
        let fut = async move {
            send_window_event(crate::events::WindowEvent::SetTitle(new_title, tx))
                .await
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to send window event: {e:?}"
                    ))
                })?;

            let received = rx.await.map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to receive response: {e:?}"
                ))
            })?;

            Ok(received)
        };

        // Convert the Rust future into a Python coroutine bound to the captured context
        let coroutine =
            pyo3_async_runtimes::tokio::future_into_py_with_locals(py, locals.clone_ref(py), fut)?;

        Ok(coroutine.into())
    }

    /// Asynchronously get the window title. Returns a Python awaitable (coroutine).
    pub fn get_title(&self, py: Python) -> PyResult<PyObject> {
        // Capture the current Tokio loop and Python context
        let locals = pyo3_async_runtimes::TaskLocals::with_running_loop(py)?.copy_context(py)?;
        let (tx, rx) = tokio::sync::oneshot::channel::<String>();

        let fut = async move {
            send_window_event(crate::events::WindowEvent::GetTitle(tx))
                .await
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to send window event: {e:?}"
                    ))
                })?;

            let received = rx.await.map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to receive response: {e:?}"
                ))
            })?;

            Ok(received)
        };

        let coroutine =
            pyo3_async_runtimes::tokio::future_into_py_with_locals(py, locals.clone_ref(py), fut)?;

        Ok(coroutine.into())
    }
}
