//! Platform-agnistic standard directory locations.
//!
//! [`xdir`][xdir] is an minimal and opinionated library for retrieving
//! _platform-agnostic_ XDG-compliant standard locations of directories.
//! Notably, this crate will always yield the same locations for subdirectories
//! of the user's home regardless of platform-specific conventions, which is
//! often more consistent with what users would expect from a command-line
//! application.
//!
//! If you are building an application that strictly follows the platform's
//! conventions, the [`dirs`][dirs] crate provides _platform-**specific**_
//! locations.
//!
//! # Support
//!
//! |  Directory  |    Environment     |        Default       |
//! |-------------|--------------------|----------------------|
//! | [`fn@home`] | `$HOME`            | Platform-specific    |
//! | [`cache`]   | `$XDG_CACHE_HOME`  | `$HOME/.cache`       |
//! | [`config`]  | `$XDG_CONFIG_HOME` | `$HOME/.config`      |
//! | [`bin`]     | `$XDG_BIN_HOME`    | `$HOME/.local/bin`   |
//! | [`data`]    | `$XDG_DATA_HOME`   | `$HOME/.local/share` |
//! | [`state`]   | `$XDG_STATE_HOME`  | `$HOME/.local/state` |
//! | [`runtime`] | `$XDG_RUNTIME_DIR` |                      |
//!
//! # Usage
//!
//! Calling the corresponding function will return the standard location if it
//! can be detected. If the path could not be found (most likely due to
//! misconfiguration of the environment), the function call will fail.
//!
//! <!-- Reference-style links -->
//! [dirs]: https://docs.rs/dirs/latest/dirs/
//! [xdir]: https://docs.rs/xdir/latest/xdir/

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
