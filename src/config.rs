use std::{
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
};

use directories::ProjectDirs;
use toml::Value;

use crate::errors::{HnCliError, Result};

pub const HNCLI_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const ENABLE_GLOBAL_SUB_SCREEN_QUIT_SHORTCUT_DEFAULT: bool = true;
pub const DISPLAY_COMMENTS_PANEL_BY_DEFAULT_DEFAULT: bool = false;
pub const SHOW_CONTEXTUAL_HELP_DEFAULT: bool = false;

/// Persisted, global application configuration.
#[derive(Debug)]
pub struct AppConfiguration {
    /// Enable the 'q' quit shortcut in sub-screens (*i.e* everything but the initial, main screen).
    enable_global_sub_screen_quit_shortcut: bool,
    /// On the item details page, should we display the comments panel by default or not?
    display_comments_panel_by_default: bool,
    /// Show the global contextual help?
    show_contextual_help: bool,
}

impl Default for AppConfiguration {
    fn default() -> Self {
        Self {
            enable_global_sub_screen_quit_shortcut: false,
            display_comments_panel_by_default: false,
            show_contextual_help: true,
        }
    }
}

// TODO: better error handling when the configuration cannot be saved (should not panic but be logged)
impl AppConfiguration {
    pub fn from_file_or_defaults() -> Self {
        Self::from_file_or_environment().unwrap_or_default()
    }

    pub fn get_enable_global_sub_screen_quit_shortcut(&self) -> bool {
        self.enable_global_sub_screen_quit_shortcut
    }

    pub fn toggle_enable_global_sub_screen_quit_shortcut(&mut self) {
        self.enable_global_sub_screen_quit_shortcut = !self.enable_global_sub_screen_quit_shortcut;
        self.save_to_file().unwrap();
    }

    pub fn get_display_comments_panel_by_default(&self) -> bool {
        self.display_comments_panel_by_default
    }

    pub fn toggle_display_comments_panel_by_default(&mut self) {
        self.display_comments_panel_by_default = !self.display_comments_panel_by_default;
        self.save_to_file().unwrap();
    }

    pub fn get_show_contextual_help(&self) -> bool {
        self.show_contextual_help
    }

    pub fn toggle_show_contextual_help(&mut self) {
        self.show_contextual_help = !self.show_contextual_help;
        self.save_to_file().unwrap();
    }

    fn save_to_file(&self) -> Result<()> {
        let config_filepath = Self::get_config_file_path()?;
        let config_directory = config_filepath.parent().unwrap();
        create_dir_all(config_directory).map_err(|err| {
            HnCliError::ConfigSynchronizationError(format!(
                "cannot create config directory ({:?}): {}",
                config_directory, err
            ))
        })?;

        // TODO: do not use manual deserialization if possible
        let config_raw = format!(
            "enable_global_sub_screen_quit_shortcut={}\n,display_comments_panel={}\nshow_contextual_help={}\n",
            self.enable_global_sub_screen_quit_shortcut,
            self.display_comments_panel_by_default,
            self.show_contextual_help
        );
        write(&config_filepath, config_raw).map_err(|err| {
            HnCliError::ConfigSynchronizationError(format!(
                "cannot save config file ({:?}): {}",
                config_filepath, err
            ))
        })
    }

    fn from_file_or_environment() -> Result<Self> {
        let config_filepath = Self::get_config_file_path()?;
        let config_raw = read_to_string(&config_filepath).map_err(|err| {
            HnCliError::ConfigSynchronizationError(format!(
                "cannot open config file ({:?}): {}",
                config_filepath, err
            ))
        })?;
        let toml = config_raw.parse::<Value>().map_err(|err| {
            HnCliError::ConfigSynchronizationError(format!(
                "cannot parse config file ({:?}): {}",
                config_filepath, err
            ))
        })?;

        Ok(Self {
            enable_global_sub_screen_quit_shortcut: toml
                .get("enable_global_sub_screen_quit_shortcut")
                .unwrap_or(&Value::Boolean(
                    ENABLE_GLOBAL_SUB_SCREEN_QUIT_SHORTCUT_DEFAULT,
                ))
                .as_bool()
                .unwrap_or(ENABLE_GLOBAL_SUB_SCREEN_QUIT_SHORTCUT_DEFAULT),
            display_comments_panel_by_default: toml
                .get("display_comments_panel")
                .unwrap_or(&Value::Boolean(DISPLAY_COMMENTS_PANEL_BY_DEFAULT_DEFAULT))
                .as_bool()
                .unwrap_or(DISPLAY_COMMENTS_PANEL_BY_DEFAULT_DEFAULT),
            show_contextual_help: toml
                .get("show_contextual_help")
                .unwrap_or(&Value::Boolean(SHOW_CONTEXTUAL_HELP_DEFAULT))
                .as_bool()
                .unwrap_or(SHOW_CONTEXTUAL_HELP_DEFAULT),
        })
    }

    fn get_config_file_path() -> Result<PathBuf> {
        let project_directories =
            ProjectDirs::from("", "pierreyoda", "hncli").ok_or_else(|| {
                HnCliError::ConfigSynchronizationError(
                    "cannot get hncli config directory from OS".into(),
                )
            })?;
        let config_directory = project_directories.config_dir();
        Ok(config_directory.join("hncli.toml"))
    }
}
