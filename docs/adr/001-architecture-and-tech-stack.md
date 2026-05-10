# ADR 001: Architecture, Technology Stack, and UX Philosophy

## Status
Approved

## Context
The project aims to build a purely local, offline system daemon and application to diagnose and optimize Linux gaming performance (addressing issues like thin-and-light laptop power throttling and Proton translation layer inefficiencies). 

The tool must satisfy two distinct user personas:
1.  **The Average Gamer:** Wants a "one-click" solution to improve framerates without understanding kernel mechanics.
2.  **The Power User:** Needs surgical control over system hardware (`sysfs`, PL1/PL2 power limits) and the ability to define complex, conditional execution logic (Node Trees/Pipelines).

Furthermore, the application must be highly performant, demanding zero overhead while a game is running, precluding the use of heavy runtime environments like Electron or standalone Node.js daemons.

## Decision

We will adopt the following architecture and technology stack:

### 1. Application Framework: Tauri v2
We will use Tauri to bridge a high-performance native backend with a modern, complex frontend.
*   **Backend (Rust):** All system-level interactions (monitoring PIDs, reading/writing to `/sys/`, executing bash scripts/plugins) will be written in Rust. Rust guarantees memory safety, predictable performance (zero garbage collection pauses), and a microscopic memory footprint.
*   **Frontend (React/Next.js via TypeScript):** The UI will be built adhering to strict `DESIGN_SYSTEM.md` constraints. It will be rendered via the operating system's native webview, ensuring a rich graphical experience (charts, node editors) without the Chromium bloat of Electron.

### 2. Local Telemetry Storage: SQLite
We will embed SQLite within the Rust backend.
*   It requires zero external dependencies or server setup.
*   It will store localized benchmarking telemetry (frametimes, thermals, CPU/GPU loads) per gaming session to power the "Diagnostics Lab" module.

### 3. UX Paradigm: Progressive Disclosure
The UI will strictly separate consumer and creator contexts:
*   **Simple Mode:** A clean interface showing installed games with a simple `[Optimize: ON/OFF]` toggle that applies safe, default profiles.
*   **Pro Mode (The Lab):** An advanced view exposing the visual Logic Pipeline builder and the telemetry analysis charts.

### 4. Data Structure: Serializable Presets
To support the future roadmap feature of a **Community Preset Repository**, all logic pipelines, node trees, and optimization profiles must be designed as highly portable, serializable data structures (e.g., structured JSON). This ensures that a power user can design a complex optimization profile for "Warframe on Asus Zephyrus" and seamlessly export/share it with average users.

## Consequences
*   **Positive:** We achieve native C-like performance with modern Web UI capabilities.
*   **Positive:** The serializable preset architecture paves the way for community-driven configuration sharing.
*   **Negative/Constraint:** Maintaining two distinct codebases (Rust backend, TypeScript frontend) increases cognitive load and requires strict interface contracts (IPC - Inter-Process Communication) between the two layers.
