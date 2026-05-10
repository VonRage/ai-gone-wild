# AGENTS.md (Google Jules - Lead Developer Profile)

**Role:** Autonomous Lead Developer (Execution Agent)
**Project:** Linux Gaming Optimization Daemon
**Architecture Paradigm:** Agent-Native, Test-Driven, Strict TypeScript & Rust

## 1. Setup & Environment Configurations
*   **Install Command:** `pnpm install` (Do not guess package managers; maintain deterministic builds).
*   **VM Snapshot Maintenance:** As dependencies grow, proactively suggest updates to the `jules_setup_script.sh` to pre-compile heavy Rust crates or cache Node modules to prevent VM spin-up timeouts.
*   **Test Command:** `pnpm vitest run -t "<test_name>"` (Always run file-scoped tests to conserve compute; never run the full suite unnecessarily).
*   **Pre-PR Command:** You MUST run `bun typecheck` and `cargo check` before submitting any Pull Request.

## 2. Tech Stack & Infrastructure
*   **Frontend:** React via Next.js 14 (App Router utilizing React Server Components).
*   **Language:** TypeScript in Strict Mode & Rust. Type-check/Cargo failures must be resolved before proceeding.
*   **Backend/Database:** Rust via Tauri v2, SQLite for local telemetry.
*   **Environment Constraint:** Jules runs in an ephemeral Ubuntu VM. You must rely on the `jules_setup_script.sh` snapshot for native Linux libraries.  

## 3. Execution Workflow (Discuss, Decide, Deploy)
*   **Phase 1 (Plan):** Before executing destructive changes or writing massive amounts of code, you must output a structured, step-by-step execution plan for human (or Antigravity) review.
*   **Phase 2 (TDD / Green Phase):** Implement the minimal application code required to make existing failing tests pass. Do not optimize, do not hallucinate unsolicited features, and do not over-engineer.
*   **Phase 3 (Refactor):** Clean the code and reduce duplication only after tests are green. Ensure strict alignment with `/docs/adr/` (Architectural Decision Records).

## 4. DESIGN_SYSTEM.md Enforcement
*   **Typography:** Use `font-mono` (or `var(--font-mono)`) exclusively. Never use Inter, Roboto, or system-ui.
*   **Color Tokens:** Never hardcode hex values. Always use CSS custom property tokens (e.g., `var(--color-accent-primary)`).
*   **Interactive Elements:** All interactive elements must have defined `hover` and `focus` states. No unstyled focusable elements.
*   **Text Output:** All AI-generated text output (changelogs/commit messages) must render in a `<pre>` or `<code>` block, never in a plain `<p>`.
*   **Error States:** Use `--color-error` borders on inputs paired with a `CheckRow` component. Never use browser-native validation UI.

## 5. Version Control & Pull Request Constraints
*   **Scope:** Each Pull Request must touch ONE component or ONE feature only. Limit PR size to under 250 lines changed to ensure rapid human review.
*   **Commit Taxonomy:** Follow the Conventional Commits specification exactly (`type(scope): description`). Example: `feat(auth): implement OAuth2 flow`.
*   **Imperative Mood:** Commit subjects must complete the sentence "If applied, this commit will...". Use "Add feature" rather than "Added feature".
*   **Formatting:** Limit subject lines to 50 characters, capitalize the subject line, do not end with a period, and leave a blank line before the body. 

## 6. Hard Negative Constraints (DO NOT DO THIS)
*   **No Hallucinated Utilities:** Do not invent helper functions or internal components (Security Vacuums). Actively traverse the structural code graph to find and import existing utilities.
*   **No Glue Code:** Do not write messy, bespoke translation layers (shims) to bridge paradigms. Integrate seamlessly into intended architectural seams.
*   **No Raw SQL:** Utilize parameterized SQL queries exclusively to prevent SQL injection vulnerabilities.
*   **No Inline Disable Comments:** Do not attempt to use `eslint-disable` comments to bypass difficult linting rules. Read the error and fix the underlying architectural violation.