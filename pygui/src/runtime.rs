use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tao::event_loop::EventLoopProxy;
use tokio::sync::Mutex;

use crate::events::{RuntimeMessage, WindowEvent};

static GLOBAL_PROXY: Lazy<Arc<Mutex<Option<Arc<EventLoopProxy<RuntimeMessage>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

/// Sync-Setter
pub fn set_proxy(proxy: EventLoopProxy<RuntimeMessage>) {
    let mut guard = futures::executor::block_on(GLOBAL_PROXY.lock());
    *guard = Some(Arc::new(proxy));
}

/// Async-Getter
async fn get_proxy() -> Option<Arc<EventLoopProxy<RuntimeMessage>>> {
    GLOBAL_PROXY.lock().await.as_ref().cloned()
}

pub async fn send_window_event(event: WindowEvent) -> Result<()> {
    let proxy = get_proxy()
        .await
        .ok_or_else(|| anyhow!("Proxy nicht gesetzt"))?;
    proxy
        .send_event(RuntimeMessage::Window(event))
        .map_err(|e| anyhow::anyhow!("Failed to send event: {:?}", e.to_string()))?;
    Ok(())
}
