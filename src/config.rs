use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Configuration for windows in the application.
///
/// JSON configuration example:
/// ```json
/// {
///   "all": {
///     "windowButtons": {
///       "insetX": 15.0,
///       "insetY": 20.0,
///       "supportRTL": true,
///       "hide": [],
///     },
///     "clickCloseToHide": true
///     "transparentWebViews": ["main"],
///     "createOverlayTitlebar": ["onboarding"]
///   },
///   "windows": [
///     {
///       "label": "main",
///       "windowButtons": {
///         "insetX": 15.0,
///         "insetY": 20.0,
///         "supportRTL": true,
///         "hide": []
///       },
///       "clickCloseToHide": false
///       "transparentWebViews": true,
///       "createOverlayTitlebar": ["main"],
///     }
///   ]
/// }
/// ```
/// The "all" config applies to all windows and only the specified webviews specified.
///
///
/// # Note:
/// - "clickCloseToHide" (macOS Only) hides the window instead of quitting
/// - "hide" can include "zoom/maximize", "minimize", "close"
/// - "transparentWebViews" and "createOverlayTitlebar" can be a boolean or an array of string, each representing a webview label (e.g., "main", "other_webview")
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecorumPluginConfig {
    #[serde(default)]
    pub all: WindowConfig,
    #[serde(default)]
    pub windows: Vec<LabeledWindowConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WindowConfig {
    #[serde(default)]
    pub window_buttons: Option<WindowButtons>,
    #[serde(default)]
    pub create_overlay_titlebar: Option<BoolOrVec>,
    #[serde(default)]
    pub transparent_webviews: Option<BoolOrVec>,
    #[serde(default)]
    pub click_close_to_hide: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LabeledWindowConfig {
    pub label: String,
    #[serde(flatten)]
    pub config: WindowConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
            window_buttons: None,
            create_overlay_titlebar: None,
            transparent_webviews: None,
            click_close_to_hide: None,
        }
    }
}

impl Default for DecorumPluginConfig {
    fn default() -> Self {
        DecorumPluginConfig {
            all: WindowConfig::default(),
            windows: Vec::new(),
        }
    }
}

impl DecorumPluginConfig {
    /// Merges the default "all" configuration with individual window configurations.
    ///
    /// This method populates the `merged` field with configurations for each window,
    /// combining window-specific settings with the default "all" settings.
    /// Window-specific settings take priority over the default settings.
    pub fn merged(&self) -> HashMap<String, WindowConfig> {
        let mut merged = HashMap::new();

        for LabeledWindowConfig { label, config } in &self.windows {
            match merge(&self.all, config) {
                Ok(config) => {
                    merged.insert(label.clone(), config);
                }
                Err(err) => {
                    eprintln!("Plugin Decorum - Skipping configs for window \"{}\" due to failure during merge: {}", label, err);
                }
            };
        }

        // Add default config for any window not explicitly defined
        for label in self.get_all_window_labels() {
            merged.entry(label).or_insert_with(|| self.all.clone());
        }

        merged
    }

    /// Retrieves all unique window labels mentioned in the configuration.
    ///
    /// This method collects labels from both the "all" section (for `create_overlay_titlebar` and `transparent_webviews`)
    /// and the individual window configurations.
    ///
    /// # Returns
    ///
    /// Returns a `HashSet<String>` containing all unique window labels.
    pub fn get_all_window_labels(&self) -> HashSet<String> {
        let mut labels = HashSet::new();

        if let Some(BoolOrVec::Items(items)) = &self.all.create_overlay_titlebar {
            labels.extend(items.iter().cloned());
        }
        if let Some(BoolOrVec::Items(items)) = &self.all.transparent_webviews {
            labels.extend(items.iter().cloned());
        }

        labels.extend(self.windows.iter().map(|w| w.label.clone()));
        labels
    }
}

// Credits: https://github.com/jondot/merge-struct/
fn to_value<T: serde::ser::Serialize>(value: &T) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(value)
}

fn from_value<T: serde::ser::Serialize + serde::de::DeserializeOwned>(
    value: serde_json::Value,
) -> Result<T, serde_json::Error> {
    serde_json::from_value(value)
}

fn merge<T: serde::ser::Serialize + serde::de::DeserializeOwned>(
    base: &T,
    overrides: &T,
) -> Result<T, serde_json::Error> {
    let mut left = to_value(base)?;
    let right = to_value(overrides)?;
    merge_value(&mut left, &right);
    from_value(left)
}

fn merge_value(a: &mut Value, b: &Value) {
    match (a, b) {
        (Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge_value(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (Value::Array(ref mut a), &Value::Array(ref b)) => {
            a.extend(b.clone());
        }
        (Value::Array(ref mut a), &Value::Object(ref b)) => {
            a.extend([Value::Object(b.clone())]);
        }
        (_, Value::Null) => {} // do nothing
        (a, b) => {
            *a = b.clone();
        }
    }
}
