use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> std::io::Result<Decorum<R>> {
    Ok(Decorum(app.clone()))
}

/// Access to the decorum APIs.
pub struct Decorum<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Decorum<R> {
    pub fn ping(&self) -> bool {
        true
    }
}
