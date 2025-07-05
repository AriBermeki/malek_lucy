use tokio::sync::oneshot::Sender;

pub enum WindowEvent {
    GetTitle(Sender<String>),
    SetTitle(String, Sender<String>),
}

pub enum RuntimeMessage {
    Window(WindowEvent),
}
