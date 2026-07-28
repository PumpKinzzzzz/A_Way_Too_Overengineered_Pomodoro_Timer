# A Way Too Overengineered Pomodoro Timer

You can access more about the code in the [Wiki](https://github.com/PumpKinzzzzz/A_Way_Too_Overengineered_Pomodoro_Timer/wiki) section of the repository, where I have documented the architecture and design decisions, as well as some of the challenges I faced and how I overcame them.

A "getting good at a techno" kind of project. The goal was to learn Tauri and Rust and to make a self assesment of my own engineering skills. I wanted to challenge my SOLID principles and see how well I can apply them in a project that is simple enough to be implemented in a reasonable amount of time, but complex enough to require some thought about architecture and design.

I have also tried to apply some good practices like test driven development, continuous integration, and code reviews. And of course, tried to have the GitHub repository as clean and well organized as possible, with a clear README, a good commit history, github project linked to the repository, and a good use of branches and pull requests.

# Features
- **Full Pomodoro Timer UI** — Start/pause/resume/reset with real-time countdown
- **Customizable Durations** — Configure work, short break, and long break lengths
- **Auto-Start Breaks** — Automatically continue to the next phase or pause at breaks
- **Session Stats** — Track completed cycles and total time elapsed
- **Skip Phases** — Jump to the next phase without waiting
- **Persistent State** — Settings and session stats saved across sessions
- **Desktop Notifications** — OS notifications on phase completion
- **Responsive UI** — Clean, minimal interface built with SvelteKit and MossyGrave styling

# Installation

## Prerequisites
- **Node.js** 18+ and npm (for frontend)
- **Rust** 1.70+ (for backend)
- **Tauri CLI** (auto-installed by npm during setup)

## Setup

Clone the repository and install dependencies:

```bash
git clone https://github.com/PumpKinzzzzz/A_Way_Too_Overengineered_Pomodoro_Timer.git
cd A_Way_Too_Overengineered_Pomodoro_Timer
cd AWTOPT
npm install
```

# Usage

## Quick Start

Launch the app in dev mode:

```bash
./scripts/testapp.sh
```

This runs `npm run tauri:dev` from the `AWTOPT/` directory and opens the desktop window.

## Available Scripts

From the repository root, the following helper scripts are available:

### `./scripts/testapp.sh`
Launches the Pomodoro timer in development mode.
```bash
./scripts/testapp.sh
```

### `./scripts/quickie.sh`
Runs format check and clippy on both frontend and backend—useful during local iteration.
```bash
./scripts/quickie.sh
```

### `./scripts/pre-push-check.sh`
Runs the full CI gate (frontend check/build, backend fmt/clippy/test/build). Run this before pushing to verify your changes pass CI.
```bash
./scripts/pre-push-check.sh
```

### `./scripts/security-check.sh`
Runs npm audit, cargo audit, and cargo deny to check for known vulnerabilities.
```bash
./scripts/security-check.sh
```

## Manual Commands

### Frontend (from `AWTOPT/`)
```bash
npm run dev              # SvelteKit dev server (used by tauri:dev)
npm run tauri:dev        # Launch Tauri app in dev mode (same as testapp.sh)
npm run check            # TypeScript type-check
npm run format           # Format with Prettier and Svelte-check
npm run build            # Production build
```

### Backend (from `AWTOPT/src-tauri/`)
```bash
cargo test --all-features                    # Run all tests
cargo test <test_name>                       # Run a specific test
cargo fmt --all -- --check                   # Format check
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo build --release                        # Production build
```

## Using the Timer

1. **Start** — Click the Start button to begin a Work phase
2. **Pause/Resume** — Pause at any time; Resume to continue
3. **Skip** — Jump to the next phase (Work → Break or Break → Work)
4. **Reset** — Return to Idle state
5. **Settings** — Click the gear icon to adjust durations and auto-start breaks behavior
6. **Stats** — Click the Stats tab to view session summary

### Settings
- **Work Duration** — Length of focus periods (default 25 min)
- **Short Break** — Length of short breaks (default 5 min)
- **Long Break** — Length of long breaks (default 15 min)
- **Auto-Start Breaks** — When OFF, timer pauses at the start of each break and waits for you to press Start manually

Changing settings resets the timer to Idle, so you can start a fresh session with the new durations.

# Contributing
If you want to contribute to the project, feel free to open a pull request or an issue. I welcome any contributions, whether it's a bug fix, a new feature, or just some feedback.

