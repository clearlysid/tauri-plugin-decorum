use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Configuration for windows in the application.
///
/// JSON configuration example:
/// ```json
/// {
///   "all": {
///     {
///       "windowButtons": {
///         "insetX": 15.0,
///         "insetY": 20.0,
///         "supportRTL": true,
///         "hide": []
///       },
///       "transparentWebViews": ["main"],
///       "createOverlayTitlebar": ["onboarding"]
///     }
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
///       "transparentWebViews": true,
///       "createOverlayTitlebar": ["main"]
///     }
///   ]
/// }
/// ```
/// The "all" config applies to all windows and only the specified webviews specified.
///
///
/// # Note:
/// - "hide" can include "zoom/maximize", "minimize", "close"
/// - "transparentWebViews" and "createOverlayTitlebar" can be a boolean or an array of string, each representing a webview label (e.g., ["main", "other_webview"])
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecorumConfig {
    #[serde(default)]
    pub all: WindowConfig,
    #[serde(default)]
    pub windows: Vec<LabeledWindowConfig>,
    #[serde(skip)]
    pub merged: HashMap<String, WindowConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    #[serde(default)]
    pub window_buttons: Option<WindowButtons>,
    #[serde(default)]
    pub create_overlay_titlebar: Option<BoolOrVec>,
    #[serde(default)]
    pub transparent_webviews: Option<BoolOrVec>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LabeledWindowConfig {
    pub label: String,
    #[serde(flatten)]
    pub config: WindowConfig,
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

impl DecorumConfig {
    /// Creates a new `DecorumConfig` with the given configurations and merges them.
    ///
    /// This constructor immediately calls `merge_configurations()` to ensure
    /// that the `merged` field is populated with the combined configurations.
    ///
    /// # Arguments
    ///
    /// * `all` - The default configuration applied to all windows
    /// * `windows` - A vector of labeled window-specific configurations
    ///
    /// # Returns
    ///
    /// A new `DecorumConfig` instance with merged configurations
    pub fn new(all: WindowConfig, windows: Vec<LabeledWindowConfig>) -> Self {
        let mut config = DecorumConfig {
            all,
            windows,
            merged: HashMap::new(),
        };
        config.merge_configurations();
        config
    }

    /// Merges the default "all" configuration with individual window configurations.
    ///
    /// This method populates the `merged` field with configurations for each window,
    /// combining window-specific settings with the default "all" settings.
    /// Window-specific settings take priority over the default settings.
    fn merge_configurations(&mut self) {
        let default_config = self.all.clone();
        let mut merged = HashMap::new();

        for LabeledWindowConfig { label, config } in &self.windows {
            let merged_config = WindowConfig {
                window_buttons: config
                    .window_buttons
                    .clone()
                    .or_else(|| default_config.window_buttons.clone()),
                create_overlay_titlebar: merge_bool_or_vec(
                    &config.create_overlay_titlebar,
                    &default_config.create_overlay_titlebar,
                ),
                transparent_webviews: merge_bool_or_vec(
                    &config.transparent_webviews,
                    &default_config.transparent_webviews,
                ),
            };
            merged.insert(label.clone(), merged_config);
        }

        // Add default config for any window not explicitly defined
        for label in self.get_all_window_labels() {
            merged
                .entry(label)
                .or_insert_with(|| default_config.clone());
        }

        self.merged = merged;
    }

    /// Retrieves all unique window labels mentioned in the configuration.
    ///
    /// This method collects labels from both the "all" section (for `create_overlay_titlebar` and `transparent_webviews`)
    /// and the individual window configurations.
    ///
    /// # Returns
    ///
    /// Returns a `HashSet<String>` containing all unique window labels.
    pub fn get_all_window_labels(&self) -> std::collections::HashSet<String> {
        let mut labels = std::collections::HashSet::new();

        // Collect labels from "all" section
        if let Some(BoolOrVec::Items(items)) = &self.all.create_overlay_titlebar {
            labels.extend(items.iter().cloned());
        }
        if let Some(BoolOrVec::Items(items)) = &self.all.transparent_webviews {
            labels.extend(items.iter().cloned());
        }

        // Collect labels from individual window configs
        labels.extend(self.windows.iter().map(|w| w.label.clone()));

        labels
    }

    /// Retrieves the merged configuration for a specific window.
    ///
    /// # Arguments
    ///
    /// * `label` - The label of the window to retrieve the configuration for
    ///
    /// # Returns
    ///
    /// Returns an `Option<&WindowConfig>` containing the merged configuration for the specified window,
    /// or `None` if no configuration exists for the given label.
    pub fn get_window_config(&self, label: &str) -> Option<&WindowConfig> {
        self.merged.get(label)
    }
}

/// Merges two `BoolOrVec` options, prioritizing the specific configuration over the default.
///
/// This is used to combine window-specific settings with the default "all" settings.
///
/// # Arguments
///
/// * `specific` - The `Option<BoolOrVec>` from a specific window configuration
/// * `default` - The `Option<BoolOrVec>` from the default "all" configuration
///
/// # Returns
///
/// Returns an `Option<BoolOrVec>` that represents the merged configuration:
/// - If `specific` is `Some(BoolOrVec::Bool(true))`, it returns that.
/// - If `specific` is `Some(BoolOrVec::Items(items))` with non-empty items, it returns that.
/// - If `specific` is `None` and `default` is `Some`, it returns the `default` value.
/// - Otherwise, it returns `None`.
pub fn merge_bool_or_vec(
    specific: &Option<BoolOrVec>,
    default: &Option<BoolOrVec>,
) -> Option<BoolOrVec> {
    match (specific, default) {
        (Some(BoolOrVec::Bool(true)), _) => Some(BoolOrVec::Bool(true)),
        (Some(BoolOrVec::Items(items)), _) if !items.is_empty() => {
            Some(BoolOrVec::Items(items.clone()))
        }
        (None, Some(default_value)) => Some(default_value.clone()),
        _ => None,
    }
}

impl Default for DecorumConfig {
    fn default() -> Self {
        let mut config = DecorumConfig {
            all: WindowConfig::default(),
            windows: Vec::new(),
            merged: HashMap::new(),
        };
        config.merge_configurations();
        config
    }
}
