# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2021-12-20
### Added
- Implement AssetLoader for SVG
- Support for displaying SVGs with 3D cameras
- New 3D examples
- This file! 🚀

### Changed
- Refactored and changed some internal, like how an when the different `y`-axis origin gets changed.

### Fixed
- Fix problem with drawing a SVG multiple times
- Fix washed out colors

### Removed
- Ability to load a `SVG` file directly from the file system, you now need to use `asset_server.load("path/to/file.svg")`
