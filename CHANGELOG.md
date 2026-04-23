# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- New architecture based on metaphor: "The Workshop"
  - `contracts/`: DTOs and traits (the blueprints)
  - `warehouse/`: External tools wrappers (serde_json, etc.)
  - `workshop/`: Specialized workers (Timer, Settings, Session, JSON)
  - `desk/`: Orchestrator coordinating all workers
- JsonWorker for handling serialization/deserialization
- Comprehensive test suite (11 tests passing)

### Changed
- **BREAKING**: Complete architecture refactoring
  - Removed hexagonal architecture (domain/ports)
  - Implemented first draft of the Mossy Archi (yay): contracts → warehouse → workshop → desk
  - Workers are now concrete types (removed unnecessary traits)
  - Only external tools use traits (JsonTrait, NotifierTrait, etc.)
- Simplified generic implementations with proper HRTB patterns
- Internal domain models moved to `workshop/models/`

### Removed
- `domain/` folder (moved to `workshop/models/`)
- `ports/` folder (replaced by `contracts/` traits)
- Unnecessary worker traits (TimerWorkerTrait, SettingsWorkerTrait, SessionWorkerTrait)
- Unused `from_value` method in JSON tool

## [0.2.0] - 2026-03-16

### Added
- Complete CI/CD configuration with GitHub Actions
- Automated multi-platform testing (Linux, Windows, macOS)
- Automatic release pipeline with cross-platform builds
- Security analysis and dependency auditing
- Code quality metrics and coverage
- cargo-deny configuration for license management

### Changed
- Project structure organized according to hexagonal architecture

### Fixed
- Tauri configuration for production builds

---

## Types of changes

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.