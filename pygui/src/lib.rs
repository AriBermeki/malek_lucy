use crate::{events::RuntimeMessage, runtime::set_proxy};
use pyo3::prelude::*;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
};
mod events;
mod executpy;
mod ipc_req;
mod runtime;
mod window_handel;
/// Formats the sum of two numbers as string.
///
#[pyfunction]
fn create_webframe(handler: Py<PyAny>, html: String) -> PyResult<()> {
    let event_loop = EventLoopBuilder::<RuntimeMessage>::with_user_event().build();

    set_proxy(event_loop.create_proxy().clone());

    let window = tao::window::WindowBuilder::new()
        .with_title("PyFrame")
        .build(&event_loop)
        .map_err(|err| pyo3::exceptions::PyOSError::new_err(err.to_string()))?;

    let _webview = wry::WebViewBuilder::new()
        .with_ipc_handler(ipc_req::handle_ipc_req(handler))
        .with_html(&html)
        .build(&window)
        .map_err(|err| pyo3::exceptions::PyRuntimeError::new_err(err.to_string()))?;

    event_loop.run(move |event, _window_target, flow: &mut ControlFlow| {
        *flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                window_id, event, ..
            } => match event {
                WindowEvent::CloseRequested => {
                    println!("Close requested for window {:?}", window_id);
                    *flow = ControlFlow::Exit;
                }
                _ => {}
            },
            Event::UserEvent(user_event) => match user_event {
                RuntimeMessage::Window(window_event) => match window_event {
                    events::WindowEvent::GetTitle(sender) => {
                        let _ = sender.send(window.title());
                    }
                    events::WindowEvent::SetTitle(title, sender) => {
                        window.set_title(&title);
                        let _ = sender.send("Ok".to_string());
                    }
                },
            },
            _ => {}
        }
    });
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyframe(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_webframe, m)?)?;
    m.add_class::<window_handel::WindowHandle>()?;
    Ok(())
}
