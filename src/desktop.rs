use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> std::io::Result<TransparentTitlebar<R>> {
    Ok(TransparentTitlebar(app.clone()))
}

/// Access to the transparent-titlebar APIs.
pub struct TransparentTitlebar<R: Runtime>(AppHandle<R>);

impl<R: Runtime> TransparentTitlebar<R> {
    pub fn ping(&self) -> bool {
        true
    }
}
