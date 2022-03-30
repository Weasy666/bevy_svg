# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0] - 2022-04-21
### Added
- Added missing origins `BottomLeft`, `BottomRight`, `TopRight`

### Changed
- Tessellation of a SVG file will now happen on load in the `AssetLoader`
- Use [`copyless`](https://crates.io/crates/copyless) to avoid unnecessary allocations.
- Update mesh vertex buffer layout. This includes changing vertex color from `[f32, 4]` to `u32`.
- The origin will now not be applied to the transform in the `app world`, which could mess with the transform of childrens. It will instead be applied to the transform in the `render world`, which doesn't have the concept of a transform hierarchy.

### Fixed
- Typos in `README.md`

## [0.6.0] - 2022-01-09
### Added
- Added features `2d` and `3d`, both are activae on default, but can be used separately if needed.

### Changed
- Rendering now uses the new renderer introduced in Bevy `0.6`
- Now using `WGSL` instead of `GLSL` shaders
- `SvgBundle` is replaced by `Svg2dBundle` and `Svg3dBundle`
- Updated `usvg` to version `0.20`

## [0.5.0]
Skipped this version number to be in sync with bevy.

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
