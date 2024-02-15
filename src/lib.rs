//! Platform-agnistic standard directory locations.
//!
//! `xdir` is a minimal and opinionated library for retrieving
//! _platform-agnostic_ XDG-compliant standard locations of directories.
//!
//! # Usage
//!
//! Calling a directory's corresponding function will return its standard
//! location if it can be detected. If the path could not be found (most likely
//! due to misconfiguration of environment variables), the function call will
//! fail, returning [`None`].
//!
//! ## Directories
//!
//! |  Function   |    Environment     |        Default       |
//! |-------------|--------------------|----------------------|
//! | [`fn@home`] | `$HOME`            | Platform-specific    |
//! | [`cache`]   | `$XDG_CACHE_HOME`  | `$HOME/.cache`       |
//! | [`config`]  | `$XDG_CONFIG_HOME` | `$HOME/.config`      |
//! | [`bin`]     | `$XDG_BIN_HOME`    | `$HOME/.local/bin`   |
//! | [`data`]    | `$XDG_DATA_HOME`   | `$HOME/.local/share` |
//! | [`state`]   | `$XDG_STATE_HOME`  | `$HOME/.local/state` |
//! | [`runtime`] | `$XDG_RUNTIME_DIR` | None                 |
//!
//! ## Examples
//!
//! To get the configuration file of an application:
//!
//! ```
//! # use std::path::PathBuf;
//! #
//! fn config() -> PathBuf {
//!     xdir::config()
//!         // Append the application name to the path to avoid cluttering the
//!         // general config directory.
//!         .map(|path| path.join("myapp"))
//!         // If the standard path could not be found (e.g.`$HOME` is not set),
//!         // default to the current directory.
//!         .unwrap_or_default()
//!         // Finally, append the config file to the directory path.
//!         .join("config.toml")
//! }
//! ```

#![warn(clippy::pedantic)]

use std::env;
use std::path::PathBuf;

pub use home::home_dir as home;

macro_rules! path {
    ($var:tt, $dir:tt) => {
        env::var($var)
            .into_iter()
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
            .next()
            .or_else(|| home().map(|path| path.join($dir)))
    };
}

/// Returns the path to the user's executable directory.
pub fn bin() -> Option<PathBuf> {
    path!("XDG_BIN_HOME", ".local/bin")
}

/// Returns the path to the user's cache directory.
pub fn cache() -> Option<PathBuf> {
    path!("XDG_CACHE_HOME", ".cache")
}

/// Returns the path to the user's config directory.
pub fn config() -> Option<PathBuf> {
    path!("XDG_CONFIG_HOME", ".config")
}

/// Returns the path to the user's data directory.
pub fn data() -> Option<PathBuf> {
    path!("XDG_DATA_HOME", ".local/share")
}

/// Returns the path to the user's runtime directory.
pub fn runtime() -> Option<PathBuf> {
    env::var("XDG_RUNTIME_DIR").map(PathBuf::from).ok()
}

/// Returns the path to the user's state directory.
pub fn state() -> Option<PathBuf> {
    path!("XDG_STATE_HOME", ".local/state")
}
