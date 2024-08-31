use serde::{Deserialize, Serialize};

/// Configuration for windows in the application.
///
/// Default JSON configuration:
/// ```json
/// {
///   "windows": [
///     {
///       "label": "main",
///       "windowButtons": {
///         "insetX": 15.0,
///         "insetY": 20.0,
///         "supportRTL": true,
///         "hide": []
///       },
///       "transparentWebViews": true,
///       "createOverlayTitlebar": ["main"]
///     }
///   ]
/// }
/// ```
///
/// # Note:
/// - "hide" can include "zoom/maximize", "minimize", "close"
/// - "transparentWebViews" and "createOverlayTitlebar" can be a boolean or an array of string, each representing a webview label (e.g., ["main", "other_webview"])
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecorumConfig {
    #[serde(default)]
    pub windows: Vec<WindowConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub window_buttons: Option<WindowButtons>,
    #[serde(default)]
    pub create_overlay_titlebar: Option<BoolOrVec>,
    #[serde(default)]
    pub transparent_webviews: Option<BoolOrVec>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowButtons {
    #[serde(default)]
    pub inset_x: Option<f64>,
    #[serde(default)]
    pub inset_y: Option<f64>,
    #[serde(default)]
    pub support_rtl: Option<bool>,
    #[serde(default)]
    pub hide: Option<BoolOrVec>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum BoolOrVec {
    Bool(bool),
    Items(Vec<String>),
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            label: String::new(),
            window_buttons: Some(WindowButtons::default()),
            create_overlay_titlebar: Some(BoolOrVec::default()),
            transparent_webviews: Some(BoolOrVec::default()),
        }
    }
}

impl Default for WindowButtons {
    fn default() -> Self {
        WindowButtons {
            inset_x: Some(10.0),
            inset_y: Some(15.0),
            support_rtl: Some(false),
            hide: Some(BoolOrVec::default()),
        }
    }
}

impl Default for BoolOrVec {
    fn default() -> Self {
        BoolOrVec::Bool(false)
    }
}
