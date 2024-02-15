# xdir

[![latest version][version.badge]][version.hyper]
[![dependency status][deps.badge]][deps.hyper]
[![documentation][docs.badge]][docs.hyper]
[![license][license.badge]](#license)

## About

This crate provides a minimal and opinionated library for retrieving
_platform-agnostic_ XDG-compliant standard locations of directories. Notably,
this crate will always yield the same locations for subdirectories of the user's
home regardless of platform-specific conventions, which is often more consistent
with what users would expect from a command-line application.

If you are building an application that strictly follows the platform's
conventions, the [`dirs`][dirs] crate provides _platform-**specific**_
locations.

## Usage

`xdir` provides a minimal [API][docs.hyper] to return the standard location of a
given directory. This allows users to configure where they would like certain
files to be stored using environment variables as per the [XDG Base Directory
Specification][xdg].

For example, to get the configuration file of an application:

```rust
fn config() -> PathBuf {
    xdir::config()
        // Append the application name to the path to avoid cluttering the
        // general config directory.
        .map(|path| path.join("myapp"))
        // If the standard path could not be found (e.g.`$HOME` is not set),
        // default to the current directory.
        .unwrap_or_default()
        // Finally, append the config file to the directory path.
        .join("config.toml")
}
```

## License

This project is dual-licensed under both [MIT License](./LICENSE-MIT) and
[Apache License 2.0](./LICENSE-APACHE). You have permission to use this code
under the conditions of either license pursuant to the rights granted by the
chosen license.

<!--
  Reference-style links
-->

<!-- Badges -->
[deps.badge]:    https://deps.rs/repo/github/kaplanz/xdir/status.svg
[deps.hyper]:    https://deps.rs/repo/github/kaplanz/xdir
[docs.badge]:    https://docs.rs/xdir/badge.svg
[docs.hyper]:    https://docs.rs/xdir
[license.badge]: https://img.shields.io/crates/l/xdir.svg
[version.badge]: https://img.shields.io/crates/v/xdir.svg
[version.hyper]: https://crates.io/crates/xdir

<!-- About -->
[dirs]: https://docs.rs/dirs

<!-- Usage -->
[xdg]:  https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
