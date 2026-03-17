# A Way Too Overengineered Pomodoro Timer

An intentionally overengineered Pomodoro timer.

The business logic is simple (start, pause, resume, reset, tick), but the goal of this project is not complexity in features. The goal is to practice writing software with the best engineering standards I can apply today: architecture, code quality, CI/CD, repository hygiene, and release discipline.

## Why This Project Exists

This is a deliberate learning project with a clear constraint:

- Keep product logic trivial.
- Treat everything else like a production codebase.


## Project Goals

- Apply clean architecture / hexagonal thinking on a tiny domain.
- Keep boundaries explicit between domain, inbound ports, and outbound ports.
- Maintain strong quality gates (formatting, linting, tests, builds).
- Run robust CI/CD across platforms (Linux, Windows, macOS).
- Build repeatable release workflows and repository practices.

## Tech Stack

- Desktop app framework: Tauri v2
- Frontend: SvelteKit + TypeScript
- Backend/core: Rust
- Automation: GitHub Actions

## Architecture Overview

Rust code is organized around ports and domain responsibilities:

- `AWTOPT/src-tauri/src/domain/`: core business entities and rules.
- `AWTOPT/src-tauri/src/ports/inbound/`: use-case interfaces and orchestration.
- `AWTOPT/src-tauri/src/ports/outbound/`: abstractions for side effects (notify, save, schedule).

The point is to keep the core logic isolated and testable while infrastructure concerns stay behind interfaces.

## Repository Structure

```text
AWTOPT/
	src/                 # Svelte frontend
	src-tauri/           # Rust + Tauri app core
scripts/
	quickie.sh           # Fast local checks
	pre-push-check.sh    # Full local validation
	security-check.sh    # Security scans
.github/workflows/
	ci.yml               # Main CI
	quality.yml          # Quality metrics and analysis
	release.yml          # Tagged releases
```

## Getting Started

### Prerequisites

- Node.js 20+
- npm
- Rust (stable toolchain)
- Tauri build dependencies for your OS (Linux needs GTK/WebKit packages)

### Install

```bash
cd AWTOPT
npm ci
```

### Run in Development

```bash
cd AWTOPT
npm run tauri dev
```

### Build

```bash
cd AWTOPT
npm run tauri build
```

## Local Quality Checks

Quick checks:

```bash
./scripts/quickie.sh
```

Full pre-push checks:

```bash
./scripts/pre-push-check.sh
```

Security checks:

```bash
./scripts/security-check.sh
```

## CI/CD

The project uses three workflows:

- `ci.yml`: frontend + backend checks, integration build, and security reporting.
- `quality.yml`: coverage, benchmarks, dependency analysis, quality metrics.
- `release.yml`: tag-driven, cross-platform release artifacts.

Linux Tauri dependencies are centralized in:

- `.github/actions/install-tauri-linux-deps/action.yml`

This avoids drift across workflows when Ubuntu/Tauri requirements change.

For additional CI/CD details, see `.github/README.md`.

## Release Flow

1. Update `CHANGELOG.md`.
2. Create a version tag:

```bash
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

3. Let GitHub Actions build and publish release artifacts.

## Current Mindset

This repository is a snapshot of my current engineering level. The idea is to keep shipping improvements over time and raise the quality bar each iteration.

If something looks "too much" for a Pomodoro timer, that is the point.

