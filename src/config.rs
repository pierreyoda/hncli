use std::{
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
};

use directories::ProjectDirs;
use log::warn;
use serde::{Deserialize, Serialize};

use crate::errors::{HnCliError, Result};

pub const HNCLI_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const ENABLE_GLOBAL_SUB_SCREEN_QUIT_SHORTCUT_DEFAULT: bool = true;
pub const DISPLAY_COMMENTS_PANEL_BY_DEFAULT_DEFAULT: bool = false;
pub const DISPLAY_MAIN_ITEMS_LIST_ITEM_META: bool = false;
pub const SHOW_CONTEXTUAL_HELP_DEFAULT: bool = true;

/// Persisted, global application configuration.
#[derive(Debug, Serialize)]
pub struct AppConfiguration {
    /// Enable the 'q' quit shortcut in sub-screens (*i.e* everything but the initial, main screen).
    enable_global_sub_screen_quit_shortcut: bool,
    /// On the item details page, should we display the comments panel by default or not?
    display_comments_panel_by_default: bool,
    /// On the main items list (home screen), should we display the items' metadata (score, number of comments, etc.)?
    display_main_items_list_item_meta: bool,
    /// Show the global contextual help?
    show_contextual_help: bool,
}

impl Default for AppConfiguration {
    fn default() -> Self {
        Self {
            enable_global_sub_screen_quit_shortcut: ENABLE_GLOBAL_SUB_SCREEN_QUIT_SHORTCUT_DEFAULT,
            display_comments_panel_by_default: DISPLAY_COMMENTS_PANEL_BY_DEFAULT_DEFAULT,
            display_main_items_list_item_meta: DISPLAY_MAIN_ITEMS_LIST_ITEM_META,
            show_contextual_help: SHOW_CONTEXTUAL_HELP_DEFAULT,
        }
    }
}

/// Intermediate structure used solely for deserialization.
///
/// This is needed due to potentially missing values in the TOML configuration,
/// for instance when adding a new configuration option.
#[derive(Debug, Deserialize)]
struct DeserializableAppConfiguration {
    enable_global_sub_screen_quit_shortcut: Option<bool>,
    display_comments_panel_by_default: Option<bool>,
    display_main_items_list_item_meta: Option<bool>,
    show_contextual_help: Option<bool>,
}

impl AppConfiguration {
    pub fn from_file_or_defaults() -> Self {
        match Self::from_file_or_environment() {
            Ok(config) => config,
            Err(why) => {
                warn!("AppConfiguration loading error, using defaults. {}", why);
                Self::default()
            }
        }
    }

    pub fn get_enable_global_sub_screen_quit_shortcut(&self) -> bool {
        self.enable_global_sub_screen_quit_shortcut
    }

    pub fn toggle_enable_global_sub_screen_quit_shortcut(&mut self) {
        self.enable_global_sub_screen_quit_shortcut = !self.enable_global_sub_screen_quit_shortcut;
        self.save_to_file_warn_if_fail();
    }

    pub fn get_display_comments_panel_by_default(&self) -> bool {
        self.display_comments_panel_by_default
    }

    pub fn toggle_display_comments_panel_by_default(&mut self) {
        self.display_comments_panel_by_default = !self.display_comments_panel_by_default;
        self.save_to_file_warn_if_fail();
    }

    pub fn get_display_main_items_list_item_meta(&self) -> bool {
        self.display_main_items_list_item_meta
    }

    pub fn toggle_display_main_items_list_item_meta(&mut self) {
        self.display_main_items_list_item_meta = !self.display_main_items_list_item_meta;
        self.save_to_file_warn_if_fail();
    }

    pub fn get_show_contextual_help(&self) -> bool {
        self.show_contextual_help
    }

    pub fn toggle_show_contextual_help(&mut self) {
        self.show_contextual_help = !self.show_contextual_help;
        self.save_to_file_warn_if_fail();
    }

    fn save_to_file(&self) -> Result<()> {
        let config_filepath = Self::get_config_file_path()?;
        let config_directory = config_filepath
            .parent()
            .expect("AppConfiguration.save_to_file: config filepath parent folder can be read");
        create_dir_all(config_directory).map_err(|err| {
            HnCliError::ConfigSynchronizationError(format!(
                "cannot create config directory ({}): {}",
                config_directory.display(),
                err
            ))
        })?;

        let config_raw = toml::to_string(self).map_err(|err| {
            HnCliError::ConfigSynchronizationError(format!("cannot serialize config: {}", err))
        })?;

        write(&config_filepath, config_raw).map_err(|err| {
            HnCliError::ConfigSynchronizationError(format!(
                "cannot save config file ({}): {}",
                config_filepath.display(),
                err
            ))
        })
    }

    fn save_to_file_warn_if_fail(&self) {
        match self.save_to_file() {
            Ok(()) => (),
            Err(why) => {
                warn!("AppConfiguration saving error, skipping. {}", why);
            }
        }
    }

    fn from_file_or_environment() -> Result<Self> {
        let config_filepath = Self::get_config_file_path()?;

        let load_defaults_and_save = || {
            let default_config = Self::default();
            if let Err(why) = default_config.save_to_file() {
                warn!("{}", why);
            }
            Ok(default_config)
        };

        // File existence/permissions check
        match config_filepath.try_exists().map_err(|err| {
            HnCliError::ConfigSynchronizationError(format!(
                "cannot check if config file ({}) exists: {}",
                config_filepath.display(),
                err
            ))
        }) {
            Err(why) => {
                warn!("{}", why);
                return load_defaults_and_save();
            }
            Ok(exists) => {
                if !exists {
                    warn!(
                        "config file ({}) does not exist yet",
                        config_filepath.display()
                    );
                    return load_defaults_and_save();
                }
            }
        }

        // Read
        let config_raw = match read_to_string(&config_filepath).map_err(|err| {
            HnCliError::ConfigSynchronizationError(format!(
                "cannot open config file ({}): {}",
                config_filepath.display(),
                err
            ))
        }) {
            Ok(raw) => raw,
            Err(why) => {
                warn!("{}", why);
                return Ok(Self::default());
            }
        };

        // Deserialize
        let deserializable_config: DeserializableAppConfiguration =
            match toml::from_str(&config_raw).map_err(|err| {
                HnCliError::ConfigSynchronizationError(format!(
                    "cannot deserialize config: {}",
                    err
                ))
            }) {
                Ok(read_config) => read_config,
                Err(why) => {
                    warn!("{}", why);
                    return Ok(Self::default());
                }
            };

        Ok(Self {
            enable_global_sub_screen_quit_shortcut: deserializable_config
                .enable_global_sub_screen_quit_shortcut
                .unwrap_or(ENABLE_GLOBAL_SUB_SCREEN_QUIT_SHORTCUT_DEFAULT),
            display_comments_panel_by_default: deserializable_config
                .display_comments_panel_by_default
                .unwrap_or(DISPLAY_COMMENTS_PANEL_BY_DEFAULT_DEFAULT),
            display_main_items_list_item_meta: deserializable_config
                .display_main_items_list_item_meta
                .unwrap_or(DISPLAY_MAIN_ITEMS_LIST_ITEM_META),
            show_contextual_help: deserializable_config
                .show_contextual_help
                .unwrap_or(SHOW_CONTEXTUAL_HELP_DEFAULT),
        })
    }

    fn get_config_file_path() -> Result<PathBuf> {
        let project_os_directory = get_project_os_directory()?;
        Ok(project_os_directory.join("hncli.toml"))
    }
}

pub fn get_project_os_directory() -> Result<PathBuf> {
    let project_directories = ProjectDirs::from("", "pierreyoda", "hncli").ok_or_else(|| {
        HnCliError::ConfigSynchronizationError("cannot get hncli config directory from OS".into())
    })?;
    Ok(project_directories.config_dir().to_path_buf())
}
