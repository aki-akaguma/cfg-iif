# Changelog: cfg-iif

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.9] - 2026-05-12
### Added
- Support for `else if` chains in `cfg_iif!` macro.
- Support for multiple `cfg` predicates in `cfg_iif!` macro (implicitly combined with `all()`).
- More examples and tests for new features.
- Verification of `no_std` compatibility by building for bare-metal targets (`thumbv7m-none-eabi`).

### Changed
- Refactor `cfg_iif!` macro internals to use a normalized recursive structure, reducing code duplication and improving maintainability.

### Fixed
- Bug where multiple `cfg` predicates caused a compilation error in `else` branches.

## [0.2.8] - 2025-09-24
### Added
- Specifications in `specs` directory.
- More tests.

### Changed
- Update `rust-version` to `1.60.0`.

## [0.2.7] - 2024-06-19
### Changed
- Rewrite `cfg(Unix)` to `cfg(target_family = "unix")`.

### Fixed
- Clippy warning: `unexpected_cfgs`.

## [0.2.6] - 2023-02-12
### Added
- GitHub Actions workflows: `.github/workflows/test-ubuntu.yml`, `.github/workflows/test-macos.yml`, and `.github/workflows/test-windows.yml`.
- Test status badges in `README.tpl`.

### Changed
- Refactor `Makefile`.

### Removed
- `COPYING` file.

### Fixed
- Licenses: `LICENSE-APACHE` and `LICENSE-MIT`.

## [0.2.5] - 2023-01-28
### Added
- GitHub Actions workflow: `.github/workflows/test.yml`.
- Test status badges in `README.tpl`.

### Fixed
- Makefile: update `rustc` version from `1.66.0` to `1.66.1`.

## [0.2.4] - 2023-01-10
### Added
- `rust-version = "1.56.0"` in `Cargo.toml`.
- `all-test-version` target in `Makefile`.
- Badges in `README.tpl`.

### Changed
- Reformat `CHANGELOG.md`.

## [0.2.3] - 2022-06-13
### Changed
- Switch to edition 2021.

## [0.2.2] - 2021-11-14
### Added
- More documentation.

## [0.2.1] - 2021-07-01
### Fixed
- License.

## [0.2.0] - 2021-06-26
### Changed
- Initial release on GitHub.

## [0.1.4] - 2021-06-26
### Changed
- Move repository to GitHub.

## [0.1.3] - 2021-06-26
### Added
- Documentation, `README.tpl`, and `Makefile`.

## [0.1.2] - 2020-11-18
### Added
- `README.md`, `COPYING`, `LICENSE-APACHE`, and `LICENSE-MIT`.

## [0.1.1] - 2020-05-10
### Added
- "No cfg keyword" syntax.

### Changed
- Switch from edition 2015 to 2018.

### Fixed
- Deprecation issue from `rustc` 1.42.0.

## [0.1.0] - 2018-06-13
### Added
- Initial commit.

[unreleased]: https://github.com/aki-akaguma/cfg-iif/compare/v0.2.9..HEAD
[0.2.9]: https://github.com/aki-akaguma/cfg-iif/compare/v0.2.8..v0.2.9
[0.2.8]: https://github.com/aki-akaguma/cfg-iif/compare/v0.2.7..v0.2.8
[0.2.7]: https://github.com/aki-akaguma/cfg-iif/compare/v0.2.6..v0.2.7
[0.2.6]: https://github.com/aki-akaguma/cfg-iif/compare/v0.2.5..v0.2.6
[0.2.5]: https://github.com/aki-akaguma/cfg-iif/compare/v0.2.4..v0.2.5
[0.2.4]: https://github.com/aki-akaguma/cfg-iif/compare/v0.2.3..v0.2.4
[0.2.3]: https://github.com/aki-akaguma/cfg-iif/compare/v0.2.2..v0.2.3
[0.2.2]: https://github.com/aki-akaguma/cfg-iif/compare/v0.2.1..v0.2.2
[0.2.1]: https://github.com/aki-akaguma/cfg-iif/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/cfg-iif/compare/v0.1.3..v0.2.0
[0.1.3]: https://github.com/aki-akaguma/cfg-iif/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/cfg-iif/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/cfg-iif/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/cfg-iif/releases/tag/v0.1.0
