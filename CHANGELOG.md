# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

## [0.2.3](https://github.com/bmblb3/autofoam/compare/v0.2.2...v0.2.3) - 2025-08-25

### Fixed

- fix release asset creation and tagging
- add autofoam-scalar-deviation binary to gh actions

## [0.2.2](https://github.com/bmblb3/autofoam/compare/v0.2.1...v0.2.2) - 2025-08-25

### Added

- add scalar deviation CLI tool for VTP files

### Other

- add a simple description to README
- simplify gh workflow to use release_plz
    - only build for linux-x86_64, remove the prev targets in the matrix
- refacgor scalar-area-threshold into libraries, add tests

## [0.2.1] - 2025-08-20

### Fixed
- CD for multiple binaries


## [0.2.0] - 2025-08-20

### Added

- BINARY: *(autofoam-scalar-area-threshold)*
- BINARY: *(autofoam-stl-bbox)*
- LIB   : *(stl)* For stl related operations
- LIB   : *(vtk)* For vtk related operations
- LIB   : *(coordinates)* For [f32;3] related operations


## [0.1.0] - 2025-08-20

Init crate
