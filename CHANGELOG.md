# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.0] - 2024-07-28
### Changed
- Update bevy to `0.14` ([#41](https://github.com/Weasy666/bevy_svg/pull/41))
- Update usvg to `0.42` and svgtypes to `0.15` ([#41](https://github.com/Weasy666/bevy_svg/pull/41))

## [0.12.0] - 2023-11-05
### Changed
- Update bevy to `0.12` ([#35](https://github.com/Weasy666/bevy_svg/pull/35))
### Fixed
- Fix SVG scaling issues (hopefully) (Fixes #18)

## [0.11.0] - 2023-07-12
### Changed
- Update bevy to `0.11` ([#33](https://github.com/Weasy666/bevy_svg/pull/33) Thanks @NiseVoid)
- Change `rust-version` to 1.70

## [0.10.1] - 2023-04-23
### Added
- Added a `from_bytes` function on the `Svg` struct that loads in an SVG from byte data ([#30](https://github.com/Weasy666/bevy_svg/pull/30))
- Added a `tesselate` function on the `Svg` struct that calculates the bevy mesh. ([#30](https://github.com/Weasy666/bevy_svg/pull/30))

## [0.10.0] - 2023-04-10
### Changed
- Update bevy to `0.10`
- because `GlobalTransform.translation_mut()` was made private we now replace the `GlobalTransform` of an `Svg` when its `Origin` changes.

### Removed
- custom pipelines for 2D and 3D

## [0.9.0] - 2023-01-17
### Added
- add new examples
- add common lib for examples with shared functionallity
- support for HDR rendering

### Changed
- Fix README example and also add `bevy_svg::prelude::*`
- Update bevy to `0.9`
- Add common lib to every example
- Update usvg to `0.27` and svgtypes to `0.9`
- Rendering now takes view visibility into account

### Fixed
- Change how the origin is applied (Fixes #11)
- Update mesh components for entities which have had their `Handle<Svg>` component modified (Fixes #10) thanks @tasgon

## [0.8.0] - 2022-09-23
### Changed
- Update bevy to `0.8`
- Update lyon to `1.0`

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
